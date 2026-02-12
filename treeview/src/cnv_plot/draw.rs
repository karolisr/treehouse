use std::collections::HashMap;

use super::St;
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
        let x_max = plt.ltt_plot_data.x_max - plt.ltt_plot_data.x_min;

        let gts_data = gts_data();

        let ranks_to_draw = [
            // GtsRank::Eon,
            GtsRank::Era,
            GtsRank::Period,
            // GtsRank::SubPeriod,
            GtsRank::Epoch,
            // GtsRank::Age,
        ];

        let mut rank_indexes: HashMap<&GtsRank, usize> =
            HashMap::with_capacity(ranks_to_draw.len());

        for (i, rank) in ranks_to_draw.iter().enumerate() {
            _ = rank_indexes.insert(rank, i);
        }

        f.push_transform();
        f.translate(st.plt_vs.trans);

        let gts_label_font_size = st.text_size;
        let h_unit = h / ranks_to_draw.len() as Float;

        for gts in gts_data {
            let beg = gts.beg;
            let end = gts.end;
            let rank = &gts.rank;
            let name = &gts.name;
            let color = &gts.color.scale_alpha(0.75);

            if (beg > x_max && end < x_max)
                || (beg <= x_max && end >= x_min)
                || (beg > x_min && end < x_min)
            {
                let rank_index_opt = rank_indexes.get(rank);

                if let Some(rank_index) = rank_index_opt {
                    let y1 = *rank_index as Float * h_unit;
                    let y2 = (*rank_index + 1) as Float * h_unit;

                    let br = transformed_relative_value(
                        beg, x_min, x_max, plt.x_axis_scale_type,
                    )
                    .unwrap_or(0e0);

                    let er = transformed_relative_value(
                        end, x_min, x_max, plt.x_axis_scale_type,
                    )
                    .unwrap_or(0e0);

                    let beg_rel = (1.0 - br).max(0e0);
                    let end_rel = (1.0 - er).min(1e0);

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
                    f.stroke(&path, STRK_1_BLK_25);

                    let mut name_len = 0;
                    name.split(" ").for_each(|word| {
                        name_len = name_len.max(word.len() + 2);
                    });

                    if box_w > st.char_width * name_len as Float {
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
            &st.ltt_plot_data, plt.x_axis_scale_type, plt.y_axis_scale_type,
            st.plt_vs.w, st.plt_vs.h,
        );

        let path_ltt = pb_ltt.build();

        f.push_transform();
        f.translate(st.plt_vs.trans);
        f.stroke(&path_ltt, STRK_4_WHT_75);
        f.stroke(&path_ltt, STRK_2_BLU);
        f.pop_transform();
    }));
}

