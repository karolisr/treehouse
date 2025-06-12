#![feature(iter_collect_into)]
#![feature(const_float_round_methods)]
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
mod consts;
mod edge_utils;
mod elements;
mod iced;
mod icons;
mod path_utils;
mod style;
mod treestate;
mod treeview;
mod view;

pub type Float = f32;

pub use consts::{SF, TXT_SIZE};
pub use treeview::{SidebarPosition, TreeView, TvMsg};

use std::collections::HashSet;
use std::f32 as float;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::RangeInclusive;
use std::rc::Rc;

use cnv_plot::{AxisScaleType, PlotCnv, PlotDataType};
use cnv_tree::TreeCnv;
use consts::*;
use dendros::{Edge, LttPoint, Node, NodeId, Tree, TreeFloat, flatten_tree, ltt, write_newick};
use icons::Icon;
use num_traits::FromPrimitive;
use path_utils::PathBuilder;
use rayon::prelude::*;
use treestate::TreeState;
use treeview::{NODE_ORD_OPTS, NodeOrd, TRE_STY_OPTS, TreSty, TvPane};
use utils::{Clr, TextWidth, text_width};

pub type IndexRange = RangeInclusive<usize>;

fn lab_text(txt: String, pt: iced::Point, size: Float, template: iced::CnvText) -> iced::CnvText {
    let mut text = template.clone();
    text.content = txt;
    text.position = pt;
    text.size = size.into();
    text
}

fn draw_labels(
    labels: &[Label], offset: iced::Vector, trans: Option<iced::Vector>, rot: Float,
    f: &mut iced::Frame,
) {
    f.push_transform();
    if let Some(trans) = trans {
        f.translate(trans);
    }
    f.rotate(rot);
    f.translate(offset);

    for Label { text, width, angle } in labels {
        let mut text = text.clone();
        // -----------------------------------------------------------------------------------------
        let magic = text.size.0 / 8.75;
        let adjust_h_base = text.size.0 / TWO - magic;
        let mut adjust_h = match text.align_y {
            iced::Vertical::Top => adjust_h_base.floor() + TWO - magic / 3.15,
            iced::Vertical::Center => ZERO,
            iced::Vertical::Bottom => -adjust_h_base.ceil(),
        };
        text.align_y = iced::Vertical::Center;
        // -----------------------------------------------------------------------------------------
        if let Some(angle) = angle {
            let mut angle = *angle;
            let mut adjust_w = match text.align_x {
                iced::TextAlignment::Left => offset.x,
                iced::TextAlignment::Right => -offset.x,
                _ => ZERO,
            };
            adjust_h += offset.y;
            // = Rotate labels on the left side of the circle by 180 degrees =======================
            let a = (angle + rot) % TAU;
            if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
                angle += PI;
                adjust_w = match text.align_x {
                    iced::TextAlignment::Left => -width - offset.x,
                    iced::TextAlignment::Right => width + offset.x,
                    _ => ZERO,
                };
            } // ===================================================================================
            f.push_transform();
            let (sin, cos) = angle.sin_cos();
            f.translate(iced::Vector {
                x: text.position.x - offset.x + cos * adjust_w - sin * adjust_h,
                y: text.position.y - offset.y + sin * adjust_w + cos * adjust_h,
            });
            text.position = ORIGIN;
            f.rotate(angle);
            f.fill_text(text);
            f.pop_transform();
        } else {
            f.push_transform();
            f.translate(iced::Vector { x: ZERO, y: adjust_h });
            f.fill_text(text);
            f.pop_transform();
        }
    }
    f.pop_transform();
}

#[derive(Debug, Clone, Default)]
struct Label {
    text: iced::CnvText,
    width: Float,
    angle: Option<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct EdgePoints {
    p0: iced::Point,
    p_mid: iced::Point,
    p1: iced::Point,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct NodeData {
    edge_idx: usize,
    points: EdgePoints,
    angle: Option<Float>,
    y_parent: Option<Float>,
    angle_parent: Option<Float>,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct NodeDataCart {
    edge_idx: usize,
    points: EdgePoints,
    y_parent: Option<Float>,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct NodeDataPol {
    edge_idx: usize,
    points: EdgePoints,
    angle: Float,
    angle_parent: Option<Float>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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
    cntr: iced::Vector<T>,
    trans: iced::Vector<T>,
}

impl RectVals<Float> {
    pub fn cnv(bounds: iced::Rectangle) -> Self {
        let x = ZERO;
        let y = ZERO;
        let w = bounds.width as Float;
        let h = bounds.height as Float;
        iced::Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn wh(w: Float, h: Float) -> Self {
        let x = ZERO;
        let y = ZERO;
        iced::Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn corners(x0: Float, y0: Float, x1: Float, y1: Float) -> Self {
        let x = x0;
        let y = y0;
        let w = x1 - x0;
        let h = y1 - y0;
        iced::Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn padded(&self, left: Float, right: Float, top: Float, bottom: Float) -> RectVals<Float> {
        let x = self.x0 + left;
        let y = self.y0 + top;
        let width = self.w - right - left;
        let height = self.h - bottom - top;
        iced::Rectangle { x, y, width, height }.into()
    }

    pub fn transfer_x_from(&self, other: &RectVals<Float>) -> RectVals<Float> {
        let x = other.x0;
        let y = self.y0;
        let width = other.w;
        let height = self.h;
        iced::Rectangle { x, y, width, height }.into()
    }

    pub fn transfer_y_from(&self, other: &RectVals<Float>) -> RectVals<Float> {
        let x = self.x0;
        let y = other.y0;
        let width = self.w;
        let height = other.h;
        iced::Rectangle { x, y, width, height }.into()
    }
}

impl From<iced::Rectangle<Float>> for RectVals<Float> {
    fn from(r: iced::Rectangle<Float>) -> Self {
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
        let cntr = iced::Vector { x: cntr_x, y: cntr_y };

        let trans = iced::Vector { x: x0, y: y0 };

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

impl<T> From<RectVals<T>> for iced::Rectangle<T> {
    fn from(v: RectVals<T>) -> Self {
        iced::Rectangle { x: v.x0, y: v.y0, width: v.w, height: v.h }
    }
}

impl Display for RectVals<Float> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:7.2}, {:7.2}), ({:7.2}, {:7.2})", self.x0, self.y0, self.x1, self.y1)
    }
}
