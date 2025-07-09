mod consts;
mod menu;
mod ops;
mod platform;
mod win;

use consts::*;
use dendros::parse_newick;
use menu::{AppMenu, AppMenuItemId, ContextMenu};
use riced::{
    Clr, Element, Font, IcedAppSettings, Key, Modifiers, Pixels, Subscription,
    Task, Theme, WindowEvent, WindowId, close_window, exit, on_key_press,
    open_window, window_events,
};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use riced::{HasWindowHandle, RawWindowHandle, run_with_handle};
use std::path::PathBuf;
use treeview::{SidebarPosition, TreeView, TvContextMenuListing, TvMsg};
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
    ShowContextMenu(TvContextMenuListing),
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
    fn toggle_explain(&mut self) {
        self.explain = !self.explain;
    }

    pub fn boot() -> (Self, Task<AppMsg>) {
        #[cfg(target_os = "macos")]
        platform::register_ns_application_delegate_handlers();
        (
            App {
                winid: None,
                treeview: None,
                menu: None,
                title: None,
                explain: false,
            },
            Task::done(AppMsg::AppInitialized),
        )
    }

    pub fn view(&'_ self, _: WindowId) -> Element<'_, AppMsg> {
        if let Some(treeview) = &self.treeview {
            if !treeview.are_any_trees_loaded() {
                riced::container(
                    riced::btn_txt("Open a Tree File", Some(AppMsg::OpenFile))
                        .width(riced::BTN_H1 * 5e0),
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
        let mut task: Option<Task<AppMsg>> = None;
        match app_msg {
            AppMsg::KeysPressed(key, modifiers) => {
                if let Key::Character(k) = key {
                    let k: &str = k.as_str();
                    match modifiers {
                        mods if mods.contains(
                            Modifiers::COMMAND | Modifiers::SHIFT,
                        ) =>
                        {
                            match k {
                                "d" => {
                                    if let Some(treeview) = &mut self.treeview {
                                        treeview.toggle_draw_debug();
                                    }
                                }
                                "e" => {
                                    self.toggle_explain();
                                }
                                _ => {}
                            }
                        }
                        Modifiers::COMMAND => {
                            #[cfg(any(
                                target_os = "windows",
                                target_os = "linux"
                            ))]
                            match k {
                                "f" => {
                                    task = Some(Task::done(AppMsg::TvMsg(
                                        TvMsg::ToggleSearchBar,
                                    )));
                                }
                                "o" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        AppMenuItemId::OpenFile,
                                    )));
                                }
                                "s" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        AppMenuItemId::SaveAs,
                                    )));
                                }
                                "[" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        AppMenuItemId::SetSideBarPositionLeft,
                                    )));
                                }
                                "]" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        AppMenuItemId::SetSideBarPositionRight,
                                    )));
                                }
                                _ => {}
                            }
                            #[cfg(any(
                                target_os = "macos",
                                target_os = "linux"
                            ))]
                            match k {
                                "w" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        AppMenuItemId::CloseWindow,
                                    )));
                                }
                                _ => {}
                            }
                            #[cfg(target_os = "linux")]
                            match k {
                                "q" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        AppMenuItemId::CloseWindow,
                                    )));
                                }
                                _ => {}
                            }
                            match k {
                                "=" => {
                                    task = Some(Task::done(AppMsg::TvMsg(
                                        TvMsg::CnvHeightIncrement,
                                    )));
                                }
                                "-" => {
                                    task = Some(Task::done(AppMsg::TvMsg(
                                        TvMsg::CnvHeightDecrement,
                                    )));
                                }
                                _ => {}
                            }
                        }
                        _ => match k {
                            "l" => {
                                task = Some(Task::done(AppMsg::TvMsg(
                                    TvMsg::AddRemoveCladeLabelForSelectedNode,
                                )));
                            }
                            _ => {}
                        },
                    }
                }
            }
            AppMsg::Other(opt_msg) => {
                if let Some(msg) = opt_msg {
                    println!("AppMsg::Other({msg})");
                }
            }
            AppMsg::MenuEvent(miid) => {
                if let Some(menu) = &mut self.menu {
                    menu.update(&miid);
                }
                task = Some(Task::done(miid.into()));
            }
            AppMsg::ShowContextMenu(tree_view_context_menu_listing) => {
                #[cfg(any(target_os = "windows", target_os = "macos"))]
                if let Some(winid) = self.winid {
                    let task_to_return = run_with_handle(winid, |h| {
                        if let Ok(handle) = h.window_handle() {
                            let context_menu: ContextMenu = tree_view_context_menu_listing.into();

                            let muda_menu: muda::Menu = context_menu.into();

                            #[cfg(target_os = "macos")]
                            unsafe {
                                if let RawWindowHandle::AppKit(handle_raw) = handle.as_raw() {
                                    _ = muda::ContextMenu::show_context_menu_for_nsview(
                                        &muda_menu,
                                        handle_raw.ns_view.as_ptr(),
                                        None,
                                    );
                                }
                            }
                            #[cfg(target_os = "windows")]
                            unsafe {
                                if let RawWindowHandle::Win32(handle_raw) = handle.as_raw() {
                                    _ = muda::ContextMenu::show_context_menu_for_hwnd(
                                        &muda_menu,
                                        handle_raw.hwnd.into(),
                                        None,
                                    );
                                }
                            }
                        }
                    })
                    .discard();
                    task = Some(task_to_return);
                }
                #[cfg(target_os = "linux")]
                println!(
                    "AppMsg::ShowContextMenu({tree_view_context_menu_listing})"
                );
            }
            AppMsg::TvMsg(tv_msg) => {
                if let Some(treeview) = &mut self.treeview {
                    task = Some(
                        treeview.update(tv_msg.clone()).map(AppMsg::TvMsg),
                    );
                    match tv_msg {
                        TvMsg::ContextMenuInteractionBegin(
                            tree_view_context_menu_listing,
                        ) => {
                            task = Some(Task::done(AppMsg::ShowContextMenu(
                                tree_view_context_menu_listing,
                            )));
                        }
                        TvMsg::SetSidebarPos(sidebar_position) => {
                            if let Some(menu) = &mut self.menu {
                                match sidebar_position {
                                    SidebarPosition::Left => {
                                        menu.update(&AppMenuItemId::SetSideBarPositionLeft);
                                    }
                                    SidebarPosition::Right => {
                                        menu.update(&AppMenuItemId::SetSideBarPositionRight);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            AppMsg::OpenFile => {
                if self.winid.is_none() {
                    task = Some(
                        Task::done(AppMsg::WinOpen)
                            .chain(Task::done(ops::choose_file_to_open_sync())),
                    );
                } else {
                    task = Some(Task::future(ops::choose_file_to_open()));
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
                                "tree" | "trees" | "nexus" | "nex" => {
                                    FileType::Nexus
                                }
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
                                ParsedData::Trees(parse_newick(
                                    ops::read_text_file(path_buf.clone()),
                                ))
                            }
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
                                    menu.enable(
                                        &AppMenuItemId::SideBarPosition,
                                    );
                                    menu.enable(
                                        &AppMenuItemId::ToggleSearchBar,
                                    );
                                };
                                task = Some(Task::done(AppMsg::TvMsg(
                                    TvMsg::TreesLoaded(trees),
                                )));
                            }
                            None => {
                                println!("ParsedData::Trees(None)");
                                if let Some(menu) = &mut self.menu {
                                    menu.disable(&AppMenuItemId::SaveAs);
                                };
                            }
                        },
                        ParsedData::Other(s) => {
                            println!("ParsedData::Other({s})");
                            if let Some(menu) = &mut self.menu {
                                menu.disable(&AppMenuItemId::SaveAs);
                            };
                        }
                        ParsedData::Exception => {
                            if let Some(menu) = &mut self.menu {
                                menu.disable(&AppMenuItemId::SaveAs);
                            };
                        }
                    }
                }
            }
            AppMsg::SaveAs => {
                task = Some(Task::future(ops::choose_file_to_save()));
            }
            AppMsg::PathToSave(path_buf_opt) => {
                if let Some(path_buf) = path_buf_opt {
                    println!("{path_buf:?}");
                    let file_type: FileType = match path_buf.extension() {
                        Some(ext_os_str) => match ext_os_str.to_str() {
                            Some(ext) => match ext {
                                "newick" | "tre" => FileType::Newick,
                                "tree" | "trees" | "nexus" | "nex" => {
                                    FileType::Nexus
                                }
                                ext => FileType::Other(ext.to_string()),
                            },
                            None => FileType::Exception,
                        },
                        None => FileType::Exception,
                    };

                    match file_type {
                        FileType::Other(_) => {}
                        FileType::Exception => {}
                        file_type => match file_type {
                            FileType::Newick => {
                                if let Some(tv) = &self.treeview {
                                    let newick_string = &tv.newick_string();
                                    ops::write_text_file(
                                        &path_buf, newick_string,
                                    );
                                    self.title = Some(
                                        path_buf
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy()
                                            .to_string(),
                                    );
                                }
                            }
                            FileType::Nexus => {} // Save Nexus file
                            _ => {}
                        },
                    }
                }
            }
            AppMsg::AppInitialized => {
                self.menu = AppMenu::new(consts::SIDEBAR_POSITION);
                if let Some(menu) = &mut self.menu {
                    menu.disable(&AppMenuItemId::SaveAs);
                }
                task = Some(Task::done(AppMsg::WinOpen));
            }
            AppMsg::WinEvent(e) => match e {
                WindowEvent::Opened { position: _, size: _ } => {
                    task = Some(Task::done(AppMsg::WinOpened));
                }
                WindowEvent::CloseRequested => {
                    task = Some(Task::done(AppMsg::WinCloseRequested));
                }
                WindowEvent::Closed => {
                    task = Some(Task::done(AppMsg::WinClosed));
                }
                WindowEvent::FileDropped(path_buf) => {
                    task = Some(Task::done(AppMsg::PathToOpen(Some(path_buf))));
                }
                _ => {}
            },
            AppMsg::WinOpen => {
                if self.winid.is_none() {
                    let (window_id, open_window_task) =
                        open_window(window_settings());
                    self.winid = Some(window_id);
                    self.treeview =
                        Some(TreeView::new(consts::SIDEBAR_POSITION));
                    task = Some(open_window_task.discard());
                } else {
                    eprintln!("AppMsg::OpenWindow -> Window is already open.");
                }
            }
            AppMsg::WinOpened => {
                if let Some(menu) = &mut self.menu {
                    menu.enable(&AppMenuItemId::CloseWindow);
                }

                #[cfg(any(debug_assertions, target_os = "windows"))]
                let mut task_to_return = Task::none();

                #[cfg(all(not(debug_assertions), target_os = "macos"))]
                let task_to_return = Task::none();

                #[cfg(all(not(debug_assertions), target_os = "linux"))]
                let task_to_return = Task::none();

                #[cfg(target_os = "windows")]
                if let Some(id) = self.winid {
                    task_to_return = riced::get_raw_id::<AppMsg>(id)
                        .map(AppMsg::AddMenuForHwnd);
                }

                #[cfg(debug_assertions)]
                {
                    task_to_return = task_to_return.chain({
                        let path_buf =
                            PathBuf::from("tests/data/tree02.newick");
                        let path: &std::path::Path =
                            &path_buf.clone().into_boxed_path();
                        if path.exists() {
                            Task::done(AppMsg::PathToOpen(Some(path_buf)))
                        } else {
                            Task::none()
                        }
                    });
                }

                task = Some(task_to_return);
            }
            AppMsg::WinCloseRequested => {
                if self.winid.is_some() {
                    task = Some(Task::done(AppMsg::WinClose));
                } else {
                    eprintln!(
                        "AppMsg::CloseWindow -> There is no window to close."
                    );
                }
            }
            AppMsg::WinClose => {
                if let Some(window_id) = self.winid {
                    self.winid = None;
                    self.treeview = None;
                    task = Some(close_window(window_id));
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
                    task = Some(Task::done(AppMsg::Quit));
                }
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                {
                    task = Some(Task::done(AppMsg::Quit))
                }
            }
            AppMsg::Quit => task = Some(exit()),
            #[cfg(target_os = "windows")]
            AppMsg::AddMenuForHwnd(hwnd) => {
                if let Some(menu) = &self.menu {
                    menu.init_for_hwnd(hwnd)
                }
            }
        }

        match task {
            Some(task) => task,
            None => Task::none(),
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
        subs.push(on_key_press(|key, mods| {
            Some(AppMsg::KeysPressed(key, mods))
        }));
        Subscription::batch(subs)
    }

    pub fn title(&self, _: WindowId) -> String {
        if let Some(title) = &self.title {
            title.clone()
        } else {
            String::from("")
        }
    }

    pub fn scale_factor(&self, _: WindowId) -> f64 {
        APP_SCALE_FACTOR
    }

    pub fn theme(&self, _: WindowId) -> Theme {
        Theme::default()
    }

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
