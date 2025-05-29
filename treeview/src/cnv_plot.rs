use crate::iced::*;
use crate::path_utils::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum PlotData {
    Simple(Simple),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Simple {
    x: Float,
    y: Float,
}

// impl From<PlotData> for Point {
//     fn from(pd: PlotData) -> Self {
//         match pd {
//             PlotData::Simple(simple) => Point { x: simple.x, y: simple.y },
//         }
//     }
// }

// impl From<&PlotData> for Point {
//     fn from(pd: &PlotData) -> Self { (*pd).into() }
// }

impl From<LttPoint> for PlotData {
    fn from(lttp: LttPoint) -> Self { (&lttp).into() }
}

impl From<&LttPoint> for PlotData {
    fn from(lttp: &LttPoint) -> Self { PlotData::Simple(Simple { x: lttp.time as Float, y: lttp.count as Float }) }
}

#[derive(Debug, Default)]
pub(super) struct PlotCnv {
    plot_data: Vec<PlotData>,
    pub(super) draw_cursor_line: bool,
    pub(super) tre_padd: Float,
    pub(super) crsr_x_rel: Option<Float>,
    pub(super) cache_bnds: Cache,
    pub(super) cache_cursor_line: Cache,
    pub(super) cache_plot: Cache,
}

impl PlotCnv {
    pub(super) fn clear_cache_bnds(&self) { self.cache_bnds.clear() }
    pub(super) fn clear_cache_plot(&self) { self.cache_plot.clear() }
    pub(super) fn clear_cache_cursor_line(&self) { self.cache_cursor_line.clear() }
    pub fn set_plot_data(&mut self, data: &[PlotData]) { self.plot_data = data.to_vec(); }
    pub fn clear_plot_data(&mut self) { self.plot_data.clear(); }
}

#[derive(Debug, Default)]
pub struct St {
    pub(crate) bnds: Rectangle<Float>,

    pub(crate) cursor_tracking_point: Option<Point>,
    pub(crate) translation: Vector,

    pub(crate) cnv_vs: RectVals<Float>,
    pub(crate) cnv_rect: Rectangle<Float>,

    pub(crate) plt_vs: RectVals<Float>,
    pub(crate) plt_rect: Rectangle<Float>,
}

impl St {
    pub(super) fn cursor_tracking_point(&mut self, crsr: Cursor) -> Option<Action<TvMsg>> {
        if let Some(mouse) = crsr.position_in(self.bnds) {
            let adj = mouse - self.translation;
            let crsr_x_rel = adj.x / self.plt_vs.w;
            if (ZRO - EPSILON..=ONE + EPSILON).contains(&crsr_x_rel)
                && (ZRO - EPSILON..=self.plt_vs.h + EPSILON).contains(&adj.y)
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
    fn mouse_interaction(&self, _st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction { Interaction::default() }
    fn update(&self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor) -> Option<Action<TvMsg>> {
        // -------------------------------------------------------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // -------------------------------------------------------------------------------------------------------------
        if bnds != st.bnds {
            st.bnds = bnds;
            st.cnv_vs = RectVals::cnv(bnds);
            st.plt_vs = RectVals::tre(st.cnv_vs, self.tre_padd);
            st.cnv_rect = st.cnv_vs.into();
            st.plt_rect = st.plt_vs.into();
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
        // -------------------------------------------------------------------------------------------------------------
        if let Some(crsr_x_rel) = self.crsr_x_rel {
            st.cursor_tracking_point = Some(Point { x: crsr_x_rel * st.plt_vs.w, y: ZRO });
        }
        // -------------------------------------------------------------------------------------------------------------
        action
    }

    fn draw(&self, st: &St, rndr: &Renderer, _thm: &Theme, bnds: Rectangle, _crsr: Cursor) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();
        // -----------------------------------------------------------
        let size = bnds.size();
        draw_bounds(self, st, rndr, bnds, &mut geoms);
        draw_cursor_line(self, st, rndr, size, &mut geoms);
        draw_plot(self, st, rndr, size, &mut geoms);
        // -----------------------------------------------------------
        geoms
    }
}

fn draw_bounds(plt: &PlotCnv, st: &St, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>) {
    g.push(plt.cache_bnds.draw(rndr, bnds.size(), |f| {
        stroke_rect(st.cnv_rect, STRK_5_BLU_50, f);
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
            let p0 = Point { x: p.x, y: ZRO };
            let p1 = Point { x: p.x, y: st.plt_vs.h };
            f.stroke(&PathBuilder::new().move_to(p0).line_to(p1).build(), STRK_CRSR_LINE);
            f.pop_transform();
        }
    }));
}

fn draw_plot(plt: &PlotCnv, st: &St, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(plt.cache_plot.draw(rndr, sz, |f| {
        let mut pb: PathBuilder = PathBuilder::new();

        let mut max_count: Float = ZRO;
        for PlotData::Simple(Simple { x: _, y }) in &plt.plot_data {
            max_count = max_count.max(*y)
        }

        let calc_y =
            |count: Float| st.plt_vs.h - (((count as Float).log10() / (max_count as Float).log10()) * st.plt_vs.h);

        let PlotData::Simple(Simple { x: x0, y: y0 }) = &plt.plot_data[0];
        let pt0 = Point { x: *x0 * st.plt_vs.w, y: calc_y(*y0) };
        pb = pb.move_to(pt0);

        for PlotData::Simple(Simple { x, y }) in &plt.plot_data[1..] {
            let pt_x = *x * st.plt_vs.w;
            let pt = Point { x: pt_x, y: calc_y(*y) };
            pb = pb.line_to(pt);
        }

        f.push_transform();
        f.translate(st.translation);
        f.stroke(&pb.build(), STRK_1_RED_75);
        f.pop_transform();
    }));
}
