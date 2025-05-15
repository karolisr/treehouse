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

pub use treeview::{SidebarPos, TreeView, TvMsg};

pub(crate) type Float = f32;
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TREE_LAB_FONT_NAME: &str = "JetBrains Mono";

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
    pub cntr_untrans: iced::Point<T>,
    pub cntr_x: T,
    pub cntr_y: T,
    pub cntr: iced::Point<T>,
    pub trans: iced::Vector<T>,
}

impl RectVals<Float> {
    pub fn clip(bounds: iced::Rectangle) -> Self {
        let x = 0e0;
        let y = 0e0;
        let w = bounds.width as Float;
        let h = bounds.height as Float;
        iced::Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn tree(clip: RectVals<Float>, padding: Float) -> Self {
        let x = clip.x + padding;
        let y = clip.y + padding;
        let w = clip.w - padding * 2e0;
        let h = clip.h - padding * 2e0;
        iced::Rectangle { x, y, width: w, height: h }.into()
    }
}

impl From<iced::Rectangle<Float>> for RectVals<Float> {
    fn from(r: iced::Rectangle<Float>) -> Self {
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
        let cntr_untrans = iced::Point { x: cntr_untrans_x, y: cntr_untrans_y };

        let cntr_x = cntr_untrans_x + x;
        let cntr_y = cntr_untrans_y + y;
        let cntr = iced::Point { x: cntr_x, y: cntr_y };

        let trans = iced::Vector { x, y };

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

impl<T> From<RectVals<T>> for iced::Rectangle<T> {
    fn from(v: RectVals<T>) -> Self {
        iced::Rectangle { x: v.x, y: v.y, width: v.w, height: v.h }
    }
}
