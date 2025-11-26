use std::collections::HashMap;

use super::super::AppMsg;
use super::AppMenuItemId;
use riced::{
    Button, Column, Container, Element, PADDING, Point, SF, Task, WindowId,
    btn_txt, container, context_menu_element, sty_cont_message,
};
use treeview::TvContextMenuListing;

#[derive(Default, Clone, Debug)]
pub struct AppMenu {
    items: Vec<MenuItem>,
    item_indexes: HashMap<AppMenuItemId, usize>,
}

#[derive(Clone, Debug)]
pub struct MenuItem {
    id: AppMenuItemId,
    label: String,
    enabled: bool,
}

#[derive(Default, Clone, Debug)]
pub struct ContextMenu {
    menu: AppMenu,
    position: Point,
}

impl AppMenu {
    pub fn new() -> Option<Self> {
        Some(Self::default())
    }

    pub fn with_items(items: Vec<MenuItem>) -> Self {
        let mut item_indexes: HashMap<AppMenuItemId, usize> = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            _ = item_indexes.insert(item.id.clone(), i);
        }
        Self { items, item_indexes }
    }

    pub fn enable(&mut self, id: &AppMenuItemId) {
        self.set_enabled(id, true);
    }

    pub fn disable(&mut self, id: &AppMenuItemId) {
        self.set_enabled(id, false);
    }

    fn set_enabled(&mut self, id: &AppMenuItemId, state: bool) {
        if let Some(&item_index) = self.item_indexes.get(id) {
            self.items[item_index].enabled = state;
        }
    }

    pub fn update(&mut self, _id: &AppMenuItemId) {
        // println!("AppMenu::update({id})");
    }
}

impl ContextMenu {
    pub fn new(items: Vec<MenuItem>, position: Point) -> Self {
        let menu = AppMenu::with_items(items);
        Self { menu, position }
    }

    pub fn element<'a>(
        &'a self,
        base: Element<'a, AppMsg>,
    ) -> Element<'a, AppMsg> {
        context_menu_element(
            base,
            self.context_menu_container(),
            self.position,
            AppMsg::HideContextMenu,
        )
    }

    fn context_menu_container(&self) -> Container<'_, AppMsg> {
        let mut btns: Vec<Element<'_, AppMsg>> = Vec::new();
        for item in &self.menu.items {
            let btn: Button<'_, AppMsg> = btn_txt(
                &item.label,
                match item.enabled {
                    true => Some(AppMsg::MenuEvent(item.id.clone())),
                    false => None,
                },
            );
            btns.push(btn.width(SF * 100.0).into());
        }
        container(Column::from_vec(btns).spacing(PADDING).padding(PADDING))
            .style(sty_cont_message)
    }
}

pub fn show_context_menu(
    tree_view_context_menu_listing: TvContextMenuListing,
    _winid: WindowId,
) -> Task<AppMsg> {
    Task::done(AppMsg::SetCustomContextMenu(
        tree_view_context_menu_listing.into(),
    ))
}

impl MenuItem {
    fn new(id: AppMenuItemId, label: String, enabled: bool) -> Self {
        Self { id, label, enabled }
    }
}

impl From<TvContextMenuListing> for ContextMenu {
    fn from(tv_context_menu_listing: TvContextMenuListing) -> Self {
        let mut menu_items: Vec<MenuItem> = Vec::new();
        tv_context_menu_listing.items().iter().enumerate().for_each(
            |(idx, item)| {
                let menu_item = MenuItem::new(
                    AppMenuItemId::ContextMenuIndex(idx),
                    item.label.clone(),
                    item.enabled,
                );
                menu_items.push(menu_item);
            },
        );
        ContextMenu::new(menu_items, tv_context_menu_listing.position())
    }
}
