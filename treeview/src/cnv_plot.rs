use crate::iced::*;
use crate::path_utils::*;
use crate::*;

#[derive(Debug, Default)]
pub(super) struct PlotCnv {
    pub(super) draw_cursor_line: bool,
    pub(super) tre_padd: Float,
    pub(super) crsr_x_rel: Option<Float>,
    pub(super) cache_bnds: Cache,
    pub(super) cache_cursor_line: Cache,
}
impl PlotCnv {
    pub(super) fn clear_cache_bnds(&self) { self.cache_bnds.clear() }
    pub(super) fn clear_cache_cursor_line(&self) { self.cache_cursor_line.clear() }
}

#[derive(Debug, Default)]
pub struct St {
    pub(crate) cursor_tracking_point: Option<Point>,
    pub(crate) bnds: Rectangle<Float>,
    pub(crate) translation: Vector,
    pub(crate) cnv_vs: RectVals<Float>,
    pub(crate) plt_vs: RectVals<Float>,
    pub(crate) cnv_rect: Rectangle<Float>,
    pub(crate) plt_rect: Rectangle<Float>,
}

impl St {
    pub(super) fn cursor_tracking_point(&mut self, crsr: Cursor) -> Option<Action<TvMsg>> {
        if let Some(mouse) = crsr.position_in(self.bnds) {
            let adj = mouse - self.translation;
            let crsr_x_rel = (adj.x / self.plt_vs.w).clamp(0e0, 1e0);
            if crsr_x_rel > 0e0 && crsr_x_rel < 1e0 && adj.y > 0e0 && adj.y < self.plt_vs.h {
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
            st.cursor_tracking_point = Some(Point { x: crsr_x_rel * st.plt_vs.w, y: 0e0 });
        }
        // -------------------------------------------------------------------------------------------------------------
        action
    }

    fn draw(&self, st: &St, rndr: &Renderer, _thm: &Theme, bnds: Rectangle, _crsr: Cursor) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();
        // -----------------------------------------------------------
        let size = bnds.size();
        // draw_bounds(self, st, rndr, bnds, &mut geoms);
        draw_cursor_line(self, st, rndr, size, &mut geoms);
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
            let p0 = Point { x: p.x, y: 0e0 };
            let p1 = Point { x: p.x, y: st.plt_vs.h };
            f.stroke(&PathBuilder::new().move_to(p0).line_to(p1).build(), STRK_1_RED_50_DASH);
            f.pop_transform();
        }
    }));
}
