mod context_menu;
mod menu_bar;
mod ui;

use riced::Element;

use crate::AppMsg;

use super::super::app_menu_bar::app_menu_bar;
use super::super::app_menu_item_id::AppMenuItemId;
use super::super::menu_model::Menu;

use menu_bar::menu_bar;

pub(crate) use context_menu::ContextMenu;
pub(crate) use context_menu::show_tv_context_menu;

#[derive(Clone, Debug)]
pub(crate) struct AppMenu {
    menu: Menu,
}

impl AppMenu {
    pub(crate) fn new() -> Self {
        // println!("app::menu::AppMenu::new");
        let menu = app_menu_bar();
        AppMenu { menu }
    }

    pub(crate) fn enable(&mut self, app_menu_item_id: AppMenuItemId) {
        // println!("app::menu::AppMenu -> enable({app_menu_item_id})");
    }

    pub(crate) fn disable(&mut self, app_menu_item_id: AppMenuItemId) {
        // println!("app::menu::AppMenu -> disable({app_menu_item_id})");
    }

    pub(crate) fn update(&mut self, app_menu_item_id: AppMenuItemId) {
        // println!("app::menu::AppMenu -> update({app_menu_item_id})");
    }

    pub(crate) fn menu_bar<'a>(
        &self,
        base: Element<'a, AppMsg>,
    ) -> Element<'a, AppMsg> {
        menu_bar(self.menu.clone(), base)
    }
}
