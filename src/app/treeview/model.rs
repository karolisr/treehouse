use super::{NodeOrderingOption, TreeReprOption};
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

    pub selected_node_ordering_option: NodeOrderingOption,
    pub selected_tree_repr_option: TreeReprOption,

    pub drawing_enabled: bool,
    pub has_brlen: bool,
    pub has_int_labels: bool,
    pub has_tip_labels: bool,
    pub is_rooted: bool,
    pub is_ultrametric: Option<bool>,
    pub node_count: usize,
    pub tip_count: usize,
    pub int_node_count: usize,
    pub tree_height: Float,

    pub window_w: Float,
    pub window_h: Float,

    pub scroll_w: Float,
    pub not_scroll_w: Float,

    pub opn_angle: Float,
    pub selected_opn_angle_idx: u16,
    pub min_opn_angle_idx: u16,
    pub max_opn_angle_idx: u16,

    pub rot_angle: Float,
    pub selected_rot_angle_idx: u16,
    pub min_rot_angle_idx: u16,
    pub max_rot_angle_idx: u16,

    pub canvas_w: Float,
    pub min_canvas_w: Float,
    pub min_canvas_w_idx: u16,
    pub max_canvas_w_idx: u16,
    pub selected_canvas_w_idx: u16,

    pub canvas_h: Float,
    pub min_canvas_h: Float,
    pub cnv_y0: Float,
    pub cnv_y1: Float,

    pub node_size: Float,
    pub min_node_size: Float,
    pub max_node_size: Float,
    pub min_node_size_idx: u16,
    pub max_node_size_idx: u16,
    pub selected_node_size_idx: u16,

    pub min_label_size: Float,
    pub max_label_size: Float,
    pub tip_label_size: Float,
    pub branch_label_size: Float,
    pub int_label_size: Float,

    pub max_count_of_tip_labels_to_draw: usize,

    pub min_label_size_idx: u16,
    pub max_label_size_idx: u16,
    pub selected_tip_label_size_idx: u16,
    pub selected_branch_label_size_idx: u16,
    pub selected_int_label_size_idx: u16,

    pub extra_space_for_tip_labels: Float,
    pub tip_label_w: Float,
    pub tip_label_offset_x: Float,
    pub branch_label_offset_y: Float,
    pub int_label_offset_x: Float,

    pub draw_tip_branch_labels_allowed: bool,
    pub draw_tip_labels: bool,
    pub draw_int_labels: bool,
    pub draw_branch_labels: bool,
    pub draw_legend: bool,

    pub pointer_geom_cache: Cache,
    pub selected_nodes_geom_cache: Cache,
    pub edge_geom_cache: Cache,
    pub legend_geom_cache: Cache,
    pub tip_labels_geom_cache: Cache,
    pub branch_labels_geom_cache: Cache,
    pub int_labels_geom_cache: Cache,
    #[cfg(debug_assertions)]
    pub debug_geom_cache: Cache,

    pub tree: Tree,
    pub tree_tip_edges: Vec<Edge>,
    pub tree_chunked_edges: Vec<Edges>,

    pub tree_original: Tree,
    pub tree_original_chunked_edges: Option<Vec<Edges>>,
    pub tree_srtd_asc: Option<Tree>,
    pub tree_srtd_asc_chunked_edges: Option<Vec<Edges>>,
    pub tree_srtd_desc: Option<Tree>,
    pub tree_srtd_desc_chunked_edges: Option<Vec<Edges>>,

    pub selected_node_ids: HashSet<NodeId>,
    pub tallest_tips: Vec<Edge>,
}

impl TreeView {
    pub fn new() -> Self {
        Self {
            threads: 6,

            selected_node_ordering_option: NodeOrderingOption::Unordered,
            selected_tree_repr_option: TreeReprOption::Phylogram,

            window_w: window_settings().size.width,
            window_h: SF,

            scroll_w: SF,
            not_scroll_w: SIDE_COL_W + SCROLL_TOOL_W,

            selected_opn_angle_idx: 359,
            min_opn_angle_idx: 45,
            max_opn_angle_idx: 359,

            selected_rot_angle_idx: 0,
            min_rot_angle_idx: 0,
            max_rot_angle_idx: 360,

            canvas_w: window_settings().size.width - SIDE_COL_W - SCROLL_TOOL_W,
            min_canvas_w: SF,
            min_canvas_w_idx: 1,
            max_canvas_w_idx: 24,

            cnv_y0: SF,
            cnv_y1: SF,
            canvas_h: SF,
            min_canvas_h: SF,
            node_size: SF,
            min_node_size: SF,
            max_node_size: SF,
            min_node_size_idx: 1,
            max_node_size_idx: 24,
            min_label_size_idx: 1,
            max_label_size_idx: 24,

            draw_tip_branch_labels_allowed: false,
            draw_tip_labels: true,
            draw_branch_labels: true,
            draw_int_labels: true,
            draw_legend: true,

            selected_node_size_idx: 1,
            selected_canvas_w_idx: 1,
            selected_tip_label_size_idx: 12,
            selected_branch_label_size_idx: 12,
            selected_int_label_size_idx: 12,

            tip_label_size: SF,
            branch_label_size: SF,
            int_label_size: SF,

            tip_label_offset_x: SF * 3e0,
            branch_label_offset_y: SF * -1e0,
            int_label_offset_x: SF * 3e0,

            tip_label_w: SF,
            extra_space_for_tip_labels: SF,

            min_label_size: SF * 1e0,
            max_label_size: SF * 24e0,

            max_count_of_tip_labels_to_draw: 200,

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

    TreeViewScrolled(ScrollableViewport),
    WindowResized(Float, Float),

    SelectDeselectNode(NodeId),
    SelectNode(NodeId),
    DeselectNode(NodeId),
    Unroot,
    Root(NodeId),

    TreeUpdated(Tree),

    TreeReprOptionChanged(TreeReprOption),
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
}
