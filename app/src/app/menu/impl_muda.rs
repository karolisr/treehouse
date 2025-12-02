mod context_menu;
mod events;

use std::collections::HashMap;

use muda::CheckMenuItem as MudaCheckMenuItem;
use muda::Menu as MudaMenu;
use muda::MenuItem as MudaMenuItem;
use muda::MenuItemKind as MudaMenuItemKind;
use muda::PredefinedMenuItem as MudaPredefinedMenuItem;
use muda::Submenu as MudaSubmenu;
use muda::accelerator::Accelerator;
use muda::accelerator::CMD_OR_CTRL;
use muda::accelerator::Code;

#[cfg(target_os = "windows")]
use muda::accelerator::Modifiers;

use super::AppMenuAction;

pub use context_menu::show_context_menu;
pub use events::menu_events;

#[derive(Default, Clone)]
pub struct AppMenu {
    _muda_menu: Option<MudaMenu>,
    items: HashMap<AppMenuAction, MudaMenuItemKind>,
}

impl AppMenu {
    pub fn enable(&mut self, id: &AppMenuAction) {
        self.set_enabled(id, true);
    }

    pub fn disable(&mut self, id: &AppMenuAction) {
        self.set_enabled(id, false);
    }

    fn set_enabled(&mut self, id: &AppMenuAction, state: bool) {
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

    pub fn update(&mut self, id: &AppMenuAction) {
        if let Some(mik) = self.items.get(id) {
            match mik {
                MudaMenuItemKind::MenuItem(_) => (),
                MudaMenuItemKind::Submenu(_) => (),
                MudaMenuItemKind::Predefined(_) => (),
                MudaMenuItemKind::Check(_) => (),
                MudaMenuItemKind::Icon(_) => (),
            }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn init_for_hwnd(&self, hwnd: u64) {
        unsafe {
            if let Some(menu) = &self._muda_menu {
                _ = menu.init_for_hwnd(hwnd as isize);
            }
        };
    }

    pub fn new() -> Option<Self> {
        let menu: MudaMenu;
        let muda_menu: Option<MudaMenu>;
        let items: HashMap<AppMenuAction, MudaMenuItemKind>;
        (menu, items) = Self::prepare_app_menu();
        #[cfg(target_os = "macos")]
        menu.init_for_nsapp();
        muda_menu = Some(menu);
        Some(Self { _muda_menu: muda_menu, items })
    }

    fn prepare_app_menu() -> (MudaMenu, HashMap<AppMenuAction, MudaMenuItemKind>)
    {
        let menu = MudaMenu::default();
        let mut items: HashMap<AppMenuAction, MudaMenuItemKind> =
            HashMap::new();

        let modifier = CMD_OR_CTRL;

        #[cfg(target_os = "macos")]
        let submenu_app = MudaSubmenu::with_id("sub_app", "App", true);
        let submenu_file = MudaSubmenu::with_id("sub_file", "File", true);
        let submenu_view = MudaSubmenu::with_id("sub_view", "View", true);

        #[cfg(target_os = "macos")]
        let menu_item_about = MudaPredefinedMenuItem::about(
            None,
            Some(muda::AboutMetadata::default()),
        );

        #[cfg(target_os = "windows")]
        let menu_item_close_win = MudaMenuItem::with_id(
            AppMenuAction::CloseWindow,
            "Close Window",
            true,
            Some(Accelerator::new(Some(Modifiers::ALT), Code::F4)),
        );

        #[cfg(target_os = "macos")]
        let menu_item_quit = MudaMenuItem::with_id(
            AppMenuAction::Quit,
            "Quit",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyQ)),
        );

        let menu_item_open = MudaMenuItem::with_id(
            AppMenuAction::OpenFile,
            "Open File",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyO)),
        );

        let menu_item_save_as = MudaMenuItem::with_id(
            AppMenuAction::SaveAs,
            "Save As...",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyS)),
        );

        let menu_item_export_subtree = MudaMenuItem::with_id(
            AppMenuAction::ExportSubtree,
            "Export Subtree",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyE)),
        );

        let menu_item_export_pdf = MudaMenuItem::with_id(
            AppMenuAction::ExportPdf,
            "Export PDF",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyP)),
        );

        #[cfg(all(target_os = "windows", debug_assertions))]
        let menu_item_register_filetypes = MudaMenuItem::with_id(
            AppMenuAction::RegisterFileTypes,
            "Register File Associations",
            true,
            None,
        );

        #[cfg(all(target_os = "windows", debug_assertions))]
        let menu_item_unregister_filetypes = MudaMenuItem::with_id(
            AppMenuAction::UnregisterFileTypes,
            "Unregister File Associations",
            true,
            None,
        );

        let menu_item_toggle_search_bar = MudaMenuItem::with_id(
            AppMenuAction::ToggleSearchBar,
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
            _ = submenu_file.append(&MudaPredefinedMenuItem::separator()).ok();
            _ = submenu_file.append(&menu_item_register_filetypes).ok();
            _ = submenu_file.append(&menu_item_unregister_filetypes).ok();
            _ = submenu_file.append(&MudaPredefinedMenuItem::separator()).ok();
        }
        #[cfg(target_os = "windows")]
        {
            _ = submenu_file.append(&menu_item_close_win).ok();
        }

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
                MudaMenuItemKind::MenuItem(menu_item_quit),
            );
        }
        _ = items.insert(
            menu_item_open.clone().into(),
            MudaMenuItemKind::MenuItem(menu_item_open),
        );
        _ = items.insert(
            menu_item_save_as.clone().into(),
            MudaMenuItemKind::MenuItem(menu_item_save_as),
        );
        _ = items.insert(
            menu_item_export_subtree.clone().into(),
            MudaMenuItemKind::MenuItem(menu_item_export_subtree),
        );
        _ = items.insert(
            menu_item_export_pdf.clone().into(),
            MudaMenuItemKind::MenuItem(menu_item_export_pdf),
        );
        #[cfg(target_os = "windows")]
        {
            _ = items.insert(
                menu_item_close_win.clone().into(),
                MudaMenuItemKind::MenuItem(menu_item_close_win),
            );
            #[cfg(debug_assertions)]
            {
                _ = items.insert(
                    menu_item_register_filetypes.clone().into(),
                    MudaMenuItemKind::MenuItem(menu_item_register_filetypes),
                );
                _ = items.insert(
                    menu_item_unregister_filetypes.clone().into(),
                    MudaMenuItemKind::MenuItem(menu_item_unregister_filetypes),
                );
            }
        }
        _ = items.insert(
            menu_item_toggle_search_bar.clone().into(),
            MudaMenuItemKind::MenuItem(menu_item_toggle_search_bar),
        );

        (menu, items)
    }
}

impl From<MudaMenuItem> for AppMenuAction {
    fn from(value: MudaMenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<MudaCheckMenuItem> for AppMenuAction {
    fn from(value: MudaCheckMenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<MudaSubmenu> for AppMenuAction {
    fn from(value: MudaSubmenu) -> Self {
        value.id().0.clone().into()
    }
}
