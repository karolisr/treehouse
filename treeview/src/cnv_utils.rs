use crate::{cnv_plot::AxisDataType, *};

pub(crate) fn lab_text(
    txt: String,
    pt: Point,
    size: Float,
    template: CnvText,
    dimmed: bool,
) -> CnvText {
    let mut text = template.clone();
    text.content = txt;
    text.position = pt;
    text.size = size.into();
    if dimmed {
        text.color = Clr::BLK_50;
    }
    text
}

pub(crate) fn draw_labels(
    labels: &[Label],
    offset: Vector,
    trans: Option<Vector>,
    rot: Float,
    f: &mut Frame,
) {
    f.push_transform();
    if let Some(trans) = trans {
        f.translate(trans);
    }
    f.rotate(rot);
    f.translate(offset);

    for Label { text, width, angle, aligned_from } in labels {
        let mut text = text.clone();
        // ---------------------------------------------------------------------
        let magic = text.size.0 / 8.75;
        let adjust_h_base = text.size.0 / TWO - magic;
        let mut adjust_h = match text.align_y {
            Vertical::Top => adjust_h_base.floor() + TWO - magic / 3.15,
            Vertical::Center => ZRO,
            Vertical::Bottom => -adjust_h_base.ceil(),
        };
        text.align_y = Vertical::Center;
        // ---------------------------------------------------------------------
        if *angle != 0.0 {
            let angle_orig = *angle;
            let mut angle = *angle;
            let mut adjust_w = match text.align_x {
                TextAlignment::Left => offset.x,
                TextAlignment::Right => -offset.x,
                _ => ZRO,
            };
            adjust_h += offset.y;
            // = Rotate labels on the left side of the circle by 180 degrees ===
            let a = (angle + rot) % TAU;
            if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
                angle += PI;
                adjust_w = match text.align_x {
                    TextAlignment::Left => -width - offset.x,
                    TextAlignment::Right => width + offset.x,
                    _ => ZRO,
                };
            } // ===============================================================
            if let Some(aligned_from) = aligned_from {
                let (sin, cos) = angle_orig.sin_cos();

                let aligned_from_adj = Point {
                    x: aligned_from.x + cos * (offset.x + SF),
                    y: aligned_from.y + sin * (offset.x + SF),
                };

                if aligned_from.distance(text.position) > offset.x * 2e0 {
                    let path = PathBuilder::new()
                        .move_to(text.position)
                        .line_to(aligned_from_adj)
                        .build();

                    f.push_transform();
                    f.translate(Vector { x: -offset.x, y: -offset.y });
                    f.stroke(&path, STRK_EDGE_LAB_ALN);
                    f.pop_transform();
                }
            } // ---------------------------------------------------------------
            f.push_transform();
            let (sin, cos) = angle.sin_cos();
            f.translate(Vector {
                x: text.position.x - offset.x + cos * adjust_w - sin * adjust_h,
                y: text.position.y - offset.y + sin * adjust_w + cos * adjust_h,
            });
            text.position = ORIGIN;
            f.rotate(angle);
            f.fill_text(text);
            f.pop_transform();
        } else {
            f.push_transform();
            f.translate(Vector { x: ZRO, y: adjust_h });
            // -----------------------------------------------------------------
            if let Some(aligned_from) = aligned_from {
                let aligned_from_adj =
                    Point { x: aligned_from.x + SF, y: aligned_from.y };
                let text_pos_adj =
                    Point { x: text.position.x - offset.x, y: text.position.y };
                if aligned_from.distance(text.position) > offset.x * 2e0 {
                    let path = PathBuilder::new()
                        .move_to(text_pos_adj)
                        .line_to(aligned_from_adj)
                        .build();
                    f.stroke(&path, STRK_EDGE_LAB_ALN);
                }
            } // ---------------------------------------------------------------
            f.fill_text(text);
            f.pop_transform();
        }
    }
    f.pop_transform();
}

pub fn calc_ticks(
    tick_count: usize,
    scale_type: AxisScaleType,
    data_type: AxisDataType,
    min: Float,
    max: Float,
    axis_reversed: bool,
) -> (Vec<Tick>, usize) {
    let (min, max) = match axis_reversed {
        true => (0e0, max - min),
        false => (min, max),
    };

    let mut decimals: usize = 0;

    let mut offset = normalize_scale_value(min, scale_type);
    while offset > min {
        offset /= 2e0;
        if offset < 1e0 {
            offset = 0e0;
        }
    }

    let mut tick_value: Float = 0e0;
    let mut max_lab_nchar: usize = 0;
    let mut ticks: Vec<Tick> = Vec::with_capacity(tick_count);
    let mut mult: usize = 0;
    while tick_value < max {
        max_lab_nchar = 0;
        ticks.clear();
        mult += 1;

        let tick_count = tick_count * mult;

        let mut linear_delta = 1e0;
        if scale_type == AxisScaleType::Linear {
            let range = normalize_scale_value(max - offset, scale_type);

            linear_delta =
                normalize_scale_value(range / tick_count as Float, scale_type);

            if data_type == AxisDataType::Discrete && linear_delta < 1e0 {
                linear_delta = 1e0;
            }

            let ldfrac = linear_delta.fract();
            if ldfrac > 0e0 {
                decimals = (format!("{ldfrac:.4}").trim_end_matches("0").len()
                    - 2)
                .min(4);
            }
        }

        let calc_tick_value = |x: usize| {
            offset
                + match scale_type {
                    AxisScaleType::Linear => linear_delta * x as Float,
                    AxisScaleType::LogTwo => (2e0 as Float).powi(x as Integer),
                    AxisScaleType::LogTen => (1e1 as Float).powi(x as Integer),
                }
        };

        for i in 1..=tick_count {
            tick_value = calc_tick_value(i * mult);

            let rp_opt =
                transformed_relative_value(tick_value, min, max, scale_type);

            let Some(mut relative_position) = rp_opt else {
                continue;
            };

            relative_position = match axis_reversed {
                true => 1.0 - relative_position,
                false => relative_position,
            };

            if !(0e0..=1e0).contains(&relative_position) {
                continue;
            }

            let nchar = 1 + tick_value.log10().floor() as usize + decimals;
            max_lab_nchar = max_lab_nchar.max(nchar);

            let tick = Tick {
                relative_position,
                label: format!(
                    "{:nchar$.decimals$}",
                    tick_value,
                    nchar = nchar,
                    decimals = decimals
                ),
            };

            ticks.push(tick);
        }
    }

    (ticks, max_lab_nchar)
}
