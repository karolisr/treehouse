use crate::*;

const TEMPLATE_STRK: Strk = Strk {
    width: ONE,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    style: Solid(Clr::BLK),
    line_dash: LineDash { segments: &[], offset: 0 },
};

pub(crate) const DASH_001: LineDash =
    LineDash { segments: &[SF * 1e0, SF * 1e0], offset: 0 };
pub(crate) const DASH_002: LineDash =
    LineDash { segments: &[SF * 2e0, SF * 2e0], offset: 0 };
pub(crate) const DASH_003: LineDash =
    LineDash { segments: &[SF * 3e0, SF * 3e0], offset: 0 };
pub(crate) const DASH_004: LineDash =
    LineDash { segments: &[SF * 4e0, SF * 4e0], offset: 0 };
pub(crate) const DASH_005: LineDash =
    LineDash { segments: &[SF * 5e0, SF * 5e0], offset: 0 };
pub(crate) const DASH_006: LineDash =
    LineDash { segments: &[SF * 6e0, SF * 6e0], offset: 0 };
pub(crate) const DASH_007: LineDash =
    LineDash { segments: &[SF * 7e0, SF * 7e0], offset: 0 };
pub(crate) const DASH_008: LineDash =
    LineDash { segments: &[SF * 8e0, SF * 8e0], offset: 0 };
pub(crate) const DASH_009: LineDash =
    LineDash { segments: &[SF * 9e0, SF * 9e0], offset: 0 };
pub(crate) const DASH_010: LineDash =
    LineDash { segments: &[SF * 1e1, SF * 1e1], offset: 0 };

pub(crate) const STRK_H: Strk = Strk { width: SF * 0.5, ..TEMPLATE_STRK };
pub(crate) const STRK_1: Strk = Strk { width: SF * 1e0, ..TEMPLATE_STRK };
pub(crate) const STRK_2: Strk = Strk { width: SF * 2e0, ..TEMPLATE_STRK };
pub(crate) const STRK_3: Strk = Strk { width: SF * 3e0, ..TEMPLATE_STRK };
pub(crate) const STRK_4: Strk = Strk { width: SF * 4e0, ..TEMPLATE_STRK };
pub(crate) const STRK_5: Strk = Strk { width: SF * 5e0, ..TEMPLATE_STRK };

pub(crate) const STRK_H_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_H };
pub(crate) const STRK_1_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_1 };
pub(crate) const STRK_1_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_1 };
pub(crate) const STRK_1_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_1 };
pub(crate) const STRK_1_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_1 };
pub(crate) const STRK_1_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_1 };
pub(crate) const STRK_1_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_1 };
pub(crate) const STRK_1_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_1 };

pub(crate) const STRK_1_BLK_25: Strk =
    Strk { style: Solid(Clr::BLK_25), ..STRK_1 };
pub(crate) const STRK_1_RED_25: Strk =
    Strk { style: Solid(Clr::RED_25), ..STRK_1 };
pub(crate) const STRK_1_GRN_25: Strk =
    Strk { style: Solid(Clr::GRN_25), ..STRK_1 };
pub(crate) const STRK_1_BLU_25: Strk =
    Strk { style: Solid(Clr::BLU_25), ..STRK_1 };
pub(crate) const STRK_1_YEL_25: Strk =
    Strk { style: Solid(Clr::YEL_25), ..STRK_1 };
pub(crate) const STRK_1_CYA_25: Strk =
    Strk { style: Solid(Clr::CYA_25), ..STRK_1 };
pub(crate) const STRK_1_MAG_25: Strk =
    Strk { style: Solid(Clr::MAG_25), ..STRK_1 };

pub(crate) const STRK_1_BLK_50: Strk =
    Strk { style: Solid(Clr::BLK_50), ..STRK_1 };
pub(crate) const STRK_1_RED_50: Strk =
    Strk { style: Solid(Clr::RED_50), ..STRK_1 };
pub(crate) const STRK_1_GRN_50: Strk =
    Strk { style: Solid(Clr::GRN_50), ..STRK_1 };
pub(crate) const STRK_1_BLU_50: Strk =
    Strk { style: Solid(Clr::BLU_50), ..STRK_1 };
pub(crate) const STRK_1_YEL_50: Strk =
    Strk { style: Solid(Clr::YEL_50), ..STRK_1 };
