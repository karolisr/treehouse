use std::collections::HashMap;

use super::*;

use crate::cnv_plot::{
    AxisDataType, AxisScaleType, PlotData, transformed_relative_value,
};
use crate::cnv_utils::calc_ticks;
use crate::consts::{
    STRK_1_BLK, STRK_1_BLK_25, STRK_2_BLK, STRK_2_BLU, STRK_4_WHT_75,
    STRK_EDGE, STRK_ROOT, TWO, ZRO,
};
use crate::gts::{GtsRank, gts_data};
use crate::normalize_scale_value;
use crate::path_builders::{
    path_builder_ltt, path_builder_ticks_x, path_builder_ticks_y,
    path_builder_x_axis, path_builder_y_axis, path_clade_highlight,
    path_edges_fan, path_edges_phygrm, path_root_edge_fan,
    path_root_edge_phygrm,
};
use crate::{Float, Rc, RectVals, TreSty, TreUnit, TreeState};
use dendros::Edge;
use oxidize_pdf::text::{Font, measure_text};
use oxidize_pdf::{Page, PdfError, graphics::GraphicsContext};
use riced::{PADDING, PathBuilder, Point, Rectangle, SF};

pub(super) fn draw_clade_highlights(
    tree_state: Rc<TreeState>,
    cnv_vs: &RectVals<Float>,
    tre_vs: &RectVals<Float>,
    root_len: Float,
    opn_angle: Float,
    tree_style: TreSty,
    gc: &mut GraphicsContext,
) -> Result<(), PdfError> {
    let highlighted_clades = tree_state.highlighted_clades();
    for node_id in tree_state.node_ids_srtd_asc() {
        if highlighted_clades.contains_key(&node_id) {
            let clade_highlight = highlighted_clades.get(&node_id).unwrap();

            let iced_path = path_clade_highlight(
                node_id, &tree_state, tre_vs.w, tre_vs.h, cnv_vs.x1,
                tre_vs.radius_min, cnv_vs.radius_min, root_len, opn_angle,
                tree_style,
            );

            let color = color_from_iced_color(clade_highlight.color);
            let alpha = alpha_from_iced_color(clade_highlight.color);

            _ = gc.save_state();
            _ = apply_iced_path_to_gc(iced_path.clone(), gc);
            _ = gc.set_fill_color(color).set_alpha_fill(alpha)?;
            _ = gc.fill();
            _ = gc.restore_state();
        }
    }
    Ok(())
}

pub(super) fn draw_bounds(
    cnv_vs: &RectVals<Float>,
    tre_vs: &RectVals<Float>,
    gc: &mut GraphicsContext,
) {
    let path_cnv_bounds = PathBuilder::new()
        .rectangle(Rectangle {
            x: 0e0,
            y: 0e0,
            width: cnv_vs.w,
            height: cnv_vs.h,
        })
        .build();
    let path_tre_bounds = PathBuilder::new()
        .rectangle(Rectangle {
            x: tre_vs.x0,
            y: tre_vs.y0,
            width: tre_vs.w,
            height: tre_vs.h,
        })
        .build();
    _ = gc.save_state();
    _ = apply_iced_path_to_gc(path_cnv_bounds, gc);
    _ = apply_iced_path_to_gc(path_tre_bounds, gc);
    _ = gc.stroke();
    _ = gc.restore_state();
}

pub(super) fn draw_root(
    tre_vs: &RectVals<Float>,
    opn_angle: Float,
    root_len: Float,
    root_edge: &Edge,
    scaling: f64,
    tree_style: TreSty,
    gc: &mut GraphicsContext,
) {
    _ = apply_iced_path_to_gc(
        match tree_style {
            TreSty::PhyGrm => {
                path_root_edge_phygrm(tre_vs.w, tre_vs.h, root_len, root_edge)
            }
            TreSty::Fan => path_root_edge_fan(
                tre_vs.radius_min, opn_angle, root_len, root_edge,
            ),
        },
        apply_iced_stroke_to_gc(STRK_ROOT, scaling, gc),
    )
    .stroke();
}

