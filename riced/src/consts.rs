pub(crate) mod icons;

use crate::*;

type Float = f32;

pub(crate) const ZERO: Float = 0e0;
pub(crate) const ONE: Float = 1e0;
pub(crate) const TWO: Float = 2e0;
pub(crate) const THREE: Float = 3e0;
#[allow(dead_code)]
pub(crate) const FOUR: Float = 4e0;
pub(crate) const FIVE: Float = 5e0;

pub const SF: Float = ONE;
pub const TXT_SIZE: Float = SF * 12.0;

pub const LINE_H: Float = TXT_SIZE + SF * THREE;
pub const LINE_H_PIX: Pixels = Pixels(LINE_H);
pub const BORDER_W: Float = SF;
pub const WIDGET_H_UNIT: Float = (((LINE_H / FIVE) as i32 as Float) * TWO) - SF;
pub const WIDGET_RADIUS: Float = WIDGET_H_UNIT - SF;
pub const PADDING: Float = WIDGET_H_UNIT + SF;
pub const BTN_H1: Float = WIDGET_H_UNIT * FIVE + SF * TWO;
pub const BTN_H2: Float = BTN_H1 - SF * TWO;
pub const SLIDER_H: Float = WIDGET_H_UNIT * THREE;
pub const TOGGLER_H: Float = WIDGET_H_UNIT * THREE + SF;
pub const CHECKBOX_H: Float = WIDGET_H_UNIT * THREE;
pub const SCROLLBAR_W: Float = WIDGET_H_UNIT * TWO;
pub const TEXT_INPUT_H: Float = BTN_H2;

#[cfg(target_os = "macos")]
pub const WINDOW_BORDER_RADIUS: Float = SF * 11.0;

#[cfg(target_os = "linux")]
pub const WINDOW_BORDER_RADIUS: Float = WIDGET_RADIUS;

#[cfg(target_os = "windows")]
pub const WINDOW_BORDER_RADIUS: Float = SF * 5.0;
