mod draw;
mod state;

use crate::cnv_utils::*;
use crate::*;
use draw::*;
use state::St;

#[derive(Debug, Default)]
pub(super) struct PlotCnv {
    ltt_plot_data: PlotData,
    pub(super) tre_unit: TreUnit,
    pub(super) x_axis_is_reversed: bool,
    pub(super) x_axis_scale_type: AxisScaleType,
    pub(super) y_axis_scale_type: AxisScaleType,
    pub(super) draw_debug: bool,
    pub(super) draw_ltt: bool,
    pub(super) draw_gts: bool,
    pub(super) draw_cursor_line: bool,
    pub(super) crsr_x_rel: Option<Float>,
    pub(super) padd_l: Float,
    pub(super) padd_r: Float,
    pub(super) padd_t: Float,
    pub(super) padd_b: Float,
    pub(super) vis_x0: Float,
    pub(super) vis_y0: Float,
    pub(super) cache_cnv_gts: CnvCache,
    pub(super) cache_cnv_ltt: CnvCache,
    pub(super) cache_cnv_cursor_line: CnvCache,
    pub(super) cache_cnv_axes: CnvCache,
    pub(super) cache_cnv_ticks: CnvCache,
    pub(super) cache_cnv_bnds: CnvCache,
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
        let mut action: Option<Action<TvMsg>> = None;

        if st.tre_unit != self.tre_unit
            || st.ltt_plot_data.x_min != self.ltt_plot_data.x_min
            || st.ltt_plot_data.y_min != self.ltt_plot_data.y_min
            || st.ltt_plot_data.x_max != self.ltt_plot_data.x_max
            || st.ltt_plot_data.y_max != self.ltt_plot_data.y_max
            || st.x_axis_scale_type != self.x_axis_scale_type
            || st.y_axis_scale_type != self.y_axis_scale_type
            || st.bnds.width != bnds.width
            || st.bnds.height != bnds.height
            || st.plt_padd_l != self.padd_l
            || st.plt_padd_r != self.padd_r
            || st.plt_padd_t != self.padd_t
            || st.plt_padd_b != self.padd_b
        {
            st.ltt_plot_data = self.ltt_plot_data.clone();
            st.x_axis_scale_type = self.x_axis_scale_type;
            st.y_axis_scale_type = self.y_axis_scale_type;
            st.tre_unit = self.tre_unit;
            st.bnds = bnds;
            st.plt_padd_l = self.padd_l;
            st.plt_padd_r = self.padd_r;
            st.plt_padd_t = self.padd_t;
            st.plt_padd_b = self.padd_b;

            st.text_size = SF * 1e1;
            st.char_width = st.text_size * 6e-1;
            st.tick_size = st.char_width;
            st.lab_offset = st.char_width / TWO;
            st.axes_padd = ZRO;

            let padd_t =
                self.padd_t + (st.text_size / TWO).max(st.axes_padd + SF);

            let padd_b = self.padd_b
                + st.axes_padd
                + st.tick_size
                + st.lab_offset
                + st.text_size;

            let n_ticks_x = ((st.bnds.width - padd_t - padd_b)
                / (st.char_width * 20.0))
                .floor() as usize;

            let n_ticks_y = ((st.bnds.height - padd_t - padd_b)
                / (st.text_size * 3.0))
                .floor() as usize;

            let (ticks_x, x_max_lab_nchar) = calc_ticks(
                n_ticks_x, self.x_axis_scale_type,
                self.ltt_plot_data.x_data_type, self.ltt_plot_data.x_min,
                self.ltt_plot_data.x_max, self.x_axis_is_reversed,
            );

            let (ticks_y, y_max_lab_nchar) = calc_ticks(
                n_ticks_y, self.y_axis_scale_type,
                self.ltt_plot_data.y_data_type, self.ltt_plot_data.y_min,
                self.ltt_plot_data.y_max, false,
            );

            st.ticks_x = ticks_x;
            st.ticks_y = ticks_y;

            let padd_l = self.padd_l
                + st.axes_padd
                + st.tick_size
                + st.lab_offset
                + st.char_width * y_max_lab_nchar as Float;

            let padd_r = self.padd_r
                + (st.char_width / TWO * x_max_lab_nchar as Float)
                    .max(st.axes_padd + SF);

            st.plt_vs =
                RectVals::cnv(bnds).padded(padd_l, padd_r, padd_t, padd_b);

            st.plt_rect = st.plt_vs.clone().into();
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

        if let Some(crsr_x_rel) = self.crsr_x_rel {
            st.cursor_tracking_point =
                Some(Point { x: crsr_x_rel * st.plt_vs.w, y: ZRO });
        }

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
        if self.draw_debug {
            draw_bounds(self, st, rndr, bnds, &mut geoms);
        }

        if self.draw_gts && self.tre_unit == TreUnit::MillionYears {
            draw_gts(self, st, rndr, bnds.size(), &mut geoms);
        }

        if self.draw_ltt {
            draw_ltt(self, st, rndr, bnds.size(), &mut geoms);
        }

        if self.draw_cursor_line {
            draw_cursor_line(self, st, rndr, bnds.size(), &mut geoms);
        }

        draw_ticks(self, st, rndr, bnds.size(), &mut geoms);
        draw_axes(self, st, rndr, bnds.size(), &mut geoms);
        // ---------------------------------------------------------------------
        geoms
    }
}

