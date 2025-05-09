use super::{PlotCnv, TreeCnv, TreeStyle, TreeViewMsg};
use crate::{
    Float, NodeOrd,
    utils::{IndexRange, NodePoint},
};
use dendros::{Edges, NodeId, Tree, chunk_edges, flatten_tree};
use iced::{Point, Task};
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct TreeState {
    threads: usize,

    pub(crate) tre_cnv: TreeCnv,
    pub(crate) ltt_cnv: PlotCnv,

    pub(crate) sel_node_ids: HashSet<NodeId>,
    tip_idx_range: Option<IndexRange>,
    found_edge_pt: Option<Point>,

    tip_only_search: bool,
    search_string: String,
    found_edge_idx: usize,
    found_edges: Edges,
    found_node_ids: HashSet<NodeId>,

    pub(crate) int_node_count: usize,
    pub(crate) node_count: usize,
    pub(crate) tip_count: usize,
    pub(crate) tree_height: Float,
    pub(crate) has_brlen: bool,
    pub(crate) has_int_labs: bool,
    pub(crate) has_tip_labs: bool,
    pub(crate) is_rooted: bool,
    pub(crate) is_ultrametric: Option<bool>,

    tree: Tree,
    tree_orig: Tree,

    pub(crate) tree_edges: Edges,
    pub(crate) tallest_tips: Edges,
    pub(crate) tree_tip_edges: Edges,
    pub(crate) tree_edges_chunked: Vec<Edges>,

    tree_srtd_asc: Option<Tree>,
    tree_srtd_desc: Option<Tree>,

    tree_orig_edges: Option<Edges>,
    tree_srtd_asc_edges: Option<Edges>,
    tree_srtd_desc_edges: Option<Edges>,

    tree_orig_edges_chunked: Option<Vec<Edges>>,
    tree_srtd_asc_edges_chunked: Option<Vec<Edges>>,
    tree_srtd_desc_edges_chunked: Option<Vec<Edges>>,
}

#[derive(Debug, Clone)]
pub enum TreeStateMsg {
    Init(Tree, NodeOrd),
    Sort(NodeOrd),
    Unroot,
    Root(NodeId),
    // -------------------------------------------
    SelectDeselectNode(NodeId),
    SelectNode(NodeId),
    DeselectNode(NodeId),
}

impl TreeState {
    fn init(&mut self, tree: Tree) {
        self.tree_srtd_asc = None;
        self.tree_srtd_desc = None;
        self.tree_srtd_asc_edges_chunked = None;
        self.tree_srtd_desc_edges_chunked = None;
        self.tree_orig_edges_chunked = None;
        self.tree_orig = tree;
        self.tree = self.tree_orig.clone();
        self.cache()
    }

    fn cache(&mut self) {
        let epsilon = self.tree_orig.height() / 1e2;
        self.is_ultrametric = self.tree_orig.is_ultrametric(epsilon);
        self.node_count = self.tree_orig.node_count_all();
        self.tip_count = self.tree_orig.tip_count_all();
        self.int_node_count = self.tree_orig.internal_node_count_all();
        self.has_brlen = self.tree_orig.has_branch_lengths();
        self.has_int_labs = self.tree_orig.has_int_labels();
        self.has_tip_labs = self.tree_orig.has_tip_labels();
        self.tree_height = self.tree_orig.height() as Float;
        self.is_rooted = self.tree_orig.is_rooted();

        self.tre_cnv.is_rooted = self.is_rooted;
        self.tre_cnv.tree_height = self.tree_height;
    }

    pub(crate) fn can_root(&self, node_id: NodeId) -> bool {
        self.tree.can_root(node_id)
    }

    pub fn update(&mut self, msg: TreeStateMsg) -> Task<TreeViewMsg> {
        // println!("TreeState -> {msg:?}");
        match msg {
            TreeStateMsg::Init(tree, node_ord_opt) => {
                self.init(tree);
                // self.tallest_tips = self.tallest_tips();
                Task::done(TreeViewMsg::TreeStateMsg(TreeStateMsg::Sort(node_ord_opt)))
            }

            TreeStateMsg::Sort(node_ord_opt) => {
                self.sort(node_ord_opt);
                // self.tree_tip_edges = self.tree_tip_edges();
                Task::done(TreeViewMsg::TreeUpdated)
            }

            TreeStateMsg::Root(node_id) => {
                let mut tree = self.tree.clone();
                let rslt = tree.root(node_id);
                match rslt {
                    Ok(_) => {
                        self.tree_orig = tree;
                        self.cache();
                        Task::done(TreeViewMsg::TreeUpdated)
                    }
                    Err(err) => {
                        println!("{err}");
                        Task::none()
                    }
                }
            }

            TreeStateMsg::Unroot => {
                self.tree_orig.unroot();
                self.cache();
                Task::done(TreeViewMsg::TreeUpdated)
            }

            // ------------------------------------------------------------------------------------
            TreeStateMsg::SelectDeselectNode(node_id) => {
                if self.sel_node_ids.contains(&node_id) {
                    Task::done(TreeViewMsg::TreeStateMsg(TreeStateMsg::DeselectNode(node_id)))
                } else {
                    Task::done(TreeViewMsg::TreeStateMsg(TreeStateMsg::SelectNode(node_id)))
                }
            }

            TreeStateMsg::SelectNode(node_id) => {
                self.sel_node_ids.insert(node_id);
                Task::none()
            }

            TreeStateMsg::DeselectNode(node_id) => {
                self.sel_node_ids.remove(&node_id);
                Task::none()
            } // ----------------------------------------------------------------------------------
        }
    }

