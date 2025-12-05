mod consts;
mod menu;
mod ops;
mod platform;
mod window;

use consts::*;
use dendros::parse_trees;

use menu::*;
use riced::{
    Clr, Element, Font, IcedAppSettings, Key, Modifiers, Pixels, Subscription,
    Task, Theme, ThemeStyle, WindowEvent, WindowId, close_window,
    error_container, exit, modal_element, on_key_press, open_window,
    window_events,
};
use std::path::PathBuf;
use treeview::{TreeView, TvContextMenuListing, TvMsg};
use window::window_settings;

pub struct App {
    winid: Option<WindowId>,
    treeview: Option<TreeView>,
    menu: Option<AppMenu>,
    title: Option<String>,
    error: Option<String>,
    explain: bool,
    #[cfg(feature = "menu-custom")]
    active_context_menu: Option<ContextMenu>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    Other(Option<String>),
    MenuEvent(MenuItemId),
    // -------------------------------------------------------------------------
    ErrorSet(String),
    ErrorClear,
    // -------------------------------------------------------------------------
    ShowContextMenu(TvContextMenuListing),
    #[cfg(feature = "menu-custom")]
    SetCustomContextMenu(ContextMenu),
    #[cfg(feature = "menu-custom")]
    HideContextMenu,
    // -------------------------------------------------------------------------
    TvMsg(TvMsg),
    // -------------------------------------------------------------------------
    OpenFile,
    SaveAs,
    ExportPdf,
    ExportSubtree,
    PathToOpen(Option<PathBuf>),
    PathToSave {
        path: Option<PathBuf>,
        subtree: bool,
    },
    // -------------------------------------------------------------------------
    AppInitialized,
    // -------------------------------------------------------------------------
    WinEvent(WindowEvent),
    // -------------------------------------------------------------------------
    WinOpen,
    WinOpened,
    WinCloseRequested,
    WinClose,
    WinClosed,
    Quit,
    // -------------------------------------------------------------------------
    KeysPressed(Key, Modifiers),
    // -------------------------------------------------------------------------
    #[cfg(all(target_os = "windows", feature = "menu-muda"))]
    AddMenuForHwnd(u64),
    #[cfg(target_os = "windows")]
    RegisterFileTypes,
    #[cfg(target_os = "windows")]
    UnregisterFileTypes,
}

pub enum FileType {
    Newick,
    Nexus,
    Pdf,
    Other,
}

impl App {
    fn toggle_explain(&mut self) {
        self.explain = !self.explain;
    }

    pub fn boot() -> (Self, Task<AppMsg>) {
        #[cfg(target_os = "macos")]
        platform::register_ns_application_delegate_handlers();

        #[cfg(target_os = "windows")]
        {
            if let Err(e) = platform::setup_file_handling() {
                eprintln!("Failed to set up file type associations: {}", e);
            }
        }

        (
            App {
                winid: None,
                treeview: None,
                menu: None,
                title: None,
                error: None,
                explain: false,
                #[cfg(feature = "menu-custom")]
                active_context_menu: None,
            },
            Task::done(AppMsg::AppInitialized),
        )
    }

