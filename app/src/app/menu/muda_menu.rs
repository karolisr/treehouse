use super::events::MenuEvent;
use muda::{
    Menu, MenuItem, Submenu,
    accelerator::{Accelerator, Code, Modifiers},
};

pub fn prepare_app_menu() -> Menu {
    let menu = Menu::default();

    #[cfg(target_os = "windows")]
    let modifier = Modifiers::CONTROL;
    #[cfg(target_os = "macos")]
    let modifier = Modifiers::META;

    let submenu_app = Submenu::with_id("submenu_app", "App", true);
    let submenu_file = Submenu::with_id("submenu_file", "File", true);

    let menu_item_about =
        muda::PredefinedMenuItem::about(None, Some(muda::AboutMetadata::default()));

    // let menu_item_close_win = muda::PredefinedMenuItem::close_window(None);
    let menu_item_close_win = MenuItem::with_id(
        MenuEvent::CloseWindow,
        "Close Window",
        true,
        Some(Accelerator::new(Some(modifier), Code::KeyW)),
    );

    // let menu_item_quit = muda::PredefinedMenuItem::quit(None);
    let menu_item_quit = MenuItem::with_id(
        MenuEvent::Quit,
        "Quit",
        true,
        Some(Accelerator::new(Some(modifier), Code::KeyQ)),
    );

    let menu_item_open = MenuItem::with_id(
        MenuEvent::OpenFile,
        "Open File",
        true,
        Some(Accelerator::new(Some(modifier), Code::KeyO)),
    );

    let menu_item_save_as = MenuItem::with_id(
        MenuEvent::SaveAs,
        "Save As...",
        true,
        Some(Accelerator::new(Some(modifier), Code::KeyS)),
    );

    submenu_app.append(&menu_item_about).ok();
    submenu_app.append(&menu_item_quit).ok();

    submenu_file.append(&menu_item_open).ok();
    submenu_file.append(&menu_item_save_as).ok();
    submenu_file.append(&menu_item_close_win).ok();

    #[cfg(target_os = "macos")]
    menu.append(&submenu_app).ok();
    menu.append(&submenu_file).ok();

    menu
}
