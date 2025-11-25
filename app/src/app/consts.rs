#[cfg(target_os = "linux")]
pub const APP_ID: &str = "TreeHouse";

use treeview::SF;
pub use treeview::TXT_SIZE;

pub(crate) const APP_SCALE_FACTOR: f32 = 1e0 / SF;