fn path_builder_ltt(
    data: &PlotData,
    x_axis_scale_type: AxisScaleType,
    y_axis_scale_type: AxisScaleType,
    w: Float,
    h: Float,
) -> PathBuilder {
    let mut first = true;
    let mut pb: PathBuilder = PathBuilder::new();

    let x_min = data.x_min;
    let x_max = data.x_max;
    let y_min = data.y_min;
    let y_max = data.y_max;

    for plot_point in &data.plot_points {
        let x_rel = transformed_relative_value(
            plot_point.x, x_min, x_max, x_axis_scale_type,
        )
        .unwrap_or(0e0)
        .clamp(ZRO, ONE);

        let y_rel = transformed_relative_value(
            plot_point.y, y_min, y_max, y_axis_scale_type,
        )
        .unwrap_or(0e0)
        .clamp(ZRO, ONE);

        let pt = Point { x: x_rel * w, y: (1e0 - y_rel) * h };

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

pub(super) fn draw_ticks(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    size: Size,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_ticks.draw(rndr, size, |f| {
        let (pb_ticks, labs_x, labs_y) = path_builder_ticks(
            st.plt_vs.w, st.plt_vs.h, st.axes_padd, &st.ticks_x, &st.ticks_y,
            st.tick_size, st.text_size,
        );

        f.push_transform();
        f.translate(st.plt_vs.trans);
        f.stroke(&pb_ticks.build(), STRK_1_BLK);
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

fn path_builder_ticks(
    w: Float,
    h: Float,
    axes_padd: Float,
    ticks_x: &[Tick],
    ticks_y: &[Tick],
    tick_size: Float,
    lab_size: Float,
) -> (PathBuilder, Vec<Label>, Vec<Label>) {
    let mut pb: PathBuilder = PathBuilder::new();

    let left = -axes_padd;
    let bottom = h + axes_padd;

    let mut labs_x: Vec<Label> = Vec::with_capacity(ticks_x.len());
    let mut labs_y: Vec<Label> = Vec::with_capacity(ticks_y.len());

    // x-axis ticks ------------------------------------------------------------
    for Tick { relative_position, label } in ticks_x {
        let x = relative_position * w;
        let tick_pt1 = Point { x, y: bottom };
        let tick_pt2 = Point { x, y: bottom + tick_size };
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

    // y-axis ticks ------------------------------------------------------------
    for Tick { relative_position, label } in ticks_y {
        let y = (ONE - relative_position) * h;
        let tick_pt1 = Point { x: left, y };
        let tick_pt2 = Point { x: left - tick_size, y };
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

pub(super) fn draw_axes(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    size: Size,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_axes.draw(rndr, size, |f| {
        f.push_transform();
        f.translate(st.plt_vs.trans);
        f.stroke(
            &path_builder_axes(st.plt_vs.w, st.plt_vs.h, st.axes_padd).build(),
            STRK_1_BLK,
        );
        f.pop_transform();
    }));
}

fn path_builder_axes(w: Float, h: Float, axes_padd: Float) -> PathBuilder {
    let mut pb: PathBuilder = PathBuilder::new();

    let left = -axes_padd;
    let right = w + axes_padd;
    let top = -axes_padd;
    let bottom = h + axes_padd;

    // x-axis line bottom ------------------------------------------------------
    pb = pb.move_to(Point { x: left, y: bottom });
    pb = pb.line_to(Point { x: right, y: bottom });

    // x-axis line top ---------------------------------------------------------
    pb = pb.move_to(Point { x: left, y: top });
    pb = pb.line_to(Point { x: right, y: top });

    // y-axis line left --------------------------------------------------------
    pb = pb.move_to(Point { x: left, y: top });
    pb = pb.line_to(Point { x: left, y: bottom });

    // y-axis line right -------------------------------------------------------
    pb = pb.move_to(Point { x: right, y: top });
    pb = pb.line_to(Point { x: right, y: bottom });

    pb
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
            let x_offset;
            let y_offset;
            let x_val_range = plt.ltt_plot_data.x_max - plt.ltt_plot_data.x_min;

            let tree_height_at_x = if let Some(crsr_x_rel) = plt.crsr_x_rel {
                x_offset = PADDING;
                y_offset = PADDING;
                txt_template.align_y = Vertical::Top;
                match plt.x_axis_is_reversed {
                    true => x_val_range * (1.0 - crsr_x_rel),
                    false => plt.ltt_plot_data.x_min + x_val_range * crsr_x_rel,
                }
            } else {
                x_offset = PADDING * 2e0;
                y_offset = PADDING * 2e0;
                let crsr_x_rel = p.x / st.plt_vs.w;
                match plt.x_axis_is_reversed {
                    true => x_val_range * (1.0 - crsr_x_rel),
                    false => plt.ltt_plot_data.x_min + x_val_range * crsr_x_rel,
                }
            };

            let units = match plt.x_axis_is_reversed {
                true => match plt.tre_unit {
                    TreUnit::Unitless => "",
                    TreUnit::Substitutions => "",
                    TreUnit::MillionYears => " MYA",
                    TreUnit::CoalescentUnits => "",
                },
                false => match plt.tre_unit {
                    TreUnit::Unitless => "",
                    TreUnit::Substitutions => " Subs./site",
                    TreUnit::MillionYears => " MY",
                    TreUnit::CoalescentUnits => "Coales. units",
                },
            };

            let name = format!("{tree_height_at_x:.3}{units}");
            let text = lab_text(name, p, st.text_size, txt_template, false);
            let label =
                Label { text, width: ZRO, angle: 0.0, aligned_from: None };

            draw_labels(
                &[label],
                Vector { x: x_offset, y: y_offset },
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
