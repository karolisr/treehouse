mod events;
pub use events::{MenuEvent, MenuEventReplyMsg};

#[cfg(any(target_os = "windows", target_os = "macos"))]
mod macos_windows;
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub use macos_windows::{menu_events, prepare_app_menu};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::menu_events;
