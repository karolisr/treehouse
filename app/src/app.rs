mod consts;
mod menu;
mod ops;
mod platform;
mod win;

use consts::*;
use dendros::parse_newick;
use riced::{
    Clr, Element, Font, IcedAppSettings, Key, Modifiers, Pixels, Subscription, Task, Theme,
    WindowEvent, WindowId, close_window, exit, on_key_press, open_window, window_events,
};

use menu::{AppMenu, AppMenuItemId};
use std::path::PathBuf;
use treeview::{SidebarPosition, TreeView, TvMsg};
use win::window_settings;

pub struct App {
    winid: Option<WindowId>,
    treeview: Option<TreeView>,
    menu: Option<AppMenu>,
    title: Option<String>,
    explain: bool,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Other(Option<String>),
    MenuEvent(AppMenuItemId),
    // --------------------------------
    TvMsg(TvMsg),
    // --------------------------------
    OpenFile,
    SaveAs,
    PathToOpen(Option<PathBuf>),
    PathToSave(Option<PathBuf>),
    // --------------------------------
    AppInitialized,
    // --------------------------------
    WinEvent(WindowEvent),
    // --------------------------------
    WinOpen,
    WinOpened,
    WinCloseRequested,
    WinClose,
    WinClosed,
    Quit,
    // --------------------------------
    KeysPressed(Key, Modifiers),
    // --------------------------------
    #[cfg(target_os = "windows")]
    AddMenuForHwnd(u64),
}

pub enum FileType {
    Newick,
    Nexus,
    Other(String),
    Exception,
}

pub enum ParsedData {
    Trees(Option<Vec<dendros::Tree>>),
    Other(String),
    Exception,
}

impl App {
    fn toggle_explain(&mut self) { self.explain = !self.explain; }

    pub fn boot() -> (Self, Task<AppMsg>) {
        #[cfg(target_os = "macos")]
        platform::register_ns_application_delegate_handlers();
        (
            App { winid: None, treeview: None, menu: None, title: None, explain: false },
            Task::done(AppMsg::AppInitialized),
        )
    }

