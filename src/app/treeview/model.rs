use super::{Ltt, NodeOrderingOption, TreeStyleOption};
use crate::{
    Edge, Edges, Float, NodeId, Tree,
    app::{SCROLL_TOOL_W, SF, SIDE_COL_W, windows::window_settings},
};
use iced::{
    widget::{canvas::Cache, scrollable::Viewport as ScrollableViewport},
    window::Id as WinId,
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct TreeView {
    pub win_id: Option<WinId>,
    pub threads: usize,

    pub sel_node_ord_opt: NodeOrderingOption,
    pub sel_tree_style_opt: TreeStyleOption,

    pub ltt: Ltt,
    pub show_ltt: bool,
    pub drawing_enabled: bool,
    pub has_brlen: bool,
    pub has_int_labs: bool,
    pub has_tip_labs: bool,
    pub is_rooted: bool,
    pub is_ultrametric: Option<bool>,
    pub node_count: usize,
    pub tip_count: usize,
    pub int_node_count: usize,
    pub tree_height: Float,

    pub window_w: Float,
    pub window_h: Float,

    pub tree_scroll_w: Float,
    pub tree_scroll_h: Float,
    pub side_with_padding_w: Float,

    pub opn_angle: Float,
    pub sel_opn_angle_idx: u16,
    pub min_opn_angle_idx: u16,
    pub max_opn_angle_idx: u16,

    pub rot_angle: Float,
    pub sel_rot_angle_idx: u16,
    pub min_rot_angle_idx: u16,
    pub max_rot_angle_idx: u16,

    pub ltt_cnv_w: Float,
    pub tre_cnv_w: Float,
    pub min_tre_cnv_w: Float,
    pub min_tre_cnv_w_idx: u16,
    pub max_tre_cnv_w_idx: u16,
    pub sel_tre_cnv_w_idx: u16,

    pub tre_cnv_h: Float,
    pub min_tre_cnv_h: Float,
    pub tre_cnv_y0: Float,
    pub tre_cnv_y1: Float,

    pub node_size: Float,
    pub min_node_size: Float,
    pub max_node_size: Float,
    pub min_node_size_idx: u16,
    pub max_node_size_idx: u16,
    pub sel_node_size_idx: u16,

    pub min_lab_size: Float,
    pub max_lab_size: Float,
    pub tip_lab_size: Float,
    pub brnch_lab_size: Float,
    pub int_lab_size: Float,

    pub max_tip_labs_to_draw: usize,

    pub min_lab_size_idx: u16,
    pub max_lab_size_idx: u16,
    pub sel_tip_lab_size_idx: u16,
    pub sel_brnch_lab_size_idx: u16,
    pub sel_int_lab_size_idx: u16,

    pub extra_space_for_tip_labs: Float,
    pub tip_lab_w: Float,
    pub tip_lab_offset_x: Float,
    pub brnch_lab_offset_y: Float,
    pub int_lab_offset_x: Float,

    pub tip_brnch_labs_allowed: bool,
    pub draw_tip_labs: bool,
    pub draw_int_labs: bool,
    pub draw_brnch_labs: bool,
    pub draw_legend: bool,

    pub g_edge: Cache,
    pub g_lab_tip: Cache,
    pub g_lab_int: Cache,
    pub g_lab_brnch: Cache,
    pub g_legend: Cache,
    pub g_node_hover: Cache,
    pub g_node_sel: Cache,

    #[cfg(debug_assertions)]
    pub g_bounds: Cache,
    #[cfg(debug_assertions)]
    pub g_palette: Cache,

    pub tree: Tree,
    pub tree_tip_edges: Vec<Edge>,
    pub tree_chunked_edges: Vec<Edges>,

    pub tree_orig: Tree,
    pub tree_orig_chunked_edges: Option<Vec<Edges>>,
    pub tree_srtd_asc: Option<Tree>,
    pub tree_srtd_asc_chunked_edges: Option<Vec<Edges>>,
    pub tree_srtd_desc: Option<Tree>,
    pub tree_srtd_desc_chunked_edges: Option<Vec<Edges>>,

    pub sel_node_ids: HashSet<NodeId>,
    pub tallest_tips: Vec<Edge>,
}

impl TreeView {
    pub fn new() -> Self {
        Self {
            threads: 6,

            sel_node_ord_opt: NodeOrderingOption::Unordered,
            sel_tree_style_opt: TreeStyleOption::Phylogram,

            window_w: window_settings().size.width,
            window_h: SF,

            tree_scroll_w: SF,
            tree_scroll_h: SF,
            side_with_padding_w: SIDE_COL_W + SCROLL_TOOL_W,

            sel_opn_angle_idx: 359,
            min_opn_angle_idx: 45,
            max_opn_angle_idx: 359,

            sel_rot_angle_idx: 360,
            min_rot_angle_idx: 360 - 180,
            max_rot_angle_idx: 360 + 180,

            tre_cnv_w: window_settings().size.width - SIDE_COL_W - SCROLL_TOOL_W,
            min_tre_cnv_w: SF,
            min_tre_cnv_w_idx: 1,
            max_tre_cnv_w_idx: 24,

            tre_cnv_y0: SF,
            tre_cnv_y1: SF,
            tre_cnv_h: SF,
            min_tre_cnv_h: SF,
            node_size: SF,
            min_node_size: SF,
            max_node_size: SF,
            min_node_size_idx: 1,
            max_node_size_idx: 24,
            min_lab_size_idx: 1,
            max_lab_size_idx: 24,

            tip_brnch_labs_allowed: false,
            draw_tip_labs: true,
            draw_brnch_labs: false,
            draw_int_labs: true,
            draw_legend: true,
            show_ltt: false,

            sel_node_size_idx: 1,
            sel_tre_cnv_w_idx: 1,
            sel_tip_lab_size_idx: 4,
            sel_brnch_lab_size_idx: 4,
            sel_int_lab_size_idx: 4,

            tip_lab_size: SF,
            brnch_lab_size: SF,
            int_lab_size: SF,

            tip_lab_offset_x: SF * 3e0,
            brnch_lab_offset_y: SF * -1e0,
            int_lab_offset_x: SF * 3e0,

            tip_lab_w: SF,
            extra_space_for_tip_labs: SF,

            min_lab_size: SF * 1e0,
            max_lab_size: SF * 24e0,

            max_tip_labs_to_draw: 200,

            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum TreeViewMsg {
    OpenFile,
    SetWinId(WinId),
    Init,
    EnableDrawing,
    Refresh,

    LttCanvasScrolled(ScrollableViewport),
    TreeCanvasScrolled(ScrollableViewport),
    WindowResized(Float, Float),

    SelectDeselectNode(NodeId),
    SelectNode(NodeId),
    DeselectNode(NodeId),
    Unroot,
    Root(NodeId),

    TreeUpdated(Tree),

    TreeReprOptionChanged(TreeStyleOption),
    NodeOrderingOptionChanged(NodeOrderingOption),

    NodeSizeSelectionChanged(u16),
    CanvasWidthSelectionChanged(u16),

    OpnAngleSelectionChanged(u16),
    RotAngleSelectionChanged(u16),

    TipLabelVisibilityChanged(bool),
    TipLabelSizeSelectionChanged(u16),
    IntLabelVisibilityChanged(bool),
    IntLabelSizeSelectionChanged(u16),
    BranchLabelVisibilityChanged(bool),
    BranchLabelSizeSelectionChanged(u16),
    LegendVisibilityChanged(bool),
    LttVisibilityChanged(bool),
}
