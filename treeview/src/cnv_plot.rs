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
    pub(super) crsr_x_rel: Option<Float>,
    pub(super) cache_bnds: CnvCache,
    pub(super) cache_cursor_line: CnvCache,
    pub(super) cache_plot: CnvCache,
    pub(super) padd_l: Float,
    pub(super) padd_r: Float,
    pub(super) padd_t: Float,
    pub(super) padd_b: Float,
    pub(super) vis_x0: Float,
    pub(super) vis_y0: Float,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum AxisScaleType {
    #[default]
    Linear,
    LogTwo,
    LogNat,
    LogTen,
}

pub(super) const AXIS_SCALE_TYPE_OPTS: [AxisScaleType; 4] = [
    AxisScaleType::Linear,
    AxisScaleType::LogTwo,
    AxisScaleType::LogNat,
    AxisScaleType::LogTen,
];

impl Display for AxisScaleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            AxisScaleType::Linear => "Linear",
            AxisScaleType::LogTwo => "Log Base 2",
            AxisScaleType::LogNat => "Natural Log",
            AxisScaleType::LogTen => "Log Base 10",
        })
    }
}

#[derive(Debug, Default, Clone)]
pub enum PlotDataType {
    #[default]
    Continuous,
    Discrete,
}

#[derive(Debug, Default, Clone)]
pub struct PlotPoint {
    x: Float,
    y: Float,
}

#[derive(Debug, Default, Clone)]
pub struct PlotData {
    x_data_type: PlotDataType,
    y_data_type: PlotDataType,
    // x_min: Float,
    // y_min: Float,
    x_max: Float,
    y_max: Float,
    plot_points: Vec<PlotPoint>,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self}")
    }
}

impl From<&Vec<LttPoint>> for PlotData {
    fn from(ltt_points: &Vec<LttPoint>) -> Self {
        // let mut x_min: Float = Float::MAX;
        let mut x_max: Float = Float::MIN;
        // let mut y_min: Float = Float::MAX;
        let mut y_max: Float = Float::MIN;
        for LttPoint { height, count } in ltt_points {
            // x_min = x_min.min(*height as Float);
            x_max = x_max.max(*height as Float);
            // y_min = y_min.min(*count as Float);
            y_max = y_max.max(*count as Float);
        }
        let mut plot_points = Vec::with_capacity(ltt_points.len());
        for LttPoint { height, count } in ltt_points {
            plot_points
                .push(PlotPoint { x: *height as Float, y: *count as Float });
        }
        PlotData {
            x_data_type: PlotDataType::Continuous,
            y_data_type: PlotDataType::Discrete,
            // x_min,
            x_max,
            // y_min,
            y_max,
            plot_points,
        }
    }
}

impl PlotCnv {
    pub(super) fn clear_cache_bnds(&self) {
        self.cache_bnds.clear();
    }

    pub(super) fn clear_cache_cursor_line(&self) {
        self.cache_cursor_line.clear();
    }

    pub(super) fn clear_cache_plot(&self) {
        self.cache_plot.clear();
    }

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
            st.text_size = SF * 10.0;
            let extra_padding = SF * TEN;
            st.plt_vs = RectVals::cnv(bnds).padded(
                self.padd_l + extra_padding * ZRO,
                self.padd_r + extra_padding * ZRO,
                self.padd_t + extra_padding * ONE,
                self.padd_b + extra_padding * TWO,
            );
            st.plt_rect = st.plt_vs.clone().into();
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
            st.cursor_tracking_point =
                Some(Point { x: crsr_x_rel * st.plt_vs.w, y: ZRO });
        }
        // -----------------------------------------------------------------------------------------
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
