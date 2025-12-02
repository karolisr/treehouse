// -------------------------------------
// #![allow(dead_code)]
// #![allow(unused_mut)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(clippy::single_match)]
// #![allow(clippy::collapsible_if)]
// #![allow(clippy::derivable_impls)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::collapsible_match)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::vec_init_then_push)]
// #![allow(clippy::needless_range_loop)]
// -------------------------------------

mod cnv_plot;
mod cnv_tree;
mod cnv_utils;
mod consts;
mod context_menu;
mod edge_utils;
mod path_builders;
mod pdf;
mod tables;
mod treestate;
mod treeview;
mod view;

pub type Float = f32;

pub use context_menu::{TvContextMenuItem, TvContextMenuListing};
pub use riced::{SF, TXT_SIZE};
pub use treeview::{TreeView, TvMsg};

use std::collections::HashSet;
use std::f32 as float;
use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;

use cnv_plot::AXIS_SCALE_TYPE_OPTS;
use cnv_plot::{AxisScaleType, PlotCnv, PlotDataType};
use cnv_tree::TreeCnv;
use consts::*;
use dendros::{Edge, LttPoint, Node, NodeId, Tree, ltt, write_newick};
use rayon::prelude::*;
use riced::*;
use tables::nodes_table;
use treestate::{EdgeSortField, TreeState};
use treeview::{
    TRE_NODE_ORD_OPTS, TRE_STY_OPTS, TreNodeOrd, TreSty, TreeViewPane,
};

#[derive(Debug, Clone, Copy)]
pub enum SortOrd {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub(crate) enum Zone {
    Top,
    TopLeft,
    TopRight,
    Left,
    Right,
    BottomLeft,
    BottomRight,
    Bottom,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum CladeHighlightType {
    Outside,
    Inside,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct CladeHighlight {
    node_id: NodeId,
    color: Color,
    label: String,
    highlight_type: CladeHighlightType,
}

#[derive(Debug, Clone, Default)]
struct Label {
    text: CnvText,
    width: Float,
    angle: Float,
    aligned_from: Option<Point>,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct EdgePoints {
    p0: Point,
    p_mid: Point,
    p1: Point,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct NodeData {
    node_id: NodeId,
    edge_idx: usize,
    points: EdgePoints,
    angle: Float,
    y_parent: Float,
    angle_parent: Float,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct NodeDataCart {
    node_id: NodeId,
    edge_idx: usize,
    points: EdgePoints,
    y_parent: Float,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct NodeDataPol {
    node_id: NodeId,
    edge_idx: usize,
    points: EdgePoints,
    angle: Float,
    angle_parent: Float,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RectVals<T> {
    x0: T,
    y0: T,
    x1: T,
    y1: T,
    w: T,
    h: T,
    dim_min: T,
    dim_max: T,
    radius_min: T,
    radius_max: T,
    cntr_x: T,
    cntr_y: T,
    cntr: Vector<T>,
    trans: Vector<T>,
}

impl RectVals<Float> {
    pub fn cnv(bounds: Rectangle) -> Self {
        let x = ZRO;
        let y = ZRO;
        let w = bounds.width as Float;
        let h = bounds.height as Float;
        Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn wh(w: Float, h: Float) -> Self {
        let x = ZRO;
        let y = ZRO;
        Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn corners(x0: Float, y0: Float, x1: Float, y1: Float) -> Self {
        let x = x0;
        let y = y0;
        let w = x1 - x0;
        let h = y1 - y0;
        Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn padded(
        &self,
        left: Float,
        right: Float,
        top: Float,
        bottom: Float,
    ) -> RectVals<Float> {
        let x = self.x0 + left;
        let y = self.y0 + top;
        let width = self.w - right - left;
        let height = self.h - bottom - top;
        Rectangle { x, y, width, height }.into()
    }

    pub fn transfer_x_from(&self, other: &RectVals<Float>) -> RectVals<Float> {
        let x = other.x0;
        let y = self.y0;
        let width = other.w;
        let height = self.h;
        Rectangle { x, y, width, height }.into()
    }

    pub fn transfer_y_from(&self, other: &RectVals<Float>) -> RectVals<Float> {
        let x = self.x0;
        let y = other.y0;
        let width = self.w;
        let height = other.h;
        Rectangle { x, y, width, height }.into()
    }
}

impl From<Rectangle<Float>> for RectVals<Float> {
    fn from(r: Rectangle<Float>) -> Self {
        let x0 = r.x;
        let y0 = r.y;
        let w = r.width;
        let h = r.height;
        let x1 = x0 + w;
        let y1 = y0 + h;

        let dim_min = w.min(h);
        let dim_max = w.max(h);
        let radius_min = dim_min / TWO;
        let radius_max = dim_min.hypot(dim_max);

        let cntr_untrans_x = w / TWO;
        let cntr_untrans_y = h / TWO;

        let cntr_x = cntr_untrans_x + x0;
        let cntr_y = cntr_untrans_y + y0;
        let cntr = Vector { x: cntr_x, y: cntr_y };

        let trans = Vector { x: x0, y: y0 };

        RectVals {
            x0,
            y0,
            x1,
            y1,
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

impl<T> From<RectVals<T>> for Rectangle<T>
where
    T: Clone,
{
    fn from(v: RectVals<T>) -> Self {
        Rectangle { x: v.x0, y: v.y0, width: v.w, height: v.h }
    }
}

impl Display for RectVals<Float> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({:7.2}, {:7.2}), ({:7.2}, {:7.2})",
            self.x0, self.y0, self.x1, self.y1
        )
    }
}

fn ellipsize_unicode(name: impl Into<String>, width: usize) -> String {
    let tmp = name.into();
    let mut rv = tmp.char_indices().fold(
        String::new(),
        |mut string_accum, (i, character)| {
            if i < width {
                string_accum.push(character);
            }
            string_accum
        },
    );
    if tmp.len() > rv.len() {
        rv.push('\u{2026}'); // ellipsis
    }
    rv
}
