#![allow(dead_code)]

use crate::iced::*;
use crate::*;

pub const SF: Float = 1e0;
pub const TXT_SIZE: Float = SF * 14.0;
pub(crate) const BORDER_W: Float = SF * ONE;
pub const TXT_LINE_HEIGHT: Pixels = Pixels((TXT_SIZE + TXT_SIZE / TWO + SF * TWO).floor());

pub(crate) const FNT_NAME: &str = "JetBrains Mono";
pub(crate) const FNT_NAME_LAB: &str = FNT_NAME;

pub(crate) const TIP_LAB_SIZE_IDX: u16 = 12;
pub(crate) const INTERNAL_LAB_SIZE_IDX: u16 = 12;
pub(crate) const BRANCH_LAB_SIZE_IDX: u16 = 12;

pub(crate) const SIDE_BAR_W: Float = SF * 220e0;
pub(crate) const SCROLL_BAR_W: Float = SF * TEN;

pub(crate) const PADDING: Float = TXT_SIZE / TWO;
pub(crate) const TREE_PADDING: Float = PADDING;

pub(crate) const WIDGET_RADIUS: Float = PADDING / TWO;

pub(crate) const TREE_CANVAS_SIZE_DELTA: Float = SF * 7e2;

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

pub(crate) const FILL_BLK: CnvFill = CnvFill { style: Solid(Clr::BLK), ..TEMPLATE_FILL };
pub(crate) const FILL_RED: CnvFill = CnvFill { style: Solid(Clr::RED), ..TEMPLATE_FILL };
pub(crate) const FILL_GRN: CnvFill = CnvFill { style: Solid(Clr::GRN), ..TEMPLATE_FILL };
pub(crate) const FILL_BLU: CnvFill = CnvFill { style: Solid(Clr::BLU), ..TEMPLATE_FILL };
pub(crate) const FILL_YEL: CnvFill = CnvFill { style: Solid(Clr::YEL), ..TEMPLATE_FILL };
pub(crate) const FILL_CYA: CnvFill = CnvFill { style: Solid(Clr::CYA), ..TEMPLATE_FILL };
pub(crate) const FILL_MAG: CnvFill = CnvFill { style: Solid(Clr::MAG), ..TEMPLATE_FILL };

pub(crate) const FILL_BLK_25: CnvFill = CnvFill { style: Solid(Clr::BLK_25), ..TEMPLATE_FILL };
pub(crate) const FILL_RED_25: CnvFill = CnvFill { style: Solid(Clr::RED_25), ..TEMPLATE_FILL };
pub(crate) const FILL_GRN_25: CnvFill = CnvFill { style: Solid(Clr::GRN_25), ..TEMPLATE_FILL };
pub(crate) const FILL_BLU_25: CnvFill = CnvFill { style: Solid(Clr::BLU_25), ..TEMPLATE_FILL };
pub(crate) const FILL_YEL_25: CnvFill = CnvFill { style: Solid(Clr::YEL_25), ..TEMPLATE_FILL };
pub(crate) const FILL_CYA_25: CnvFill = CnvFill { style: Solid(Clr::CYA_25), ..TEMPLATE_FILL };
pub(crate) const FILL_MAG_25: CnvFill = CnvFill { style: Solid(Clr::MAG_25), ..TEMPLATE_FILL };

pub(crate) const FILL_BLK_50: CnvFill = CnvFill { style: Solid(Clr::BLK_50), ..TEMPLATE_FILL };
pub(crate) const FILL_RED_50: CnvFill = CnvFill { style: Solid(Clr::RED_50), ..TEMPLATE_FILL };
pub(crate) const FILL_GRN_50: CnvFill = CnvFill { style: Solid(Clr::GRN_50), ..TEMPLATE_FILL };
pub(crate) const FILL_BLU_50: CnvFill = CnvFill { style: Solid(Clr::BLU_50), ..TEMPLATE_FILL };
pub(crate) const FILL_YEL_50: CnvFill = CnvFill { style: Solid(Clr::YEL_50), ..TEMPLATE_FILL };
pub(crate) const FILL_CYA_50: CnvFill = CnvFill { style: Solid(Clr::CYA_50), ..TEMPLATE_FILL };
pub(crate) const FILL_MAG_50: CnvFill = CnvFill { style: Solid(Clr::MAG_50), ..TEMPLATE_FILL };

