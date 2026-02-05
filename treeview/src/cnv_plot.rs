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
    pub(super) draw_ltt: bool,
    pub(super) draw_gts: bool,
    pub(super) tre_unit: TreUnit,
    pub(super) time_axis_reversed: bool,
    pub(super) crsr_x_rel: Option<Float>,
    pub(super) cache_cnv_gts: CnvCache,
    pub(super) cache_cnv_ltt: CnvCache,
    pub(super) cache_cnv_cursor_line: CnvCache,
    pub(super) cache_cnv_axes: CnvCache,
    pub(super) cache_cnv_ticks: CnvCache,
    pub(super) cache_cnv_bnds: CnvCache,
    pub(super) padd_l: Float,
    pub(super) padd_r: Float,
    pub(super) padd_t: Float,
    pub(super) padd_b: Float,
    pub(super) vis_x0: Float,
    pub(super) vis_y0: Float,
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AxisDataType {
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
    x_data_type: AxisDataType,
    y_data_type: AxisDataType,
    pub(crate) x_min: Float,
    pub(crate) x_max: Float,
    pub(crate) y_min: Float,
    pub(crate) y_max: Float,
    pub(crate) plot_points: Vec<PlotPoint>,
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

pub fn transformed_relative_value(
    value: Float,
    min_value: Float,
    max_value: Float,
    axis_scale_type: AxisScaleType,
) -> Float {
    let value_after_offset = value - min_value;
    let max_value_after_offset = max_value - min_value;

    if value_after_offset <= 0e0 {
        0e0
    } else {
        transform_value(value_after_offset, axis_scale_type)
            / transform_value(max_value_after_offset, axis_scale_type)
    }
}

pub fn calc_ticks(
    tick_count: usize,
    scale_type: AxisScaleType,
    data_type: AxisDataType,
    min: Float,
    max: Float,
    axis_reversed: bool,
) -> (Vec<Tick>, usize) {
    let mut ticks: Vec<Tick> = Vec::with_capacity(tick_count);

    let (min, max) = match axis_reversed {
        true => (0e0, max - min),
        false => (min, max),
    };

    let range = normalize_scale_value(max - min, scale_type);
    let mut linear_delta =
        normalize_scale_value(range / tick_count as Float, scale_type);
    let offset = normalize_scale_value(min, scale_type);

    if data_type == AxisDataType::Discrete && linear_delta < 1e0 {
        linear_delta = 1e0;
    }

    let mut decimals: usize = 0;
    let mut max_lab_nchar: usize = 0;
    if scale_type == AxisScaleType::Linear {
        let ldfrac = linear_delta.fract();
        if ldfrac > 0e0 {
            decimals = format!("{ldfrac}").len() - 2;
        }
    }

    for i in 1..=tick_count {
        let tick_value = offset
            + match scale_type {
                AxisScaleType::Linear => linear_delta * i as Float,
                AxisScaleType::LogTwo => (2e0 as Float).powi(i as Integer),
                AxisScaleType::LogTen => (1e1 as Float).powi(i as Integer),
            };

        let mut relative_position =
            transformed_relative_value(tick_value, min, max, scale_type);

        relative_position = match axis_reversed {
            true => 1.0 - relative_position,
            false => relative_position,
        };

        if !(0e0..=1e0).contains(&relative_position) {
            continue;
        }

        let nchar = 1 + tick_value.log10().floor() as usize + decimals;
        max_lab_nchar = max_lab_nchar.max(nchar);

        let tick = Tick {
            relative_position,
            label: format!(
                "{:nchar$.decimals$}",
                tick_value,
                nchar = nchar,
                decimals = decimals
            ),
        };

        ticks.push(tick);
    }

    (ticks, max_lab_nchar)
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
        self.clear_cache_cnv_axes();
        self.clear_cache_cnv_ticks();
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

        let n_ticks_x = 10;
        let n_ticks_y = 20;

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
            || st.x_max_lab_nchar != x_max_lab_nchar
            || st.y_max_lab_nchar != y_max_lab_nchar
        {
            self.clear_cache_cnv_bnds();

            st.bnds = bnds;
            st.text_size = SF * 9.0;
            st.char_width = st.text_size * 0.6;
            st.tick_size = st.char_width;
            st.lab_offset = st.char_width / TWO;
            st.axes_padd = ZRO;

            st.plt_vs = RectVals::cnv(bnds).padded(
                self.padd_l
                    + st.axes_padd
                    + st.tick_size
                    + st.lab_offset
                    + st.char_width * y_max_lab_nchar as Float,
                self.padd_r
                    + (st.char_width / TWO * x_max_lab_nchar as Float)
                        .max(st.axes_padd + SF),
                self.padd_t + (st.text_size / TWO).max(st.axes_padd + SF),
                self.padd_b
                    + st.axes_padd
                    + st.tick_size
                    + st.lab_offset
                    + st.text_size,
            );

            st.plt_rect = st.plt_vs.clone().into();

            st.plt_padd_l = self.padd_l;
            st.plt_padd_r = self.padd_r;
            st.plt_padd_t = self.padd_t;
            st.plt_padd_b = self.padd_b;

            st.x_max_lab_nchar = x_max_lab_nchar;
            st.y_max_lab_nchar = y_max_lab_nchar;
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
        let size = bnds.size();
        if self.draw_debug {
            draw_bounds(self, st, rndr, bnds, &mut geoms);
        }

        if self.draw_gts && self.tre_unit == TreUnit::My {
            draw_gts(self, st, rndr, size, &mut geoms);
        }

        if self.draw_ltt {
            draw_ltt(self, st, rndr, size, &mut geoms);
        }

        draw_axes(self, st, rndr, size, &mut geoms);
        draw_ticks(self, st, rndr, size, &mut geoms);

        draw_cursor_line(self, st, rndr, size, &mut geoms);
        // ---------------------------------------------------------------------
        geoms
    }
}
