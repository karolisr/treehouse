use super::AppMenuItemId;
use treeview::SidebarPosition;

#[derive(Default, Clone)]
pub struct AppMenu {}

impl AppMenu {
    pub fn new(sidebar_pos: SidebarPosition) -> Option<Self> { None }
    pub fn enable(&mut self, id: &AppMenuItemId) {}
    pub fn disable(&mut self, id: &AppMenuItemId) {}
    pub fn update(&mut self, id: &AppMenuItemId) {}
}