pub fn transformed_relative_value(
    value: Float,
    min_value: Float,
    max_value: Float,
    axis_scale_type: AxisScaleType,
) -> Option<Float> {
    let value_after_offset = value - min_value;
    let max_value_after_offset = max_value - min_value;

    if value_after_offset < 0e0 {
        None
    } else {
        Some(
            transform_value(value_after_offset, axis_scale_type)
                / transform_value(max_value_after_offset, axis_scale_type),
        )
    }
}

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
    pub(super) fn new(
        draw_debug: bool,
        draw_ltt: bool,
        draw_gts: bool,
        tre_unit: TreUnit,
    ) -> Self {
        Self { draw_debug, draw_ltt, draw_gts, tre_unit, ..Default::default() }
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

    pub(super) fn clear_cache_cnv_ticks(&self) {
        self.cache_cnv_ticks.clear();
    }

    pub(super) fn clear_cache_cnv_bnds(&self) {
        self.cache_cnv_bnds.clear();
    }

    pub(super) fn clear_caches_cnv_all(&self) {
        self.clear_cache_cnv_gts();
        self.clear_cache_cnv_ltt();
        self.clear_cache_cnv_cursor_line();
        self.clear_cache_cnv_axes();
        self.clear_cache_cnv_ticks();
        self.clear_cache_cnv_bnds();
    }

    pub(super) fn set_ltt_plot_data(&mut self, data: PlotData) {
        self.ltt_plot_data = data;
        self.clear_caches_cnv_all();
        // self.is_stale = true;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AxisScaleType {
    #[default]
    Linear,
    LogTwo,
    LogTen,
}

impl Display for AxisScaleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            AxisScaleType::Linear => "Linear",
            AxisScaleType::LogTwo => "Log Base 2",
            AxisScaleType::LogTen => "Log Base 10",
        })
    }
}

pub(super) const AXIS_SCALE_TYPE_OPTS: [AxisScaleType; 3] =
    [AxisScaleType::Linear, AxisScaleType::LogTwo, AxisScaleType::LogTen];

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AxisDataType {
    #[default]
    Continuous,
    Discrete,
}

#[derive(Debug, Default, Clone)]
pub struct PlotPoint {
    pub x: Float,
    pub y: Float,
}

#[derive(Debug, Default, Clone)]
pub struct PlotData {
    pub x_data_type: AxisDataType,
    pub y_data_type: AxisDataType,
    pub x_min: Float,
    pub x_max: Float,
    pub y_min: Float,
    pub y_max: Float,
    pub plot_points: Vec<PlotPoint>,
}

pub struct Tick {
    pub relative_position: Float,
    pub label: String,
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
