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
mod edge_utils;
mod elements;
mod treestate;
mod treeview;

use std::ops::RangeInclusive;

pub(crate) use cnv_plot::PlotCnv;
pub(crate) use cnv_utils::*;
pub(crate) use dendros::{Edge, Node, NodeId, Tree, TreeFloat, flatten_tree};
pub(crate) use edge_utils::*;
pub(crate) use treestate::TreeState;
pub(crate) use treeview::{NODE_ORD_OPTS, NodeOrd, TREE_STYLE_OPTS, TreeStyle, TvPane};
pub use treeview::{SidebarPos, TreeView, TvMsg};
pub use utils::{Clr, lerp, text_width};

pub(crate) type Float = f32;
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TAU: Float = std::f32::consts::TAU;
pub(crate) const FRAC_PI_2: Float = std::f32::consts::FRAC_PI_2;
pub(crate) const SF: Float = 1e0;
pub(crate) const FNT_NAME: &str = "JetBrains Mono";
pub(crate) const TXT_SIZE: Float = 13.0 * SF;
pub(crate) const FNT_NAME_LAB: &str = FNT_NAME;
pub(crate) const TXT_SIZE_LAB: Float = TXT_SIZE;

use iced::alignment::Vertical;
use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::widget::canvas::Text as CanvasText;
use iced::widget::canvas::stroke::{LineCap, LineDash, LineJoin};
use iced::widget::canvas::stroke::{Stroke as Strk, Style::Solid};
use iced::widget::text::{Alignment as TextAlignment, LineHeight, Shaping};
use iced::{Font, Pixels, Point, Rectangle, Vector};

pub type IndexRange = RangeInclusive<usize>;

#[derive(Debug, Clone, Default)]
pub struct Label {
    text: CanvasText,
    angle: Option<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EdgePoints {
    p0: Point,
    p_mid: Point,
    p1: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeData<'a> {
    Phylogram(&'a [NodeDataPhylogram]),
    Rad(&'a [NodeDataRad]),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeDataPhylogram {
    edge: Edge,
    points: EdgePoints,
    y_parent: Option<Float>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeDataRad {
    edge: Edge,
    points: EdgePoints,
    angle: Float,
    angle_parent: Option<Float>,
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
    y0: Float, y1: Float, node_size: Float, tips: &[Edge],
) -> Option<IndexRange> {
    let i0: i64 = (y0 / node_size) as i64;
    let i1: i64 = (y1 / node_size) as i64;
    let idx_tip_0: usize = i0.max(0) as usize;
    let idx_tip_1: usize = i1.min(tips.len() as i64 - 1) as usize;
    if idx_tip_0 < idx_tip_1 { Some(IndexRange::new(idx_tip_0, idx_tip_1)) } else { None }
}

pub fn node_idx_range_for_tip_idx_range(tip_idx_range: &IndexRange, tips: &[Edge]) -> IndexRange {
    let idx_tip_0 = &tips[*tip_idx_range.start()];
    let idx_tip_1 = &tips[*tip_idx_range.end()];
    let idx_node_0 = idx_tip_0.edge_idx;
    let idx_node_1 = idx_tip_1.edge_idx;
    IndexRange::new(idx_node_0, idx_node_1)
}

pub(crate) const STRK_TMPL: Strk = Strk {
    width: 1e0,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    style: Solid(Clr::BLK),
    line_dash: LineDash { segments: &[], offset: 0 },
};

pub(crate) const STRK_1: Strk = Strk { width: 1e0, ..STRK_TMPL };
pub(crate) const STRK_2: Strk = Strk { width: 2e0, ..STRK_TMPL };
pub(crate) const STRK_3: Strk = Strk { width: 3e0, ..STRK_TMPL };
pub(crate) const STRK_4: Strk = Strk { width: 4e0, ..STRK_TMPL };
pub(crate) const STRK_5: Strk = Strk { width: 5e0, ..STRK_TMPL };

pub(crate) const STRK_EDGE: Strk = STRK_1;

pub(crate) const STRK_1_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_1 };
pub(crate) const STRK_1_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_1 };
pub(crate) const STRK_1_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_1 };
pub(crate) const STRK_1_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_1 };
pub(crate) const STRK_1_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_1 };
pub(crate) const STRK_1_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_1 };
pub(crate) const STRK_1_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_1 };

