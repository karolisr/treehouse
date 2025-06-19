pub(crate) mod icons;

use crate::*;

type Float = f32;

pub const SF: Float = 4e0;
pub const TXT_SIZE: Float = SF * 13e0;
pub const LINE_HEIGHT: Pixels = Pixels(TXT_SIZE + TXT_SIZE / 2e0);
pub const WIDGET_H_UNIT: Float = (LINE_HEIGHT.0 / 4e0 + SF * 1.5).floor() + 0.5;
pub const BTN_H: Float = WIDGET_H_UNIT * 5e0 - SF;
pub const SLIDER_H: Float = WIDGET_H_UNIT * 3e0;
pub const TOGGLER_H: Float = WIDGET_H_UNIT * 3e0;
pub const CHECKBOX_H: Float = WIDGET_H_UNIT * 3e0 - SF * 3e0;
pub const TEXT_INPUT_H: Float = TXT_SIZE + SF * 2e0;
pub const BORDER_W: Float = SF;
pub const SCROLL_BAR_W: Float = SF * 1e1;
pub const PADDING: Float = ((TXT_SIZE / (1e1 * SF)).floor().max(1e0) / 2e0) * 1e1 * SF;
pub const WIDGET_RADIUS: Float = WIDGET_H_UNIT / 2e0;
