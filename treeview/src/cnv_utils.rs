use crate::*;

pub(crate) fn transform_value(raw: Float, scale: &AxisScaleType) -> Float {
    match scale {
        AxisScaleType::Linear => raw,
        AxisScaleType::LogTwo => raw.log2(),
        AxisScaleType::LogNat => raw.ln(),
        AxisScaleType::LogTen => raw.log10(),
    }
}

pub(crate) fn lab_text(
    txt: String, pt: iced::Point, size: Float, template: iced::CnvText,
) -> iced::CnvText {
    let mut text = template.clone();
    text.content = txt;
    text.position = pt;
    text.size = size.into();
    text
}

pub(crate) fn draw_labels(
    labels: &[Label], offset: iced::Vector, trans: Option<iced::Vector>, rot: Float,
    f: &mut iced::Frame,
) {
    f.push_transform();
    if let Some(trans) = trans {
        f.translate(trans);
    }
    f.rotate(rot);
    f.translate(offset);

    for Label { text, width, angle } in labels {
        let mut text = text.clone();
        // -----------------------------------------------------------------------------------------
        let magic = text.size.0 / 8.75;
        let adjust_h_base = text.size.0 / TWO - magic;
        let mut adjust_h = match text.align_y {
            iced::Vertical::Top => adjust_h_base.floor() + TWO - magic / 3.15,
            iced::Vertical::Center => ZERO,
            iced::Vertical::Bottom => -adjust_h_base.ceil(),
        };
        text.align_y = iced::Vertical::Center;
        // -----------------------------------------------------------------------------------------
        if let Some(angle) = angle {
            let mut angle = *angle;
            let mut adjust_w = match text.align_x {
                iced::TextAlignment::Left => offset.x,
                iced::TextAlignment::Right => -offset.x,
                _ => ZERO,
            };
            adjust_h += offset.y;
            // = Rotate labels on the left side of the circle by 180 degrees =======================
            let a = (angle + rot) % TAU;
            if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
                angle += PI;
                adjust_w = match text.align_x {
                    iced::TextAlignment::Left => -width - offset.x,
                    iced::TextAlignment::Right => width + offset.x,
                    _ => ZERO,
                };
            } // ===================================================================================
            f.push_transform();
            let (sin, cos) = angle.sin_cos();
            f.translate(iced::Vector {
                x: text.position.x - offset.x + cos * adjust_w - sin * adjust_h,
                y: text.position.y - offset.y + sin * adjust_w + cos * adjust_h,
            });
            text.position = ORIGIN;
            f.rotate(angle);
            f.fill_text(text);
            f.pop_transform();
        } else {
            f.push_transform();
            f.translate(iced::Vector { x: ZERO, y: adjust_h });
            f.fill_text(text);
            f.pop_transform();
        }
    }
    f.pop_transform();
}
