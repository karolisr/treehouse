#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{os_events, register_ns_application_delegate_handlers};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{
    register_file_associations, setup_file_handling,
    unregister_file_associations,
};
