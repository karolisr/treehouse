use super::{AxisScaleType, PlotData, St, Tick};
use crate::cnv_utils::*;
use crate::*;

pub(super) fn draw_plot(
    plt: &PlotCnv,
    st: &St,
    rndr: &Renderer,
    size: Size,
    g: &mut Vec<Geometry>,
) {
    g.push(plt.cache_cnv_plot.draw(rndr, size, |f| {
        let (pb_axes, labs_x, labs_y) = path_builder_axes(
            &st.ticks_x, &st.ticks_y, st.plt_vs.w, st.plt_vs.h, st.tick_size,
            st.text_size,
        );

        let pb_plot = path_builder_plot(
            &st.plot_data, plt.scale_x, plt.scale_y, st.plt_vs.w, st.plt_vs.h,
        );

        f.push_transform();
        f.translate(st.plt_vs.trans);
        f.stroke(&pb_plot.build(), STRK_1_BLK);
        f.stroke(&pb_axes.build(), STRK_1_BLK);
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

fn path_builder_plot(
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
                plt.plot_data.x_min + x_val_range * crsr_x_rel
            } else {
                let crsr_x_rel = p.x / st.plt_vs.w;
                plt.plot_data.x_min + x_val_range * crsr_x_rel
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
