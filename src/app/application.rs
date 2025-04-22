#[cfg(target_os = "macos")]
use super::macos::register_ns_application_delegate_handlers;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use super::menus::prepare_app_menu;
use super::{
    APP_SCALE_FACTOR,
    menus::{MenuEvent, MenuEventReplyMsg, menu_events},
    treeview::TreeViewMsg,
    windows::{AppWin, AppWinType, TreeWin, TreeWinMsg, window_settings},
};
use crate::{Tree, parse_newick};
use iced::{
    Element, Subscription, Task, exit,
    futures::channel::mpsc::Sender,
    keyboard::{Key, Modifiers, on_key_press},
    widget,
    window::{
        Event as WinEvent, Id as WinId, close as close_window, close_events, close_requests,
        events, gain_focus, open, open_events,
    },
};
#[cfg(debug_assertions)]
use std::path::Path;
use std::{
    collections::HashMap,
    fs::{read, write},
    path::PathBuf,
};

#[derive(Default)]
pub struct App {
    pub windows: HashMap<WinId, AppWin>,
    menu_events_sender: Option<Sender<MenuEventReplyMsg>>,
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    menu: Option<muda::Menu>,
}

#[derive(Debug)]
pub enum AppMsg {
    AppInitialized,
    TreeWinMsg(WinId, TreeWinMsg),
    MenuEvent(bool, MenuEvent),
    MenuEventsSender(Sender<MenuEventReplyMsg>),
    OpenWin(AppWinType),
    PathToOpen(WinId, PathBuf),
    PathToSave(WinId, PathBuf),
    TerminateApp,
    TerminationConfirmed,
    Win(WinId, WinEvent),
    WinCloseRequested(WinId),
    WinClosed(WinId),
    WinOpened(WinId),
    #[cfg(target_os = "windows")]
    AddMenuForHwnd(u64),
    KeysPressed(Key, Modifiers),
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
            AppMsg::KeysPressed(key, modifiers) => match modifiers {
                Modifiers::CTRL => match key {
                    Key::Character(k) => {
                        #[cfg(any(target_os = "windows", target_os = "linux"))]
                        {
                            let k: &str = k.as_str();
                            match k {
                                "o" => Task::done(AppMsg::MenuEvent(false, MenuEvent::OpenFile)),
                                "s" => Task::done(AppMsg::MenuEvent(false, MenuEvent::SaveAs)),
                                "w" => Task::done(AppMsg::MenuEvent(false, MenuEvent::CloseWindow)),
                                "q" => Task::done(AppMsg::MenuEvent(false, MenuEvent::Quit)),
                                _ => Task::none(),
                            }
                        }
                        #[cfg(target_os = "macos")]
                        {
                            println!("Ctrl + {k}");
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                },
                _ => Task::none(),
            },
            AppMsg::AppInitialized => {
                #[cfg(any(target_os = "windows", target_os = "macos"))]
                let menu = prepare_app_menu();
                #[cfg(target_os = "macos")]
                menu.init_for_nsapp();
                #[cfg(any(target_os = "windows", target_os = "macos"))]
                {
                    self.menu = Some(menu);
                }
                Task::done(AppMsg::OpenWin(AppWinType::TreeWin))
            }
            AppMsg::OpenWin(win) => open_window(self, win),
            AppMsg::Win(id, e) => match e {
                WinEvent::FileDropped(path_buf) => Task::done(AppMsg::PathToOpen(id, path_buf)),
                _ => Task::none(),
            },
            AppMsg::PathToSave(id, path_buf) => {
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

                match file_type {
                    FileType::Other(_) => Task::none(),
                    FileType::Exception => Task::none(),
                    file_type => match file_type {
                        FileType::Newick => {
                            Task::done(AppMsg::TreeWinMsg(id, TreeWinMsg::SaveNewick(id, path_buf)))
                        }
                        FileType::Nexus => Task::none(),
                        _ => Task::none(),
                    },
                }
            }
            AppMsg::PathToOpen(id, path_buf) => {
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
                        FileType::Newick => {
                            ParsedData::Tree(parse_newick(read_text_file(path_buf.clone())))
                        }
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
                                TreeWinMsg::SetTitle(String::from(
                                    path_buf
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default(),
                                )),
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
            AppMsg::MenuEvent(is_real, menu_event) => {
                if is_real {
                    let _ = menu_event_reply(self, MenuEventReplyMsg::Ack);
                }
                let id = iced::window::get_latest();

                match menu_event {
                    MenuEvent::OpenFile => id.and_then(|id| Task::future(choose_file_to_open(id))),
                    MenuEvent::SaveAs => id.and_then(|id| Task::future(choose_file_to_save(id))),
                    MenuEvent::CloseWindow => close_last_window(self),
                    MenuEvent::Quit => close_last_window(self),
                    MenuEvent::QuitInternal => Task::none(),
                    MenuEvent::Undefined(s) => {
                        id.and_then(move |id| Task::done(AppMsg::PathToOpen(id, s.clone().into())))
                    }
                }
            }
            AppMsg::TerminateApp => match menu_event_reply(self, MenuEventReplyMsg::Terminate) {
                Ok(_) => Task::none(),
                Err(_) => Task::done(AppMsg::TerminationConfirmed),
            },
            AppMsg::TreeWinMsg(id, msg) => {
                let app_task = match msg {
                    TreeWinMsg::TreeViewMsg(id, TreeViewMsg::OpenFile) => {
                        Task::future(choose_file_to_open(id))
                    }
                    TreeWinMsg::SaveNewickAck(_id, ref newick_str, ref path_buf) => {
                        write_text_file(path_buf, newick_str);
                        Task::none()
                    }
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
            #[cfg(target_os = "windows")]
            AppMsg::AddMenuForHwnd(hwnd) => {
                unsafe {
                    if let Some(menu) = &self.menu {
                        let _rslt = menu.init_for_hwnd(hwnd as isize);
                    }
                };
                Task::none()
            }
            AppMsg::WinOpened(id) => match self.windows.get(&id) {
                Some(AppWin::TreeWin(_)) => Task::none()
                    .chain({
                        #[cfg(target_os = "windows")]
                        {
                            iced::window::get_raw_id::<AppMsg>(id).map(AppMsg::AddMenuForHwnd)
                        }
                        #[cfg(target_os = "macos")]
                        {
                            Task::none()
                        }
                        #[cfg(target_os = "linux")]
                        {
                            Task::none()
                        }
                    })
                    .chain(Task::done(AppMsg::TreeWinMsg(
                        id,
                        TreeWinMsg::TreeViewMsg(id, TreeViewMsg::SetWinId(id)),
                    )))
                    .chain({
                        #[cfg(not(debug_assertions))]
                        {
                            Task::none()
                        }
                        #[cfg(debug_assertions)]
                        {
                            let path_buf = PathBuf::from("tests/data/tree02.newick");
                            let path: &Path = &path_buf.clone().into_boxed_path();
                            if path.exists() {
                                Task::done(AppMsg::PathToOpen(id, path_buf))
                            } else {
                                Task::none()
                            }
                        }
                    }),
                None => Task::none(),
            },
            AppMsg::WinCloseRequested(id) => match self.windows.get(&id) {
                Some(AppWin::TreeWin(_)) => {
                    #[cfg(any(target_os = "macos", target_os = "windows"))]
                    {
                        muda::MenuEvent::send(muda::MenuEvent {
                            id: muda::MenuId(MenuEvent::QuitInternal.to_string()),
                        });
                    }
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
        let app = Self { ..Default::default() };
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
    let menu_events = menu_events();
    let keyboard_events = on_key_press(|key, modifiers| Some(AppMsg::KeysPressed(key, modifiers)));
    // let url_events = listen_url().map(AppMsg::Url);
    // let runtime_events = listen().map(AppMsg::RuntimeEvent);
    // let raw_events = listen_raw(|e, status, id| Some(AppMsg::RawEvent(e, status, id)));

    Subscription::batch([
        open_events,
        close_requests,
        close_events,
        menu_events,
        all_window_events,
        keyboard_events,
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

async fn choose_file_to_open(id: WinId) -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick", "tre"])
        .add_filter("nexus", &["tree", "trees", "nex", "nexus"])
        .pick_file()
        .await;
    AppMsg::PathToOpen(
        id,
        match chosen {
            Some(pb) => pb.path().into(),
            None => PathBuf::new(),
        },
    )
}

async fn choose_file_to_save(id: WinId) -> AppMsg {
    let chosen = rfd::AsyncFileDialog::new()
        .add_filter("newick", &["newick", "tre"])
        .save_file()
        .await;
    AppMsg::PathToSave(
        id,
        match chosen {
            Some(pb) => pb.path().into(),
            None => PathBuf::new(),
        },
    )
}

pub fn read_text_file(path_buf: PathBuf) -> String {
    let data = read(path_buf)
        .map_err(|e| {
            eprintln!("IO error: {:?}", e);
        })
        .unwrap();
    String::from_utf8(data).unwrap()
}

pub fn write_text_file(path_buf: &PathBuf, s: &str) {
    write(path_buf, s)
        .map_err(|e| {
            eprintln!("IO error: {:?}", e);
        })
        .unwrap();
}
