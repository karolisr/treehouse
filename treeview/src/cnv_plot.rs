use std::fmt::Debug;

use crate::iced::*;
use crate::path_utils::*;
use crate::*;

#[derive(Debug, Default)]
pub(super) struct PlotCnv {
    plot_data: PlotData,
    pub(super) draw_debug: bool,
    pub(super) draw_cursor_line: bool,
    pub(super) crsr_x_rel: Option<Float>,
    pub(super) cache_bnds: Cache,
    pub(super) cache_cursor_line: Cache,
    pub(super) cache_plot: Cache,
    pub(super) padd_l: Float,
    pub(super) padd_r: Float,
    pub(super) padd_t: Float,
    pub(super) padd_b: Float,
    pub(super) vis_x0: Float,
    pub(super) vis_y0: Float,
}

impl PlotCnv {
    pub(super) fn clear_cache_bnds(&self) { self.cache_bnds.clear() }
    pub(super) fn clear_cache_cursor_line(&self) { self.cache_cursor_line.clear() }
    pub(super) fn clear_cache_plot(&self) { self.cache_plot.clear() }

    pub(super) fn clear_caches_all(&self) {
        self.clear_cache_bnds();
        self.clear_cache_cursor_line();
        self.clear_cache_plot();
    }

    pub(super) fn set_plot_data(&mut self, data: impl Into<PlotData>) {
        self.clear_cache_plot();
        self.plot_data = data.into();
    }
}

#[derive(Debug, Default)]
pub struct St {
    pub(super) bnds: Rectangle<Float>,
    pub(super) cursor_tracking_point: Option<Point>,
    pub(super) translation: Vector,
    pub(super) plt_vs: RectVals<Float>,
    pub(super) plt_rect: Rectangle<Float>,
    pub(super) plt_padd_l: Float,
    pub(super) plt_padd_r: Float,
    pub(super) plt_padd_t: Float,
    pub(super) plt_padd_b: Float,
    pub(super) text_w: Option<TextWidth<'static>>,
    pub(super) text_size: Float,
}

impl St {
    pub(super) fn cursor_tracking_point(&mut self, crsr: Cursor) -> Option<Action<TvMsg>> {
        if let Some(mouse) = crsr.position_in(self.bnds) {
            let adj = mouse - self.translation;
            let crsr_x_rel = adj.x / self.plt_vs.w;
            if (ZERO - EPSILON..=ONE + EPSILON).contains(&crsr_x_rel)
                && (ZERO - EPSILON..=self.plt_vs.h + EPSILON).contains(&adj.y)
            {
                self.cursor_tracking_point = Some(adj);
                Some(Action::publish(TvMsg::CursorOnLttCnv { x: Some(crsr_x_rel) }))
            } else {
                self.cursor_tracking_point = None;
                Some(Action::publish(TvMsg::CursorOnLttCnv { x: None }))
            }
        } else {
            self.cursor_tracking_point = None;
            None
        }
    }
}

impl Program<TvMsg> for PlotCnv {
    type State = St;
    fn mouse_interaction(&self, _st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction {
        Interaction::default()
    }
    fn update(
        &self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        // -----------------------------------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // -----------------------------------------------------------------------------------------
        if bnds != st.bnds
            || st.plt_padd_l != self.padd_l
            || st.plt_padd_r != self.padd_r
            || st.plt_padd_t != self.padd_t
            || st.plt_padd_b != self.padd_b
        {
            self.clear_cache_bnds();
            self.clear_cache_plot();
            st.bnds = bnds;
            st.text_size = SF * 12.0;
            let extra_padding = SF * TEN;
            st.plt_vs = RectVals::cnv(bnds).padded(
                self.padd_l + extra_padding * ZERO,
                self.padd_r + extra_padding * ZERO,
                self.padd_t + extra_padding * ONE,
                self.padd_b + extra_padding * THREE,
            );
            st.plt_rect = st.plt_vs.into();
            st.plt_padd_l = self.padd_l;
            st.plt_padd_r = self.padd_r;
            st.plt_padd_t = self.padd_t;
            st.plt_padd_b = self.padd_b;
        }

        st.translation = st.plt_vs.trans;

        if let Event::Mouse(mouse_ev) = ev {
            match mouse_ev {
                MouseEvent::CursorEntered => {
                    self.clear_cache_cursor_line();
                    st.cursor_tracking_point = None;
                }
                MouseEvent::CursorMoved { position: _ } => {
                    self.clear_cache_cursor_line();
                    action = st.cursor_tracking_point(crsr);
                }
                MouseEvent::CursorLeft => {
                    self.clear_cache_cursor_line();
                    st.cursor_tracking_point = None;
                }
                _ => {}
            }
        }
        // -----------------------------------------------------------------------------------------
        if let Some(crsr_x_rel) = self.crsr_x_rel {
            st.cursor_tracking_point = Some(Point { x: crsr_x_rel * st.plt_vs.w, y: ZERO });
        }
        // -----------------------------------------------------------------------------------------
        action
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, _thm: &Theme, bnds: Rectangle, _crsr: Cursor,
    ) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();
        // -----------------------------------------------------------------------------------------
        let size = bnds.size();
        if self.draw_debug {
            draw_bounds(self, st, rndr, bnds, &mut geoms);
        }
        draw_plot(self, st, rndr, size, &mut geoms);
        draw_cursor_line(self, st, rndr, size, &mut geoms);
        // -----------------------------------------------------------------------------------------
        geoms
    }
}

