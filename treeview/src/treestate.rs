use crate::iced::*;
use crate::*;

#[derive(Default, Debug)]
pub(super) struct TreeState {
    id: usize,
    sel_node_ids: HashSet<NodeId>,

    t: Tree,
    t_orig: Tree,
    t_srtd_asc: Option<Tree>,
    t_srtd_desc: Option<Tree>,

    edge_root: Option<Edge>,

    // edges_srtd_x: Vec<Edge>,
    edges_srtd_y: Vec<Edge>,
    edges_tip: Vec<Edge>,
    edges_tip_idx: Vec<usize>,
    edges_tip_tallest: Vec<Edge>,

    edges_orig: Option<Vec<Edge>>,
    edges_srtd_asc: Option<Vec<Edge>>,
    edges_srtd_desc: Option<Vec<Edge>>,

    cache_edge: Cache,
    cache_lab_tip: Cache,
    cache_lab_int: Cache,
    cache_lab_brnch: Cache,

    // Memoized Values ---------------------------------------------------
    cache_tip_count: Option<usize>,
    cache_node_count: Option<usize>,
    cache_tre_height: Option<TreeFloat>,
    cache_has_tip_labs: Option<bool>,
    cache_has_int_labs: Option<bool>,
    cache_has_brlen: Option<bool>,
    cache_is_ultrametric: Option<Option<bool>>,
    cache_is_rooted: Option<bool>,
}

impl TreeState {
    pub(super) fn edges_srtd_y(&self) -> &Vec<Edge> { &self.edges_srtd_y }
    // pub(super) fn edges_srtd_x(&self) -> &Vec<Edge> { &self.edges_srtd_x }
    pub(super) fn edges_tip(&self) -> &Vec<Edge> { &self.edges_tip }
    pub(super) fn edges_tip_tallest(&self) -> &Vec<Edge> { &self.edges_tip_tallest }
    pub(super) fn edges_tip_idx(&self) -> &Vec<usize> { &self.edges_tip_idx }
    pub(super) fn edge_root(&self) -> Option<Edge> { self.edge_root.clone() }

    // Memoized Methods --------------------------------------------------
    pub(super) fn tip_count(&self) -> usize {
        if let Some(cached) = self.cache_tip_count { cached } else { self.t.tip_count_all() }
    }

    pub(super) fn node_count(&self) -> usize {
        if let Some(cached) = self.cache_node_count { cached } else { self.t.node_count_all() }
    }

