// -------------------------------------
// #![allow(clippy::collapsible_if)]
// #![allow(clippy::collapsible_match)]
// #![allow(clippy::derivable_impls)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::vec_init_then_push)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(unused_variables)]
// -------------------------------------

mod treeview;

use dendros::Edge;
use iced::{Point, widget::canvas::Text};
pub use treeview::{TreeView, TreeViewMsg};

pub(crate) type Float = f32;
pub(crate) const PI: Float = std::f32::consts::PI;

pub const SF: f32 = 1e0;
pub const TEXT_SIZE: f32 = 14.0 * SF;

pub(crate) const TREE_LAB_FONT_NAME: &str = "JetBrains Mono";
pub(crate) const BORDER_W: f32 = SF;
pub(crate) const RADIUS_WIDGET: f32 = 0e0 * SF;
pub(crate) const PADDING: f32 = 1e1 * SF;
pub(crate) const SIDEBAR_W: f32 = 2e2 + PADDING * 2e0;
pub(crate) const STATUSBAR_H: f32 = 28.0 * SF;

// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct IndexRange {
    pub(crate) b: usize,
    pub(crate) e: usize,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ChunkEdgeRange {
    pub(crate) chnk: IndexRange,
    pub(crate) edge: IndexRange,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NodePoint {
    pub(crate) point: Point,
    pub(crate) edge: Edge,
    pub(crate) angle: Option<Float>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NodePoints {
    pub(crate) points: Vec<NodePoint>,
    pub(crate) center: Point,
    pub(crate) size: Float,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Label {
    pub(crate) text: Text,
    pub(crate) angle: Option<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct EdgePoints {
    pub(crate) pt_0: Point,
    pub(crate) pt_1: Point,
}
