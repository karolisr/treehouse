mod events;

pub use super::events::AppMenuItemId;
pub use events::menu_events;
#[cfg(target_os = "windows")]
use muda::accelerator::Modifiers;
use muda::{
    // CheckMenuItem,
    MenuItem,
    MenuItemKind,
    Submenu,
    accelerator::{Accelerator, CMD_OR_CTRL, Code},
};
use std::collections::HashMap;
use treeview::{
    // SidebarPosition,
    TvContextMenuListing,
};

impl From<muda::MenuItem> for AppMenuItemId {
    fn from(value: muda::MenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::CheckMenuItem> for AppMenuItemId {
    fn from(value: muda::CheckMenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::Submenu> for AppMenuItemId {
    fn from(value: muda::Submenu) -> Self {
        value.id().0.clone().into()
    }
}

#[allow(missing_debug_implementations)]
#[derive(Default, Clone)]
pub struct ContextMenu {
    muda_menu: muda::Menu,
}

impl ContextMenu {
    pub fn new() -> Self {
        let muda_menu = muda::Menu::new();
        Self { muda_menu }
    }

    pub fn with_muda_menu(muda_menu: muda::Menu) -> Self {
        Self { muda_menu }
    }
}

impl From<ContextMenu> for muda::Menu {
    fn from(context_menu: ContextMenu) -> Self {
        context_menu.muda_menu
    }
}

impl From<TvContextMenuListing> for ContextMenu {
    fn from(tv_context_menu_listing: TvContextMenuListing) -> Self {
        let muda_menu = muda::Menu::new();
        tv_context_menu_listing.items().iter().enumerate().for_each(
            |(idx, item)| {
                let mii = AppMenuItemId::ContextMenuIndex(idx);
                let mmi = MenuItem::with_id(
                    mii,
                    item.label.clone(),
                    item.enabled,
                    None,
                );
                let _ = muda_menu.append(&mmi);
            },
        );
        ContextMenu::with_muda_menu(muda_menu)
    }
}

#[derive(Default, Clone)]
pub struct AppMenu {
    _muda_menu: Option<muda::Menu>,
    items: HashMap<AppMenuItemId, MenuItemKind>,
}

impl AppMenu {
    pub fn enable(&mut self, id: &AppMenuItemId) {
        self.set_enabled(id, true);
    }

    pub fn disable(&mut self, id: &AppMenuItemId) {
        self.set_enabled(id, false);
    }

    fn set_enabled(&mut self, id: &AppMenuItemId, state: bool) {
        if let Some(mik) = self.items.get(id) {
            if let Some(mi) = mik.as_menuitem() {
                mi.set_enabled(state);
            } else if let Some(mi) = mik.as_check_menuitem() {
                mi.set_enabled(state);
            } else if let Some(mi) = mik.as_icon_menuitem() {
                mi.set_enabled(state);
            } else if let Some(mi) = mik.as_submenu() {
                mi.set_enabled(state);
            }
        }
    }

    pub fn update(&mut self, id: &AppMenuItemId) {
        if let Some(mik) = self.items.get(id) {
            match mik {
                MenuItemKind::MenuItem(_) => (),
                MenuItemKind::Submenu(_) => (),
                MenuItemKind::Predefined(_) => (),
                MenuItemKind::Check(mi) => match id {
                    AppMenuItemId::SetSideBarPositionLeft => {
                        mi.set_checked(true);
                        mi.set_enabled(false);
                        if let Some(miko) = self
                            .items
                            .get(&AppMenuItemId::SetSideBarPositionRight)
                            && let Some(mio) = miko.as_check_menuitem()
                        {
                            mio.set_checked(false);
                            mio.set_enabled(true);
                        }
                    }
                    AppMenuItemId::SetSideBarPositionRight => {
                        mi.set_checked(true);
                        mi.set_enabled(false);
                        if let Some(miko) = self
                            .items
                            .get(&AppMenuItemId::SetSideBarPositionLeft)
                            && let Some(mio) = miko.as_check_menuitem()
                        {
                            mio.set_checked(false);
                            mio.set_enabled(true);
                        }
                    }
                    _ => (),
                },
                MenuItemKind::Icon(_) => (),
            }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn init_for_hwnd(&self, hwnd: u64) {
        unsafe {
            if let Some(menu) = &self._muda_menu {
                let _rslt = menu.init_for_hwnd(hwnd as isize);
            }
        };
    }

    // pub fn new(sidebar_pos: SidebarPosition) -> Option<Self>
    pub fn new() -> Option<Self> {
        let menu: muda::Menu;
        let muda_menu: Option<muda::Menu>;
        let items: HashMap<AppMenuItemId, MenuItemKind>;
        // (menu, items) = Self::prepare_app_menu(sidebar_pos);
        (menu, items) = Self::prepare_app_menu();
        #[cfg(target_os = "macos")]
        menu.init_for_nsapp();
        muda_menu = Some(menu);
        Some(Self { _muda_menu: muda_menu, items })
    }

    // fn prepare_app_menu(sidebar_pos: SidebarPosition) -> (muda::Menu, HashMap<AppMenuItemId, MenuItemKind>)
    fn prepare_app_menu() -> (muda::Menu, HashMap<AppMenuItemId, MenuItemKind>)
    {
        let menu = muda::Menu::default();
        let mut items: HashMap<AppMenuItemId, MenuItemKind> = HashMap::new();

        let modifier = CMD_OR_CTRL;

        #[cfg(target_os = "macos")]
        let submenu_app = Submenu::with_id("sub_app", "App", true);
        let submenu_file = Submenu::with_id("sub_file", "File", true);
        let submenu_view = Submenu::with_id("sub_view", "View", true);
        let submenu_sidebar_pos = Submenu::with_id(
            AppMenuItemId::SideBarPosition,
            "Sidebar Position",
            false,
        );

        #[cfg(target_os = "macos")]
        let menu_item_about = muda::PredefinedMenuItem::about(
            None,
            Some(muda::AboutMetadata::default()),
        );

        #[cfg(target_os = "windows")]
        let menu_item_close_win = MenuItem::with_id(
            AppMenuItemId::CloseWindow,
            "Close Window",
            true,
            Some(Accelerator::new(Some(Modifiers::ALT), Code::F4)),
        );

        #[cfg(target_os = "macos")]
        let menu_item_quit = MenuItem::with_id(
            AppMenuItemId::Quit,
            "Quit",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyQ)),
        );

        let menu_item_open = MenuItem::with_id(
            AppMenuItemId::OpenFile,
            "Open File",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyO)),
        );

        let menu_item_save_as = MenuItem::with_id(
            AppMenuItemId::SaveAs,
            "Save As...",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyS)),
        );

        let menu_item_export_subtree = MenuItem::with_id(
            AppMenuItemId::ExportSubtree,
            "Export Subtree",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyE)),
        );

        let menu_item_export_pdf = MenuItem::with_id(
            AppMenuItemId::ExportPdf,
            "Export PDF",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyP)),
        );

        #[cfg(all(target_os = "windows", debug_assertions))]
        let menu_item_register_filetypes = MenuItem::with_id(
            AppMenuItemId::RegisterFileTypes,
            "Register File Associations",
            true,
            None,
        );

        #[cfg(all(target_os = "windows", debug_assertions))]
        let menu_item_unregister_filetypes = MenuItem::with_id(
            AppMenuItemId::UnregisterFileTypes,
            "Unregister File Associations",
            true,
            None,
        );

        // let menu_item_sidebar_pos_left = CheckMenuItem::with_id(
        //     AppMenuItemId::SetSideBarPositionLeft,
        //     "Left",
        //     sidebar_pos != SidebarPosition::Left,
        //     sidebar_pos == SidebarPosition::Left,
        //     Some(Accelerator::new(Some(modifier), Code::BracketLeft)),
        // );

        // let menu_item_sidebar_pos_right = CheckMenuItem::with_id(
        //     AppMenuItemId::SetSideBarPositionRight,
        //     "Right",
        //     sidebar_pos != SidebarPosition::Right,
        //     sidebar_pos == SidebarPosition::Right,
        //     Some(Accelerator::new(Some(modifier), Code::BracketRight)),
        // );

        let menu_item_toggle_search_bar = MenuItem::with_id(
            AppMenuItemId::ToggleSearchBar,
            "Search...",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyF)),
        );

        #[cfg(target_os = "macos")]
        {
            _ = submenu_app.append(&menu_item_about).ok();
        }
        #[cfg(target_os = "macos")]
        {
            _ = submenu_app.append(&menu_item_quit).ok();
        }

        _ = submenu_file.append(&menu_item_open).ok();
        _ = submenu_file.append(&menu_item_save_as).ok();
        _ = submenu_file.append(&menu_item_export_subtree).ok();
        _ = submenu_file.append(&menu_item_export_pdf).ok();
        #[cfg(all(target_os = "windows", debug_assertions))]
        {
            _ = submenu_file
                .append(&muda::PredefinedMenuItem::separator())
                .ok();
            _ = submenu_file.append(&menu_item_register_filetypes).ok();
            _ = submenu_file.append(&menu_item_unregister_filetypes).ok();
            _ = submenu_file
                .append(&muda::PredefinedMenuItem::separator())
                .ok();
        }
        #[cfg(target_os = "windows")]
        {
            _ = submenu_file.append(&menu_item_close_win).ok();
        }

        // _ = submenu_sidebar_pos.append(&menu_item_sidebar_pos_left).ok();
        // _ = submenu_sidebar_pos.append(&menu_item_sidebar_pos_right).ok();
        // _ = submenu_view.append(&submenu_sidebar_pos).ok();
        _ = submenu_view.append(&menu_item_toggle_search_bar).ok();

        #[cfg(target_os = "macos")]
        {
            _ = menu.append(&submenu_app).ok();
        }
        _ = menu.append(&submenu_file).ok();
        _ = menu.append(&submenu_view).ok();

        #[cfg(target_os = "macos")]
        {
            _ = items.insert(
                menu_item_quit.clone().into(),
                MenuItemKind::MenuItem(menu_item_quit),
            );
        }
        _ = items.insert(
            menu_item_open.clone().into(),
            MenuItemKind::MenuItem(menu_item_open),
        );
        _ = items.insert(
            menu_item_save_as.clone().into(),
            MenuItemKind::MenuItem(menu_item_save_as),
        );
        _ = items.insert(
            menu_item_export_subtree.clone().into(),
            MenuItemKind::MenuItem(menu_item_export_subtree),
        );
        _ = items.insert(
            menu_item_export_pdf.clone().into(),
            MenuItemKind::MenuItem(menu_item_export_pdf),
        );
        #[cfg(target_os = "windows")]
        {
            _ = items.insert(
                menu_item_close_win.clone().into(),
                MenuItemKind::MenuItem(menu_item_close_win),
            );
            #[cfg(debug_assertions)]
            {
                _ = items.insert(
                    menu_item_register_filetypes.clone().into(),
                    MenuItemKind::MenuItem(menu_item_register_filetypes),
                );
                _ = items.insert(
                    menu_item_unregister_filetypes.clone().into(),
                    MenuItemKind::MenuItem(menu_item_unregister_filetypes),
                );
            }
        }
        // _ = items.insert(
        //     menu_item_sidebar_pos_left.clone().into(),
        //     MenuItemKind::Check(menu_item_sidebar_pos_left),
        // );
        // _ = items.insert(
        //     menu_item_sidebar_pos_right.clone().into(),
        //     MenuItemKind::Check(menu_item_sidebar_pos_right),
        // );
        _ = items.insert(
            menu_item_toggle_search_bar.clone().into(),
            MenuItemKind::MenuItem(menu_item_toggle_search_bar),
        );
        _ = items.insert(
            submenu_sidebar_pos.clone().into(),
            MenuItemKind::Submenu(submenu_sidebar_pos),
        );

        (menu, items)
    }
}