pub(crate) const FILL_BLK_75: CnvFill = CnvFill { style: Solid(Clr::BLK_75), ..TEMPLATE_FILL };
pub(crate) const FILL_RED_75: CnvFill = CnvFill { style: Solid(Clr::RED_75), ..TEMPLATE_FILL };
pub(crate) const FILL_GRN_75: CnvFill = CnvFill { style: Solid(Clr::GRN_75), ..TEMPLATE_FILL };
pub(crate) const FILL_BLU_75: CnvFill = CnvFill { style: Solid(Clr::BLU_75), ..TEMPLATE_FILL };
pub(crate) const FILL_YEL_75: CnvFill = CnvFill { style: Solid(Clr::YEL_75), ..TEMPLATE_FILL };
pub(crate) const FILL_CYA_75: CnvFill = CnvFill { style: Solid(Clr::CYA_75), ..TEMPLATE_FILL };
pub(crate) const FILL_MAG_75: CnvFill = CnvFill { style: Solid(Clr::MAG_75), ..TEMPLATE_FILL };

pub(crate) const EPS: Float = Float::EPSILON;
pub(crate) const ZRO: Float = 0e0;
pub(crate) const ONE: Float = 1e0;
pub(crate) const TWO: Float = 2e0;
pub(crate) const TEN: Float = 1e1;
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TAU: Float = std::f32::consts::TAU;
pub(crate) const FRAC_PI_2: Float = std::f32::consts::FRAC_PI_2;

pub(crate) const ORIGIN: Point = Point::ORIGIN;

pub(crate) const DASH_001: LineDash = LineDash { segments: &[SF * 1e0, SF * 1e0], offset: 0 };
pub(crate) const DASH_002: LineDash = LineDash { segments: &[SF * 2e0, SF * 2e0], offset: 0 };
pub(crate) const DASH_003: LineDash = LineDash { segments: &[SF * 3e0, SF * 3e0], offset: 0 };
pub(crate) const DASH_004: LineDash = LineDash { segments: &[SF * 4e0, SF * 4e0], offset: 0 };
pub(crate) const DASH_005: LineDash = LineDash { segments: &[SF * 5e0, SF * 5e0], offset: 0 };
pub(crate) const DASH_006: LineDash = LineDash { segments: &[SF * 6e0, SF * 6e0], offset: 0 };
pub(crate) const DASH_007: LineDash = LineDash { segments: &[SF * 7e0, SF * 7e0], offset: 0 };
pub(crate) const DASH_008: LineDash = LineDash { segments: &[SF * 8e0, SF * 8e0], offset: 0 };
pub(crate) const DASH_009: LineDash = LineDash { segments: &[SF * 9e0, SF * 9e0], offset: 0 };
pub(crate) const DASH_010: LineDash = LineDash { segments: &[SF * 1e1, SF * 1e1], offset: 0 };

const STRK_1: Strk = Strk { width: SF * 1e0, ..TEMPLATE_STRK };
const STRK_2: Strk = Strk { width: SF * 2e0, ..TEMPLATE_STRK };
const STRK_3: Strk = Strk { width: SF * 3e0, ..TEMPLATE_STRK };
const STRK_4: Strk = Strk { width: SF * 4e0, ..TEMPLATE_STRK };
const STRK_5: Strk = Strk { width: SF * 5e0, ..TEMPLATE_STRK };

pub(crate) const STRK_1_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_1 };
pub(crate) const STRK_1_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_1 };
pub(crate) const STRK_1_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_1 };
pub(crate) const STRK_1_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_1 };
pub(crate) const STRK_1_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_1 };
pub(crate) const STRK_1_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_1 };
pub(crate) const STRK_1_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_1 };

pub(crate) const STRK_1_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_1 };
pub(crate) const STRK_1_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_1 };
pub(crate) const STRK_1_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_1 };
pub(crate) const STRK_1_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_1 };
pub(crate) const STRK_1_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_1 };
pub(crate) const STRK_1_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_1 };
pub(crate) const STRK_1_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_1 };

pub(crate) const STRK_1_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_1 };
pub(crate) const STRK_1_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_1 };
pub(crate) const STRK_1_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_1 };
pub(crate) const STRK_1_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_1 };
pub(crate) const STRK_1_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_1 };
pub(crate) const STRK_1_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_1 };
pub(crate) const STRK_1_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_1 };