    pub(super) fn tre_height(&self) -> TreeFloat {
        if let Some(cached) = self.cache_tre_height { cached } else { self.t.height() }
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

    // Rooting -----------------------------------------------------------
    pub(super) fn can_root(&self, node_id: &NodeId) -> bool { self.t.can_root(node_id) }

    pub(super) fn root(&mut self, node_id: &NodeId) -> Option<NodeId> {
        let mut tre = self.t.clone();
        let rslt = tre.root(*node_id);
        match rslt {
            Ok(node_id) => {
                self.init(tre);
                Some(node_id)
            }
            Err(err) => {
                println!("{err}");
                None
            }
        }
    }

    pub(super) fn unroot(&mut self) -> Option<Node> {
        let mut tre = self.t_orig.clone();
        if let Some(node) = tre.unroot() {
            self.init(tre);
            Some(node)
        } else {
            None
        }
    }

    // Sorting -----------------------------------------------------------
    pub(super) fn sort(&mut self, node_ord_opt: NodeOrd) {
        match node_ord_opt {
            NodeOrd::Unordered => {
                self.t = self.t_orig.clone();
                self.edges_srtd_y = match &self.edges_orig {
                    Some(tre_orig_edges) => tre_orig_edges.to_vec(),
                    None => {
                        let edges = flatten_tree(&self.t);
                        self.edges_orig = Some(edges.to_vec());
                        edges
                    }
                };
            }

            NodeOrd::Ascending => match &self.t_srtd_asc {
                Some(tre_srtd_asc) => {
                    self.t = tre_srtd_asc.clone();
                    if let Some(edges_srtd_asc) = &self.edges_srtd_asc {
                        self.edges_srtd_y = edges_srtd_asc.to_vec();
                    }
                }
                None => {
                    let mut tmp = self.t_orig.clone();
                    tmp.sort(false);
                    self.t = tmp.clone();
                    self.t_srtd_asc = Some(tmp);
                    let edges = flatten_tree(&self.t);
                    self.edges_srtd_y = edges.to_vec();
                    self.edges_srtd_asc = Some(edges);
                }
            },

            NodeOrd::Descending => match &self.t_srtd_desc {
                Some(tre_srtd_desc) => {
                    self.t = tre_srtd_desc.clone();
                    if let Some(edges_srtd_desc) = &self.edges_srtd_desc {
                        self.edges_srtd_y = edges_srtd_desc.to_vec();
                    }
                }
                None => {
                    let mut tmp = self.t_orig.clone();
                    tmp.sort(true);
                    self.t = tmp.clone();
                    self.t_srtd_desc = Some(tmp);
                    let edges = flatten_tree(&self.t);
                    self.edges_srtd_y = edges.to_vec();
                    self.edges_srtd_desc = Some(edges);
                }
            },
        };

        (self.edges_tip, self.edges_tip_idx) = self.edges_tip_prep();
        self.edges_tip_tallest = self.edges_tip_tallest_prep();
        // self.edges_srtd_x = self.edges_srtd_x_prep();
        self.edge_root = self.edge_root_prep();
    }

    // fn edges_srtd_x_prep(&self) -> Vec<Edge> {
    //     let mut edges_srtd_by_x = self.edges_srtd_y.to_vec();
    //     edges_srtd_by_x.par_sort_by(|a, b| a.x1.total_cmp(&b.x1));
    //     edges_srtd_by_x
    // }

    fn edges_tip_prep(&mut self) -> (Vec<Edge>, Vec<usize>) {
        let mut rv_tip = Vec::new();
        let mut rv_tip_idx = Vec::new();
        for (i_e, edge) in &mut self.edges_srtd_y.iter_mut().enumerate() {
            edge.edge_idx = i_e;
            if edge.is_tip {
                rv_tip.push(edge.clone());
                rv_tip_idx.push(i_e);
            }
        }
        (rv_tip, rv_tip_idx)
    }

    fn edges_tip_tallest_prep(&self) -> Vec<Edge> {
        let n: i32 = 10;
        let mut tmp = self.edges_tip().clone();
        let tmp_len_min: usize = 0.max(tmp.len() as i32 - n) as usize;
        tmp.sort_by(|a, b| a.x1.total_cmp(&b.x1));
        let mut rv = tmp[tmp_len_min..tmp.len()].to_vec();
        tmp.sort_by(|a, b| a.name.clone().map(|name| name.len()).cmp(&b.name.clone().map(|name| name.len())));
        rv.append(&mut tmp[tmp_len_min..tmp.len()].to_vec());
        rv
    }

    fn edge_root_prep(&mut self) -> Option<Edge> {
        if self.is_rooted() {
            Some(self.edges_srtd_y.iter().find(|j| j.parent_node_id.is_none()).expect("Should have root!").clone())
        } else {
            None
        }
    }

    // Selection ---------------------------------------------------------
    pub(super) fn sel_node_ids(&self) -> &HashSet<dendros::NodeId> { &self.sel_node_ids }

    pub(super) fn select_deselect_node(&mut self, node_id: &NodeId) {
        if self.sel_node_ids.contains(node_id) {
            self.deselect_node(node_id);
        } else {
            self.select_node(node_id);
        }
    }

    fn select_node(&mut self, node_id: &NodeId) { self.sel_node_ids.insert(*node_id); }
    fn deselect_node(&mut self, node_id: &NodeId) { self.sel_node_ids.remove(node_id); }

    // Cached Geometries -------------------------------------------------
    pub(super) fn cache_edge(&self) -> &Cache { &self.cache_edge }
    pub(super) fn clear_cache_edge(&self) { self.cache_edge.clear(); }
    pub(super) fn cache_lab_tip(&self) -> &Cache { &self.cache_lab_tip }
    pub(super) fn clear_cache_lab_tip(&self) { self.cache_lab_tip.clear(); }
    pub(super) fn cache_lab_int(&self) -> &Cache { &self.cache_lab_int }
    pub(super) fn clear_cache_lab_int(&self) { self.cache_lab_int.clear(); }
    pub(super) fn cache_lab_brnch(&self) -> &Cache { &self.cache_lab_brnch }
    pub(super) fn clear_cache_lab_brnch(&self) { self.cache_lab_brnch.clear(); }

    // Setup -------------------------------------------------------------
    pub(super) fn new(id: usize) -> Self { Self { id, ..Default::default() } }
    pub(super) fn id(&self) -> usize { self.id }

    pub(super) fn init(&mut self, tre: Tree) {
        self.t_srtd_asc = None;
        self.t_srtd_desc = None;
        self.t_orig = tre;
        self.t = self.t_orig.clone();

        self.edges_orig = None;
        self.edges_srtd_asc = None;
        self.edges_srtd_desc = None;
        self.edges_srtd_y = vec![];
        self.edges_tip_idx = vec![];
        self.edges_tip = vec![];

        self.cache_has_brlen = None;
        self.cache_has_int_labs = None;
        self.cache_has_tip_labs = None;
        self.cache_is_rooted = None;
        self.cache_is_ultrametric = None;
        self.cache_node_count = None;
        self.cache_tip_count = None;
        self.cache_tre_height = None;

        self.cache_has_brlen = Some(self.has_brlen());
        self.cache_has_int_labs = Some(self.has_int_labs());
        self.cache_has_tip_labs = Some(self.has_tip_labs());
        self.cache_is_rooted = Some(self.is_rooted());
        self.cache_is_ultrametric = Some(self.is_ultrametric());
        self.cache_node_count = Some(self.node_count());
        self.cache_tip_count = Some(self.tip_count());
        self.cache_tre_height = Some(self.tre_height());
    }
}
