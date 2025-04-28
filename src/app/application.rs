#[cfg(target_os = "macos")]
use super::macos::register_ns_application_delegate_handlers;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use super::menus::prepare_app_menu;
use super::{
    APP_SCALE_FACTOR,
    menus::{MenuEvent, MenuEventReplyMsg, menu_events},
    treeview::TreeViewMsg,
    windows::{AppWin, AppWinType, PlayWin, PlayWinMsg, TreeWin, TreeWinMsg, window_settings},
};
use crate::{Tree, parse_newick};
use iced::{
    Element, Subscription, Task, Theme, exit,
    futures::channel::mpsc::Sender,
    keyboard::{Key, Modifiers, on_key_press},
    widget,
    window::{
        Event as WinEvent, Id as WinId, close as close_window, close_events, close_requests,
        events, gain_focus, open, open_events,
    },
};
use std::{
    collections::HashMap,
    fs::{read, write},
    path::PathBuf,
};

#[derive(Default)]
pub struct App {
    win_id_app_win_map: HashMap<WinId, AppWin>,
    win_type_win_id_map: HashMap<AppWinType, WinId>,
    focused_win_id: Option<WinId>,
    menu_events_sender: Option<Sender<MenuEventReplyMsg>>,
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    menu: Option<muda::Menu>,
    menu_item_quit: Option<muda::MenuItem>,
    menu_item_close_win: Option<muda::MenuItem>,
    sel_theme_idx: usize,
    theme: Theme,
}

