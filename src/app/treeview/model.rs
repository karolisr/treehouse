use super::NodeOrderingOption;
use crate::{Edge, Edges, Float, NodeId, Tree, app::SF};
use iced::{
    widget::{canvas::Cache, scrollable::Viewport as ScrollableViewport},
    window::Id as WinId,
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct TreeView {
    pub win_id: Option<WinId>,

    pub threads: usize,
    pub selected_node_ordering_option: Option<NodeOrderingOption>,
    pub drawing_enabled: bool,

    pub node_count: usize,
    pub tip_count: usize,
    pub int_node_count: usize,

    pub canvas_h: Float,
    pub cnv_y0: Float,
    pub cnv_y1: Float,

    pub window_w: Float,
    pub window_h: Float,

    pub min_label_size: Float,
    pub max_label_size: Float,
    pub tip_label_size: Float,
    pub int_label_size: Float,

    pub max_count_of_tip_labels_to_draw: usize,

    pub min_label_size_idx: u8,
    pub max_label_size_idx: u8,
    pub selected_tip_label_size_idx: u8,
    pub selected_int_label_size_idx: u8,

    pub available_vertical_space: Float,
    pub node_size: Float,
    pub min_node_size: Float,
    pub max_node_size: Float,

    pub min_node_size_idx: u8,
    pub max_node_size_idx: u8,
    pub selected_node_size_idx: u8,

    pub tip_labels_w_scale_factor: Float,
    pub tip_label_w: Float,
    pub tip_label_offset: Float,
    pub int_label_offset: Float,

    pub draw_tip_labels_allowed: bool,
    pub draw_tip_labels_selection: bool,
    pub draw_int_labels_selection: bool,

    pub pointer_geom_cache: Cache,
    pub selected_nodes_geom_cache: Cache,
    pub edge_geom_cache: Cache,
    pub tip_labels_geom_cache: Cache,
    pub int_labels_geom_cache: Cache,

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
}

impl TreeView {
    pub fn new() -> Self {
        Self {
            threads: 1,
            selected_node_ordering_option: Some(NodeOrderingOption::Unordered),

            canvas_h: SF,
            cnv_y0: SF,
            cnv_y1: SF,

            window_w: SF,
            window_h: SF,

            min_node_size_idx: 1,
            min_label_size_idx: 1,
            max_node_size_idx: 24,
            max_label_size_idx: 24,

            selected_node_size_idx: 1,

            tip_label_size: SF * 5e0,
            selected_tip_label_size_idx: 5,

            int_label_size: SF * 8e0,
            selected_int_label_size_idx: 8,

            node_size: SF,
            min_node_size: SF,
            max_node_size: SF,
            min_label_size: SF * 1e0,
            max_label_size: SF * 24e0,

            max_count_of_tip_labels_to_draw: 200,

            available_vertical_space: SF,

            tip_labels_w_scale_factor: 1e0,
            tip_label_w: SF,
            tip_label_offset: SF * 3e0,
            int_label_offset: SF * 3e0,

            draw_tip_labels_allowed: false,
            draw_tip_labels_selection: true,
            draw_int_labels_selection: false,

            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum TreeViewMsg {
    SetWinId(WinId),
    TreeUpdated(Tree),
    NodeOrderingOptionChanged(NodeOrderingOption),
    WindowResized(Float, Float),
    NodeSizeSelectionChanged(u8),
    TipLabelSizeSelectionChanged(u8),
    IntLabelSizeSelectionChanged(u8),
    TipLabelVisibilityChanged(bool),
    IntLabelVisibilityChanged(bool),
    TreeViewScrolled(ScrollableViewport),
    SelectDeselectNode(NodeId),
    SelectNode(NodeId),
    DeselectNode(NodeId),
    Root(NodeId),
    Unroot,
    OpenFile,
}
