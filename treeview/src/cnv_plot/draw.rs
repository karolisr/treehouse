use std::collections::HashMap;

use super::{AxisScaleType, PlotData, St, Tick};
use crate::cnv_utils::*;
use crate::*;

pub(super) fn draw_gts(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    size: Size,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_gts.draw(rndr, size, |f| {
        let w = st.plt_vs.w;
        let h = st.plt_vs.h;

        let x_min = 0e0;
        let x_max = plt.plot_data.x_max - plt.plot_data.x_min;

        let gts_data = gts_data();

        let ranks_to_draw = [
            // GtsRank::Eon,
            GtsRank::Era,
            GtsRank::Period,
            // GtsRank::SubPeriod,
            GtsRank::Epoch,
            GtsRank::Age,
        ];

        let mut rank_indexes = HashMap::with_capacity(ranks_to_draw.len());

        for (i, r) in ranks_to_draw.iter().enumerate() {
            _ = rank_indexes.insert(r, i);
        }

        f.push_transform();
        f.translate(st.plt_vs.trans);

        let gts_label_font_size = st.text_size;
        let char_width = gts_label_font_size * 0.6;
        let h_unit = h / ranks_to_draw.len() as Float;

        for gts in gts_data {
            let beg = gts.beg;
            let end = gts.end;
            let rank = &gts.rank;
            let name = &gts.name;
            let color = &gts.color.scale_alpha(0.5);

            if (beg > x_max && end < x_max)
                || (beg <= x_max && end >= x_min)
                || (beg > x_min && end < x_min)
            {
                let rank_index_opt = rank_indexes.get(rank);

                if let Some(rank_index) = rank_index_opt {
                    let y1 = *rank_index as Float * h_unit;
                    let y2 = (*rank_index + 1) as Float * h_unit;

                    let beg_rel = (1.0
                        - transform_value(beg - x_min, AxisScaleType::Linear)
                            / transform_value(
                                x_max - x_min,
                                AxisScaleType::Linear,
                            ))
                    .max(0e0);

                    let end_rel = (1.0
                        - transform_value(end - x_min, AxisScaleType::Linear)
                            / transform_value(
                                x_max - x_min,
                                AxisScaleType::Linear,
                            ))
                    .min(1e0);

                    let mut pb: PathBuilder = PathBuilder::new();

                    let p1 = Point { x: w * beg_rel, y: y1 };
                    let p2 = Point { x: w * end_rel, y: y1 };
                    let p3 = Point { x: w * end_rel, y: y2 };
                    let p4 = Point { x: w * beg_rel, y: y2 };

                    let box_w = p3.x - p1.x;
                    let box_h = p3.y - p1.y;

                    let p_center =
                        Point { x: box_w / 2e0 + p1.x, y: box_h / 2e0 + p1.y };

                    pb = pb.move_to(p1);
                    pb = pb.line_to(p2);
                    pb = pb.line_to(p3);
                    pb = pb.line_to(p4);
                    pb = pb.close();

                    let path = pb.build();
                    f.fill(
                        &path,
                        CnvFill {
                            style: GeomStyle::Solid(*color),
                            rule: FillRule::EvenOdd,
                        },
                    );
                    f.stroke(&path, STRK_1_BLK);

                    let mut name_len = 0;
                    name.split(" ").for_each(|word| {
                        name_len = name_len.max(word.len() + 2);
                    });

                    if box_w > char_width * name_len as Float {
                        let label_text = lab_text(
                            name.replace(" ", "\n"),
                            p_center,
                            gts_label_font_size,
                            TEMPLATE_TXT_LAB_GTS,
                            false,
                        );

                        let label = Label {
                            text: label_text,
                            width: ZRO,
                            angle: 0.0,
                            aligned_from: None,
                        };

                        draw_labels(
                            &[label],
                            Vector { x: ZRO, y: ZRO },
                            None,
                            ZRO,
                            f,
                        );
                    }
                }
            }
        }

        f.pop_transform();
    }));
}

pub(super) fn draw_ltt(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    size: Size,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_ltt.draw(rndr, size, |f| {
        let pb_ltt = path_builder_ltt(
            &st.ltt_plot_data, plt.scale_x, plt.scale_y, st.plt_vs.w,
            st.plt_vs.h,
        );

        let path_ltt = pb_ltt.build();

        f.push_transform();
        f.translate(st.plt_vs.trans);
        f.stroke(&path_ltt, STRK_4_WHT);
        f.stroke(&path_ltt, STRK_1_BLK);
        f.pop_transform();
    }));
}

pub(super) fn draw_axes(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    size: Size,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_axes.draw(rndr, size, |f| {
        let (pb_axes, labs_x, labs_y) = path_builder_axes(
            &st.ticks_x, &st.ticks_y, st.plt_vs.w, st.plt_vs.h, st.tick_size,
            st.text_size,
        );

        let path_axes = pb_axes.build();

        f.push_transform();
        f.translate(st.plt_vs.trans);
        f.stroke(&path_axes, STRK_1_BLK);
        f.pop_transform();

        draw_labels(
            &labs_x,
            Vector { x: ZRO, y: st.lab_offset },
            Some(st.plt_vs.trans),
            ZRO,
            f,
        );

        draw_labels(
            &labs_y,
            Vector { x: -st.lab_offset, y: ZRO },
            Some(st.plt_vs.trans),
            ZRO,
            f,
        );
    }));
}

