mod context_menu;
mod menu_bar;
mod ui;

use riced::Element;
use riced::Key as RicedKey;
use riced::Modifiers as RicedModifiers;
use riced::SF;

use crate::AppMsg;
use crate::app::menu::menu_model::Accelerator;
use crate::app::menu::menu_model::KeyCode;
use crate::app::menu::menu_model::Modifier;

use super::super::app_menu_bar::app_menu_bar;
use super::super::app_menu_item_id::AppMenuItemId;
use super::super::menu_model::Menu;

use menu_bar::menu_bar;

pub(crate) use context_menu::ContextMenu;
pub(crate) use context_menu::show_tv_context_menu;

pub(super) const SUBMENU_W: f32 = 165.0 * SF;

#[derive(Clone, Debug)]
pub(crate) struct AppMenu {
    menu: Menu,
}

impl AppMenu {
    pub(crate) fn new() -> Self {
        let menu = app_menu_bar();
        AppMenu { menu }
    }

    pub(crate) fn enable(&mut self, app_menu_item_id: AppMenuItemId) {
        self.set_enabled(app_menu_item_id, true);
    }

    pub(crate) fn disable(&mut self, app_menu_item_id: AppMenuItemId) {
        self.set_enabled(app_menu_item_id, false);
    }

    fn set_enabled(&mut self, app_menu_item_id: AppMenuItemId, state: bool) {
        self.menu.set_enabled(app_menu_item_id.into(), state);
    }

    pub(crate) fn update(&mut self, _app_menu_item_id: AppMenuItemId) {}

    pub(crate) fn process_menu_accelerator(
        &self,
        riced_modifiers: RicedModifiers,
        riced_key: RicedKey,
    ) -> Option<AppMenuItemId> {
        let modifier: Modifier = riced_modifiers.into();
        let key_code: KeyCode = riced_key.into();
        let accel = Accelerator { modifier: Some(modifier), key: key_code };
        let menu_item_id = self.menu.menu_item_id_for_accelerator(accel);
        menu_item_id.map(std::convert::Into::into)
    }

    pub(crate) fn menu_bar<'a>(
        &self,
        base: Element<'a, AppMsg>,
    ) -> Element<'a, AppMsg> {
        menu_bar(self.menu.clone(), base)
    }
}
