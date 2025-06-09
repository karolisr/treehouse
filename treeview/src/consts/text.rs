use crate::iced::*;
use crate::*;

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
