use crate::Float;
use iced::{Point, Rectangle, widget::canvas::Cache};

mod draw;
mod drawables;
mod program;

#[derive(Debug, Default)]
pub(crate) struct TreeCnv {
    // pub(crate) brnch_lab_offset_y: Float,
    // pub(crate) brnch_lab_size: Float,
    // pub(crate) center: Point,
    // pub(crate) cursor_x_fraction: Option<Float>,
    // pub(crate) extra_space_for_tip_labs: Float,
    // pub(crate) int_lab_offset_x: Float,
    // pub(crate) int_lab_size: Float,
    // pub(crate) max_lab_size: Float,
    // pub(crate) max_node_size: Float,
    // pub(crate) max_tip_labs_to_draw: usize,
    // pub(crate) min_lab_size: Float,
    // pub(crate) min_node_size: Float,
    // pub(crate) node_radius: Float,
    // pub(crate) node_size: Float,
    // pub(crate) size: Float,
    // pub(crate) tip_lab_offset_x: Float,
    // pub(crate) tip_lab_size: Float,
    // pub(crate) tip_lab_w: Float,
    //
    // pub(crate) g_cursor_line: Cache,
    // pub(crate) g_edge: Cache,
    // pub(crate) g_frame: Cache,
    // pub(crate) g_lab_brnch: Cache,
    // pub(crate) g_lab_int: Cache,
    // pub(crate) g_lab_tip: Cache,
    // pub(crate) g_legend: Cache,
    // pub(crate) g_node_found_iter: Cache,
    // pub(crate) g_node_found: Cache,
    // pub(crate) g_node_hover: Cache,
    // pub(crate) g_node_sel: Cache,
    //
    #[cfg(debug_assertions)]
    pub(crate) g_bounds: Cache,
    // #[cfg(debug_assertions)]
    // pub(crate) g_palette: Cache,
    //
}

impl TreeCnv {}
