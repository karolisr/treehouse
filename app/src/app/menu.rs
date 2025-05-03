mod events;
// #[cfg(target_os = "linux")]
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod muda_events;
#[cfg(any(target_os = "windows", target_os = "macos"))]
mod muda_menu;

pub use events::MenuEvent;
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub use muda_events::menu_events;
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub use muda_menu::prepare_app_menu;
