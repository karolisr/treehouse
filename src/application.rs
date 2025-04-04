use iced::{Pixels, widget::text::LineHeight};

pub mod app;
pub mod colors;
#[cfg(target_os = "macos")]
pub mod macos;
pub mod menus;
pub mod treeview;
pub mod windows;

pub type Float = f32;

pub const SF: Float = 1e0;
pub const APP_SCALE_FACTOR: f64 = 1e0 / SF as f64;

pub const TEXT_SIZE: Float = 14.0 * SF;
pub const LINE_H: LineHeight = LineHeight::Absolute(Pixels(TEXT_SIZE * 1.5));
pub const PADDING: Float = 1e1 * SF;
pub const PADDING_INNER: Float = PADDING / 1.5;
pub const SPACING: Float = 1e1 * SF;
pub const SCROLL_BAR_W: Float = PADDING;
