#![allow(dead_code)]

mod fill;
mod stroke;
mod text;

pub(crate) use fill::*;
pub(crate) use stroke::*;
pub(crate) use text::*;

use crate::CnvStrk as Strk;
use crate::*;

pub(crate) const FNT_NAME: &str = "JetBrains Mono";
pub(crate) const FNT_NAME_LAB: &str = FNT_NAME;

pub(crate) const TIP_LAB_SIZE_IDX: u16 = 12;
pub(crate) const INTERNAL_LAB_SIZE_IDX: u16 = 12;
pub(crate) const BRANCH_LAB_SIZE_IDX: u16 = 12;

pub(crate) const SIDE_BAR_W: Float = TXT_SIZE * 14.0;
pub(crate) const TREE_CNV_SIZE_DELTA: Float = SF * 5e2;

pub(crate) const PLOT_PADDING: Float = SF;

pub(crate) const STRK_EDGE: Strk = STRK_1_BLK;
pub(crate) const STRK_EDGE_LAB_ALN: Strk =
    Strk { line_dash: DASH_001, ..STRK_H_BLK };
pub(crate) const STRK_ROOT: Strk = Strk { line_dash: DASH_002, ..STRK_EDGE };
pub(crate) const STRK_CRSR_LINE: Strk = STRK_1_RED;

pub(crate) const STRK_NODE_HOVER: Strk = STRK_1_BLU;
pub(crate) const STRK_NODE_SELECTED: Strk = STRK_1_RED;
pub(crate) const STRK_NODE_FILTERED: Strk = STRK_1_GRN;
pub(crate) const STRK_NODE_CURRENT: Strk = STRK_1_MAG;

pub(crate) const FILL_NODE_HOVER: CnvFill = FILL_YEL_25;
pub(crate) const FILL_NODE_SELECTED: CnvFill = FILL_YEL_50;
pub(crate) const FILL_NODE_FILTERED: CnvFill = FILL_CYA_25;
pub(crate) const FILL_NODE_CURRENT: CnvFill = FILL_RED_25;

pub(crate) const EPSILON: Float = Float::EPSILON;

pub(crate) const ZRO: Float = 0e0;
pub(crate) const ONE: Float = 1e0;
pub(crate) const TWO: Float = 2e0;
pub(crate) const TEN: Float = 1e1;

pub(crate) const E: Float = float::consts::E;
pub(crate) const PI: Float = float::consts::PI;
pub(crate) const TAU: Float = float::consts::TAU;
pub(crate) const FRAC_PI_2: Float = float::consts::FRAC_PI_2;
pub(crate) const FRAC_PI_3: Float = float::consts::FRAC_PI_3;
pub(crate) const FRAC_PI_4: Float = float::consts::FRAC_PI_4;
pub(crate) const FRAC_PI_6: Float = float::consts::FRAC_PI_6;

pub(crate) const ORIGIN: Point = Point::ORIGIN;
