mod draw;
mod state;

use draw::*;
use state::St;

use crate::*;

#[derive(Debug, Default)]
pub(super) struct PlotCnv {
    plot_data: PlotData,
    pub(super) scale_x: AxisScaleType,
    pub(super) scale_y: AxisScaleType,
    pub(super) draw_debug: bool,
    pub(super) draw_cursor_line: bool,
    pub(super) time_axis_reversed: bool,
    pub(super) crsr_x_rel: Option<Float>,
    pub(super) cache_cnv_gts: CnvCache,
    pub(super) cache_cnv_ltt: CnvCache,
    pub(super) cache_cnv_axes: CnvCache,
    pub(super) cache_cnv_cursor_line: CnvCache,
    pub(super) cache_cnv_bnds: CnvCache,
    pub(super) padd_l: Float,
    pub(super) padd_r: Float,
    pub(super) padd_t: Float,
    pub(super) padd_b: Float,
    pub(super) vis_x0: Float,
    pub(super) vis_y0: Float,
}

pub(super) const AXIS_SCALE_TYPE_OPTS: [AxisScaleType; 3] =
    [AxisScaleType::Linear, AxisScaleType::LogTwo, AxisScaleType::LogTen];

pub fn plot_data_from_ltt_points(
    ltt_points: &[LttPoint],
    x_offset: Float,
) -> PlotData {
    let x_min: Float = x_offset;
    let mut x_max: Float = Float::MIN;
    let y_min: Float = 0.0;
    let mut y_max: Float = Float::MIN;

    for LttPoint { time, count } in ltt_points {
        x_max = x_max.max(x_offset + *time as Float);
        y_max = y_max.max(*count as Float);
    }

    let mut plot_points = Vec::with_capacity(ltt_points.len());

    for LttPoint { time, count } in ltt_points {
        plot_points.push(PlotPoint {
            x: x_offset + *time as Float,
            y: *count as Float,
        });
    }

    PlotData {
        x_data_type: AxisDataType::Continuous,
        y_data_type: AxisDataType::Discrete,
        x_min,
        x_max,
        y_min,
        y_max,
        plot_points,
    }
}

impl PlotCnv {
    pub(super) fn new(draw_debug: bool) -> Self {
        Self { draw_debug, ..Default::default() }
    }

    pub(super) fn clear_cache_cnv_gts(&self) {
        self.cache_cnv_gts.clear();
    }

    pub(super) fn clear_cache_cnv_ltt(&self) {
        self.cache_cnv_ltt.clear();
    }

    pub(super) fn clear_cache_cnv_cursor_line(&self) {
        self.cache_cnv_cursor_line.clear();
    }

    pub(super) fn clear_cache_cnv_axes(&self) {
        self.cache_cnv_axes.clear();
    }

    pub(super) fn clear_cache_cnv_bnds(&self) {
        self.cache_cnv_bnds.clear();
    }

    pub(super) fn clear_caches_cnv_all(&self) {
        self.clear_cache_cnv_gts();
        self.clear_cache_cnv_ltt();
        self.clear_cache_cnv_axes();
        self.clear_cache_cnv_cursor_line();
        self.clear_cache_cnv_bnds();
    }

    pub(super) fn set_ltt_plot_data(&mut self, data: PlotData) {
        self.clear_cache_cnv_ltt();
        self.plot_data = data;
    }
}

impl Program<TvMsg> for PlotCnv {
    type State = St;

    fn mouse_interaction(
        &self,
        _st: &St,
        _bnds: Rectangle,
        _crsr: Cursor,
    ) -> MouseInteraction {
        MouseInteraction::default()
    }

    fn update(
        &self,
        st: &mut St,
        ev: &Event,
        bnds: Rectangle,
        crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        // ---------------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // ---------------------------------------------------------------------

        let n_ticks_x = 13;
        let n_ticks_y = 19;

        let (ticks_x, x_max_lab_nchar) = calc_ticks(
            n_ticks_x, self.scale_x, self.plot_data.x_data_type,
            self.plot_data.x_min, self.plot_data.x_max,
            self.time_axis_reversed,
        );

        let (ticks_y, y_max_lab_nchar) = calc_ticks(
            n_ticks_y, self.scale_y, self.plot_data.y_data_type,
            self.plot_data.y_min, self.plot_data.y_max, false,
        );

        st.ticks_x = ticks_x;
        st.ticks_y = ticks_y;

        st.ltt_plot_data = self.plot_data.clone();

        if bnds != st.bnds
            || st.plt_padd_l != self.padd_l
            || st.plt_padd_r != self.padd_r
            || st.plt_padd_t != self.padd_t
            || st.plt_padd_b != self.padd_b
        {
            self.clear_cache_cnv_bnds();
            self.clear_cache_cnv_ltt();
            st.bnds = bnds;

            st.text_size = SF * 11.0;
            let char_width = st.text_size * 0.6;
            st.tick_size = char_width;
            st.lab_offset = char_width * 0.5;

            let extra_padding = ZRO;

            st.plt_vs = RectVals::cnv(bnds).padded(
                self.padd_l
                    + st.tick_size
                    + st.lab_offset
                    + char_width * y_max_lab_nchar as Float
                    + extra_padding * ONE,
                self.padd_r
                    + 0.5 * char_width * x_max_lab_nchar as Float
                    + extra_padding * ONE,
                self.padd_t + st.text_size / TWO + extra_padding * ONE,
                self.padd_b
                    + st.tick_size
                    + st.lab_offset
                    + st.text_size
                    + extra_padding * ONE,
            );
            st.plt_rect = st.plt_vs.clone().into();
            // st.plt_padd_l = self.padd_l;
            // st.plt_padd_r = self.padd_r;
            // st.plt_padd_t = self.padd_t;
            // st.plt_padd_b = self.padd_b;
        }

        if let Event::Mouse(mouse_ev) = ev {
            match mouse_ev {
                MouseEvent::CursorEntered => {
                    self.clear_cache_cnv_cursor_line();
                    st.cursor_tracking_point = None;
                }
                MouseEvent::CursorMoved { position: _ } => {
                    self.clear_cache_cnv_cursor_line();
                    action = st.cursor_tracking_point(crsr);
                }
                MouseEvent::CursorLeft => {
                    self.clear_cache_cnv_cursor_line();
                    st.cursor_tracking_point = None;
                }
                _ => {}
            }
        }
        // ---------------------------------------------------------------------
        if let Some(crsr_x_rel) = self.crsr_x_rel {
            st.cursor_tracking_point =
                Some(Point { x: crsr_x_rel * st.plt_vs.w, y: ZRO });
        }
        // ---------------------------------------------------------------------
        action
    }

    fn draw(
        &self,
        st: &St,
        rndr: &Renderer,
        _thm: &Theme,
        bnds: Rectangle,
        _crsr: Cursor,
    ) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();
        // ---------------------------------------------------------------------
        let size = bnds.size();
        if self.draw_debug {
            draw_bounds(self, st, rndr, bnds, &mut geoms);
        }

        draw_gts(self, st, rndr, size, &mut geoms);
        draw_ltt(self, st, rndr, size, &mut geoms);
        draw_axes(self, st, rndr, size, &mut geoms);

        draw_cursor_line(self, st, rndr, size, &mut geoms);
        // ---------------------------------------------------------------------
        geoms
    }
}
