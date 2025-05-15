use crate::{
    Float, RectVals,
    cnv_utils::{NodePoint, all_nodes},
    treeview::{NodeOrd, TreeStyle},
};
use dendros::{Edges, Node, NodeId, Tree, TreeFloat, flatten_tree};
use iced::widget::canvas::Cache;
use std::collections::HashSet;

#[derive(Default)]
pub(super) struct TreeState {
    id: usize,
    sel_node_ids: HashSet<NodeId>,

    t: Tree,
    t_orig: Tree,
    t_srtd_asc: Option<Tree>,
    t_srtd_desc: Option<Tree>,

    edges: Edges,
    edges_orig: Option<Edges>,
    edges_srtd_asc: Option<Edges>,
    edges_srtd_desc: Option<Edges>,

    all_nodepoints: Vec<NodePoint>,

    cache_edge: Cache,
    cache_lab_tip: Cache,
    cache_lab_int: Cache,
    cache_lab_brnch: Cache,
}

impl TreeState {
    pub(super) fn new(id: usize) -> Self {
        Self { id, ..Default::default() }
    }

    pub(super) fn init(&mut self, tree: Tree) {
        self.t_srtd_asc = None;
        self.t_srtd_desc = None;
        self.t_orig = tree;
        self.t = self.t_orig.clone();
    }

    pub(super) fn select_deselect_node(&mut self, node_id: &NodeId) {
        if self.sel_node_ids.contains(node_id) {
            self.deselect_node(node_id);
        } else {
            self.select_node(node_id);
        }
    }

    pub(super) fn all_nodepoints_calc(
        &mut self, tree_vals: RectVals<Float>, rot_angle: Float, opn_angle: Float,
        tree_style: TreeStyle,
    ) {
        self.all_nodepoints = all_nodes(
            tree_vals.w,
            tree_vals.h,
            tree_vals.cntr_untrans,
            tree_vals.radius_min,
            rot_angle,
            opn_angle,
            tree_style,
            self.edges(),
        );
    }

    pub(super) fn all_nodepoints(&self) -> &Vec<NodePoint> {
        &self.all_nodepoints
    }

    pub(super) fn clear_cache_edge(&self) {
        self.cache_edge.clear();
    }
    pub(super) fn cache_edge(&self) -> &Cache {
        &self.cache_edge
    }

    pub(super) fn clear_cache_lab_tip(&self) {
        self.cache_lab_tip.clear();
    }
    pub(super) fn cache_lab_tip(&self) -> &Cache {
        &self.cache_lab_tip
    }

    pub(super) fn clear_cache_lab_int(&self) {
        self.cache_lab_int.clear();
    }
    pub(super) fn cache_lab_int(&self) -> &Cache {
        &self.cache_lab_int
    }

    pub(super) fn clear_cache_lab_brnch(&self) {
        self.cache_lab_brnch.clear();
    }
    pub(super) fn cache_lab_brnch(&self) -> &Cache {
        &self.cache_lab_brnch
    }

    pub(super) fn edges(&self) -> &Edges {
        &self.edges
    }

    fn select_node(&mut self, node_id: &NodeId) {
        self.sel_node_ids.insert(*node_id);
    }

    fn deselect_node(&mut self, node_id: &NodeId) {
        self.sel_node_ids.remove(node_id);
    }

    pub(super) fn selected_node_ids(&self) -> &HashSet<dendros::NodeId> {
        &self.sel_node_ids
    }

    pub(super) fn id(&self) -> usize {
        self.id
    }

    pub(super) fn tip_count(&self) -> usize {
        self.t.tip_count_all()
    }

    pub(super) fn node_count(&self) -> usize {
        self.t.node_count_all()
    }

    pub(super) fn tree_height(&self) -> TreeFloat {
        self.t.height()
    }

    pub(super) fn is_rooted(&self) -> bool {
        self.t.is_rooted()
    }

    pub(super) fn has_brlen(&self) -> bool {
        self.t.has_branch_lengths()
    }

    pub(super) fn has_tip_labs(&self) -> bool {
        self.t.has_tip_labels()
    }

    pub(super) fn has_int_labs(&self) -> bool {
        self.t.has_int_labels()
    }

    pub(super) fn is_ultrametric(&self) -> Option<bool> {
        let epsilon = self.t.height() / 1e2;
        self.t.is_ultrametric(epsilon)
    }

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
    }
}
