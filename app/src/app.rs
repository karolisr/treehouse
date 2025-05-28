mod consts;
mod menu;
mod ops;
mod platform;
mod win;

// use consts::*;
use dendros::parse_newick;
use iced::{
    Element, Subscription, Task, Theme, exit,
    window::{Event as WinEvent, Id as WinId, close as close_window, events as window_events, open as open_window},
};
use menu::{AppMenu, AppMenuItemId};
use std::path::PathBuf;
use treeview::{SidebarPos, TreeView, TvMsg};
use win::window_settings;

pub struct App {
    winid: Option<WinId>,
    treeview: Option<TreeView>,
    menu: Option<AppMenu>,
    title: Option<String>,
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
    WinEvent(WinEvent),
    // --------------------------------
    WinOpen,
    WinOpened,
    WinCloseRequested,
    WinClose,
    WinClosed,
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
    pub fn boot() -> (Self, Task<AppMsg>) {
        #[cfg(target_os = "macos")]
        platform::register_ns_application_delegate_handlers();
        (App { winid: None, treeview: None, menu: None, title: None }, Task::done(AppMsg::AppInitialized))
    }

    pub fn view(&self, _: WinId) -> Element<AppMsg> {
        if let Some(treeview) = &self.treeview {
            treeview.view().map(AppMsg::TvMsg)
        } else {
            iced::widget::container(iced::widget::text!("App::view"))
                .width(iced::Fill)
                .height(iced::Fill)
                .center(iced::Fill)
                .into()
        }
    }

    pub fn update(&mut self, app_msg: AppMsg) -> Task<AppMsg> {
        match app_msg {
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
                            SidebarPos::Left => menu.update(&AppMenuItemId::SetSideBarPositionLeft),
                            SidebarPos::Right => menu.update(&AppMenuItemId::SetSideBarPositionRight),
                        }
                    }
                    treeview.update(tv_msg).map(AppMsg::TvMsg)
                } else {
                    Task::none()
                }
            }
            AppMsg::OpenFile => Task::future(ops::choose_file_to_open()),
            AppMsg::PathToOpen(path_buf_opt) => {
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
                            FileType::Newick => ParsedData::Trees(parse_newick(ops::read_text_file(path_buf.clone()))),
                            FileType::Nexus => ParsedData::Trees(None),
                            _ => ParsedData::Exception,
                        },
                    };

                    match parsed_data {
                        ParsedData::Trees(trees) => match trees {
                            Some(trees) => {
                                self.title =
                                    Some(path_buf.file_name().unwrap_or_default().to_string_lossy().to_string());
                                Task::done(AppMsg::TvMsg(TvMsg::TreesLoaded(trees)))
                            }
                            None => {
                                println!("ParsedData::Trees(None)");
                                Task::none()
                            }
                        },
                        ParsedData::Other(s) => {
                            println!("ParsedData::Other({s})");
                            Task::none()
                        }
                        ParsedData::Exception => Task::none(),
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
                            FileType::Newick => Task::none(), // Save Newick file
                            FileType::Nexus => Task::none(),  // Save Nexus file
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
                WinEvent::Opened { position: _, size: _ } => Task::done(AppMsg::WinOpened),
                WinEvent::CloseRequested => Task::done(AppMsg::WinCloseRequested),
                WinEvent::Closed => Task::done(AppMsg::WinClosed),
                WinEvent::FileDropped(path_buf) => Task::done(AppMsg::PathToOpen(Some(path_buf))),
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

            AppMsg::WinOpened => Task::none()
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
                .chain(Task::none())
                .chain({
                    #[cfg(not(debug_assertions))]
                    {
                        Task::none()
                    }
                    #[cfg(debug_assertions)]
                    {
                        // let path_buf = PathBuf::from("tests/data/100_starting_trees.newick");
                        // let path_buf = PathBuf::from("tests/data/tree01.newick");
                        let path_buf = PathBuf::from("tests/data/tree02.newick");
                        let path: &std::path::Path = &path_buf.clone().into_boxed_path();
                        if path.exists() { Task::done(AppMsg::PathToOpen(Some(path_buf))) } else { Task::none() }
                    }
                }),

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
                    exit()
                }
            }

            AppMsg::WinClosed => exit(),

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
        let mut subs: Vec<Subscription<AppMsg>> = Vec::new();
        #[cfg(target_os = "macos")]
        {
            subs.push(platform::os_events());
        }
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        subs.push(menu::menu_events());
        subs.push(window_events().map(|(_, e)| AppMsg::WinEvent(e)));
        Subscription::batch(subs)
    }

    pub fn title(&self, _: WinId) -> String {
        if let Some(title) = &self.title { title.clone() } else { String::from("") }
    }
    pub fn scale_factor(&self, _: WinId) -> f64 { 1e0 }
    pub fn theme(&self, _: WinId) -> Theme { iced::Theme::default() }
    pub fn settings() -> iced::Settings {
        iced::Settings {
            id: None,
            fonts: vec![],
            default_font: iced::Font::DEFAULT,
            default_text_size: iced::Pixels(13e0),
            antialiasing: true,
            #[cfg(target_os = "macos")]
            allows_automatic_window_tabbing: false,
        }
    }
}