pub(super) fn draw_edges(
    tre_vs: &RectVals<Float>,
    tree_state: Rc<TreeState>,
    opn_angle: Float,
    root_len: Float,
    scaling: f64,
    tree_style: TreSty,
    gc: &mut GraphicsContext,
) {
    if let Some(edges) = tree_state.edges() {
        _ = apply_iced_path_to_gc(
            match tree_style {
                TreSty::PhyGrm => path_edges_phygrm(edges, tre_vs.w, tre_vs.h),
                TreSty::Fan => path_edges_fan(
                    edges, opn_angle, root_len, tre_vs.radius_min,
                ),
            },
            apply_iced_stroke_to_gc(STRK_EDGE, scaling, gc),
        )
        .stroke();
    }
}

// Mirrors `cnv_plot` layering: background -> GTS -> LTT -> ticks -> axes
// (minus the cursor line, which the PDF never draws).
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_plot(
    tre_vs: &RectVals<Float>,
    plot_y0: Float,
    plot_h: Float,
    draw_gts: bool,
    draw_ltt: bool,
    tre_unit: TreUnit,
    x_axis_scale_type: AxisScaleType,
    y_axis_scale_type: AxisScaleType,
    x_axis_is_reversed: bool,
    ltt_plot_data: &PlotData,
    scaling: f64,
    font: Font,
    pg: &mut Page,
) -> Result<(), PdfError> {
    let x0 = tre_vs.x0;
    let y0 = plot_y0;
    let w = tre_vs.w;
    let h = plot_h;

    let bg_path = PathBuilder::new()
        .rectangle(Rectangle { x: x0, y: y0, width: w, height: h })
        .build();
    let bg_color = color_from_iced_color(riced::Color::WHITE);
    _ = pg.graphics().save_state();
    _ = apply_iced_path_to_gc(bg_path, pg.graphics());
    _ = pg.graphics().set_fill_color(bg_color);
    _ = pg.graphics().fill();
    _ = pg.graphics().restore_state();

    // Local plot-space transform: (0, 0) is the top-left of the plot rect,
    // in the same iced-style y-down convention `path_builder_ltt` and the
    // GTS band math below assume (mirrors `cnv_plot/draw.rs`).
    _ = pg.graphics().save_state();
    _ = pg.graphics().translate(x0 as f64, -y0 as f64);

    if draw_gts && tre_unit == TreUnit::MillionYears {
        draw_gts_bands(
            w,
            h,
            x_axis_scale_type,
            ltt_plot_data,
            scaling,
            font.clone(),
            pg,
        )?;
    }

    if draw_ltt && !ltt_plot_data.plot_points.is_empty() {
        draw_ltt_curve(
            ltt_plot_data, x_axis_scale_type, y_axis_scale_type, w, h, scaling,
            pg,
        );
    }

    draw_plot_ticks_and_axes(
        w, h, x_axis_scale_type, y_axis_scale_type, x_axis_is_reversed,
        ltt_plot_data, scaling, font, pg,
    );

    _ = pg.graphics().restore_state();

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_plot_ticks_and_axes(
    w: Float,
    h: Float,
    x_axis_scale_type: AxisScaleType,
    y_axis_scale_type: AxisScaleType,
    x_axis_is_reversed: bool,
    ltt_plot_data: &PlotData,
    scaling: f64,
    font: Font,
    pg: &mut Page,
) {
    let text_size = SF * 1e1 * scaling as Float;
    let char_width = text_size * 6e-1;
    let tick_size = char_width;
    let lab_offset = char_width / TWO;
    let axes_padd = ZRO;

    let n_ticks_x = (w / (char_width * 20.0)).floor().max(3.0) as usize;
    let n_ticks_y = (h / (text_size * 3.0)).floor().max(3.0) as usize;

    let (ticks_x, _) = calc_ticks(
        n_ticks_x, x_axis_scale_type, ltt_plot_data.x_data_type,
        ltt_plot_data.x_min, ltt_plot_data.x_max, x_axis_is_reversed,
    );
    let (ticks_y, _) = calc_ticks(
        n_ticks_y, y_axis_scale_type, ltt_plot_data.y_data_type,
        ltt_plot_data.y_min, ltt_plot_data.y_max, false,
    );

    let (pb_ticks_x, labs_x) =
        path_builder_ticks_x(w, h, axes_padd, &ticks_x, tick_size, text_size);
    let (pb_ticks_y, labs_y) =
        path_builder_ticks_y(h, axes_padd, &ticks_y, tick_size, text_size);

    _ = apply_iced_path_to_gc(
        pb_ticks_x.build(),
        apply_iced_stroke_to_gc(STRK_1_BLK, scaling, pg.graphics()),
    )
    .stroke();

    _ = apply_iced_path_to_gc(
        pb_ticks_y.build(),
        apply_iced_stroke_to_gc(STRK_1_BLK, scaling, pg.graphics()),
    )
    .stroke();

    for lab in labs_x {
        let text = lab.text.content;
        let x = lab.text.position.x as f64;
        let y = lab.text.position.y as f64;
        let size = text_size as f64;
        let text_w = measure_text(&text, &font, size);
        write_label_below(
            &text,
            x,
            y,
            size,
            lab_offset as f64,
            text_w,
            font.clone(),
            pg,
        );
    }

    for lab in labs_y {
        let text = lab.text.content;
        let x = lab.text.position.x as f64;
        let y = lab.text.position.y as f64;
        write_label_left(
            &text,
            x,
            y,
            text_size as f64,
            lab_offset as f64,
            font.clone(),
            pg,
        );
    }

    let pb_axis_x = path_builder_x_axis(w, h, axes_padd);
    let pb_axis_y = path_builder_y_axis(w, h, axes_padd);

    _ = apply_iced_path_to_gc(
        pb_axis_x.build(),
        apply_iced_stroke_to_gc(STRK_1_BLK, scaling, pg.graphics()),
    )
    .stroke();

    _ = apply_iced_path_to_gc(
        pb_axis_y.build(),
        apply_iced_stroke_to_gc(STRK_1_BLK, scaling, pg.graphics()),
    )
    .stroke();
}

#[allow(clippy::too_many_arguments)]
fn draw_gts_bands(
    w: Float,
    h: Float,
    x_axis_scale_type: AxisScaleType,
    ltt_plot_data: &PlotData,
    scaling: f64,
    font: Font,
    pg: &mut Page,
) -> Result<(), PdfError> {
    let x_min = 0e0;
    let x_max = ltt_plot_data.x_max - ltt_plot_data.x_min;

    let ranks_to_draw = [GtsRank::Era, GtsRank::Period, GtsRank::Epoch];

    let mut rank_indexes: HashMap<&GtsRank, usize> =
        HashMap::with_capacity(ranks_to_draw.len());
    for (i, rank) in ranks_to_draw.iter().enumerate() {
        _ = rank_indexes.insert(rank, i);
    }

    let text_size = SF * 1e1 * scaling as Float;
    let char_width = text_size * 6e-1;
    let h_unit = h / ranks_to_draw.len() as Float;

    for gts in gts_data() {
        let beg = gts.beg;
        let end = gts.end;
        let rank = &gts.rank;
        let name = &gts.name;
        let color = gts.color.scale_alpha(0.77);

        if (beg > x_max && end < x_max)
            || (beg <= x_max && end >= x_min)
            || (beg > x_min && end < x_min)
        {
            let Some(rank_index) = rank_indexes.get(rank) else {
                continue;
            };

            let y1 = *rank_index as Float * h_unit;
            let y2 = (*rank_index + 1) as Float * h_unit;

            let br = transformed_relative_value(
                beg, x_min, x_max, x_axis_scale_type,
            )
            .unwrap_or(0e0);
            let er = transformed_relative_value(
                end, x_min, x_max, x_axis_scale_type,
            )
            .unwrap_or(0e0);

            let beg_rel = (1.0 - br).max(0e0);
            let end_rel = (1.0 - er).min(1e0);

            let p1 = Point { x: w * beg_rel, y: y1 };
            let p2 = Point { x: w * end_rel, y: y1 };
            let p3 = Point { x: w * end_rel, y: y2 };
            let p4 = Point { x: w * beg_rel, y: y2 };

            let box_w = p3.x - p1.x;
            let box_h = p3.y - p1.y;
            let p_center =
                Point { x: box_w / 2e0 + p1.x, y: box_h / 2e0 + p1.y };

            let path = PathBuilder::new()
                .move_to(p1)
                .line_to(p2)
                .line_to(p3)
                .line_to(p4)
                .close()
                .build();

            let fill_color = color_from_iced_color(color);
            let fill_alpha = alpha_from_iced_color(color);

            _ = pg.graphics().save_state();
            _ = apply_iced_path_to_gc(path.clone(), pg.graphics());
            _ = pg
                .graphics()
                .set_fill_color(fill_color)
                .set_alpha_fill(fill_alpha)?;
            _ = pg.graphics().fill();
            _ = pg.graphics().restore_state();

            _ = apply_iced_path_to_gc(
                path,
                apply_iced_stroke_to_gc(STRK_1_BLK_25, scaling, pg.graphics()),
            )
            .stroke();

            let mut name_len = 0;
            name.split(' ').for_each(|word| {
                name_len = name_len.max(word.len() + 2);
            });

            if box_w > char_width * name_len as Float {
                write_label_gts_centered(
                    &name.replace(' ', "\n"),
                    p_center.x as f64,
                    p_center.y as f64,
                    text_size as f64,
                    font.clone(),
                    pg,
                );
            }
        }
    }

    Ok(())
}

fn draw_ltt_curve(
    ltt_plot_data: &PlotData,
    x_axis_scale_type: AxisScaleType,
    y_axis_scale_type: AxisScaleType,
    w: Float,
    h: Float,
    scaling: f64,
    pg: &mut Page,
) {
    let path_ltt = path_builder_ltt(
        ltt_plot_data, x_axis_scale_type, y_axis_scale_type, w, h,
    )
    .build();

    _ = apply_iced_path_to_gc(
        path_ltt.clone(),
        apply_iced_stroke_to_gc(STRK_4_WHT_75, scaling, pg.graphics()),
    )
    .stroke();

    _ = apply_iced_path_to_gc(
        path_ltt,
        apply_iced_stroke_to_gc(STRK_2_BLU, scaling, pg.graphics()),
    )
    .stroke();
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_scale_bar(
    tree_style: TreSty,
    cnv_vs: &RectVals<Float>,
    tre_vs: &RectVals<Float>,
    root_len: Float,
    subtree_node_len: Float,
    tre_height: Float,
    tre_unit: TreUnit,
    scaling: f64,
    font: Font,
    pg: &mut Page,
) {
    let stroke = STRK_2_BLK;
    let lab_size = SF * 12e0 * scaling as Float;
    let lab_y_offset = SF * 10e0 * scaling as Float;

    let w = match tree_style {
        TreSty::PhyGrm => tre_vs.w,
        TreSty::Fan => tre_vs.radius_min - root_len,
    };
    let tre_height = tre_height + subtree_node_len;

    let prelim_w_for_sb = w / 4.0;
    let prelim_sb_tre_height_frac = prelim_w_for_sb / w;
    let prelim_sb_len = prelim_sb_tre_height_frac * tre_height;
    let sb_len = normalize_scale_value(prelim_sb_len, AxisScaleType::LogTen);
    let sb_frac = sb_len / tre_height;
    let scale_bar_width = sb_frac * w;

    let number = if sb_len < 0.01 {
        format!("{sb_len:.2E}")
    } else if sb_len <= 1.0 {
        format!("{sb_len:0.3}")
    } else {
        format!("{sb_len:0.0}")
    };
    let units = match tre_unit {
        TreUnit::Unitless => "",
        TreUnit::Substitutions => " Substitutions per site",
        TreUnit::MillionYears => " Million years",
        TreUnit::CoalescentUnits => " Coalescent units",
    };
    let label = number + units;

    let char_width = lab_size * 6e-1;
    let lab_width = char_width * label.len() as Float;
    let total_width = lab_width.max(scale_bar_width);
    let lab_sb_width_diff = lab_width - scale_bar_width;
    let left_offset_due_to_label =
        if lab_sb_width_diff > ZRO { lab_sb_width_diff / TWO } else { 0e0 };

    let pad = PADDING * scaling as Float;
    let layout_x0 = cnv_vs.x0 + pad * TWO;
    let x = layout_x0 + left_offset_due_to_label;
    let y = cnv_vs.y0 + cnv_vs.h
        - stroke.width * scaling as Float / TWO
        - lab_size
        - lab_y_offset
        - pad * TWO;

    let p0 = Point { x, y };
    let p1 = Point { x: x + scale_bar_width, y };
    let path = PathBuilder::new().move_to(p0).line_to(p1).build();

    _ = apply_iced_path_to_gc(
        path,
        apply_iced_stroke_to_gc(stroke, scaling, pg.graphics()),
    )
    .stroke();

    // UI Label.width = total_width; Center on the layout box (same as bar
    // midpoint after left_offset). Horizontal half-width uses lab_width so
    // centering stays consistent with the char-width left_offset math.
    let p_lab_x = layout_x0 + total_width / TWO;
    write_label_below(
        &label, p_lab_x as f64, y as f64, lab_size as f64, lab_y_offset as f64,
        lab_width as f64, font, pg,
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_height_axis(
    cnv_vs: &RectVals<Float>,
    tre_vs: &RectVals<Float>,
    draw_labs_tip: bool,
    lab_size_tip: Float,
    height_axis_scale_type: AxisScaleType,
    height_axis_min: Float,
    height_axis_max: Float,
    height_axis_is_reversed: bool,
    height_axis_text_size: Float,
    height_axis_char_width: Float,
    height_axis_tick_size: Float,
    height_axis_lab_offset: Float,
    tree_style: TreSty,
    scaling: f64,
    font: Font,
    pg: &mut Page,
) {
    match tree_style {
        TreSty::Fan => {}
        TreSty::PhyGrm => {
            let n_ticks_x = (tre_vs.w / (height_axis_char_width * 10.0))
                .floor()
                .max(3e0) as usize;

            let (ticks_x, _) = calc_ticks(
                n_ticks_x,
                height_axis_scale_type,
                AxisDataType::Continuous,
                height_axis_min,
                height_axis_max,
                height_axis_is_reversed,
            );

            let bottom = cnv_vs.y1
                + PADDING * TWO * (scaling as Float)
                + if draw_labs_tip { -lab_size_tip / TWO } else { ZRO };

            let (pb_ticks_x, labs_x) = path_builder_ticks_x(
                tre_vs.w, bottom, ZRO, &ticks_x, height_axis_tick_size,
                height_axis_text_size,
            );

            let pb_axis_x = PathBuilder::new()
                .move_to(Point { x: ZRO, y: bottom })
                .line_to(Point { x: tre_vs.w, y: bottom })
                .build();

            // Shift into canvas X (tree origin), keep Y in canvas space.
            _ = pg.graphics().save_state();
            _ = pg.graphics().translate(tre_vs.trans.x as f64, 0e0);

            _ = apply_iced_path_to_gc(
                pb_ticks_x.build(),
                apply_iced_stroke_to_gc(STRK_1_BLK, scaling, pg.graphics()),
            )
            .stroke();

            _ = apply_iced_path_to_gc(
                pb_axis_x,
                apply_iced_stroke_to_gc(STRK_2_BLK, scaling, pg.graphics()),
            )
            .stroke();

            for lab in labs_x {
                let text = lab.text.content;
                let x = lab.text.position.x as f64;
                let y = lab.text.position.y as f64;
                let size = height_axis_text_size as f64;
                let text_w = measure_text(&text, &font, size);
                write_label_below(
                    &text,
                    x,
                    y,
                    size,
                    height_axis_lab_offset as f64,
                    text_w,
                    font.clone(),
                    pg,
                );
            }

            _ = pg.graphics().restore_state();
        }
    }
}
