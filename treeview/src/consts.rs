#![allow(dead_code)]

use crate::*;

pub(crate) const TIP_LAB_SIZE: u16 = 3;
pub(crate) const INT_LAB_SIZE: u16 = 3;
pub(crate) const BRNCH_LAB_SIZE: u16 = 3;

pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TAU: Float = std::f32::consts::TAU;
pub(crate) const FRAC_PI_2: Float = std::f32::consts::FRAC_PI_2;
pub(crate) const SF: Float = 1e0;
pub(crate) const FNT_NAME: &str = "JetBrains Mono";
pub(crate) const TXT_SIZE: Float = 13.0 * SF;
pub(crate) const FNT_NAME_LAB: &str = FNT_NAME;
pub(crate) const TXT_SIZE_LAB: Float = TXT_SIZE;

pub(crate) const STRK_TMPL: Strk = Strk {
    width: 1e0,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    style: Solid(Clr::BLK),
    line_dash: LineDash { segments: &[], offset: 0 },
};

pub(crate) const STRK_1: Strk = Strk { width: 1e0, ..STRK_TMPL };
pub(crate) const STRK_2: Strk = Strk { width: 2e0, ..STRK_TMPL };
pub(crate) const STRK_3: Strk = Strk { width: 3e0, ..STRK_TMPL };
pub(crate) const STRK_4: Strk = Strk { width: 4e0, ..STRK_TMPL };
pub(crate) const STRK_5: Strk = Strk { width: 5e0, ..STRK_TMPL };

pub(crate) const STRK_EDGE: Strk = STRK_1;

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

pub(crate) const TXT_LAB_TMPL: CanvasText = CanvasText {
    color: Clr::BLK,
    size: Pixels(TXT_SIZE_LAB),
    line_height: LineHeight::Absolute(Pixels(TXT_SIZE_LAB)),
    align_x: TextAlignment::Left,
    align_y: Vertical::Center,
    content: String::new(),
    max_width: Float::INFINITY,
    position: Point::ORIGIN,
    shaping: Shaping::Basic,
    font: Font {
        family: Family::Name(FNT_NAME_LAB),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: FontStyle::Normal,
    },
};

pub(crate) const TXT_LAB_TMPL_BRNCH: CanvasText = CanvasText {
    color: Clr::BLK,
    size: Pixels(TXT_SIZE_LAB),
    line_height: LineHeight::Absolute(Pixels(TXT_SIZE_LAB)),
    align_x: TextAlignment::Center,
    align_y: Vertical::Bottom,
    content: String::new(),
    max_width: Float::INFINITY,
    position: Point::ORIGIN,
    shaping: Shaping::Basic,
    font: Font {
        family: Family::Name(FNT_NAME_LAB),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: FontStyle::Normal,
    },
};
