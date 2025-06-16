use super::{AxisScaleType, PlotData, St, Tick};
use crate::cnv_utils::*;
use crate::iced::*;
use crate::path_utils::*;
use crate::*;

pub(super) fn draw_plot(plt: &PlotCnv, st: &St, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(plt.cache_plot.draw(rndr, sz, |f| {
        let pb_plot =
            path_builder_plot(&plt.plot_data, &plt.scale_x, &plt.scale_y, st.plt_vs.w, st.plt_vs.h);

        let (pb_axes, labs_x, labs_y) = path_builder_axes(
            &plt.plot_data, &plt.scale_x, &plt.scale_y, st.plt_vs.w, st.plt_vs.h, st.text_size,
        );

        f.push_transform();
        f.translate(st.translation);
        f.stroke(&pb_plot.build(), STRK_1_BLK);
        f.stroke(&pb_axes.build(), STRK_1_BLK);
        f.pop_transform();

        let lab_offset = SF * FIVE;

        draw_labels(&labs_x, Vector { x: ZERO, y: lab_offset }, Some(st.translation), ZERO, f);
        draw_labels(&labs_y, Vector { x: lab_offset, y: ZERO }, Some(st.translation), ZERO, f);
    }));
}

fn path_builder_plot(
    data: &PlotData, scale_x: &AxisScaleType, scale_y: &AxisScaleType, w: Float, h: Float,
) -> PathBuilder {
    let mut first = true;
    let mut pb: PathBuilder = PathBuilder::new();
    for plot_point in &data.plot_points {
        let x_relative =
            transform_value(plot_point.x, scale_x) / transform_value(data.x_max, scale_x);
        let y_relative =
            transform_value(plot_point.y, scale_y) / transform_value(data.y_max, scale_y);

        let pt = Point { x: x_relative * w, y: (ONE - y_relative) * h };
        if first {
            pb = pb.move_to(pt);
            first = false;
        } else {
            pb = pb.line_to(pt);
        }
    }
    pb
}

fn calc_ticks(
    tick_count: usize, scale: &AxisScaleType, data_type: &PlotDataType, max: Float,
) -> Vec<Tick> {
    let mut ticks: Vec<Tick> = Vec::new();
    let max_transformed = transform_value(max, scale);
    let mut tt1: Float = max_transformed * ONE / (tick_count) as Float;
    if *scale != AxisScaleType::Linear {
        tt1 = tt1.floor().max(ONE)
    } else if tt1 > TEN {
        tt1 = (tt1 / TEN).floor() * TEN
    } else if tt1 > TWO {
        tt1 = tt1.floor()
    } else {
        tt1 = (tt1 * 2e1).floor() / 2e1
    }

    for i in 0..=tick_count - 1 {
        let i_float = i as Float;
        let tick = match scale {
            AxisScaleType::Linear => tt1 + i_float * tt1,
            AxisScaleType::LogTwo => TWO.powf(tt1 + i_float * tt1),
            AxisScaleType::LogNat => E.powf(tt1 + i_float * tt1),
            AxisScaleType::LogTen => TEN.powf(tt1 + i_float * tt1),
        };

        ticks.push(Tick {
            relative_position: transform_value(tick, scale) / transform_value(max, scale),
            label: match data_type {
                PlotDataType::Continuous => format!("{tick:.2}"),
                PlotDataType::Discrete => format!("{tick:.2}",),
            },
        });
    }

    ticks
}

fn path_builder_axes(
    data: &PlotData, scale_x: &AxisScaleType, scale_y: &AxisScaleType, w: Float, h: Float,
    lab_size: Float,
) -> (PathBuilder, Vec<Label>, Vec<Label>) {
    let ticks_x = calc_ticks(6, scale_x, &data.x_data_type, data.x_max);
    let ticks_y = calc_ticks(5, scale_y, &data.y_data_type, data.y_max);

    let tick_size = SF * FIVE;
    let y_for_ticks_x = h;
    let x_for_ticks_y = ZERO;

    let mut pb: PathBuilder = PathBuilder::new();
    let mut labs_x: Vec<Label> = Vec::with_capacity(ticks_x.len());
    let mut labs_y: Vec<Label> = Vec::with_capacity(ticks_y.len());

    // x-axis ticks --------------------------------------------------------------------------------
    let pt_min = Point { x: ZERO, y: y_for_ticks_x };
    let pt_max = Point { x: w, y: y_for_ticks_x };
    pb = pb.move_to(pt_min);
    pb = pb.line_to(pt_max);
    for Tick { relative_position, label } in ticks_x {
        let x = relative_position * w;
        let pt0 = Point { x, y: y_for_ticks_x };
        let pt1 = Point { x, y: y_for_ticks_x + tick_size };
        pb = pb.move_to(pt0);
        pb = pb.line_to(pt1);

        let text = lab_text(label.to_string(), pt1, lab_size, TEMPLATE_TXT_LAB_PLOT_AXIS_X);
        let label = Label { text, width: ZERO, angle: None };
        labs_x.push(label);
    } // -------------------------------------------------------------------------------------------

    // y-axis ticks --------------------------------------------------------------------------------
    let pt_min = Point { x: x_for_ticks_y, y: ZERO };
    let pt_max = Point { x: x_for_ticks_y, y: h };
    pb = pb.move_to(pt_min);
    pb = pb.line_to(pt_max);
    for Tick { relative_position, label } in ticks_y {
        let y = (ONE - relative_position) * h;
        let pt0 = Point { x: x_for_ticks_y, y };
        let pt1 = Point { x: x_for_ticks_y + tick_size, y };
        pb = pb.move_to(pt0);
        pb = pb.line_to(pt1);

        let text = lab_text(label.to_string(), pt1, lab_size, TEMPLATE_TXT_LAB_PLOT_AXIS_Y);
        let label = Label { text, width: ZERO, angle: None };
        labs_y.push(label);
    } // -------------------------------------------------------------------------------------------

    (pb, labs_x, labs_y)
}

pub(super) fn draw_bounds(
    plt: &PlotCnv, st: &St, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_bnds.draw(rndr, bnds.size(), |f| {
        stroke_rect(st.plt_rect, STRK_3_GRN_50, f);
    }));
}

pub(super) fn draw_cursor_line(
    plt: &PlotCnv, st: &St, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cursor_line.draw(rndr, sz, |f| {
        if let Some(p) = st.cursor_tracking_point
            && plt.draw_cursor_line
        {
            f.push_transform();
            f.translate(st.translation);
            let p0 = Point { x: p.x, y: ZERO };
            let p1 = Point { x: p.x, y: st.plt_vs.h };
            f.stroke(&PathBuilder::new().move_to(p0).line_to(p1).build(), STRK_CRSR_LINE);
            f.pop_transform();

            let mut txt_template = TEMPLATE_TXT_CURSOR_TEXT;
            let mut y_offset = -PADDING;
            let tree_height_at_x = if let Some(crsr_x_rel) = plt.crsr_x_rel {
                y_offset = st.plt_padd_t;
                txt_template.align_y = Vertical::Top;
                plt.plot_data.x_max * crsr_x_rel
            } else {
                plt.plot_data.x_max * p.x / st.plt_vs.w
            };

            let name = format!("{tree_height_at_x:.3}");
            let text = lab_text(name, p, st.text_size, txt_template);
            let label = Label { text, width: ZERO, angle: None };
            draw_labels(
                &[label],
                Vector { x: PADDING, y: y_offset },
                Some(st.translation),
                ZERO,
                f,
            );
        }
    }));
}
