use super::AppMenuItemId;
use super::menu_model::Accelerator;
use super::menu_model::KeyCode;
use super::menu_model::Menu;
use super::menu_model::MenuItem;
use super::menu_model::Modifier;

pub(crate) fn app_menu_bar() -> Menu {
    let modifier = Some(Modifier::CmdOrCtrl);

    let mut menu = Menu::new();

    #[cfg(target_os = "macos")]
    {
        let mut subm_app_items = vec![];

        let mi_about =
            MenuItem::item("About", true, AppMenuItemId::About, None);

        let mi_quit = MenuItem::item(
            "Quit",
            true,
            AppMenuItemId::Quit,
            Some(Accelerator { modifier, key: KeyCode::KeyQ }),
        );

        subm_app_items.push(mi_about);
        subm_app_items.push(MenuItem::separator());
        subm_app_items.push(mi_quit);

        let subm_app = MenuItem::submenu(
            "App",
            true,
            AppMenuItemId::Submenu,
            subm_app_items,
        );

        menu.append(subm_app);
    }

    let mut subm_file_items = vec![];
    let mut subm_view_items = vec![];

    let mi_open = MenuItem::item(
        "Open File",
        true,
        AppMenuItemId::OpenFile,
        Some(Accelerator { modifier, key: KeyCode::KeyO }),
    );

    let mi_save_as = MenuItem::item(
        "Save As...",
        true,
        AppMenuItemId::SaveAs,
        Some(Accelerator { modifier, key: KeyCode::KeyS }),
    );

    let mi_export_subtree = MenuItem::item(
        "Save Current Subtree",
        false,
        AppMenuItemId::ExportSubtree,
        Some(Accelerator { modifier, key: KeyCode::KeyE }),
    );

    let mi_export_pdf = MenuItem::item(
        "Export as PDF",
        false,
        AppMenuItemId::ExportPdf,
        Some(Accelerator { modifier, key: KeyCode::KeyP }),
    );

    let mi_toggle_search_bar = MenuItem::item(
        "Find",
        false,
        AppMenuItemId::ToggleSearchBar,
        Some(Accelerator { modifier, key: KeyCode::KeyF }),
    );

    subm_file_items.push(mi_open);
    subm_file_items.push(MenuItem::separator());
    subm_file_items.push(mi_save_as);
    subm_file_items.push(mi_export_subtree);
    subm_file_items.push(MenuItem::separator());
    subm_file_items.push(mi_export_pdf);

    #[cfg(all(target_os = "windows", debug_assertions))]
    {
        let mi_register_filetypes = MenuItem::item(
            "Register File Associations",
            true,
            AppMenuItemId::RegisterFileTypes,
            None,
        );

        let mi_unregister_filetypes = MenuItem::item(
            "Unregister File Associations",
            true,
            AppMenuItemId::UnregisterFileTypes,
            None,
        );

        subm_file_items.push(MenuItem::separator());
        subm_file_items.push(mi_register_filetypes);
        subm_file_items.push(mi_unregister_filetypes);
        subm_file_items.push(MenuItem::separator());
    }

    #[cfg(target_os = "windows")]
    {
        let mi_close_win = MenuItem::item(
            "Close Window",
            true,
            AppMenuItemId::CloseWindow,
            Some(Accelerator {
                modifier: Some(Modifier::Alt),
                key: KeyCode::F4,
            }),
        );
        subm_file_items.push(mi_close_win);
    }

    subm_view_items.push(mi_toggle_search_bar);

    let subm_file = MenuItem::submenu(
        "File",
        true,
        AppMenuItemId::Submenu,
        subm_file_items,
    );

    let subm_view = MenuItem::submenu(
        "View",
        true,
        AppMenuItemId::Submenu,
        subm_view_items,
    );

    menu.append(subm_file);
    menu.append(subm_view);

    menu
}