pub(crate) const STRK_1_CYA_50: Strk =
    Strk { style: Solid(Clr::CYA_50), ..STRK_1 };
pub(crate) const STRK_1_MAG_50: Strk =
    Strk { style: Solid(Clr::MAG_50), ..STRK_1 };

pub(crate) const STRK_1_BLK_75: Strk =
    Strk { style: Solid(Clr::BLK_75), ..STRK_1 };
pub(crate) const STRK_1_RED_75: Strk =
    Strk { style: Solid(Clr::RED_75), ..STRK_1 };
pub(crate) const STRK_1_GRN_75: Strk =
    Strk { style: Solid(Clr::GRN_75), ..STRK_1 };
pub(crate) const STRK_1_BLU_75: Strk =
    Strk { style: Solid(Clr::BLU_75), ..STRK_1 };
pub(crate) const STRK_1_YEL_75: Strk =
    Strk { style: Solid(Clr::YEL_75), ..STRK_1 };
pub(crate) const STRK_1_CYA_75: Strk =
    Strk { style: Solid(Clr::CYA_75), ..STRK_1 };
pub(crate) const STRK_1_MAG_75: Strk =
    Strk { style: Solid(Clr::MAG_75), ..STRK_1 };

pub(crate) const STRK_2_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_2 };
pub(crate) const STRK_2_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_2 };
pub(crate) const STRK_2_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_2 };
pub(crate) const STRK_2_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_2 };
pub(crate) const STRK_2_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_2 };
pub(crate) const STRK_2_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_2 };
pub(crate) const STRK_2_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_2 };

pub(crate) const STRK_2_BLK_25: Strk =
    Strk { style: Solid(Clr::BLK_25), ..STRK_2 };
pub(crate) const STRK_2_RED_25: Strk =
    Strk { style: Solid(Clr::RED_25), ..STRK_2 };
pub(crate) const STRK_2_GRN_25: Strk =
    Strk { style: Solid(Clr::GRN_25), ..STRK_2 };
pub(crate) const STRK_2_BLU_25: Strk =
    Strk { style: Solid(Clr::BLU_25), ..STRK_2 };
pub(crate) const STRK_2_YEL_25: Strk =
    Strk { style: Solid(Clr::YEL_25), ..STRK_2 };
pub(crate) const STRK_2_CYA_25: Strk =
    Strk { style: Solid(Clr::CYA_25), ..STRK_2 };
pub(crate) const STRK_2_MAG_25: Strk =
    Strk { style: Solid(Clr::MAG_25), ..STRK_2 };

pub(crate) const STRK_2_BLK_50: Strk =
    Strk { style: Solid(Clr::BLK_50), ..STRK_2 };
pub(crate) const STRK_2_RED_50: Strk =
    Strk { style: Solid(Clr::RED_50), ..STRK_2 };
pub(crate) const STRK_2_GRN_50: Strk =
    Strk { style: Solid(Clr::GRN_50), ..STRK_2 };
pub(crate) const STRK_2_BLU_50: Strk =
    Strk { style: Solid(Clr::BLU_50), ..STRK_2 };
pub(crate) const STRK_2_YEL_50: Strk =
    Strk { style: Solid(Clr::YEL_50), ..STRK_2 };
pub(crate) const STRK_2_CYA_50: Strk =
    Strk { style: Solid(Clr::CYA_50), ..STRK_2 };
pub(crate) const STRK_2_MAG_50: Strk =
    Strk { style: Solid(Clr::MAG_50), ..STRK_2 };

pub(crate) const STRK_2_BLK_75: Strk =
    Strk { style: Solid(Clr::BLK_75), ..STRK_2 };
pub(crate) const STRK_2_RED_75: Strk =
    Strk { style: Solid(Clr::RED_75), ..STRK_2 };
pub(crate) const STRK_2_GRN_75: Strk =
    Strk { style: Solid(Clr::GRN_75), ..STRK_2 };
pub(crate) const STRK_2_BLU_75: Strk =
    Strk { style: Solid(Clr::BLU_75), ..STRK_2 };
pub(crate) const STRK_2_YEL_75: Strk =
    Strk { style: Solid(Clr::YEL_75), ..STRK_2 };
