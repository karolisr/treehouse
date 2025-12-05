mod menu_bar_spec;
mod menu_item_ids;

#[cfg(feature = "menu-muda")]
mod impl_muda;

#[cfg(feature = "menu-muda")]
pub use impl_muda::{AppMenu, menu_events, show_context_menu};

#[cfg(feature = "menu-custom")]
mod impl_custom;

#[cfg(feature = "menu-custom")]
pub use impl_custom::{AppMenu, ContextMenu, show_context_menu};

pub(crate) use menu_bar_spec::menu_bar_items;
pub(crate) use menu_item_ids::MenuItemId;

#[derive(Clone, Debug)]
pub struct MenuItemAccelerator {}

#[derive(Clone, Debug)]
pub enum MenuItem {
    Item {
        id: MenuItemId,
        label: String,
        enabled: bool,
        accelerator: Option<MenuItemAccelerator>,
    },
}

impl MenuItem {
    fn new_item(
        id: MenuItemId,
        label: String,
        enabled: bool,
        accelerator: Option<MenuItemAccelerator>,
    ) -> Self {
        Self::Item { id, label, enabled, accelerator }
    }
}