#[derive(Debug)]
pub enum AppMsg {
    AppInitialized,
    TreeWinMsg(TreeWinMsg),
    PlayWinMsg(PlayWinMsg),
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
    SetThemeNext,
    SetThemePrev,
    Refresh,
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
        match &self.win_id_app_win_map.get(&id) {
            Some(AppWin::TreeWin(x)) => x.view(id).map(AppMsg::TreeWinMsg),
            Some(AppWin::PlayWin(x)) => x.view(id).map(AppMsg::PlayWinMsg),
            None => widget::horizontal_space().into(),
        }
    }

    pub fn update(&mut self, app_msg: AppMsg) -> Task<AppMsg> {
        match app_msg {
            AppMsg::Refresh => Task::batch([Task::done(AppMsg::TreeWinMsg(
                TreeWinMsg::TreeViewMsg(TreeViewMsg::Refresh),
            ))]),
            AppMsg::SetThemeNext => {
                let idx = (Theme::ALL.len() - 1).min(self.sel_theme_idx + 1);
                let theme = Theme::ALL[idx].clone();
                if self.theme.extended_palette().is_dark == theme.extended_palette().is_dark {
                    self.theme = theme;
                    self.sel_theme_idx = idx;
                    Task::done(AppMsg::Refresh)
                } else if idx == Theme::ALL.len() - 1 {
                    Task::done(AppMsg::Refresh)
                } else {
                    self.sel_theme_idx = idx;
                    Task::done(AppMsg::SetThemeNext)
                }
            }
            AppMsg::SetThemePrev => {
                let idx = 0.max(self.sel_theme_idx as i32 - 1) as usize;
                let theme = Theme::ALL[idx].clone();
                if self.theme.extended_palette().is_dark == theme.extended_palette().is_dark {
                    self.theme = theme;
                    self.sel_theme_idx = idx;
                    Task::done(AppMsg::Refresh)
                } else if idx == 0 {
                    Task::done(AppMsg::Refresh)
                } else {
                    self.sel_theme_idx = idx;
                    Task::done(AppMsg::SetThemePrev)
                }
            }
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
                                "[" => Task::done(AppMsg::SetThemePrev),
                                "]" => Task::done(AppMsg::SetThemeNext),
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
                Modifiers::COMMAND => match key {
                    Key::Character(k) => {
                        #[cfg(target_os = "macos")]
                        {
                            let k: &str = k.as_str();
                            match k {
                                "[" => Task::done(AppMsg::SetThemePrev),
                                "]" => Task::done(AppMsg::SetThemeNext),
                                _ => Task::none(),
                            }
                        }
                        #[cfg(any(target_os = "windows", target_os = "linux"))]
                        {
                            println!("Cmd + {k}");
                            Task::none()
                        }
                    }
                    _ => Task::none(),
                },
                _ => Task::none(),
            },
            AppMsg::AppInitialized => {
                #[cfg(any(target_os = "windows", target_os = "macos"))]
                let (menu, menu_item_quit, menu_item_close_win) = prepare_app_menu();
                #[cfg(target_os = "macos")]
                menu.init_for_nsapp();
                #[cfg(any(target_os = "windows", target_os = "macos"))]
                {
                    self.menu = Some(menu);
                    self.menu_item_quit = Some(menu_item_quit);
                    self.menu_item_close_win = Some(menu_item_close_win);
                }
                Task::batch([
                    Task::done(AppMsg::OpenWin(AppWinType::TreeWin)),
                    // Task::done(AppMsg::OpenWin(AppWinType::PlayWin)),
                ])
            }
            AppMsg::OpenWin(win_type) => open_window(self, win_type),
            AppMsg::Win(id, e) => match e {
                WinEvent::FileDropped(path_buf) => Task::done(AppMsg::PathToOpen(id, path_buf)),
                WinEvent::Focused => {
                    self.focused_win_id = Some(id);
                    Task::none()
                }
                WinEvent::Unfocused => {
                    self.focused_win_id = None;
                    Task::none()
                }
                _ => Task::none(),
            },
            AppMsg::PathToSave(id, path_buf) => {
                if let Some(menu_item_quit) = &self.menu_item_quit {
                    menu_item_quit.set_enabled(true);
                }
                if let Some(menu_item_close_win) = &self.menu_item_close_win {
                    menu_item_close_win.set_enabled(true);
                }

                let proceed = match self.win_id_app_win_map.get_mut(&id) {
                    Some(AppWin::TreeWin(_)) => true,
                    Some(AppWin::PlayWin(_)) => true,
                    None => false,
                };

                if !proceed {
                    return Task::none();
                }

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
                            Task::done(AppMsg::TreeWinMsg(TreeWinMsg::SaveNewick(path_buf)))
                        }
                        FileType::Nexus => Task::none(),
                        _ => Task::none(),
                    },
                }
            }
            AppMsg::PathToOpen(id, path_buf) => {
                if let Some(menu_item_quit) = &self.menu_item_quit {
                    menu_item_quit.set_enabled(true);
                }
                if let Some(menu_item_close_win) = &self.menu_item_close_win {
                    menu_item_close_win.set_enabled(true);
                }
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
                        Some(tree) => match self.win_id_app_win_map.get_mut(&id) {
                            Some(AppWin::TreeWin(_)) => Task::done(AppMsg::TreeWinMsg(
                                TreeWinMsg::TreeUpdated(tree),
                            ))
                            .chain(Task::done(AppMsg::TreeWinMsg(TreeWinMsg::SetTitle(
                                String::from(
                                    path_buf
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_str()
                                        .unwrap_or_default(),
                                ),
                            )))),
                            Some(AppWin::PlayWin(_)) => Task::none(),
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

                if let Some(id) = self.focused_win_id {
                    match menu_event {
                        MenuEvent::OpenFile => {
                            if let Some(menu_item_quit) = &self.menu_item_quit {
                                menu_item_quit.set_enabled(false);
                            }
                            if let Some(menu_item_close_win) = &self.menu_item_close_win {
                                menu_item_close_win.set_enabled(false);
                            }
                            Task::future(choose_file_to_open(id))
                        }
                        MenuEvent::SaveAs => {
                            if let Some(menu_item_quit) = &self.menu_item_quit {
                                menu_item_quit.set_enabled(false);
                            }
                            if let Some(menu_item_close_win) = &self.menu_item_close_win {
                                menu_item_close_win.set_enabled(false);
                            }
                            Task::future(choose_file_to_save(id))
                        }
                        MenuEvent::CloseWindow => Task::done(AppMsg::WinCloseRequested(id)),
                        MenuEvent::Quit => {
                            let tree_win_id = self.win_type_win_id_map.get(&AppWinType::TreeWin);
                            if let Some(&id) = tree_win_id {
                                Task::done(AppMsg::WinCloseRequested(id))
                            } else {
                                Task::none()
                            }
                        }
                        MenuEvent::QuitInternal => Task::none(),
                        MenuEvent::Undefined(s) => {
                            Task::done(AppMsg::PathToOpen(id, s.clone().into()))
                        }
                    }
                } else if let Some(id) = self.win_type_win_id_map.get(&AppWinType::TreeWin) {
                    match menu_event {
                        MenuEvent::Undefined(s) => {
                            Task::done(AppMsg::PathToOpen(*id, s.clone().into()))
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            AppMsg::TerminateApp => match menu_event_reply(self, MenuEventReplyMsg::Terminate) {
                Ok(_) => Task::none(),
                Err(_) => Task::done(AppMsg::TerminationConfirmed),
            },
            AppMsg::TreeWinMsg(msg) => {
                let win_id = self.win_type_win_id_map.get(&AppWinType::TreeWin);
                let app_task = match msg {
                    TreeWinMsg::TreeViewMsg(TreeViewMsg::OpenFile) => {
                        if let Some(&id) = win_id {
                            Task::future(choose_file_to_open(id))
                        } else {
                            Task::none()
                        }
                    }
                    TreeWinMsg::SaveNewickAck(ref newick_str, ref path_buf) => {
                        write_text_file(path_buf, newick_str);
                        Task::none()
                    }
                    _ => Task::none(),
                };
                if let Some(&id) = win_id {
                    let win_task: Task<AppMsg> = match self.win_id_app_win_map.get_mut(&id) {
                        Some(AppWin::TreeWin(w)) => w.update(msg).map(AppMsg::TreeWinMsg),
                        _ => Task::none(),
                    };
                    app_task.chain(win_task)
                } else {
                    app_task
                }
            }
            AppMsg::PlayWinMsg(msg) => {
                let win_id = self.win_type_win_id_map.get(&AppWinType::TreeWin);
                let app_task = Task::none();
                if let Some(&id) = win_id {
                    let win_task: Task<AppMsg> = match self.win_id_app_win_map.get_mut(&id) {
                        Some(AppWin::PlayWin(w)) => w.update(msg).map(AppMsg::PlayWinMsg),
                        _ => Task::none(),
                    };
                    app_task.chain(win_task)
                } else {
                    app_task
                }
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
            AppMsg::WinOpened(id) => match self.win_id_app_win_map.get(&id) {
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
                    .chain(Task::done(AppMsg::TreeWinMsg(TreeWinMsg::TreeViewMsg(
                        TreeViewMsg::SetWinId(id),
                    ))))
                    .chain({
                        #[cfg(not(debug_assertions))]
                        {
                            Task::none()
                        }
                        #[cfg(debug_assertions)]
                        {
                            let path_buf = PathBuf::from("tests/data/tree01.newick");
                            let path: &std::path::Path = &path_buf.clone().into_boxed_path();
                            if path.exists() {
                                Task::done(AppMsg::PathToOpen(id, path_buf))
                            } else {
                                Task::none()
                            }
                        }
                    }),
                Some(AppWin::PlayWin(_)) => Task::none(),
                None => Task::none(),
            },
            AppMsg::WinCloseRequested(id) => match self.win_id_app_win_map.get(&id) {
                Some(AppWin::TreeWin(_)) => {
                    #[cfg(any(target_os = "macos", target_os = "windows"))]
                    {
                        muda::MenuEvent::send(muda::MenuEvent {
                            id: muda::MenuId(MenuEvent::QuitInternal.to_string()),
                        });
                    }
                    let play_win_id = self.win_type_win_id_map.get(&AppWinType::PlayWin);
                    if let Some(&play_win_id) = play_win_id {
                        close_window(play_win_id).chain(close_window(id))
                    } else {
                        close_window(id)
                    }
                }
                Some(AppWin::PlayWin(_)) => close_window(id),
                None => Task::none(),
            },
            AppMsg::WinClosed(id) => match self.win_id_app_win_map.remove(&id) {
                Some(app_win) => match app_win {
                    AppWin::TreeWin(_) => {
                        self.win_type_win_id_map.remove(&AppWinType::TreeWin);
                        Task::done(AppMsg::TerminateApp)
                    }
                    AppWin::PlayWin(_) => {
                        self.win_type_win_id_map.remove(&AppWinType::PlayWin);
                        Task::none()
                    }
                },
                None => Task::none(),
            },
        }
    }

    pub fn title(&self, id: WinId) -> String {
        match self.win_id_app_win_map.get(&id) {
            Some(AppWin::TreeWin(w)) => w.title(),
            Some(AppWin::PlayWin(w)) => w.title(),
            None => format!("{id:?}"),
        }
    }

    pub fn subscription(&self) -> Subscription<AppMsg> {
        subscriptions()
    }

    pub fn scale_factor(&self, _: WinId) -> f64 {
        APP_SCALE_FACTOR
    }

    pub fn theme(&self, _: WinId) -> Theme {
        self.theme.clone()
    }

    pub fn new() -> (Self, Task<AppMsg>) {
        let app = Self {
            sel_theme_idx: 1,
            theme: Theme::default(),
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

fn open_window(app: &mut App, app_win_type: AppWinType) -> Task<AppMsg> {
    let (win_id, task) = open(window_settings());
    let win: AppWin = match app_win_type {
        AppWinType::TreeWin => AppWin::TreeWin(TreeWin::new(win_id, &app_win_type)),
        AppWinType::PlayWin => AppWin::PlayWin(PlayWin::new(win_id, &app_win_type)),
    };
    app.win_id_app_win_map.insert(win_id, win);
    app.win_type_win_id_map.insert(app_win_type, win_id);
    task.discard().chain(gain_focus(win_id))
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
