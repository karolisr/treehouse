mod context_menu;

use std::collections::HashMap;

use iced_aw::Menu as AwMenu;
use iced_aw::menu::Item as AwMenuItem;
use iced_aw::menu_bar as aw_menu_bar;
use iced_aw::menu_items as aw_menu_items;

use crate::AppMsg;

use super::AppMenuAction;

pub use context_menu::ContextMenu;
pub use context_menu::show_context_menu;

#[derive(Clone, Debug)]
pub struct MenuItem {
    id: AppMenuAction,
    label: String,
    enabled: bool,
}

impl MenuItem {
    fn new(id: AppMenuAction, label: String, enabled: bool) -> Self {
        Self { id, label, enabled }
    }
}

#[derive(Default, Clone, Debug)]
pub struct AppMenu {
    items: Vec<MenuItem>,
    item_indexes: HashMap<AppMenuAction, usize>,
}

impl AppMenu {
    pub fn new() -> Option<Self> {
        Some(Self::default())
    }

    pub fn with_items(items: Vec<MenuItem>) -> Self {
        let mut item_indexes: HashMap<AppMenuAction, usize> = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            _ = item_indexes.insert(item.id.clone(), i);
        }
        Self { items, item_indexes }
    }

    pub fn enable(&mut self, id: &AppMenuAction) {
        self.set_enabled(id, true);
    }

    pub fn disable(&mut self, id: &AppMenuAction) {
        self.set_enabled(id, false);
    }

    fn set_enabled(&mut self, id: &AppMenuAction, state: bool) {
        if let Some(&item_index) = self.item_indexes.get(id) {
            self.items[item_index].enabled = state;
        }
    }

    pub fn update(&mut self, id: &AppMenuAction) {
        println!("AppMenu::update({id})");
    }
}
