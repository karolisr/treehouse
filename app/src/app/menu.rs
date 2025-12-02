mod actions;

#[cfg(feature = "menu-muda")]
mod impl_muda;

#[cfg(feature = "menu-muda")]
pub use impl_muda::{AppMenu, menu_events, show_context_menu};

#[cfg(feature = "menu-custom")]
mod impl_custom;

#[cfg(feature = "menu-custom")]
pub use impl_custom::{AppMenu, ContextMenu, show_context_menu};

pub use actions::AppMenuAction;
