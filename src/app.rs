mod application;
#[cfg(target_os = "macos")]
mod macos;
mod menus;
mod treeview;
mod windows;

use crate::Float;
pub use application::App;
pub(super) use application::AppMsg;
use iced::{Pixels, widget::text::LineHeight};
pub(super) use treeview::{TreeView, TreeViewMsg};

#[cfg(target_os = "linux")]
pub const APP_ID: &str = "TreeHouse";

pub const SF: Float = 1e0;
pub const APP_SCALE_FACTOR: f64 = 1e0 / SF as f64;
pub const TEXT_SIZE: Float = 14.0 * SF;
pub const LINE_H: LineHeight = LineHeight::Absolute(Pixels(TEXT_SIZE * 1.5));
pub const PADDING: Float = 1e1 * SF;
pub const PADDING_INNER: Float = PADDING / 1.5;
pub const SCROLL_BAR_W: Float = PADDING;
pub const TREE_LAB_FONT_NAME: &str = "JetBrains Mono";
pub const SIDE_COL_W: Float = SF * 2e2;
pub const SCROLL_TOOL_W: Float = SCROLL_BAR_W + PADDING * 2e0 + SF;