pub(crate) const STRK_2_CYA_75: Strk =
    Strk { style: Solid(Clr::CYA_75), ..STRK_2 };
pub(crate) const STRK_2_MAG_75: Strk =
    Strk { style: Solid(Clr::MAG_75), ..STRK_2 };

pub(crate) const STRK_3_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_3 };
pub(crate) const STRK_3_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_3 };
pub(crate) const STRK_3_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_3 };
pub(crate) const STRK_3_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_3 };
pub(crate) const STRK_3_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_3 };
pub(crate) const STRK_3_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_3 };
pub(crate) const STRK_3_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_3 };

pub(crate) const STRK_3_BLK_25: Strk =
    Strk { style: Solid(Clr::BLK_25), ..STRK_3 };
pub(crate) const STRK_3_RED_25: Strk =
    Strk { style: Solid(Clr::RED_25), ..STRK_3 };
pub(crate) const STRK_3_GRN_25: Strk =
    Strk { style: Solid(Clr::GRN_25), ..STRK_3 };
pub(crate) const STRK_3_BLU_25: Strk =
    Strk { style: Solid(Clr::BLU_25), ..STRK_3 };
pub(crate) const STRK_3_YEL_25: Strk =
    Strk { style: Solid(Clr::YEL_25), ..STRK_3 };
pub(crate) const STRK_3_CYA_25: Strk =
    Strk { style: Solid(Clr::CYA_25), ..STRK_3 };
pub(crate) const STRK_3_MAG_25: Strk =
    Strk { style: Solid(Clr::MAG_25), ..STRK_3 };

pub(crate) const STRK_3_BLK_50: Strk =
    Strk { style: Solid(Clr::BLK_50), ..STRK_3 };
pub(crate) const STRK_3_RED_50: Strk =
    Strk { style: Solid(Clr::RED_50), ..STRK_3 };
pub(crate) const STRK_3_GRN_50: Strk =
    Strk { style: Solid(Clr::GRN_50), ..STRK_3 };
pub(crate) const STRK_3_BLU_50: Strk =
    Strk { style: Solid(Clr::BLU_50), ..STRK_3 };
pub(crate) const STRK_3_YEL_50: Strk =
    Strk { style: Solid(Clr::YEL_50), ..STRK_3 };
pub(crate) const STRK_3_CYA_50: Strk =
    Strk { style: Solid(Clr::CYA_50), ..STRK_3 };
pub(crate) const STRK_3_MAG_50: Strk =
    Strk { style: Solid(Clr::MAG_50), ..STRK_3 };

pub(crate) const STRK_3_BLK_75: Strk =
    Strk { style: Solid(Clr::BLK_75), ..STRK_3 };
pub(crate) const STRK_3_RED_75: Strk =
    Strk { style: Solid(Clr::RED_75), ..STRK_3 };
pub(crate) const STRK_3_GRN_75: Strk =
    Strk { style: Solid(Clr::GRN_75), ..STRK_3 };
pub(crate) const STRK_3_BLU_75: Strk =
    Strk { style: Solid(Clr::BLU_75), ..STRK_3 };
pub(crate) const STRK_3_YEL_75: Strk =
    Strk { style: Solid(Clr::YEL_75), ..STRK_3 };
pub(crate) const STRK_3_CYA_75: Strk =
    Strk { style: Solid(Clr::CYA_75), ..STRK_3 };
pub(crate) const STRK_3_MAG_75: Strk =
    Strk { style: Solid(Clr::MAG_75), ..STRK_3 };

pub(crate) const STRK_4_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_4 };
pub(crate) const STRK_4_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_4 };
pub(crate) const STRK_4_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_4 };
pub(crate) const STRK_4_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_4 };
pub(crate) const STRK_4_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_4 };
pub(crate) const STRK_4_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_4 };
pub(crate) const STRK_4_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_4 };

pub(crate) const STRK_4_BLK_25: Strk =
    Strk { style: Solid(Clr::BLK_25), ..STRK_4 };
pub(crate) const STRK_4_RED_25: Strk =
    Strk { style: Solid(Clr::RED_25), ..STRK_4 };
pub(crate) const STRK_4_GRN_25: Strk =
    Strk { style: Solid(Clr::GRN_25), ..STRK_4 };
