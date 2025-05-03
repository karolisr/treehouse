#[cfg(target_os = "linux")]
pub const APP_ID: &str = "TreeHouse";

pub const SF: f32 = treeview::SF;
pub const TEXT_SIZE: f32 = treeview::TEXT_SIZE;
pub const APP_SCALE_FACTOR: f64 = 1e0 / SF as f64;
pub const ANTIALIASING: bool = true;
