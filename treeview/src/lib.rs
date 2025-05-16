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
mod elements;
mod treestate;
mod treeview;

pub(crate) use cnv_plot::PlotCnv;
pub(crate) use cnv_utils::*;
pub(crate) use treestate::TreeState;
pub(crate) use treeview::{NODE_ORD_OPTS, NodeOrd, TREE_STYLE_OPTS, TreeStyle, TvPane};
pub use treeview::{SidebarPos, TreeView, TvMsg};
pub use utils::{Clr, lerp, text_width};

pub(crate) type Float = f32;
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const SF: f32 = 1e0;
pub(crate) const FNT_NAME: &str = "JetBrains Mono";
pub(crate) const TXT_SIZE: f32 = 13.0 * SF;
pub(crate) const FNT_NAME_LAB: &str = FNT_NAME;
pub(crate) const TXT_SIZE_LAB: Float = TXT_SIZE;

use iced::alignment::Vertical;
use iced::font::{Family, Stretch, Style as FontStyle, Weight};
use iced::widget::canvas::Text as CanvasText;
use iced::widget::canvas::stroke::{LineCap, LineDash, LineJoin, Stroke, Style as StrokeStyle};
use iced::widget::text::{Alignment as TextAlignment, LineHeight, Shaping};
use iced::{Font, Pixels, Point, Rectangle, Vector};

pub(crate) const STRK_TMPL: Stroke = Stroke {
    width: 1e0,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    style: StrokeStyle::Solid(Clr::BLK),
    line_dash: LineDash { segments: &[], offset: 0 },
};

pub(crate) const STRK_EDGE: Stroke = STRK_TMPL;

pub(crate) const STRK1: Stroke =
    Stroke { width: 3e0, style: StrokeStyle::Solid(Clr::RED_50), ..STRK_TMPL };
pub(crate) const STRK2: Stroke =
    Stroke { width: 3e0, style: StrokeStyle::Solid(Clr::GRN_50), ..STRK_TMPL };
pub(crate) const STRK3: Stroke =
    Stroke { width: 3e0, style: StrokeStyle::Solid(Clr::BLU_50), ..STRK_TMPL };

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

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexRange {
    b: usize,
    e: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodePoint {
    point: Point,
    edge: dendros::Edge,
    angle: Option<Float>,
}

#[derive(Debug, Clone, Default)]
pub struct Label {
    text: CanvasText,
    angle: Option<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EdgePoints {
    p0: Point,
    p1: Point,
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
    pub cntr_untrans_x: T,
    pub cntr_untrans_y: T,
    pub cntr_untrans: Point<T>,
    pub cntr_x: T,
    pub cntr_y: T,
    pub cntr: Point<T>,
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
        let radius_max = dim_max / 2e0;

        let cntr_untrans_x = w / 2e0;
        let cntr_untrans_y = h / 2e0;
        let cntr_untrans = Point { x: cntr_untrans_x, y: cntr_untrans_y };

        let cntr_x = cntr_untrans_x + x;
        let cntr_y = cntr_untrans_y + y;
        let cntr = Point { x: cntr_x, y: cntr_y };

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
            cntr_untrans_x,
            cntr_untrans_y,
            cntr_untrans,
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
