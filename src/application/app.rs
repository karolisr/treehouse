#[cfg(target_os = "macos")]
use super::macos::register_ns_application_delegate_handlers;
use super::windows::{AppWin, AppWinType, TreeWin, TreeWinMsg};
use crate::{
    APP_SCALE_FACTOR, MenuEvent, MenuEventReplyMsg, Tree, menu_events, parse_newick,
    prepare_app_menu, window_settings,
};
use iced::{
    Element, Subscription, Task, exit,
    futures::channel::mpsc::Sender,
    widget,
    window::{
        Event as WinEvent, Id as WinId, close as close_window, close_events, close_requests,
        events, gain_focus, open, open_events,
    },
};
use std::{collections::HashMap, path::PathBuf};
use tokio::runtime::Runtime as TokioRt;

#[derive(Default)]
pub struct App {
    windows: HashMap<WinId, AppWin>,
    menu_events_sender: Option<Sender<MenuEventReplyMsg>>,
    menu: Option<muda::Menu>,
}

#[derive(Debug)]
pub enum AppMsg {
    AppInitialized,
    TreeWinMsg(WinId, TreeWinMsg),
    MenuEvent(MenuEvent),
    MenuEventsSender(Sender<MenuEventReplyMsg>),
    OpenWin(AppWinType),
    Path(WinId, PathBuf),
    TerminateApp,
    TerminationConfirmed,
    Win(WinId, WinEvent),
    WinCloseRequested(WinId),
    WinClosed(WinId),
    WinOpened(WinId),
    Nada,
}

pub enum FileType {
    Newick,
    Nexus,
    Other(String),
    Exception,
}

pub enum ParsedData {
    Tree(Option<Tree>),
    Other(String),
    Exception,
}

impl App {
    pub fn view(&self, id: WinId) -> Element<AppMsg> {
        match &self.windows.get(&id) {
            Some(AppWin::TreeWin(x)) => x.view(id).map(move |msg| AppMsg::TreeWinMsg(id, msg)),
            None => widget::horizontal_space().into(),
        }
    }

    pub fn update(&mut self, app_msg: AppMsg) -> Task<AppMsg> {
        match app_msg {
            AppMsg::Nada => Task::none(),
            AppMsg::AppInitialized => {
                let menu = prepare_app_menu();
                #[cfg(target_os = "macos")]
                menu.init_for_nsapp();
                self.menu = Some(menu);
                Task::done(AppMsg::OpenWin(AppWinType::TreeWin))
            }
            AppMsg::OpenWin(win) => open_window(self, win),
            AppMsg::Win(id, e) => match e {
                WinEvent::FileDropped(path_buf) => Task::done(AppMsg::Path(id, path_buf)),
                _ => Task::none(),
            },
            AppMsg::Path(id, path_buf) => {
                let file_type: FileType = match path_buf.extension() {
                    Some(ext_os_str) => match ext_os_str.to_str() {
                        Some(ext) => match ext {
                            "newick" | "tre" => FileType::Newick,
                            "tree" | "trees" | "nexus" | "nex" => FileType::Nexus,
                            ext => FileType::Other(ext.to_string()),
                        },
                        None => FileType::Exception,
                    },
                    None => FileType::Exception,
                };

                let parsed_data: ParsedData = match file_type {
                    FileType::Other(s) => ParsedData::Other(s),
                    FileType::Exception => ParsedData::Exception,
                    file_type => match file_type {
                        FileType::Newick => match TokioRt::new() {
                            Ok(rt) => ParsedData::Tree(parse_newick(
                                rt.block_on(async { read_text_file(path_buf.clone()).await }),
                            )),
                            Err(_) => ParsedData::Exception,
                        },
                        FileType::Nexus => ParsedData::Tree(None),
                        _ => ParsedData::Exception,
                    },
                };

                match parsed_data {
                    ParsedData::Tree(tree) => match tree {
                        Some(tree) => match self.windows.get_mut(&id) {
                            Some(AppWin::TreeWin(_)) => Task::done(AppMsg::TreeWinMsg(
                                id,
                                TreeWinMsg::TreeUpdated(id, tree),
                            ))
                            .chain(Task::done(AppMsg::TreeWinMsg(
                                id,
                                TreeWinMsg::SetTitle(
                                    id,
                                    String::from(
                                        path_buf
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_str()
                                            .unwrap_or_default(),
                                    ),
                                ),
                            ))),
                            None => Task::none(),
                        },

                        None => {
                            println!("ParsedData::Tree(None)");
                            Task::none()
                        }
                    },
                    ParsedData::Other(s) => {
                        println!("ParsedData::Other({s})");
                        Task::none()
                    }
                    ParsedData::Exception => Task::none(),
                }
            }
            AppMsg::TerminationConfirmed => exit(),
            AppMsg::MenuEventsSender(mut sender) => {
                let _ = sender.try_send(MenuEventReplyMsg::Ack);
                self.menu_events_sender = Some(sender);
                Task::none()
            }
            AppMsg::MenuEvent(menu_event) => {
                let _ = menu_event_reply(self, MenuEventReplyMsg::Ack);
                let id = iced::window::get_latest();

                match menu_event {
                    MenuEvent::OpenFile => id.and_then(|id| Task::future(choose_file(id))),
                    MenuEvent::Save => Task::none(),
                    MenuEvent::CloseWindow => close_last_window(self),
                    MenuEvent::Quit => close_last_window(self),
                    MenuEvent::QuitInternal => Task::none(),
                    MenuEvent::Undefined(s) => {
                        id.and_then(move |id| Task::done(AppMsg::Path(id, s.clone().into())))
                    }
                }
            }
            AppMsg::TerminateApp => match menu_event_reply(self, MenuEventReplyMsg::Terminate) {
                Ok(_) => Task::none(),
                Err(_) => Task::done(AppMsg::TerminationConfirmed),
            },
            AppMsg::TreeWinMsg(id, msg) => {
                let app_task = match msg {
                    TreeWinMsg::OpenFile(id) => Task::future(choose_file(id)),
                    _ => Task::none(),
                };

                let win_task: Task<AppMsg> = match self.windows.get_mut(&id) {
                    Some(AppWin::TreeWin(w)) => {
                        w.update(msg).map(move |msg| AppMsg::TreeWinMsg(id, msg))
                    }
                    None => Task::none(),
                };

                app_task.chain(win_task)
            }
            AppMsg::WinOpened(id) => match self.windows.get(&id) {
                Some(AppWin::TreeWin(_)) => {
                    #[cfg(not(debug_assertions))]
                    {
                        Task::none()
                    }
                    #[cfg(debug_assertions)]
                    {
                        Task::done(AppMsg::Path(id, PathBuf::from("tests/data/tree01.newick")))
                    }
                }
                None => Task::none(),
            },
            AppMsg::WinCloseRequested(id) => match self.windows.get(&id) {
                Some(AppWin::TreeWin(_)) => {
                    muda::MenuEvent::send(muda::MenuEvent {
                        id: muda::MenuId(MenuEvent::QuitInternal.to_string()),
                    });
                    close_window(id)
                }
                None => Task::none(),
            },
            AppMsg::WinClosed(id) => match self.windows.remove(&id) {
                Some(AppWin::TreeWin(_)) => Task::done(AppMsg::TerminateApp),
                None => Task::none(),
            },
        }
    }

