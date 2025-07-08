use super::AppMenuItemId;
use treeview::{SidebarPosition, TvContextMenuListing};

#[derive(Default, Clone)]
pub struct AppMenu {}

#[derive(Default, Clone)]
pub struct ContextMenu {}

impl AppMenu {
    pub fn new(sidebar_pos: SidebarPosition) -> Option<Self> {
        None
    }
    pub fn enable(&mut self, id: &AppMenuItemId) {}
    pub fn disable(&mut self, id: &AppMenuItemId) {}
    pub fn update(&mut self, id: &AppMenuItemId) {}
}

impl ContextMenu {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<TvContextMenuListing> for ContextMenu {
    fn from(tv_context_menu_listing: TvContextMenuListing) -> Self {
        tv_context_menu_listing
            .items()
            .iter()
            .enumerate()
            .for_each(|(idx, item)| {});
        ContextMenu::new()
    }
}
