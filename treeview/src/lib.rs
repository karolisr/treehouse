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
mod gts;
mod path_builders;
mod pdf;
mod rect_vals;
mod tables;
mod treestate;
mod treeview;
mod treeview_config;
mod view;

pub type Float = f32;
pub type Integer = i32;

pub use context_menu::{TvContextMenuItem, TvContextMenuSpecification};
pub use rect_vals::RectVals;
pub use riced::{SF, TXT_SIZE};
pub use treeview::{TreUnit, TreeView, TvMsg};
pub use treeview_config::TreeViewConfig;

use std::collections::HashSet;
use std::f32 as float;
use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;

use cnv_plot::AXIS_SCALE_TYPE_OPTS;
use cnv_plot::AxisScaleType;
use cnv_plot::PlotCnv;
use cnv_plot::PlotData;
use cnv_plot::Tick;
use cnv_plot::plot_data_from_ltt_points;
use cnv_plot::transformed_relative_value;

use cnv_tree::TreeCnv;
use consts::*;
use dendros::{
    Attribute, AttributeSelector, AttributeValue, Edge, LttPoint, Node, NodeId,
    Tree, ltt, write_newick,
};
use gts::*;
use rayon::prelude::*;
use riced::*;
use tables::{
    AttributesTableField, NodesTableField, attributes_table, nodes_table,
};
use treestate::TreeState;
use treeview::{
    TRE_NODE_ORD_OPTS, TRE_STY_OPTS, TRE_UNIT_OPTS, TreNodeOrd, TreSty,
    TreeViewPane,
};

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

fn angle_from_idx(idx: u16) -> Float {
    (idx as Float).to_radians()
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

fn transform_value(value: Float, scale: AxisScaleType) -> Float {
    match scale {
        AxisScaleType::Linear => value,
        AxisScaleType::LogTwo => value.log2(),
        AxisScaleType::LogTen => value.log10(),
    }
}

fn normalize_scale_value(value: Float, scale: AxisScaleType) -> Float {
    if value <= 0e0 {
        return 0e0;
    }

    let base: Float = match scale {
        AxisScaleType::Linear => 1e1,
        AxisScaleType::LogTwo => 2e0,
        AxisScaleType::LogTen => 1e1,
    };

    let magnitude = if scale == AxisScaleType::Linear {
        transform_value(value, AxisScaleType::LogTen).floor()
    } else {
        transform_value(value, scale).floor()
    };

    let norm_factor = base.powf(magnitude);
    let mut normalized = value / norm_factor;

    if normalized > 1e0 {
        if normalized < 2e0 {
            normalized = 1e0;
        } else if normalized < 4e0 {
            normalized = 2e0;
        } else if normalized < 5e0 {
            normalized = 5e0;
        } else if normalized < 1e1 {
            normalized = 1e1;
        }
    }

    normalized.ceil() * norm_factor
}
