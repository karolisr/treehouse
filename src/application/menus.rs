mod events;
pub use events::{MenuEvent, MenuEventReplyMsg};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{menu_events, prepare_app_menu};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::menu_events;
