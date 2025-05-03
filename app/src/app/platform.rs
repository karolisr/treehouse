#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{os_events, register_ns_application_delegate_handlers};