    fn tallest_tips(&self) -> Edges {
        let n: i32 = 10;
        let mut tmp = self.tree_tip_edges.clone();
        let tmp_len_min: usize = 0.max(tmp.len() as i32 - n) as usize;
        tmp.sort_by(|a, b| a.x1.total_cmp(&b.x1));
        let mut rv = tmp[tmp_len_min..tmp.len()].to_vec();
        tmp.sort_by(|a, b| {
            a.name.clone().map(|name| name.len()).cmp(&b.name.clone().map(|name| name.len()))
        });
        rv.append(&mut tmp[tmp_len_min..tmp.len()].to_vec());
        rv
    }

    fn tree_tip_edges(&self) -> Edges {
        let mut rv = Vec::new();
        for (i_c, chunk) in self.tree_edges_chunked.iter().enumerate() {
            for (i_e, edge) in chunk.iter().enumerate() {
                if edge.is_tip {
                    let mut e = edge.clone();
                    e.chunk_idx = i_c;
                    e.edge_idx = i_e;
                    rv.push(e);
                }
            }
        }
        rv
    }

    fn sort(&mut self, node_ord_opt: NodeOrd) {
        match node_ord_opt {
            NodeOrd::Unordered => {
                self.tree = self.tree_orig.clone();
                self.tree_edges_chunked = match &self.tree_orig_edges_chunked {
                    Some(chunked_edges) => {
                        self.tree_edges = self.tree_orig_edges.clone().unwrap();
                        chunked_edges.clone()
                    }
                    None => {
                        let edges = flatten_tree(&self.tree);
                        self.tree_orig_edges = Some(edges.clone());
                        self.tree_orig_edges_chunked = Some(chunk_edges(&edges, self.threads));
                        self.tree_edges = edges;
                        self.tree_orig_edges_chunked.clone().unwrap()
                    }
                };
            }

            NodeOrd::Ascending => match &self.tree_srtd_asc {
                Some(tree_srtd_asc) => {
                    self.tree = tree_srtd_asc.clone();
                    self.tree_edges = self.tree_srtd_asc_edges.clone().unwrap();
                    self.tree_edges_chunked = self.tree_srtd_asc_edges_chunked.clone().unwrap();
                }
                None => {
                    let mut tmp = self.tree_orig.clone();
                    tmp.sort(false);
                    self.tree_srtd_asc = Some(tmp);
                    self.tree = self.tree_srtd_asc.clone().unwrap();
                    let edges = flatten_tree(&self.tree);
                    self.tree_srtd_asc_edges = Some(edges.clone());
                    self.tree_srtd_asc_edges_chunked = Some(chunk_edges(&edges, self.threads));
                    self.tree_edges = edges;
                    self.tree_edges_chunked = self.tree_srtd_asc_edges_chunked.clone().unwrap();
                }
            },

            NodeOrd::Descending => match &self.tree_srtd_desc {
                Some(tree_srtd_desc) => {
                    self.tree = tree_srtd_desc.clone();
                    self.tree_edges = self.tree_srtd_desc_edges.clone().unwrap();
                    self.tree_edges_chunked = self.tree_srtd_desc_edges_chunked.clone().unwrap();
                }
                None => {
                    let mut tmp = self.tree_orig.clone();
                    tmp.sort(true);
                    self.tree_srtd_desc = Some(tmp);
                    self.tree = self.tree_srtd_desc.clone().unwrap();
                    let edges = flatten_tree(&self.tree);
                    self.tree_srtd_desc_edges = Some(edges.clone());
                    self.tree_srtd_desc_edges_chunked = Some(chunk_edges(&edges, self.threads));
                    self.tree_edges = edges;
                    self.tree_edges_chunked = self.tree_srtd_desc_edges_chunked.clone().unwrap();
                }
            },
        };
        /////
        self.tre_cnv.tree_edges_chunked = self.tree_edges_chunked.clone();
    }

    fn filter_nodes(&mut self) {
        self.found_node_ids.clear();
        self.found_edges.clear();
        self.found_edge_idx = 0;

        if self.search_string.len() < 3 {
            return;
        };

        let edges_to_search = match self.tip_only_search {
            true => &self.tree_tip_edges,
            false => &self.tree_edges,
        };

        for e in edges_to_search {
            if let Some(n) = &e.name {
                if let Some(_found) = n.to_lowercase().find(&self.search_string.to_lowercase()) {
                    self.found_node_ids.insert(e.node_id);
                    self.found_edges.push(e.clone());
                }
            }
        }
    }
}