    pub fn view(&'_ self, _: WindowId) -> Element<'_, AppMsg> {
        if let Some(treeview) = &self.treeview {
            if !treeview.are_any_trees_loaded() {
                riced::container(
                    riced::btn_txt("Open a Tree File", Some(AppMsg::OpenFile))
                        .width(riced::BTN_H * 5e0),
                )
                .width(riced::Length::Fill)
                .height(riced::Length::Fill)
                .center(riced::Length::Fill)
                .into()
            } else if self.explain {
                treeview.view().explain(Clr::RED).map(AppMsg::TvMsg)
            } else {
                treeview.view().map(AppMsg::TvMsg)
            }
        } else {
            riced::container(riced::txt("App::view"))
                .width(riced::Length::Fill)
                .height(riced::Length::Fill)
                .center(riced::Length::Fill)
                .into()
        }
    }

    pub fn update(&mut self, app_msg: AppMsg) -> Task<AppMsg> {
        match app_msg {
            AppMsg::KeysPressed(key, modifiers) => {
                if modifiers.contains(Modifiers::CTRL) && modifiers.contains(Modifiers::SHIFT) {
                    match key {
                        Key::Character(k) => {
                            let k: &str = k.as_str();
                            match k {
                                "d" => {
                                    if let Some(treeview) = &mut self.treeview {
                                        treeview.toggle_draw_debug();
                                    }
                                    Task::none()
                                }
                                "e" => {
                                    self.toggle_explain();
                                    Task::none()
                                }
                                _ => Task::none(),
                            }
                        }
                        _ => Task::none(),
                    }
                } else {
                    match modifiers {
                        Modifiers::CTRL => match key {
                            Key::Character(k) => {
                                #[cfg(any(target_os = "windows", target_os = "linux"))]
                                {
                                    let k: &str = k.as_str();
                                    match k {
                                        "o" => {
                                            Task::done(AppMsg::MenuEvent(AppMenuItemId::OpenFile))
                                        }
                                        "s" => Task::done(AppMsg::MenuEvent(AppMenuItemId::SaveAs)),
                                        "w" => Task::done(AppMsg::MenuEvent(
                                            AppMenuItemId::CloseWindow,
                                        )),
                                        "q" => Task::done(AppMsg::MenuEvent(AppMenuItemId::Quit)),
                                        "[" => Task::done(AppMsg::MenuEvent(
                                            AppMenuItemId::SetSideBarPositionLeft,
                                        )),
                                        "]" => Task::done(AppMsg::MenuEvent(
                                            AppMenuItemId::SetSideBarPositionRight,
                                        )),
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
                        // Modifiers::COMMAND => match key {
                        //     Key::Character(k) => {
                        //         #[cfg(target_os = "macos")]
                        //         {
                        //             let k: &str = k.as_str();
                        //             match k {
                        //                 "f" => Task::done(AppMsg::TvMsg(TvMsg::ToggleSearchBar)),
                        //                 _ => Task::none(),
                        //             }
                        //         }
                        //         #[cfg(any(target_os = "windows", target_os = "linux"))]
                        //         {
                        //             println!("Cmd + {k}");
                        //             Task::none()
                        //         }
                        //     }
                        //     _ => Task::none(),
                        // },
                        _ => Task::none(),
                    }
                }
            }

            AppMsg::Other(opt_msg) => {
                if let Some(msg) = opt_msg {
                    println!("AppMsg::Other({msg})");
                    Task::none()
                } else {
                    Task::none()
                }
            }

            AppMsg::MenuEvent(miid) => {
                if let Some(menu) = &mut self.menu {
                    menu.update(&miid);
                }
                Task::done(miid.into())
            }

            AppMsg::TvMsg(tv_msg) => {
                if let Some(treeview) = &mut self.treeview {
                    if let Some(menu) = &mut self.menu
                        && let TvMsg::SetSidebarPos(sidebar_pos) = tv_msg
                    {
                        match sidebar_pos {
                            SidebarPosition::Left => {
                                menu.update(&AppMenuItemId::SetSideBarPositionLeft)
                            }
                            SidebarPosition::Right => {
                                menu.update(&AppMenuItemId::SetSideBarPositionRight)
                            }
                        }
                    }
                    treeview.update(tv_msg).map(AppMsg::TvMsg)
                } else {
                    Task::none()
                }
            }
            AppMsg::OpenFile => {
                if self.winid.is_none() {
                    Task::done(AppMsg::WinOpen).chain(Task::done(ops::choose_file_to_open_sync()))
                } else {
                    Task::future(ops::choose_file_to_open())
                }
            }
            AppMsg::PathToOpen(path_buf_opt) => {
                if self.winid.is_none() {
                    return Task::done(AppMsg::WinOpen)
                        .chain(Task::done(AppMsg::PathToOpen(path_buf_opt)));
                }
                if let Some(path_buf) = path_buf_opt {
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
                            FileType::Newick => ParsedData::Trees(parse_newick(
                                ops::read_text_file(path_buf.clone()),
                            )),
                            FileType::Nexus => ParsedData::Trees(None),
                            _ => ParsedData::Exception,
                        },
                    };

                    match parsed_data {
                        ParsedData::Trees(trees) => match trees {
                            Some(trees) => {
                                self.title = Some(
                                    path_buf
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string(),
                                );
                                if let Some(menu) = &mut self.menu {
                                    menu.enable(&AppMenuItemId::SaveAs);
                                    menu.enable(&AppMenuItemId::SideBarPosition);
                                    menu.enable(&AppMenuItemId::ToggleSearchBar);
                                };
                                Task::done(AppMsg::TvMsg(TvMsg::TreesLoaded(trees)))
                            }
                            None => {
                                println!("ParsedData::Trees(None)");
                                if let Some(menu) = &mut self.menu {
                                    menu.disable(&AppMenuItemId::SaveAs);
                                };
                                Task::none()
                            }
                        },
                        ParsedData::Other(s) => {
                            println!("ParsedData::Other({s})");
                            if let Some(menu) = &mut self.menu {
                                menu.disable(&AppMenuItemId::SaveAs);
                            };
                            Task::none()
                        }
                        ParsedData::Exception => {
                            if let Some(menu) = &mut self.menu {
                                menu.disable(&AppMenuItemId::SaveAs);
                            };
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }

            AppMsg::SaveAs => Task::future(ops::choose_file_to_save()),
            AppMsg::PathToSave(path_buf_opt) => {
                if let Some(path_buf) = path_buf_opt {
                    println!("{path_buf:?}");
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
                                if let Some(tv) = &self.treeview {
                                    let newick_string = &tv.newick_string();
                                    ops::write_text_file(&path_buf, newick_string);
                                    self.title = Some(
                                        path_buf
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy()
                                            .to_string(),
                                    );
                                }
                                Task::none()
                            }
                            FileType::Nexus => Task::none(), // Save Nexus file
                            _ => Task::none(),
                        },
                    }
                } else {
                    Task::none()
                }
            }

            AppMsg::AppInitialized => {
                self.menu = AppMenu::new(consts::SIDEBAR_POSITION);
                if let Some(menu) = &mut self.menu {
                    menu.disable(&AppMenuItemId::SaveAs);
                }
                Task::done(AppMsg::WinOpen)
            }

            AppMsg::WinEvent(e) => match e {
                WindowEvent::Opened { position: _, size: _ } => Task::done(AppMsg::WinOpened),
                WindowEvent::CloseRequested => Task::done(AppMsg::WinCloseRequested),
                WindowEvent::Closed => Task::done(AppMsg::WinClosed),
                WindowEvent::FileDropped(path_buf) => {
                    Task::done(AppMsg::PathToOpen(Some(path_buf)))
                }
                _ => Task::none(),
            },

            AppMsg::WinOpen => {
                if self.winid.is_none() {
                    let (window_id, task) = open_window(window_settings());
                    self.winid = Some(window_id);
                    self.treeview = Some(TreeView::new(consts::SIDEBAR_POSITION));
                    task.discard()
                } else {
                    eprintln!("AppMsg::OpenWindow -> Window is already open.");
                    Task::none()
                }
            }

            AppMsg::WinOpened => {
                if let Some(menu) = &mut self.menu {
                    menu.enable(&AppMenuItemId::CloseWindow);
                }

                Task::none()
                    .chain({
                        #[cfg(target_os = "windows")]
                        {
                            if let Some(id) = self.winid {
                                iced::window::get_raw_id::<AppMsg>(id).map(AppMsg::AddMenuForHwnd)
                            } else {
                                Task::none()
                            }
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
                    .chain({
                        #[cfg(not(debug_assertions))]
                        {
                            Task::none()
                        }
                        #[cfg(debug_assertions)]
                        {
                            // let path_buf = PathBuf::from("tests/data/100_starting_trees.newick");
                            let path_buf = PathBuf::from("tests/data/tree01.newick");
                            // let path_buf = PathBuf::from("tests/data/tree02.newick");
                            let path: &std::path::Path = &path_buf.clone().into_boxed_path();
                            if path.exists() {
                                Task::done(AppMsg::PathToOpen(Some(path_buf)))
                            } else {
                                Task::none()
                            }
                        }
                    })
            }
            AppMsg::WinCloseRequested => {
                if self.winid.is_some() {
                    Task::done(AppMsg::WinClose)
                } else {
                    eprintln!("AppMsg::CloseWindow -> There is no window to close.");
                    Task::none()
                }
            }

            AppMsg::WinClose => {
                if let Some(window_id) = self.winid {
                    self.winid = None;
                    self.treeview = None;
                    close_window(window_id)
                } else {
                    Task::none()
                }
            }

            AppMsg::WinClosed => {
                if let Some(menu) = &mut self.menu {
                    menu.disable(&AppMenuItemId::CloseWindow);
                    menu.disable(&AppMenuItemId::SaveAs);
                    menu.disable(&AppMenuItemId::SideBarPosition);
                    menu.disable(&AppMenuItemId::ToggleSearchBar);
                }
                #[cfg(target_os = "macos")]
                {
                    Task::none()
                }
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                {
                    Task::done(AppMsg::Quit)
                }
            }

            AppMsg::Quit => exit(),

            #[cfg(target_os = "windows")]
            AppMsg::AddMenuForHwnd(hwnd) => {
                if let Some(menu) = &self.menu {
                    menu.init_for_hwnd(hwnd);
                }
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<AppMsg> {
        let mut subs: Vec<Subscription<AppMsg>> = Vec::with_capacity(4);
        #[cfg(target_os = "macos")]
        {
            subs.push(platform::os_events());
        }
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        subs.push(menu::menu_events());
        subs.push(window_events().map(|(_, e)| AppMsg::WinEvent(e)));
        subs.push(on_key_press(|key, mods| Some(AppMsg::KeysPressed(key, mods))));
        Subscription::batch(subs)
    }

    pub fn title(&self, _: WindowId) -> String {
        if let Some(title) = &self.title { title.clone() } else { String::from("") }
    }
    pub fn scale_factor(&self, _: WindowId) -> f64 { APP_SCALE_FACTOR }
    pub fn theme(&self, _: WindowId) -> Theme { Theme::default() }
    pub fn settings() -> IcedAppSettings {
        IcedAppSettings {
            id: None,
            fonts: vec![],
            default_font: Font::DEFAULT,
            default_text_size: Pixels(TXT_SIZE),
            antialiasing: true,
            #[cfg(target_os = "macos")]
            allows_automatic_window_tabbing: false,
        }
    }
}
