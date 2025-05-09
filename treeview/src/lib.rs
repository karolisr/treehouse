// -------------------------------------
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::single_match)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::vec_init_then_push)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(unused_variables)]
// -------------------------------------

mod treeview;
pub(crate) mod utils;

pub(crate) use treeview::{
    NODE_ORD_OPTS, NodeOrd, PlotCnv, TREE_STYLE_OPTS, TreeCnv, TreeState, TreeStateMsg, TreeStyle,
};
pub use treeview::{SidebarLocation, TreeView, TreeViewMsg};

// ------------------------------------------------------------------------------------------------
pub(crate) type Float = f32;
// ------------------------------------------------------------------------------------------------
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TREE_LAB_FONT_NAME: &str = "JetBrains Mono";
pub(crate) const SF: f32 = 1e0;
pub(crate) const TEXT_SIZE: f32 = 13.0 * SF;
pub(crate) const BORDER_W: f32 = SF;
pub(crate) const RADIUS_WIDGET: f32 = 0e0 * SF;
pub(crate) const PADDING: f32 = 1e1 * SF;
// pub(crate) const SIDEBAR_W: f32 = 2e2 + PADDING * 2e0;
// pub(crate) const STATUSBAR_H: f32 = 28.0 * SF;
// ------------------------------------------------------------------------------------------------