    pub fn title(&self, id: WinId) -> String {
        match self.windows.get(&id) {
            Some(AppWin::TreeWin(w)) => w.title(),
            None => format!("{id:?}"),
        }
    }

    pub fn subscription(&self) -> Subscription<AppMsg> {
        subscriptions()
    }

    pub fn scale_factor(&self, _: WinId) -> f64 {
        APP_SCALE_FACTOR
    }

    pub fn new() -> (Self, Task<AppMsg>) {
        let app = Self {
            ..Default::default()
        };
        #[cfg(target_os = "macos")]
        register_ns_application_delegate_handlers();
        (app, Task::done(AppMsg::AppInitialized))
    }
}

fn menu_event_reply(app: &mut App, msg: MenuEventReplyMsg) -> Result<(), ()> {
    match app.menu_events_sender.take() {
        Some(mut sender) => {
            let _ = sender.try_send(msg);
            app.menu_events_sender = Some(sender);
            Ok(())
        }
        None => Err(()),
    }
}

fn subscriptions() -> Subscription<AppMsg> {
    let open_events = open_events().map(AppMsg::WinOpened);
    let close_requests = close_requests().map(AppMsg::WinCloseRequested);
    let close_events = close_events().map(AppMsg::WinClosed);
    let all_window_events = events().map(|(id, e)| AppMsg::Win(id, e));
    // let url_events = listen_url().map(AppMsg::Url);
    // let runtime_events = listen().map(AppMsg::RuntimeEvent);
    // let raw_events = listen_raw(|e, status, id| Some(AppMsg::RawEvent(e, status, id)));
    let menu_events = menu_events();

    Subscription::batch([
        open_events,
        close_requests,
        close_events,
        menu_events,
        all_window_events,
        // url_events,
        // runtime_events,
        // raw_events,
    ])
}

fn open_window(app: &mut App, win: AppWinType) -> Task<AppMsg> {
    let (win_id, task) = open(window_settings());
    let win: AppWin = match win {
        AppWinType::TreeWin => AppWin::TreeWin(Box::new(TreeWin::new())),
    };
    app.windows.insert(win_id, win);
    task.discard().chain(gain_focus(win_id))
}

fn close_last_window(app: &App) -> Task<AppMsg> {
    let nwin = app.windows.len();
    if nwin == 1 {
        match app.windows.keys().last() {
            Some(id) => Task::done(AppMsg::WinCloseRequested(*id)),
            None => Task::none(),
        }
    } else if nwin == 0 {
        Task::done(AppMsg::TerminateApp)
    } else {
        Task::none()
    }
}

async fn choose_file(id: WinId) -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick", "tre"])
        .add_filter("nexus", &["tree", "trees", "nex", "nexus"])
        .pick_file()
        .await;
    AppMsg::Path(
        id,
        match chosen {
            Some(pb) => pb.path().into(),
            None => PathBuf::new(),
        },
    )
}

pub async fn read_text_file(path_buf: PathBuf) -> String {
    use tokio::fs::read;
    let data = read(path_buf)
        .await
        .map_err(|e| {
            eprintln!("IO error: {:?}", e);
        })
        .unwrap();
    String::from_utf8(data).unwrap()
}
