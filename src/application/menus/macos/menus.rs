pub use super::super::MenuEvent;
use muda::{
    Menu, MenuItem, Submenu,
    accelerator::{Accelerator, Code, Modifiers},
};

pub fn prepare_app_menu() -> Menu {
    let menu = Menu::default();

    let submenu_app = Submenu::with_id("submenu_app", "App", true);
    let submenu_file = Submenu::with_id("submenu_file", "File", true);

    // let menu_item_about =
    //     muda::PredefinedMenuItem::about(None, Some(muda::AboutMetadata::default()));

    // let menu_item_close_win = muda::PredefinedMenuItem::close_window(None);
    let menu_item_close_win = MenuItem::with_id(
        MenuEvent::CloseWindow,
        "Close Window",
        true,
        Some(Accelerator::new(Some(Modifiers::META), Code::KeyW)),
    );

    // let menu_item_quit = muda::PredefinedMenuItem::quit(None);
    let menu_item_quit = MenuItem::with_id(
        MenuEvent::Quit,
        "Quit",
        true,
        Some(Accelerator::new(Some(Modifiers::META), Code::KeyQ)),
    );

    let menu_item_open = MenuItem::with_id(
        MenuEvent::OpenFile,
        "Open File",
        true,
        Some(Accelerator::new(Some(Modifiers::META), Code::KeyO)),
    );

    // let menu_item_save = MenuItem::with_id(
    //     MenuEvent::Save,
    //     "Save",
    //     true,
    //     Some(Accelerator::new(Some(Modifiers::META), Code::KeyS)),
    // );

    // submenu_app.append(&menu_item_about).ok();
    submenu_app.append(&menu_item_quit).ok();

    submenu_file.append(&menu_item_open).ok();
    // submenu_file.append(&menu_item_save).ok();
    submenu_file.append(&menu_item_close_win).ok();

    menu.append(&submenu_app).ok();
    menu.append(&submenu_file).ok();

    menu
}