    pub fn view(&'_ self, _: WindowId) -> Element<'_, AppMsg> {
        let mut v: Element<'_, AppMsg>;
        if let Some(treeview) = &self.treeview {
            if !treeview.are_any_trees_loaded() {
                v = riced::container(
                    riced::btn_txt("Open a Tree File", Some(AppMsg::OpenFile))
                        .width(riced::BTN_H1 * 5e0),
                )
                .width(riced::Length::Fill)
                .height(riced::Length::Fill)
                .center(riced::Length::Fill)
                .into();
            } else {
                v = treeview.view().map(AppMsg::TvMsg);
                #[cfg(feature = "menu-custom")]
                if let Some(context_menu) = &self.active_context_menu {
                    v = context_menu.element(v);
                }
            }
        } else {
            v = riced::container(riced::txt("App::view"))
                .width(riced::Length::Fill)
                .height(riced::Length::Fill)
                .center(riced::Length::Fill)
                .into();
        }

        if self.explain {
            v = v.explain(Clr::RED);
        };

        if let Some(error) = &self.error {
            v = modal_element(v, error_container(error, AppMsg::ErrorClear));
        }

        #[cfg(feature = "menu-custom")]
        if let Some(menu) = &self.menu {
            v = menu.menu_bar(v);
        }

        v
    }

    pub fn update(&mut self, app_msg: AppMsg) -> Task<AppMsg> {
        let mut task: Option<Task<AppMsg>> = None;
        match app_msg {
            AppMsg::ErrorSet(s) => {
                self.error = Some(s);
            }

            AppMsg::ErrorClear => {
                if let Some(menu) = &mut self.menu {
                    menu.enable(&MenuItemId::OpenFile);
                }
                self.error = None;
            }

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
                                        MenuItemId::OpenFile,
                                    )));
                                }
                                "s" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        MenuItemId::SaveAs,
                                    )));
                                }
                                "p" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        MenuItemId::ExportPdf,
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
                                        MenuItemId::CloseWindow,
                                    )));
                                }
                                _ => {}
                            }
                            #[cfg(target_os = "linux")]
                            match k {
                                "q" => {
                                    task = Some(Task::done(AppMsg::MenuEvent(
                                        MenuItemId::CloseWindow,
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
                                    TvMsg::AddRemoveCladeHighlightForSelectedNode,
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
                if let Some(winid) = self.winid {
                    let task_to_return = show_context_menu(
                        tree_view_context_menu_listing, winid,
                    );
                    task = Some(task_to_return);
                }
            }

            #[cfg(feature = "menu-custom")]
            AppMsg::SetCustomContextMenu(context_menu) => {
                self.active_context_menu = Some(context_menu);
            }

            #[cfg(feature = "menu-custom")]
            AppMsg::HideContextMenu => {
                self.active_context_menu = None;
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

                        #[cfg(feature = "menu-custom")]
                        TvMsg::ContextMenuChosenIdx(_) => {
                            self.active_context_menu = None;
                        }

                        TvMsg::SetSubtreeView(_node_id) => {
                            if let Some(menu) = &mut self.menu {
                                menu.enable(&MenuItemId::ExportSubtree);
                            }
                        }

                        TvMsg::ClearSubtreeView => {
                            if let Some(menu) = &mut self.menu {
                                menu.disable(&MenuItemId::ExportSubtree);
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
                    let parsed_data = ops::read_text_file(path_buf.clone());
                    match parsed_data {
                        Ok(trees_string) => {
                            let trees_result = parse_trees(trees_string);
                            match trees_result {
                                Ok(trees) => {
                                    self.title = Some(
                                        path_buf
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy()
                                            .to_string(),
                                    );
                                    if let Some(menu) = &mut self.menu {
                                        menu.enable(&MenuItemId::SaveAs);
                                        menu.enable(&MenuItemId::ExportPdf);
                                        menu.enable(
                                            &MenuItemId::ToggleSearchBar,
                                        );
                                        menu.disable(
                                            &MenuItemId::ExportSubtree,
                                        );
                                    };
                                    task = Some(Task::done(AppMsg::TvMsg(
                                        TvMsg::TreesLoaded(trees),
                                    )));
                                }
                                Err(error) => {
                                    if let Some(menu) = &mut self.menu {
                                        menu.disable(&MenuItemId::OpenFile);
                                        menu.disable(&MenuItemId::SaveAs);
                                        menu.disable(&MenuItemId::ExportPdf);
                                    };
                                    task = Some(Task::done(AppMsg::ErrorSet(
                                        error.to_string(),
                                    )));
                                }
                            }
                        }
                        Err(file_read_error) => {
                            if let Some(menu) = &mut self.menu {
                                menu.disable(&MenuItemId::OpenFile);
                                menu.disable(&MenuItemId::SaveAs);
                                menu.disable(&MenuItemId::ExportPdf);
                            };
                            task = Some(Task::done(AppMsg::ErrorSet(
                                file_read_error.to_string(),
                            )));
                        }
                    }
                }
            }

            AppMsg::SaveAs => {
                task = Some(Task::future(ops::choose_file_to_save(false)));
            }

            AppMsg::ExportSubtree => {
                task = Some(Task::future(ops::choose_file_to_save(true)));
            }

            AppMsg::ExportPdf => {
                task = Some(Task::future(ops::choose_file_to_pdf_export()));
            }

            AppMsg::PathToSave { path: path_buf_opt, subtree } => {
                if let Some(path_buf) = path_buf_opt {
                    println!("{path_buf:?}");
                    let file_type: FileType = match path_buf.extension() {
                        Some(ext_os_str) => match ext_os_str.to_str() {
                            Some(ext) => match ext {
                                "newick" | "tre" => FileType::Newick,
                                "tree" | "trees" | "nexus" | "nex" => {
                                    FileType::Nexus
                                }
                                "pdf" => FileType::Pdf,
                                _ => FileType::Other,
                            },
                            None => FileType::Other,
                        },
                        None => FileType::Other,
                    };

                    match file_type {
                        FileType::Newick => {
                            if let Some(tv) = &self.treeview {
                                let newick_string = match subtree {
                                    true => &tv.newick_string_subtree(),
                                    false => &tv.newick_string(),
                                };
                                ops::write_text_file(&path_buf, newick_string);

                                match subtree {
                                    true => {}
                                    false => {
                                        self.title = Some(
                                            path_buf
                                                .file_name()
                                                .unwrap_or_default()
                                                .to_string_lossy()
                                                .to_string(),
                                        );
                                    }
                                }
                            }
                        }
                        FileType::Nexus => {} // Save Nexus file
                        FileType::Pdf => {
                            task = Some(Task::done(AppMsg::TvMsg(
                                TvMsg::ExportPdf(path_buf),
                            )));
                        }
                        FileType::Other => {}
                    }
                }
            }

            AppMsg::AppInitialized => {
                self.menu = AppMenu::new();
                if let Some(menu) = &mut self.menu {
                    menu.disable(&MenuItemId::SaveAs);
                    menu.disable(&MenuItemId::ExportPdf);
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
                _ => {
                    task = None;
                }
            },

            AppMsg::WinOpen => {
                if self.winid.is_none() {
                    let (window_id, open_window_task) =
                        open_window(window_settings());
                    self.winid = Some(window_id);
                    self.treeview = Some(TreeView::new());
                    task = Some(open_window_task.discard());
                } else {
                    eprintln!("AppMsg::OpenWindow -> Window is already open.");
                }
            }

            AppMsg::WinOpened => {
                if let Some(menu) = &mut self.menu {
                    menu.enable(&MenuItemId::CloseWindow);
                }

                #[cfg(any(
                    debug_assertions,
                    target_os = "windows",
                    target_os = "linux"
                ))]
                let mut task_to_return = Task::none();

                #[cfg(all(not(debug_assertions), target_os = "macos"))]
                let task_to_return = Task::none();

                // #[cfg(all(not(debug_assertions), target_os = "linux"))]
                // let mut task_to_return = Task::none();

                #[cfg(all(target_os = "windows", feature = "menu-muda"))]
                if let Some(id) = self.winid {
                    task_to_return = riced::get_raw_id::<AppMsg>(id)
                        .map(AppMsg::AddMenuForHwnd);
                }

                #[cfg(any(target_os = "windows", target_os = "linux"))]
                {
                    let args: Vec<String> = std::env::args().collect();
                    if args.len() > 1 {
                        let file_path = &args[1];
                        task_to_return = task_to_return.chain({
                            let path_buf = PathBuf::from(file_path);
                            let path: &std::path::Path =
                                &path_buf.clone().into_boxed_path();
                            if path.exists() {
                                Task::done(AppMsg::PathToOpen(Some(path_buf)))
                            } else {
                                Task::none()
                            }
                        });
                    }
                }

                #[cfg(debug_assertions)]
                {
                    task_to_return = task_to_return.chain({
                        let path_buf = PathBuf::from("tests/data/tree01.tre");
                        // let path_buf = PathBuf::from("tests/data/tree02.newick");
                        // let path_buf = PathBuf::from("tests/data/big_seed_plant_trees/ALLMB.tre");
                        // let path_buf = PathBuf::from("tests/data/Czech_Huerta-Cepas_Stamatakis_2017/Czech_Huerta-Cepas_Stamatakis_2017_unrooted__node_and_branch_attributes.newick");

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
                    menu.disable(&MenuItemId::CloseWindow);
                    menu.disable(&MenuItemId::SaveAs);
                    menu.disable(&MenuItemId::ExportPdf);
                    menu.disable(&MenuItemId::ExportSubtree);
                    menu.disable(&MenuItemId::ToggleSearchBar);
                }
                task = Some(Task::done(AppMsg::Quit));
            }

            AppMsg::Quit => task = Some(exit()),

            #[cfg(all(target_os = "windows", feature = "menu-muda"))]
            AppMsg::AddMenuForHwnd(hwnd) => {
                if let Some(menu) = &self.menu {
                    menu.init_for_hwnd(hwnd);
                }
            }

            #[cfg(target_os = "windows")]
            AppMsg::RegisterFileTypes => {
                match platform::register_file_associations() {
                    Ok(_) => {
                        println!("File type assoc. added.");
                    }
                    Err(e) => {
                        eprintln!("Failed to add file type assoc.: {}", e);
                    }
                }
            }

            #[cfg(target_os = "windows")]
            AppMsg::UnregisterFileTypes => {
                match platform::unregister_file_associations() {
                    Ok(_) => {
                        println!("File type assoc. removed.");
                    }
                    Err(e) => {
                        eprintln!("Failed to remove file type assoc.: {}", e);
                    }
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
        #[cfg(feature = "menu-muda")]
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

    pub fn scale_factor(&self, _: WindowId) -> f32 {
        APP_SCALE_FACTOR
    }

    pub fn theme(&self, _: WindowId) -> Theme {
        Theme::Light
    }

    pub fn theme_style(&self, theme: &Theme) -> ThemeStyle {
        ThemeStyle {
            background_color: theme.palette().background,
            text_color: theme.palette().text,
        }
    }

    pub fn settings() -> IcedAppSettings {
        IcedAppSettings {
            id: None,
            fonts: vec![],
            default_font: Font::DEFAULT,
            default_text_size: Pixels(TXT_SIZE),
            antialiasing: true,
            vsync: true,
            #[cfg(target_os = "macos")]
            allows_automatic_window_tabbing: false,
        }
    }
}
