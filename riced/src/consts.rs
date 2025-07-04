pub(crate) mod icons;

use crate::*;

type Float = f32;

pub(crate) const ZERO: Float = 0e0;
pub(crate) const ONE: Float = 1e0;
pub(crate) const TWO: Float = 2e0;
pub(crate) const THREE: Float = 3e0;
pub(crate) const FOUR: Float = 4e0;
pub(crate) const FIVE: Float = 5e0;

pub const SF: Float = 1.0;
pub const TXT_SIZE: Float = SF * 13.0;

pub const LINE_H: Float = TXT_SIZE + SF * THREE;
pub const LINE_H_PIX: Pixels = Pixels(LINE_H);
pub const BORDER_W: Float = SF;
pub const WIDGET_H_UNIT: Float = (LINE_H / FIVE).floor() * TWO;
pub const WIDGET_RADIUS: Float = WIDGET_H_UNIT - SF;
pub const PADDING: Float = WIDGET_H_UNIT;
pub const BTN_H: Float = WIDGET_H_UNIT * FIVE;
pub const SLIDER_H: Float = WIDGET_H_UNIT * THREE;
pub const TOGGLER_H: Float = WIDGET_H_UNIT * THREE;
pub const CHECKBOX_H: Float = WIDGET_H_UNIT * THREE;
pub const SCROLLBAR_W: Float = WIDGET_H_UNIT * TWO;
pub const TEXT_INPUT_H: Float = BTN_H;

pub const MAC_OS_WINDOW_BORDER_RADIUS: Float = SF * 11.0;
