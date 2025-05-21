#![feature(iter_collect_into)]
// -------------------------------------
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(unused_variables)]
// #![allow(clippy::collapsible_if)]
// #![allow(clippy::collapsible_match)]
// #![allow(clippy::derivable_impls)]
// #![allow(clippy::needless_range_loop)]
// #![allow(clippy::single_match)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::vec_init_then_push)]
// -------------------------------------

mod cnv_plot;
mod cnv_tree;
mod cnv_utils;
mod consts;
mod edge_utils;
mod elements;
mod treestate;
mod treeview;

pub use treeview::{SidebarPos, TreeView, TvMsg};
pub use utils::{Clr, TextWidth, lerp, text_width};

use dendros::{Edge, Node, NodeId, Tree, TreeFloat, flatten_tree};

use cnv_plot::PlotCnv;
use cnv_utils::*;
use consts::*;
use edge_utils::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use treestate::TreeState;
use treeview::{NODE_ORD_OPTS, NodeOrd, TREE_STYLE_OPTS, TreeStyle, TvPane};

use iced::alignment::Vertical;
use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::mouse::{Cursor, Event as MouseEvent, Interaction};
use iced::widget::canvas::Cache;
use iced::widget::canvas::path::Arc as PathArc;
use iced::widget::canvas::stroke::{LineCap, LineDash, LineJoin};
use iced::widget::canvas::stroke::{Stroke as Strk, Style::Solid};
use iced::widget::canvas::{Action, Frame, Geometry, Path, Program};
#[allow(unused_imports)]
use iced::widget::canvas::{Fill as CanvasFill, Text as CanvasText};
use iced::widget::text::{Alignment as TextAlignment, LineHeight, Shaping};
use iced::{Event, Font, Pixels, Point, Radians, Rectangle, Renderer, Size, Theme, Vector};

pub type Float = f32;
pub type IndexRange = RangeInclusive<usize>;

#[derive(Debug, Clone, Default)]
pub struct Label {
    text: CanvasText,
    width: Float,
    angle: Option<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EdgePoints {
    p0: Point,
    p_mid: Point,
    p1: Point,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeData {
    edge_idx: usize,
    points: EdgePoints,
    angle: Option<Float>,
    y_parent: Option<Float>,
    angle_parent: Option<Float>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeDataPhylogram {
    edge_idx: usize,
    points: EdgePoints,
    y_parent: Option<Float>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeDataRad {
    edge_idx: usize,
    points: EdgePoints,
    angle: Float,
    angle_parent: Option<Float>,
}

impl From<NodeDataPhylogram> for NodeData {
    fn from(nd: NodeDataPhylogram) -> Self {
        Self {
            edge_idx: nd.edge_idx,
            points: nd.points,
            y_parent: nd.y_parent,
            angle: None,
            angle_parent: None,
        }
    }
}

impl From<NodeDataRad> for NodeData {
    fn from(nd: NodeDataRad) -> Self {
        Self {
            edge_idx: nd.edge_idx,
            points: nd.points,
            y_parent: None,
            angle: Some(nd.angle),
            angle_parent: nd.angle_parent,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RectVals<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
    pub dim_min: T,
    pub dim_max: T,
    pub radius_min: T,
    pub radius_max: T,
    pub cntr_x: T,
    pub cntr_y: T,
    pub cntr: Vector<T>,
    pub trans: Vector<T>,
}

impl RectVals<Float> {
    pub fn clip(bounds: Rectangle) -> Self {
        let x = 0e0;
        let y = 0e0;
        let w = bounds.width as Float;
        let h = bounds.height as Float;
        Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn tree(clip: RectVals<Float>, padding: Float) -> Self {
        let x = clip.x + padding;
        let y = clip.y + padding;
        let w = clip.w - padding * 2e0;
        let h = clip.h - padding * 2e0;
        Rectangle { x, y, width: w, height: h }.into()
    }
}

impl From<Rectangle<Float>> for RectVals<Float> {
    fn from(r: Rectangle<Float>) -> Self {
        let x = r.x;
        let y = r.y;
        let w = r.width;
        let h = r.height;

        let dim_min = w.min(h);
        let dim_max = w.max(h);
        let radius_min = dim_min / 2e0;
        let radius_max = dim_min.hypot(dim_max);

        let cntr_untrans_x = w / 2e0;
        let cntr_untrans_y = h / 2e0;

        let cntr_x = cntr_untrans_x + x;
        let cntr_y = cntr_untrans_y + y;
        let cntr = Vector { x: cntr_x, y: cntr_y };

        let trans = Vector { x, y };

        RectVals {
            x,
            y,
            w,
            h,
            dim_min,
            dim_max,
            radius_min,
            radius_max,
            cntr_x,
            cntr_y,
            cntr,
            trans,
        }
    }
}

impl<T> From<RectVals<T>> for Rectangle<T> {
    fn from(v: RectVals<T>) -> Self {
        Rectangle { x: v.x, y: v.y, width: v.w, height: v.h }
    }
}

pub fn tip_idx_range_between_y_vals(
    y0: Float, y1: Float, node_size: Float, tips: &[usize],
) -> Option<IndexRange> {
    if node_size <= 0e0 {
        return None;
    }
    let i0: i64 = (y0 / node_size) as i64;
    let i1: i64 = (y1 / node_size) as i64;
    if i0 < 0 && i1 < 0 {
        return None;
    }
    let idx_tip_0: usize = i0.max(0) as usize;
    let idx_tip_1: usize = i1.min(tips.len() as i64 - 1) as usize;
    if idx_tip_0 < idx_tip_1 { Some(IndexRange::new(idx_tip_0, idx_tip_1)) } else { None }
}

pub fn node_idx_range_for_tip_idx_range(tip_idx_range: &IndexRange, tips: &[usize]) -> IndexRange {
    let idx_tip_0 = tips[*tip_idx_range.start()];
    let idx_tip_1 = tips[*tip_idx_range.end()];
    IndexRange::new(idx_tip_0, idx_tip_1)
}
