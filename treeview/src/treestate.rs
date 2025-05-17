use crate::{
    Float, IndexRange, NodeOrd, node_idx_range_for_tip_idx_range, tip_idx_range_between_y_vals,
};
use dendros::{Edges, Node, NodeId, Tree, TreeFloat, flatten_tree};
use iced::widget::canvas::Cache;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub(super) struct TreeState {
    id: usize,
    sel_node_ids: HashSet<NodeId>,

    t: Tree,
    t_orig: Tree,
    t_srtd_asc: Option<Tree>,
    t_srtd_desc: Option<Tree>,

    edges: Edges,
    edges_tip: Edges,
    edges_orig: Option<Edges>,
    edges_srtd_asc: Option<Edges>,
    edges_srtd_desc: Option<Edges>,

    cache_edge: Cache,
    cache_lab_tip: Cache,
    cache_lab_int: Cache,
    cache_lab_brnch: Cache,

    // Memoized Values ------------------------
    cache_tip_count: Option<usize>,
    cache_node_count: Option<usize>,
    cache_tree_height: Option<TreeFloat>,
    cache_has_tip_labs: Option<bool>,
    cache_has_int_labs: Option<bool>,
    cache_has_brlen: Option<bool>,
    cache_is_ultrametric: Option<Option<bool>>,
    cache_is_rooted: Option<bool>,
}

impl TreeState {
    fn edges_tip_prepare(&self) -> Edges {
        let mut rv = Vec::new();
        for (i_e, edge) in self.edges.iter().enumerate() {
            if edge.is_tip {
                let mut e = edge.clone();
                e.edge_idx = i_e;
                rv.push(e);
            }
        }
        rv
    }

    // pub(super) fn edges_tip(&self) -> &Edges {
    //     &self.edges_tip
    // }

    pub(super) fn visible_tip_idx_range(
        &self, y0: Float, y1: Float, node_size: Float,
    ) -> Option<IndexRange> {
        tip_idx_range_between_y_vals(y0, y1, node_size, &self.edges_tip)
    }

    pub(super) fn visible_node_idx_range(
        &self, y0: Float, y1: Float, node_size: Float,
    ) -> Option<IndexRange> {
        self.visible_tip_idx_range(y0, y1, node_size).map(|visible_tip_range| {
            node_idx_range_for_tip_idx_range(&visible_tip_range, &self.edges_tip)
        })
    }

    // Memoized Methods ---------------------------------------------------------------------------

    pub(super) fn tip_count(&self) -> usize {
        if let Some(cached) = self.cache_tip_count { cached } else { self.t.tip_count_all() }
    }

    pub(super) fn node_count(&self) -> usize {
        if let Some(cached) = self.cache_node_count { cached } else { self.t.node_count_all() }
    }

    pub(super) fn tree_height(&self) -> TreeFloat {
        if let Some(cached) = self.cache_tree_height { cached } else { self.t.height() }
    }

    pub(super) fn has_tip_labs(&self) -> bool {
        if let Some(cached) = self.cache_has_tip_labs { cached } else { self.t.has_tip_labels() }
    }

    pub(super) fn has_int_labs(&self) -> bool {
        if let Some(cached) = self.cache_has_int_labs { cached } else { self.t.has_int_labels() }
    }

    pub(super) fn has_brlen(&self) -> bool {
        if let Some(cached) = self.cache_has_brlen { cached } else { self.t.has_branch_lengths() }
    }

    pub(super) fn is_ultrametric(&self) -> Option<bool> {
        if let Some(cached) = self.cache_is_ultrametric {
            cached
        } else {
            let epsilon = self.t.height() / 1e2;
            self.t.is_ultrametric(epsilon)
        }
    }

    pub(super) fn is_rooted(&self) -> bool {
        if let Some(cached) = self.cache_is_rooted { cached } else { self.t.is_rooted() }
    }

    // Rooting ------------------------------------------------------------------------------------

    pub(super) fn can_root(&self, node_id: &NodeId) -> bool {
        self.t.can_root(node_id)
    }

    pub(super) fn root(&mut self, node_id: &NodeId) -> Option<NodeId> {
        let mut tree = self.t.clone();
        let rslt = tree.root(*node_id);
        match rslt {
            Ok(node_id) => {
                self.init(tree);
                Some(node_id)
            }
            Err(err) => {
                println!("{err}");
                None
            }
        }
    }

    pub(super) fn unroot(&mut self) -> Option<Node> {
        let mut tree = self.t_orig.clone();
        if let Some(node) = tree.unroot() {
            self.init(tree);
            Some(node)
        } else {
            None
        }
    }

    // Edges & Sorting ----------------------------------------------------------------------------

    pub(super) fn edges(&self) -> &Edges {
        &self.edges
    }