fn path_builder_ltt(
    data: &PlotData,
    scale_x: AxisScaleType,
    scale_y: AxisScaleType,
    w: Float,
    h: Float,
) -> PathBuilder {
    let mut first = true;
    let mut pb: PathBuilder = PathBuilder::new();
    for plot_point in &data.plot_points {
        let x_relative = transform_value(plot_point.x - data.x_min, scale_x)
            / transform_value(data.x_max - data.x_min, scale_x);

        let y_relative = transform_value(plot_point.y - data.y_min, scale_y)
            / transform_value(data.y_max - data.y_min, scale_y);

        let pt = Point { x: x_relative * w, y: (ONE - y_relative) * h };

        match first {
            true => {
                pb = pb.move_to(pt);
                first = false;
            }
            false => pb = pb.line_to(pt),
        }
    }
    pb
}

fn path_builder_axes(
    ticks_x: &[Tick],
    ticks_y: &[Tick],
    w: Float,
    h: Float,
    tick_size: Float,
    lab_size: Float,
) -> (PathBuilder, Vec<Label>, Vec<Label>) {
    let y_for_x_axis = h;
    let x_for_y_axis = ZRO;

    let mut pb: PathBuilder = PathBuilder::new();
    let mut labs_x: Vec<Label> = Vec::with_capacity(ticks_x.len());
    let mut labs_y: Vec<Label> = Vec::with_capacity(ticks_y.len());

    // x-axis line -------------------------------------------------------------
    let pt_min = Point { x: ZRO, y: y_for_x_axis };
    let pt_max = Point { x: w, y: y_for_x_axis };
    pb = pb.move_to(pt_min);
    pb = pb.line_to(pt_max);
    // x-axis ticks ------------------------------------------------------------
    for Tick { relative_position, label } in ticks_x {
        let x = relative_position * w;
        let tick_pt1 = Point { x, y: y_for_x_axis };
        let tick_pt2 = Point { x, y: y_for_x_axis + tick_size };
        pb = pb.move_to(tick_pt1);
        pb = pb.line_to(tick_pt2);

        let text = lab_text(
            label.to_string(),
            tick_pt2,
            lab_size,
            TEMPLATE_TXT_LAB_PLOT_AXIS_X,
            false,
        );
        let label = Label { text, width: ZRO, angle: 0.0, aligned_from: None };
        labs_x.push(label);
    } // -----------------------------------------------------------------------

    // y-axis line -------------------------------------------------------------
    let pt_min = Point { x: x_for_y_axis, y: ZRO };
    let pt_max = Point { x: x_for_y_axis, y: h };
    pb = pb.move_to(pt_min);
    pb = pb.line_to(pt_max);
    // y-axis ticks ------------------------------------------------------------
    for Tick { relative_position, label } in ticks_y {
        let y = (ONE - relative_position) * h;
        let tick_pt1 = Point { x: x_for_y_axis, y };
        let tick_pt2 = Point { x: x_for_y_axis - tick_size, y };
        pb = pb.move_to(tick_pt1);
        pb = pb.line_to(tick_pt2);

        let text = lab_text(
            label.to_string(),
            tick_pt2,
            lab_size,
            TEMPLATE_TXT_LAB_PLOT_AXIS_Y,
            false,
        );
        let label = Label { text, width: ZRO, angle: 0.0, aligned_from: None };
        labs_y.push(label);
    } // -----------------------------------------------------------------------

    (pb, labs_x, labs_y)
}

pub(super) fn draw_cursor_line(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    size: Size,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_cursor_line.draw(rndr, size, |f| {
        if let Some(p) = st.cursor_tracking_point
            && plt.draw_cursor_line
        {
            // line ------------------------------------------------------------
            f.push_transform();
            f.translate(st.plt_vs.trans);
            let p0 = Point { x: p.x, y: ZRO };
            let p1 = Point { x: p.x, y: st.plt_vs.h };
            f.stroke(
                &PathBuilder::new().move_to(p0).line_to(p1).build(),
                STRK_CRSR_LINE,
            );
            f.pop_transform();

            // label -----------------------------------------------------------
            let mut txt_template = TEMPLATE_TXT_CURSOR_TEXT;
            let mut y_offset = -PADDING;
            let x_val_range = plt.plot_data.x_max - plt.plot_data.x_min;
            let tree_height_at_x = if let Some(crsr_x_rel) = plt.crsr_x_rel {
                y_offset = st.plt_padd_t;
                txt_template.align_y = Vertical::Top;
                match plt.time_axis_reversed {
                    true => x_val_range * (1.0 - crsr_x_rel),
                    false => plt.plot_data.x_min + x_val_range * crsr_x_rel,
                }
            } else {
                let crsr_x_rel = p.x / st.plt_vs.w;
                match plt.time_axis_reversed {
                    true => x_val_range * (1.0 - crsr_x_rel),
                    false => plt.plot_data.x_min + x_val_range * crsr_x_rel,
                }
            };
            let name = format!("{tree_height_at_x:.3}");
            let text = lab_text(name, p, st.text_size, txt_template, false);
            let label =
                Label { text, width: ZRO, angle: 0.0, aligned_from: None };
            draw_labels(
                &[label],
                Vector { x: PADDING, y: y_offset },
                Some(st.plt_vs.trans),
                ZRO,
                f,
            );
        }
    }));
}

pub(super) fn draw_bounds(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    bnds: Rectangle,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_bnds.draw(rndr, bnds.size(), |f| {
        stroke_rect(st.plt_rect, STRK_3_GRN_50, f);
    }));
}
