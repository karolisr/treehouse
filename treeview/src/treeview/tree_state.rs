use crate::{IndexRange, NodePoint};
use dendros::{Edges, NodeId, Tree, TreeFloat};
use iced::Point;
use std::collections::HashSet;

pub(crate) struct TreeState {
    pub(crate) has_brlen: bool,
    pub(crate) has_int_labs: bool,
    pub(crate) has_tip_labs: bool,
    pub(crate) int_node_count: usize,
    pub(crate) is_rooted: bool,
    pub(crate) is_ultrametric: Option<bool>,
    pub(crate) node_count: usize,
    pub(crate) tip_count: usize,
    pub(crate) tree_height: TreeFloat,

    pub(crate) sel_node_ids: HashSet<NodeId>,
    pub(crate) tallest_tips: Edges,
    pub(crate) visible_nodes: Vec<NodePoint>,
    pub(crate) tip_idx_range: Option<IndexRange>,

    pub(crate) tip_only_search: bool,
    pub(crate) search_string: String,
    pub(crate) found_edge_idx: usize,
    pub(crate) found_edge_pt: Option<Point>,
    pub(crate) found_edges: Edges,
    pub(crate) found_node_ids: HashSet<NodeId>,

    pub(crate) tree_orig: Tree,
    pub(crate) tree: Tree,
    pub(crate) tree_tip_edges: Edges,
    pub(crate) tree_edges: Edges,
    pub(crate) tree_edges_chunked: Vec<Edges>,
    pub(crate) tree_orig_edges_chunked: Option<Vec<Edges>>,
    pub(crate) tree_orig_edges: Option<Edges>,
    pub(crate) tree_srtd_asc_edges_chunked: Option<Vec<Edges>>,
    pub(crate) tree_srtd_asc_edges: Option<Edges>,
    pub(crate) tree_srtd_asc: Option<Tree>,
    pub(crate) tree_srtd_desc_edges_chunked: Option<Vec<Edges>>,
    pub(crate) tree_srtd_desc_edges: Option<Edges>,
    pub(crate) tree_srtd_desc: Option<Tree>,
}