    pub(super) fn sort(&mut self, node_ord_opt: NodeOrd) {
        match node_ord_opt {
            NodeOrd::Unordered => {
                self.t = self.t_orig.clone();
                self.edges = match &self.edges_orig {
                    Some(tree_orig_edges) => tree_orig_edges.to_vec(),
                    None => {
                        let edges = flatten_tree(&self.t);
                        self.edges_orig = Some(edges.clone());
                        edges
                    }
                };
            }

            NodeOrd::Ascending => match &self.t_srtd_asc {
                Some(tree_srtd_asc) => {
                    self.t = tree_srtd_asc.clone();
                    self.edges = self.edges_srtd_asc.clone().unwrap();
                }
                None => {
                    let mut tmp = self.t_orig.clone();
                    tmp.sort(false);
                    self.t_srtd_asc = Some(tmp);
                    self.t = self.t_srtd_asc.clone().unwrap();
                    let edges = flatten_tree(&self.t);
                    self.edges_srtd_asc = Some(edges.clone());
                    self.edges = edges;
                }
            },

            NodeOrd::Descending => match &self.t_srtd_desc {
                Some(tree_srtd_desc) => {
                    self.t = tree_srtd_desc.clone();
                    self.edges = self.edges_srtd_desc.clone().unwrap();
                }
                None => {
                    let mut tmp = self.t_orig.clone();
                    tmp.sort(true);
                    self.t_srtd_desc = Some(tmp);
                    self.t = self.t_srtd_desc.clone().unwrap();
                    let edges = flatten_tree(&self.t);
                    self.edges_srtd_desc = Some(edges.clone());
                    self.edges = edges;
                }
            },
        };

        self.edges_tip = self.edges_tip_prepare();
    }

    // Selection ----------------------------------------------------------------------------------

    pub(super) fn select_deselect_node(&mut self, node_id: &NodeId) {
        if self.sel_node_ids.contains(node_id) {
            self.deselect_node(node_id);
        } else {
            self.select_node(node_id);
        }
    }

    pub(super) fn sel_node_ids(&self) -> &HashSet<dendros::NodeId> {
        &self.sel_node_ids
    }

    fn select_node(&mut self, node_id: &NodeId) {
        self.sel_node_ids.insert(*node_id);
    }

    fn deselect_node(&mut self, node_id: &NodeId) {
        self.sel_node_ids.remove(node_id);
    }

    // Cached Geometries --------------------------------------------------------------------------

    pub(super) fn cache_edge(&self) -> &Cache {
        &self.cache_edge
    }

    pub(super) fn clear_cache_edge(&self) {
        self.cache_edge.clear();
    }

    pub(super) fn cache_lab_tip(&self) -> &Cache {
        &self.cache_lab_tip
    }

    pub(super) fn clear_cache_lab_tip(&self) {
        self.cache_lab_tip.clear();
    }

    pub(super) fn cache_lab_int(&self) -> &Cache {
        &self.cache_lab_int
    }

    pub(super) fn clear_cache_lab_int(&self) {
        self.cache_lab_int.clear();
    }

    pub(super) fn cache_lab_brnch(&self) -> &Cache {
        &self.cache_lab_brnch
    }

    pub(super) fn clear_cache_lab_brnch(&self) {
        self.cache_lab_brnch.clear();
    }

    // Setup --------------------------------------------------------------------------------------

    pub(super) fn new(id: usize) -> Self {
        Self { id, ..Default::default() }
    }

    pub(super) fn id(&self) -> usize {
        self.id
    }

    pub(super) fn init(&mut self, tree: Tree) {
        self.t_srtd_asc = None;
        self.t_srtd_desc = None;
        self.t_orig = tree;
        self.t = self.t_orig.clone();

        self.cache_has_brlen = None;
        self.cache_has_int_labs = None;
        self.cache_has_tip_labs = None;
        self.cache_is_rooted = None;
        self.cache_is_ultrametric = None;
        self.cache_node_count = None;
        self.cache_tip_count = None;
        self.cache_tree_height = None;

        self.cache_has_brlen = Some(self.has_brlen());
        self.cache_has_int_labs = Some(self.has_int_labs());
        self.cache_has_tip_labs = Some(self.has_tip_labs());
        self.cache_is_rooted = Some(self.is_rooted());
        self.cache_is_ultrametric = Some(self.is_ultrametric());
        self.cache_node_count = Some(self.node_count());
        self.cache_tip_count = Some(self.tip_count());
        self.cache_tree_height = Some(self.tree_height());

        self.clear_cache_edge();
        self.clear_cache_lab_tip();
        self.clear_cache_lab_int();
        self.clear_cache_lab_brnch();
    }
}
