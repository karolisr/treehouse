mod events;

// #[cfg(any(target_os = "windows", target_os = "macos"))]
#[cfg(feature = "menu-muda")]
mod menu_muda;

// #[cfg(any(target_os = "windows", target_os = "macos"))]
#[cfg(feature = "menu-muda")]
pub use menu_muda::{AppMenu, menu_events, show_context_menu};

// #[cfg(target_os = "linux")]
#[cfg(feature = "menu-custom")]
mod menu_custom;

// #[cfg(target_os = "linux")]
#[cfg(feature = "menu-custom")]
pub use menu_custom::{AppMenu, ContextMenu, show_context_menu};

pub use events::AppMenuItemId;
