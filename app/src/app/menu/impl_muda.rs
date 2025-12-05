mod context_menu;
mod events;

use std::collections::HashMap;

use muda::accelerator::Accelerator as MudaAccelerator;
use muda::accelerator::CMD_OR_CTRL as MUDA_CMD_OR_CTRL;
use muda::accelerator::Code as MudaKeyCode;

#[cfg(target_os = "windows")]
use muda::accelerator::Modifiers as MudaModifiers;

use super::MenuItemId;

pub use context_menu::show_context_menu;
pub use events::menu_events;

#[derive(Default, Clone)]
pub struct AppMenu {
    #[cfg(target_os = "windows")]
    muda_menu: Option<muda::Menu>,
    items: HashMap<MenuItemId, muda::MenuItemKind>,
}

impl AppMenu {
    pub fn enable(&mut self, id: &MenuItemId) {
        self.set_enabled(id, true);
    }

    pub fn disable(&mut self, id: &MenuItemId) {
        self.set_enabled(id, false);
    }

    fn set_enabled(&mut self, id: &MenuItemId, state: bool) {
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

    pub fn update(&mut self, id: &MenuItemId) {
        if let Some(mik) = self.items.get(id) {
            match mik {
                muda::MenuItemKind::MenuItem(_) => (),
                muda::MenuItemKind::Submenu(_) => (),
                muda::MenuItemKind::Predefined(_) => (),
                muda::MenuItemKind::Check(_) => (),
                muda::MenuItemKind::Icon(_) => (),
            }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn init_for_hwnd(&self, hwnd: u64) {
        unsafe {
            if let Some(menu) = &self.muda_menu {
                _ = menu.init_for_hwnd(hwnd as isize);
            }
        };
    }

    pub fn new() -> Option<Self> {
        let (menu, items) = Self::prepare_app_menu();
        #[cfg(target_os = "macos")]
        menu.init_for_nsapp();
        Some(Self { items })
    }

    fn prepare_app_menu()
    -> (muda::Menu, HashMap<MenuItemId, muda::MenuItemKind>) {
        let modifier = MUDA_CMD_OR_CTRL;
        let mut mis: HashMap<MenuItemId, muda::MenuItemKind> = HashMap::new();

        let menu = muda::Menu::default();

        #[cfg(target_os = "macos")]
        {
            let subm_app = muda::Submenu::with_id("sub_app", "App", true);
            _ = menu.append(&subm_app);

            let mi_about = muda::PredefinedMenuItem::about(
                None,
                Some(muda::AboutMetadata::default()),
            );
            add_predefined_menu_item(mi_about, &subm_app, None);

            let mi_quit = muda::MenuItem::with_id(
                MenuItemId::Quit,
                "Quit",
                true,
                Some(MudaAccelerator::new(Some(modifier), MudaKeyCode::KeyQ)),
            );
            add_menu_item(mi_quit, &subm_app, Some(&mut mis));
        }

        let subm_file = muda::Submenu::with_id("sub_file", "File", true);
        _ = menu.append(&subm_file);

        let subm_view = muda::Submenu::with_id("sub_view", "View", true);
        _ = menu.append(&subm_view);

        let mi_open = muda::MenuItem::with_id(
            MenuItemId::OpenFile,
            "Open File",
            true,
            Some(MudaAccelerator::new(Some(modifier), MudaKeyCode::KeyO)),
        );
        add_menu_item(mi_open, &subm_file, Some(&mut mis));

        let mi_save_as = muda::MenuItem::with_id(
            MenuItemId::SaveAs,
            "Save As...",
            true,
            Some(MudaAccelerator::new(Some(modifier), MudaKeyCode::KeyS)),
        );
        add_menu_item(mi_save_as, &subm_file, Some(&mut mis));

        let mi_export_subtree = muda::MenuItem::with_id(
            MenuItemId::ExportSubtree,
            "Export Subtree",
            false,
            Some(MudaAccelerator::new(Some(modifier), MudaKeyCode::KeyE)),
        );
        add_menu_item(mi_export_subtree, &subm_file, Some(&mut mis));

        let mi_export_pdf = muda::MenuItem::with_id(
            MenuItemId::ExportPdf,
            "Export PDF",
            false,
            Some(MudaAccelerator::new(Some(modifier), MudaKeyCode::KeyP)),
        );
        add_menu_item(mi_export_pdf, &subm_file, Some(&mut mis));

        #[cfg(all(target_os = "windows", debug_assertions))]
        {
            _ = subm_file.append(&muda::PredefinedMenuItem::separator());

            let mi_register_filetypes = muda::MenuItem::with_id(
                MenuItemId::RegisterFileTypes,
                "Register File Associations",
                true,
                None,
            );
            add_menu_item(mi_register_filetypes, &subm_file, Some(&mut mis));

            let mi_unregister_filetypes = muda::MenuItem::with_id(
                MenuItemId::UnregisterFileTypes,
                "Unregister File Associations",
                true,
                None,
            );
            add_menu_item(mi_unregister_filetypes, &subm_file, Some(&mut mis));

            _ = subm_file.append(&muda::PredefinedMenuItem::separator());
        }

        #[cfg(target_os = "windows")]
        {
            let mi_close_win = muda::MenuItem::with_id(
                MenuItemId::CloseWindow,
                "Close Window",
                true,
                Some(MudaAccelerator::new(
                    Some(Modifiers::ALT),
                    MudaKeyCode::F4,
                )),
            );
            add_menu_item(mi_close_win, &subm_file, Some(&mut mis));
        }

        let mi_toggle_search_bar = muda::MenuItem::with_id(
            MenuItemId::ToggleSearchBar,
            "Search...",
            false,
            Some(MudaAccelerator::new(Some(modifier), MudaKeyCode::KeyF)),
        );
        add_menu_item(mi_toggle_search_bar, &subm_view, Some(&mut mis));

        (menu, mis)
    }
}

fn add_menu_item(
    item: muda::MenuItem,
    submenu: &muda::Submenu,
    items: Option<&mut HashMap<MenuItemId, muda::MenuItemKind>>,
) {
    _ = submenu.append(&item);
    if let Some(items) = items {
        _ = items
            .insert(item.clone().into(), muda::MenuItemKind::MenuItem(item));
    }
}

fn add_predefined_menu_item(
    item: muda::PredefinedMenuItem,
    submenu: &muda::Submenu,
    items: Option<&mut HashMap<MenuItemId, muda::MenuItemKind>>,
) {
    _ = submenu.append(&item);
    if let Some(items) = items {
        _ = items
            .insert(item.clone().into(), muda::MenuItemKind::Predefined(item));
    }
}

impl From<muda::MenuItem> for MenuItemId {
    fn from(value: muda::MenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::CheckMenuItem> for MenuItemId {
    fn from(value: muda::CheckMenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::Submenu> for MenuItemId {
    fn from(value: muda::Submenu) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::PredefinedMenuItem> for MenuItemId {
    fn from(value: muda::PredefinedMenuItem) -> Self {
        value.id().0.clone().into()
    }
}