pub(crate) const STRK_1_BLK_75: Strk = Strk { style: Solid(Clr::BLK_75), ..STRK_1 };
pub(crate) const STRK_1_RED_75: Strk = Strk { style: Solid(Clr::RED_75), ..STRK_1 };
pub(crate) const STRK_1_GRN_75: Strk = Strk { style: Solid(Clr::GRN_75), ..STRK_1 };
pub(crate) const STRK_1_BLU_75: Strk = Strk { style: Solid(Clr::BLU_75), ..STRK_1 };
pub(crate) const STRK_1_YEL_75: Strk = Strk { style: Solid(Clr::YEL_75), ..STRK_1 };
pub(crate) const STRK_1_CYA_75: Strk = Strk { style: Solid(Clr::CYA_75), ..STRK_1 };
pub(crate) const STRK_1_MAG_75: Strk = Strk { style: Solid(Clr::MAG_75), ..STRK_1 };

pub(crate) const STRK_2_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_2 };
pub(crate) const STRK_2_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_2 };
pub(crate) const STRK_2_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_2 };
pub(crate) const STRK_2_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_2 };
pub(crate) const STRK_2_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_2 };
pub(crate) const STRK_2_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_2 };
pub(crate) const STRK_2_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_2 };

pub(crate) const STRK_2_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_2 };
pub(crate) const STRK_2_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_2 };
pub(crate) const STRK_2_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_2 };
pub(crate) const STRK_2_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_2 };
pub(crate) const STRK_2_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_2 };
pub(crate) const STRK_2_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_2 };
pub(crate) const STRK_2_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_2 };

pub(crate) const STRK_2_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_2 };
pub(crate) const STRK_2_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_2 };
pub(crate) const STRK_2_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_2 };
pub(crate) const STRK_2_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_2 };
pub(crate) const STRK_2_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_2 };
pub(crate) const STRK_2_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_2 };
pub(crate) const STRK_2_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_2 };

pub(crate) const STRK_2_BLK_75: Strk = Strk { style: Solid(Clr::BLK_75), ..STRK_2 };
pub(crate) const STRK_2_RED_75: Strk = Strk { style: Solid(Clr::RED_75), ..STRK_2 };
pub(crate) const STRK_2_GRN_75: Strk = Strk { style: Solid(Clr::GRN_75), ..STRK_2 };
pub(crate) const STRK_2_BLU_75: Strk = Strk { style: Solid(Clr::BLU_75), ..STRK_2 };
pub(crate) const STRK_2_YEL_75: Strk = Strk { style: Solid(Clr::YEL_75), ..STRK_2 };
pub(crate) const STRK_2_CYA_75: Strk = Strk { style: Solid(Clr::CYA_75), ..STRK_2 };
pub(crate) const STRK_2_MAG_75: Strk = Strk { style: Solid(Clr::MAG_75), ..STRK_2 };

pub(crate) const STRK_3_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_3 };
pub(crate) const STRK_3_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_3 };
pub(crate) const STRK_3_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_3 };
pub(crate) const STRK_3_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_3 };
pub(crate) const STRK_3_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_3 };
pub(crate) const STRK_3_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_3 };
pub(crate) const STRK_3_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_3 };

pub(crate) const STRK_3_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_3 };
pub(crate) const STRK_3_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_3 };
pub(crate) const STRK_3_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_3 };
pub(crate) const STRK_3_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_3 };
pub(crate) const STRK_3_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_3 };
pub(crate) const STRK_3_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_3 };
pub(crate) const STRK_3_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_3 };

pub(crate) const STRK_3_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_3 };
pub(crate) const STRK_3_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_3 };
pub(crate) const STRK_3_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_3 };
pub(crate) const STRK_3_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_3 };
pub(crate) const STRK_3_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_3 };
pub(crate) const STRK_3_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_3 };
pub(crate) const STRK_3_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_3 };

pub(crate) const STRK_3_BLK_75: Strk = Strk { style: Solid(Clr::BLK_75), ..STRK_3 };
pub(crate) const STRK_3_RED_75: Strk = Strk { style: Solid(Clr::RED_75), ..STRK_3 };
pub(crate) const STRK_3_GRN_75: Strk = Strk { style: Solid(Clr::GRN_75), ..STRK_3 };
pub(crate) const STRK_3_BLU_75: Strk = Strk { style: Solid(Clr::BLU_75), ..STRK_3 };
pub(crate) const STRK_3_YEL_75: Strk = Strk { style: Solid(Clr::YEL_75), ..STRK_3 };
pub(crate) const STRK_3_CYA_75: Strk = Strk { style: Solid(Clr::CYA_75), ..STRK_3 };
pub(crate) const STRK_3_MAG_75: Strk = Strk { style: Solid(Clr::MAG_75), ..STRK_3 };

