#[cfg(target_os = "linux")]
pub const APP_ID: &str = "TreeHouse";

// use treeview::Float;
use treeview::SF;
use treeview::SidebarPosition;
pub use treeview::TXT_SIZE;

pub(crate) const SIDEBAR_POSITION: SidebarPosition = SidebarPosition::Right;
pub(crate) const APP_SCALE_FACTOR: f64 = 1e0 / SF as f64;