pub(crate) const STRK_1_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_1 };
pub(crate) const STRK_1_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_1 };
pub(crate) const STRK_1_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_1 };
pub(crate) const STRK_1_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_1 };
pub(crate) const STRK_1_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_1 };
pub(crate) const STRK_1_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_1 };
pub(crate) const STRK_1_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_1 };

pub(crate) const STRK_2_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_2 };
pub(crate) const STRK_2_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_2 };
pub(crate) const STRK_2_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_2 };
pub(crate) const STRK_2_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_2 };
pub(crate) const STRK_2_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_2 };
pub(crate) const STRK_2_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_2 };
pub(crate) const STRK_2_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_2 };

pub(crate) const STRK_2_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_2 };
pub(crate) const STRK_2_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_2 };
pub(crate) const STRK_2_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_2 };
pub(crate) const STRK_2_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_2 };
pub(crate) const STRK_2_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_2 };
pub(crate) const STRK_2_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_2 };
pub(crate) const STRK_2_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_2 };

pub(crate) const STRK_3_BLK_25: Strk = Strk { style: Solid(Clr::BLK_25), ..STRK_3 };
pub(crate) const STRK_3_RED_25: Strk = Strk { style: Solid(Clr::RED_25), ..STRK_3 };
pub(crate) const STRK_3_GRN_25: Strk = Strk { style: Solid(Clr::GRN_25), ..STRK_3 };
pub(crate) const STRK_3_BLU_25: Strk = Strk { style: Solid(Clr::BLU_25), ..STRK_3 };
pub(crate) const STRK_3_YEL_25: Strk = Strk { style: Solid(Clr::YEL_25), ..STRK_3 };
pub(crate) const STRK_3_CYA_25: Strk = Strk { style: Solid(Clr::CYA_25), ..STRK_3 };
pub(crate) const STRK_3_MAG_25: Strk = Strk { style: Solid(Clr::MAG_25), ..STRK_3 };

pub(crate) const STRK_3_BLK_50: Strk = Strk { style: Solid(Clr::BLK_50), ..STRK_3 };
pub(crate) const STRK_3_RED_50: Strk = Strk { style: Solid(Clr::RED_50), ..STRK_3 };
pub(crate) const STRK_3_GRN_50: Strk = Strk { style: Solid(Clr::GRN_50), ..STRK_3 };
pub(crate) const STRK_3_BLU_50: Strk = Strk { style: Solid(Clr::BLU_50), ..STRK_3 };
pub(crate) const STRK_3_YEL_50: Strk = Strk { style: Solid(Clr::YEL_50), ..STRK_3 };
pub(crate) const STRK_3_CYA_50: Strk = Strk { style: Solid(Clr::CYA_50), ..STRK_3 };
pub(crate) const STRK_3_MAG_50: Strk = Strk { style: Solid(Clr::MAG_50), ..STRK_3 };

pub(crate) const TXT_LAB_TMPL: CanvasText = CanvasText {
    color: Clr::BLK,
    size: Pixels(TXT_SIZE_LAB),
    line_height: LineHeight::Absolute(Pixels(TXT_SIZE_LAB)),
    align_x: TextAlignment::Left,
    align_y: Vertical::Center,
    content: String::new(),
    max_width: Float::INFINITY,
    position: Point::ORIGIN,
    shaping: Shaping::Basic,
    font: Font {
        family: Family::Name(FNT_NAME_LAB),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: FontStyle::Normal,
    },
};

// pub(crate) const TXT_LAB_TMPL: CanvasText = CanvasText {

// };