pub(crate) const STRK_4_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_4 };
pub(crate) const STRK_4_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_4 };
pub(crate) const STRK_4_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_4 };
pub(crate) const STRK_4_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_4 };
pub(crate) const STRK_4_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_4 };
pub(crate) const STRK_4_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_4 };
pub(crate) const STRK_4_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_4 };

pub(crate) const STRK_4_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_4 };
pub(crate) const STRK_4_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_4 };
pub(crate) const STRK_4_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_4 };
pub(crate) const STRK_4_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_4 };
pub(crate) const STRK_4_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_4 };
pub(crate) const STRK_4_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_4 };
pub(crate) const STRK_4_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_4 };

pub(crate) const STRK_4_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_4 };
pub(crate) const STRK_4_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_4 };
pub(crate) const STRK_4_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_4 };
pub(crate) const STRK_4_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_4 };
pub(crate) const STRK_4_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_4 };
pub(crate) const STRK_4_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_4 };
pub(crate) const STRK_4_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_4 };

pub(crate) const STRK_5_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_5 };
pub(crate) const STRK_5_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_5 };
pub(crate) const STRK_5_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_5 };
pub(crate) const STRK_5_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_5 };
pub(crate) const STRK_5_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_5 };
pub(crate) const STRK_5_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_5 };
pub(crate) const STRK_5_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_5 };

pub(crate) const STRK_5_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_5 };
pub(crate) const STRK_5_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_5 };
pub(crate) const STRK_5_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_5 };
pub(crate) const STRK_5_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_5 };
pub(crate) const STRK_5_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_5 };
pub(crate) const STRK_5_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_5 };
pub(crate) const STRK_5_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_5 };

pub(crate) const STRK_5_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_5 };
pub(crate) const STRK_5_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_5 };
pub(crate) const STRK_5_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_5 };
pub(crate) const STRK_5_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_5 };
pub(crate) const STRK_5_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_5 };
pub(crate) const STRK_5_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_5 };
pub(crate) const STRK_5_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_5 };

const TEMPLATE_STRK: Strk = Strk {
    width: ONE,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    style: Solid(Clr::BLK),
    line_dash: LineDash { segments: &[], offset: 0 },
};

const TEMPLATE_FILL: CnvFill = CnvFill { style: Solid(Clr::YEL_50), rule: FillRule::EvenOdd };

pub(crate) const TEMPLATE_TXT_LAB_TIP: CnvText = CnvText {
    color: Clr::BLK,
    size: Pixels(SF),
    line_height: LineHeight::Absolute(Pixels(SF)),
    align_x: TextAlignment::Left,
    align_y: Vertical::Center,
    content: String::new(),
    max_width: Float::INFINITY,
    position: ORIGIN,
    shaping: Shaping::Basic,
    font: Font {
        family: Family::Name(FNT_NAME_LAB),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: FontStyle::Normal,
    },
};

pub(crate) const TEMPLATE_TXT_LAB_INTERNAL: CnvText = CnvText {
    color: Clr::RED,
    size: Pixels(SF),
    line_height: LineHeight::Absolute(Pixels(SF)),
    align_x: TextAlignment::Left,
    align_y: Vertical::Center,
    content: String::new(),
    max_width: Float::INFINITY,
    position: ORIGIN,
    shaping: Shaping::Basic,
    font: Font {
        family: Family::Name(FNT_NAME_LAB),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: FontStyle::Normal,
    },
};

pub(crate) const TEMPLATE_TXT_LAB_BRANCH: CnvText = CnvText {
    color: Clr::BLU,
    size: Pixels(SF),
    line_height: LineHeight::Absolute(Pixels(SF)),
    align_x: TextAlignment::Center,
    align_y: Vertical::Bottom,
    content: String::new(),
    max_width: Float::INFINITY,
    position: ORIGIN,
    shaping: Shaping::Basic,
    font: Font {
        family: Family::Name(FNT_NAME_LAB),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: FontStyle::Normal,
    },
};

pub(crate) const TEMPLATE_TXT_LAB_SCALEBAR: CnvText = CnvText {
    color: Clr::BLK,
    size: Pixels(SF),
    line_height: LineHeight::Absolute(Pixels(SF)),
    align_x: TextAlignment::Center,
    align_y: Vertical::Top,
    content: String::new(),
    max_width: Float::INFINITY,
    position: ORIGIN,
    shaping: Shaping::Basic,
    font: Font {
        family: Family::Name(FNT_NAME_LAB),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: FontStyle::Normal,
    },
};
