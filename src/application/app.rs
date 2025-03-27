#[cfg(target_os = "macos")]
use super::macos::register_ns_application_delegate_handlers;
use super::windows::{AppWin, Win1, Win1Msg, Win2, Win2Msg};
use crate::{
    MenuEvent, MenuEventReplyMsg, Tree, menu_events, parse_newick, prepare_app_menu,
    window_settings,
};
use iced::Element;
use iced::Subscription;
use iced::Task;
use iced::exit;
use iced::futures::channel::mpsc::Sender;
use iced::widget;
use iced::window::Event as WindowEvent;
use iced::window::Id as WinId;
use iced::window::close as close_window;
use iced::window::gain_focus;
use iced::window::open;
use iced::window::{close_events, close_requests, events, open_events};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::runtime::Runtime as TokioRt;

#[derive(Default)]
pub struct App {
    windows: HashMap<WinId, AppWin>,
    menu_events_sender: Option<Sender<MenuEventReplyMsg>>,
    menu: Option<muda::Menu>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    AppInitialized,
    Win1Msg(Win1Msg),
    Win2Msg(Win2Msg),
    MenuEvent(MenuEvent),
    MenuEventsSender(Sender<MenuEventReplyMsg>),
    OpenWin(Win),
    Path(PathBuf),
    TerminateApp,
    TerminationConfirmed,
    Win(WinId, WindowEvent),
    WinCloseRequested(WinId),
    WinClosed(WinId),
    WinOpened(WinId),
    Nada,
}

#[derive(Debug, Clone)]
pub enum Win {
    Win1,
    Win2,
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
            Some(AppWin::Win1(x)) => x.view().map(AppMsg::Win1Msg),
            Some(AppWin::Win2(x)) => x.view().map(AppMsg::Win2Msg),
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
                Task::done(AppMsg::OpenWin(Win::Win1))
            }
            AppMsg::OpenWin(win) => open_window(self, win),
            AppMsg::Win(_id, e) => match e {
                WindowEvent::FileDropped(path_buf) => Task::done(AppMsg::Path(path_buf)),
                _ => Task::none(),
            },
            AppMsg::Path(path_buf) => {
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
                        Some(tree) => Task::done(AppMsg::Win1Msg(Win1Msg::TreeUpdated(tree)))
                            .chain(Task::done(AppMsg::Win1Msg(Win1Msg::SetTitle(
                                String::from(
                                    path_buf
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default(),
                                ),
                            )))),
                        // Some(tree) => Task::done(AppMsg::Win2Msg(Win2Msg::TreeUpdated(tree)))
                        //     .chain(Task::done(AppMsg::Win2Msg(Win2Msg::SetTitle(
                        //         String::from(
                        //             path_buf
                        //                 .file_name()
                        //                 .unwrap_or_default()
                        //                 .to_str()
                        //                 .unwrap_or_default(),
                        //         ),
                        //     )))),
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
                match menu_event {
                    MenuEvent::OpenFile => Task::future(choose_file()),
                    MenuEvent::Save => Task::none(),
                    MenuEvent::CloseWindow => close_last_window(self),
                    MenuEvent::Quit => close_last_window(self),
                    MenuEvent::QuitInternal => Task::none(),
                    MenuEvent::Undefined(s) => Task::done(AppMsg::Path(s.into())),
                }
            }
            AppMsg::TerminateApp => match menu_event_reply(self, MenuEventReplyMsg::Terminate) {
                Ok(_) => Task::none(),
                Err(_) => Task::done(AppMsg::TerminationConfirmed),
            },
            AppMsg::Win1Msg(msg) => {
                let app_task = match msg {
                    Win1Msg::OpenFile => Task::future(choose_file()),
                    _ => Task::none(),
                };

                let mut win_task: Task<AppMsg> = Task::none();
                for w in self.windows.values_mut() {
                    match w {
                        AppWin::Win1(w) => {
                            win_task = w.update(msg.clone()).map(AppMsg::Win1Msg);
                            break;
                        }
                        _ => win_task = Task::none(),
                    };
                }
                app_task.chain(win_task)
            }
            AppMsg::Win2Msg(msg) => {
                let app_task = match msg {
                    Win2Msg::OpenFile => Task::future(choose_file()),
                    _ => Task::none(),
                };

                let mut win_task: Task<AppMsg> = Task::none();
                for w in self.windows.values_mut() {
                    match w {
                        AppWin::Win2(w) => {
                            win_task = w.update(msg.clone()).map(AppMsg::Win2Msg);
                            break;
                        }
                        _ => win_task = Task::none(),
                    };
                }
                app_task.chain(win_task)
            }
            AppMsg::WinOpened(id) => match self.windows.get(&id) {
                // Some(AppWin::Win1(_)) => Task::none(),
                // Some(AppWin::Win2(_)) => Task::none(),
                Some(_) => {
                    #[cfg(not(debug_assertions))]
                    {
                        Task::none()
                    }
                    #[cfg(debug_assertions)]
                    {
                        Task::done(AppMsg::Path(PathBuf::from("tests/data/tree01.newick")))
                    }
                }
                None => Task::none(),
            },
            AppMsg::WinCloseRequested(id) => match self.windows.get(&id) {
                Some(AppWin::Win1(_)) => {
                    muda::MenuEvent::send(muda::MenuEvent {
                        id: muda::MenuId(MenuEvent::QuitInternal.to_string()),
                    });
                    close_window(id)
                }
                Some(AppWin::Win2(_)) => {
                    muda::MenuEvent::send(muda::MenuEvent {
                        id: muda::MenuId(MenuEvent::QuitInternal.to_string()),
                    });
                    close_window(id)
                }
                None => Task::none(),
            },
            AppMsg::WinClosed(id) => match self.windows.remove(&id) {
                Some(AppWin::Win1(_)) => Task::done(AppMsg::TerminateApp),
                Some(AppWin::Win2(_)) => Task::done(AppMsg::TerminateApp),
                None => Task::none(),
            },
        }
    }

    pub fn title(&self, id: WinId) -> String {
        match self.windows.get(&id) {
            Some(AppWin::Win1(w)) => w.title(),
            Some(AppWin::Win2(w)) => w.title(),
            None => format!("{id:?}"),
        }
    }

    pub fn subscription(&self) -> Subscription<AppMsg> {
        subscriptions()
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

fn open_window(app: &mut App, win: Win) -> Task<AppMsg> {
    let (win_id, task) = open(window_settings());
    let win: AppWin = match win {
        Win::Win1 => AppWin::Win1(Box::new(Win1::new())),
        Win::Win2 => AppWin::Win2(Box::new(Win2::new())),
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

async fn choose_file() -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick", "tre"])
        .add_filter("nexus", &["tree", "trees", "nex", "nexus"])
        .pick_file()
        .await;
    AppMsg::Path(match chosen {
        Some(pb) => pb.path().into(),
        None => PathBuf::new(),
    })
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
