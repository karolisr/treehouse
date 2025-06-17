#![allow(dead_code)]

mod fill;
pub(crate) mod icons;
mod stroke;
mod text;

pub(crate) use fill::*;
pub(crate) use stroke::*;
pub(crate) use text::*;

use crate::iced::*;
use crate::*;

pub const SF: Float = FOUR;
pub const TXT_SIZE: Float = SF * 13e0;
pub const LINE_HEIGHT: Pixels = Pixels(TXT_SIZE + TXT_SIZE / TWO);

pub(crate) const FNT_NAME: &str = "JetBrains Mono";
pub(crate) const FNT_NAME_LAB: &str = FNT_NAME;

pub(crate) const TIP_LAB_SIZE_IDX: u16 = 12;
pub(crate) const INTERNAL_LAB_SIZE_IDX: u16 = 12;
pub(crate) const BRANCH_LAB_SIZE_IDX: u16 = 12;

pub(crate) const WIDGET_H_UNIT: Float = (LINE_HEIGHT.0 / FOUR + SF * 1.5).floor() + 0.5;

pub(crate) const BTN_H: Float = WIDGET_H_UNIT * FIVE - SF;
pub(crate) const SLIDER_H: Float = WIDGET_H_UNIT * THREE;
pub(crate) const TOGGLER_H: Float = WIDGET_H_UNIT * THREE;
pub(crate) const CHECKBOX_H: Float = WIDGET_H_UNIT * THREE - SF * THREE;
pub(crate) const TEXT_INPUT_H: Float = TXT_SIZE + SF * TWO;

pub(crate) const SIDE_BAR_W: Float = SF * 19e1;
pub(crate) const TREE_CNV_SIZE_DELTA: Float = SF * 5e2;

pub(crate) const BORDER_W: Float = SF;
pub(crate) const SCROLL_BAR_W: Float = SF * TEN;
pub(crate) const PADDING: Float = ((TXT_SIZE / (TEN * SF)).floor().max(ONE) / TWO) * TEN * SF;
pub(crate) const TREE_PADDING: Float = SF;
pub(crate) const WIDGET_RADIUS: Float = WIDGET_H_UNIT / TWO;

pub(crate) const STRK_EDGE: Strk = STRK_1_BLK;
pub(crate) const STRK_ROOT: Strk = Strk { line_dash: DASH_002, ..STRK_EDGE };
pub(crate) const STRK_CRSR_LINE: Strk = STRK_1_BLU_75;

pub(crate) const STRK_NODE_HOVER: Strk = STRK_1_BLU;
pub(crate) const STRK_NODE_SELECTED: Strk = STRK_1_RED;
pub(crate) const STRK_NODE_FILTERED: Strk = STRK_1_GRN;
pub(crate) const STRK_NODE_CURRENT: Strk = STRK_1_MAG;

pub(crate) const FILL_NODE_HOVER: CnvFill = FILL_YEL_25;
pub(crate) const FILL_NODE_SELECTED: CnvFill = FILL_YEL_50;
pub(crate) const FILL_NODE_FILTERED: CnvFill = FILL_CYA_25;
pub(crate) const FILL_NODE_CURRENT: CnvFill = FILL_RED_25;

pub(crate) const EPSILON: Float = Float::EPSILON;

pub(crate) const ZERO: Float = 0e0;
pub(crate) const ONE: Float = 1e0;
pub(crate) const TWO: Float = 2e0;
pub(crate) const THREE: Float = 3e0;
pub(crate) const FOUR: Float = 4e0;
pub(crate) const FIVE: Float = 5e0;
pub(crate) const TEN: Float = 1e1;

pub(crate) const E: Float = float::consts::E;
pub(crate) const PI: Float = float::consts::PI;
pub(crate) const TAU: Float = float::consts::TAU;
pub(crate) const FRAC_PI_2: Float = float::consts::FRAC_PI_2;

pub(crate) const ORIGIN: Point = Point::ORIGIN;