fn draw_bounds(plt: &PlotCnv, st: &St, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>) {
    g.push(plt.cache_bnds.draw(rndr, bnds.size(), |f| {
        stroke_rect(st.plt_rect, STRK_3_GRN_50, f);
    }));
}

fn draw_cursor_line(plt: &PlotCnv, st: &St, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
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

fn path_builder_plot(data: &PlotData, w: Float, h: Float) -> PathBuilder {
    let mut first = true;
    let mut pb: PathBuilder = PathBuilder::new();
    for plot_point in &data.points {
        let pt = Point { x: plot_point.x * w, y: plot_point.y * h };
        if first {
            pb = pb.move_to(pt);
            first = false;
        } else {
            pb = pb.line_to(pt);
        }
    }
    pb
}

pub struct Tick {
    relative_position: Float,
    label: String,
}

impl Display for Tick {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(pos: {:.2}, lab: {})", self.relative_position, self.label)
    }
}

impl Debug for Tick {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{self}") }
}

fn calc_ticks(scale: AxisScale, min: Float, max: Float, _tick_count: usize) -> Vec<Tick> {
    let mut ticks: Vec<Tick> = Vec::new();
    let _range = max - min;
    match scale {
        AxisScale::Linear => {
            let tick_min = ZERO;
            let tick_max = ONE;
            ticks.push(Tick { relative_position: tick_min, label: format!("{min:.2}") });
            ticks.push(Tick { relative_position: tick_max, label: format!("{max:.2}") });
        }
        AxisScale::LogTen => {
            let tick_min = ZERO;
            let tick_max = ONE;
            ticks.push(Tick { relative_position: tick_min, label: format!("{min:.2}") });
            ticks.push(Tick { relative_position: tick_max, label: format!("{max:.2}") });
        }
    }
    ticks
}

fn path_builder_axes(
    data: &PlotData, w: Float, h: Float, lab_size: Float,
) -> (PathBuilder, Vec<Label>, Vec<Label>) {
    let ticks_x = calc_ticks(data.x_scale, data.x_min, data.x_max, 2);
    let ticks_y = calc_ticks(data.y_scale, data.y_min, data.y_max, 2);

    let tick_size = SF * TEN;
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
    }
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
    }
    // ---------------------------------------------------------------------------------------------
    (pb, labs_x, labs_y)
}

fn draw_plot(plt: &PlotCnv, st: &St, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(plt.cache_plot.draw(rndr, sz, |f| {
        let pb_plot = path_builder_plot(&plt.plot_data, st.plt_vs.w, st.plt_vs.h);
        let (pb_axes, labs_x, labs_y) =
            path_builder_axes(&plt.plot_data, st.plt_vs.w, st.plt_vs.h, st.text_size);

        f.push_transform();
        f.translate(st.translation);
        f.stroke(&pb_plot.build(), STRK_1_RED_75);
        f.stroke(&pb_axes.build(), STRK_1_BLK);
        f.pop_transform();

        draw_labels(&labs_x, Vector { x: ZERO, y: SF * FIVE }, Some(st.translation), ZERO, f);
        draw_labels(&labs_y, Vector { x: SF * FIVE, y: ZERO }, Some(st.translation), ZERO, f);
    }));
}

#[derive(Debug, Default, Clone, Copy)]
pub enum AxisScale {
    #[default]
    Linear,
    LogTen,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PlotPoint {
    x: Float,
    y: Float,
}

#[derive(Debug, Default, Clone)]
pub struct PlotData {
    x_scale: AxisScale,
    y_scale: AxisScale,
    x_min: Float,
    y_min: Float,
    x_max: Float,
    y_max: Float,
    points: Vec<PlotPoint>,
}

impl From<&Vec<LttPoint>> for PlotData {
    fn from(ltt_points: &Vec<LttPoint>) -> Self {
        let x_max = ltt_points[ltt_points.len() - 1].time as Float;

        // let mut min_count: usize = 100;
        let mut max_count: usize = 0;
        for LttPoint { time: _, count } in ltt_points {
            // min_count = min_count.min(*count);
            max_count = max_count.max(*count);
        }

        let y_max = max_count as Float;
        let y_max_log_10 = y_max.log10();

        // let y_min = min_count as Float;
        let y_min = ONE;
        let y_min_log_10 = y_min.log10();

        let calc_x = |time: Float| time / x_max;
        let calc_y = |count: Float| ONE - (count.log10() - y_min_log_10) / y_max_log_10;
        let mut points: Vec<PlotPoint> = Vec::with_capacity(ltt_points.len());

        for LttPoint { time, count } in ltt_points {
            let pt: PlotPoint = PlotPoint { x: calc_x(*time as Float), y: calc_y(*count as Float) };
            points.push(pt);
        }

        PlotData {
            x_scale: AxisScale::Linear,
            y_scale: AxisScale::LogTen,
            x_min: ZERO,
            y_min,
            x_max,
            y_max,
            points,
        }
    }
}
