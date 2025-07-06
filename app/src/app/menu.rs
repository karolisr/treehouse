mod events;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod muda;

pub use events::AppMenuItemId;

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub use muda::{AppMenu, ContextMenu, menu_events};

#[cfg(target_os = "linux")]
pub use linux::{AppMenu, ContextMenu};