pub(crate) const STRK_4_BLU_25: Strk =
    Strk { style: Solid(Clr::BLU_25), ..STRK_4 };
pub(crate) const STRK_4_YEL_25: Strk =
    Strk { style: Solid(Clr::YEL_25), ..STRK_4 };
pub(crate) const STRK_4_CYA_25: Strk =
    Strk { style: Solid(Clr::CYA_25), ..STRK_4 };
pub(crate) const STRK_4_MAG_25: Strk =
    Strk { style: Solid(Clr::MAG_25), ..STRK_4 };

pub(crate) const STRK_4_BLK_50: Strk =
    Strk { style: Solid(Clr::BLK_50), ..STRK_4 };
pub(crate) const STRK_4_RED_50: Strk =
    Strk { style: Solid(Clr::RED_50), ..STRK_4 };
pub(crate) const STRK_4_GRN_50: Strk =
    Strk { style: Solid(Clr::GRN_50), ..STRK_4 };
pub(crate) const STRK_4_BLU_50: Strk =
    Strk { style: Solid(Clr::BLU_50), ..STRK_4 };
pub(crate) const STRK_4_YEL_50: Strk =
    Strk { style: Solid(Clr::YEL_50), ..STRK_4 };
pub(crate) const STRK_4_CYA_50: Strk =
    Strk { style: Solid(Clr::CYA_50), ..STRK_4 };
pub(crate) const STRK_4_MAG_50: Strk =
    Strk { style: Solid(Clr::MAG_50), ..STRK_4 };

pub(crate) const STRK_5_BLK: Strk = Strk { style: Solid(Clr::BLK), ..STRK_5 };
pub(crate) const STRK_5_RED: Strk = Strk { style: Solid(Clr::RED), ..STRK_5 };
pub(crate) const STRK_5_GRN: Strk = Strk { style: Solid(Clr::GRN), ..STRK_5 };
pub(crate) const STRK_5_BLU: Strk = Strk { style: Solid(Clr::BLU), ..STRK_5 };
pub(crate) const STRK_5_YEL: Strk = Strk { style: Solid(Clr::YEL), ..STRK_5 };
pub(crate) const STRK_5_CYA: Strk = Strk { style: Solid(Clr::CYA), ..STRK_5 };
pub(crate) const STRK_5_MAG: Strk = Strk { style: Solid(Clr::MAG), ..STRK_5 };

pub(crate) const STRK_5_BLK_25: Strk =
    Strk { style: Solid(Clr::BLK_25), ..STRK_5 };
pub(crate) const STRK_5_RED_25: Strk =
    Strk { style: Solid(Clr::RED_25), ..STRK_5 };
pub(crate) const STRK_5_GRN_25: Strk =
    Strk { style: Solid(Clr::GRN_25), ..STRK_5 };
pub(crate) const STRK_5_BLU_25: Strk =
    Strk { style: Solid(Clr::BLU_25), ..STRK_5 };
pub(crate) const STRK_5_YEL_25: Strk =
    Strk { style: Solid(Clr::YEL_25), ..STRK_5 };
pub(crate) const STRK_5_CYA_25: Strk =
    Strk { style: Solid(Clr::CYA_25), ..STRK_5 };
pub(crate) const STRK_5_MAG_25: Strk =
    Strk { style: Solid(Clr::MAG_25), ..STRK_5 };

pub(crate) const STRK_5_BLK_50: Strk =
    Strk { style: Solid(Clr::BLK_50), ..STRK_5 };
pub(crate) const STRK_5_RED_50: Strk =
    Strk { style: Solid(Clr::RED_50), ..STRK_5 };
pub(crate) const STRK_5_GRN_50: Strk =
    Strk { style: Solid(Clr::GRN_50), ..STRK_5 };
pub(crate) const STRK_5_BLU_50: Strk =
    Strk { style: Solid(Clr::BLU_50), ..STRK_5 };
pub(crate) const STRK_5_YEL_50: Strk =
    Strk { style: Solid(Clr::YEL_50), ..STRK_5 };
pub(crate) const STRK_5_CYA_50: Strk =
    Strk { style: Solid(Clr::CYA_50), ..STRK_5 };
pub(crate) const STRK_5_MAG_50: Strk =
    Strk { style: Solid(Clr::MAG_50), ..STRK_5 };
