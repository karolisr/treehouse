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
        let (menu, items) = Self::prepare_app_menu();
        #[cfg(target_os = "macos")]
        menu.init_for_nsapp();
        Some(Self { _muda_menu: Some(menu), items })
    }

    fn prepare_app_menu() -> (MudaMenu, HashMap<AppMenuAction, MudaMenuItemKind>)
    {
        let modifier = CMD_OR_CTRL;
        let mut mis: HashMap<AppMenuAction, MudaMenuItemKind> = HashMap::new();

        let menu = MudaMenu::default();

        #[cfg(target_os = "macos")]
        {
            let subm_app = MudaSubmenu::with_id("sub_app", "App", true);
            _ = menu.append(&subm_app);

            let mi_about = MudaPredefinedMenuItem::about(
                None,
                Some(muda::AboutMetadata::default()),
            );
            add_predefined_menu_item(mi_about, &subm_app, None);

            let mi_quit = MudaMenuItem::with_id(
                AppMenuAction::Quit,
                "Quit",
                true,
                Some(Accelerator::new(Some(modifier), Code::KeyQ)),
            );
            add_menu_item(mi_quit, &subm_app, Some(&mut mis));
        }

        let subm_file = MudaSubmenu::with_id("sub_file", "File", true);
        _ = menu.append(&subm_file);

        let subm_view = MudaSubmenu::with_id("sub_view", "View", true);
        _ = menu.append(&subm_view);

        let mi_open = MudaMenuItem::with_id(
            AppMenuAction::OpenFile,
            "Open File",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyO)),
        );
        add_menu_item(mi_open, &subm_file, Some(&mut mis));

        let mi_save_as = MudaMenuItem::with_id(
            AppMenuAction::SaveAs,
            "Save As...",
            true,
            Some(Accelerator::new(Some(modifier), Code::KeyS)),
        );
        add_menu_item(mi_save_as, &subm_file, Some(&mut mis));

        let mi_export_subtree = MudaMenuItem::with_id(
            AppMenuAction::ExportSubtree,
            "Export Subtree",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyE)),
        );
        add_menu_item(mi_export_subtree, &subm_file, Some(&mut mis));

        let mi_export_pdf = MudaMenuItem::with_id(
            AppMenuAction::ExportPdf,
            "Export PDF",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyP)),
        );
        add_menu_item(mi_export_pdf, &subm_file, Some(&mut mis));

        #[cfg(all(target_os = "windows", debug_assertions))]
        {
            _ = subm_file.append(&MudaPredefinedMenuItem::separator());

            let mi_register_filetypes = MudaMenuItem::with_id(
                AppMenuAction::RegisterFileTypes,
                "Register File Associations",
                true,
                None,
            );
            add_menu_item(mi_register_filetypes, &subm_file, Some(&mut mis));

            let mi_unregister_filetypes = MudaMenuItem::with_id(
                AppMenuAction::UnregisterFileTypes,
                "Unregister File Associations",
                true,
                None,
            );
            add_menu_item(mi_unregister_filetypes, &subm_file, Some(&mut mis));

            _ = subm_file.append(&MudaPredefinedMenuItem::separator());
        }

        #[cfg(target_os = "windows")]
        {
            let mi_close_win = MudaMenuItem::with_id(
                AppMenuAction::CloseWindow,
                "Close Window",
                true,
                Some(Accelerator::new(Some(Modifiers::ALT), Code::F4)),
            );
            add_menu_item(mi_close_win, &subm_file, Some(&mut mis));
        }

        let mi_toggle_search_bar = MudaMenuItem::with_id(
            AppMenuAction::ToggleSearchBar,
            "Search...",
            false,
            Some(Accelerator::new(Some(modifier), Code::KeyF)),
        );
        add_menu_item(mi_toggle_search_bar, &subm_view, Some(&mut mis));

        (menu, mis)
    }
}

fn add_menu_item(
    item: MudaMenuItem,
    submenu: &MudaSubmenu,
    items: Option<&mut HashMap<AppMenuAction, MudaMenuItemKind>>,
) {
    _ = submenu.append(&item);
    if let Some(items) = items {
        _ = items.insert(item.clone().into(), MudaMenuItemKind::MenuItem(item));
    }
}

fn add_predefined_menu_item(
    item: MudaPredefinedMenuItem,
    submenu: &MudaSubmenu,
    items: Option<&mut HashMap<AppMenuAction, MudaMenuItemKind>>,
) {
    _ = submenu.append(&item);
    if let Some(items) = items {
        _ = items
            .insert(item.clone().into(), MudaMenuItemKind::Predefined(item));
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

impl From<MudaPredefinedMenuItem> for AppMenuAction {
    fn from(value: MudaPredefinedMenuItem) -> Self {
        value.id().0.clone().into()
    }
}
