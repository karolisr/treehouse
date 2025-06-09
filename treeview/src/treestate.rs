use crate::iced::*;
use crate::*;

#[derive(Default, Debug)]
pub(super) struct TreeState {
    id: usize,
    sel_edge_idxs: Vec<usize>,
    sel_node_ids: HashSet<NodeId>,

    t: Tree,
    t_orig: Tree,
    t_srtd_asc: Option<Tree>,
    t_srtd_desc: Option<Tree>,

    edge_root: Option<Edge>,

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
    cache_sel_nodes: Cache,
    cache_filtered_nodes: Cache,

    // Memoized Values ---------------------------------------------------
    cache_tip_count: Option<usize>,
    cache_node_count: Option<usize>,
    cache_tre_height: Option<TreeFloat>,
    cache_has_tip_labs: Option<bool>,
    cache_has_int_labs: Option<bool>,
    cache_has_brlen: Option<bool>,
    cache_is_ultrametric: Option<Option<bool>>,
    cache_is_rooted: Option<bool>,

    // Search & Filter ---------------------------------------------------
    vec_idx_to_found_edge_idxs: usize,
    found_edge_idxs: Vec<usize>,
    found_node_ids: HashSet<NodeId>,
    tmp_found_node_id: Option<NodeId>,
}

impl TreeState {
    pub(super) fn tree(&self) -> &Tree { &self.t }
    pub(super) fn tree_original(&self) -> &Tree { &self.t_orig }

    // Search & Filter -----------------------------------------------------------------------------
    pub(super) fn found_edge_idxs(&self) -> &Vec<usize> { &self.found_edge_idxs }
    pub(super) fn found_node_ids(&self) -> &HashSet<NodeId> { &self.found_node_ids }
    pub(super) fn found_edge_idx(&self) -> usize { self.vec_idx_to_found_edge_idxs }

    pub(super) fn current_found_edge(&self) -> Option<Edge> {
        if !self.found_edge_idxs.is_empty() {
            let edge = &self.edges_srtd_y[self.found_edge_idxs[self.vec_idx_to_found_edge_idxs]];
            Some(edge.clone())
        } else {
            None
        }
    }

    pub(super) fn current_found_node_id(&self) -> Option<NodeId> {
        self.current_found_edge().map(|e| e.node_id)
    }

    pub(super) fn prev_result(&mut self) {
        self.clear_cache_filtered_nodes();
        if self.vec_idx_to_found_edge_idxs > 0 {
            self.vec_idx_to_found_edge_idxs -= 1;
        }
    }

    pub(super) fn next_result(&mut self) {
        self.clear_cache_filtered_nodes();
        if self.vec_idx_to_found_edge_idxs < self.found_edge_idxs.len() - 1 {
            self.vec_idx_to_found_edge_idxs += 1;
        }
    }

    pub(super) fn add_found_to_sel(&mut self) {
        let max_capacity = self.sel_node_ids.len() + self.found_node_ids.len();
        let mut sel_node_ids: HashSet<NodeId> = HashSet::with_capacity(max_capacity);
        self.sel_node_ids.union(&self.found_node_ids).collect_into(&mut sel_node_ids);
        self.sel_node_ids = sel_node_ids;
        self.sel_edge_idxs = self.sel_edge_idxs_prep();
        self.clear_cache_sel_nodes();
    }

    pub(super) fn rem_found_from_sel(&mut self) {
        let mut sel_node_ids: HashSet<NodeId> = HashSet::with_capacity(self.sel_node_ids.len());
        self.sel_node_ids.difference(&self.found_node_ids).collect_into(&mut sel_node_ids);
        self.sel_node_ids = sel_node_ids;
        self.sel_edge_idxs = self.sel_edge_idxs_prep();
        self.clear_cache_sel_nodes();
    }

    pub(super) fn filter_nodes(&mut self, query: &str, tips_only: bool) -> &Vec<usize> {
        self.found_node_ids.clear();
        self.found_edge_idxs.clear();
        self.clear_cache_filtered_nodes();
        self.vec_idx_to_found_edge_idxs = 0;

        if query.len() < 3 {
            return &self.found_edge_idxs;
        };

        let edges_to_search = match tips_only {
            true => &self.edges_tip,
            false => &self.edges_srtd_y,
        };

        for e in edges_to_search {
            if let Some(n) = &e.name
                && let Some(_) = n.to_lowercase().find(&query.to_lowercase())
            {
                self.found_node_ids.insert(e.node_id);
                self.found_edge_idxs.push(e.edge_idx);
            }
        }

        &self.found_edge_idxs
    }

    // Accessors -----------------------------------------------------------------------------------
    pub(super) fn edges_srtd_y(&self) -> &Vec<Edge> { &self.edges_srtd_y }
    pub(super) fn edges_tip(&self) -> &Vec<Edge> { &self.edges_tip }
    pub(super) fn edges_tip_tallest(&self) -> &Vec<Edge> { &self.edges_tip_tallest }
    pub(super) fn edges_tip_idx(&self) -> &Vec<usize> { &self.edges_tip_idx }
    pub(super) fn edge_root(&self) -> Option<Edge> { self.edge_root.clone() }

    // Memoized Methods ----------------------------------------------------------------------------
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

    // Rooting -------------------------------------------------------------------------------------
    pub(super) fn can_root(&self, node_id: &NodeId) -> bool { self.t.can_root(node_id) }

    pub(super) fn root(&mut self, node_id: &NodeId) -> Option<NodeId> {
        self.tmp_found_node_id = self.current_found_node_id();
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
        self.tmp_found_node_id = self.current_found_node_id();
        let mut tre = self.t_orig.clone();
        if let Some(yanked_node) = tre.unroot() {
            if let Some(yanked_node_id) = yanked_node.node_id()
                && self.sel_node_ids.contains(yanked_node_id)
            {
                self.deselect_node(yanked_node_id);
            }
            self.init(tre);
            Some(yanked_node)
        } else {
            None
        }
    }

    // Sorting -------------------------------------------------------------------------------------
    pub(super) fn sort(&mut self, node_ord_opt: NodeOrd) {
        let current_found_node_id;
        if let Some(node_id) = self.tmp_found_node_id {
            current_found_node_id = Some(node_id);
        } else {
            current_found_node_id = self.current_found_node_id();
        }
        self.tmp_found_node_id = None;

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
        self.edge_root = self.edge_root_prep();
        self.update_filter_results(current_found_node_id);
        self.sel_edge_idxs = self.sel_edge_idxs_prep();
        self.clear_caches_all();
    }

    fn update_filter_results(&mut self, current_found_node_id: Option<NodeId>) {
        self.vec_idx_to_found_edge_idxs = 0;
        self.found_edge_idxs.clear();
        if let Some(current_found_node_id) = current_found_node_id {
            let found_node_ids = self.found_node_ids.to_owned();
            self.found_node_ids.clear();
            let mut idx: usize = 0;
            for e in &self.edges_srtd_y {
                if found_node_ids.contains(&e.node_id) {
                    self.found_node_ids.insert(e.node_id);
                    self.found_edge_idxs.push(e.edge_idx);
                    if e.node_id == current_found_node_id {
                        self.vec_idx_to_found_edge_idxs = idx;
                    }
                    idx += 1;
                }
            }
        } else {
            self.found_node_ids.clear();
        }
    }

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
        tmp.sort_by(|a, b| {
            a.name.clone().map(|name| name.len()).cmp(&b.name.clone().map(|name| name.len()))
        });
        rv.append(&mut tmp[tmp_len_min..tmp.len()].to_vec());
        rv
    }

    fn edge_root_prep(&mut self) -> Option<Edge> {
        if self.is_rooted() {
            Some(
                self.edges_srtd_y
                    .iter()
                    .find(|j| j.parent_node_id.is_none())
                    .expect("Should have root!")
                    .clone(),
            )
        } else {
            None
        }
    }

    // Selection -----------------------------------------------------------------------------------
    pub(super) fn sel_edge_idxs(&self) -> &Vec<usize> { &self.sel_edge_idxs }
    pub(super) fn sel_node_ids(&self) -> &HashSet<NodeId> { &self.sel_node_ids }

    fn sel_edge_idxs_prep(&self) -> Vec<usize> {
        let sel_node_ids = &self.sel_node_ids;
        let rv_edge_idx: Vec<usize> = self
            .edges_srtd_y
            .par_iter()
            .filter_map(|edge| {
                if sel_node_ids.contains(&edge.node_id) { Some(edge.edge_idx) } else { None }
            })
            .collect();
        rv_edge_idx
    }

    pub(super) fn select_deselect_node(&mut self, node_id: &NodeId) {
        if self.sel_node_ids.contains(node_id) {
            self.deselect_node(node_id);
        } else {
            self.select_node(node_id);
        }
        self.sel_edge_idxs = self.sel_edge_idxs_prep();
    }

    pub(super) fn select_node(&mut self, node_id: &NodeId) {
        self.sel_node_ids.insert(*node_id);
        self.clear_cache_sel_nodes();
    }
    pub(super) fn deselect_node(&mut self, node_id: &NodeId) {
        self.sel_node_ids.remove(node_id);
        self.clear_cache_sel_nodes();
    }

    // Cached Geometries ---------------------------------------------------------------------------
    pub(super) fn cache_edge(&self) -> &Cache { &self.cache_edge }
    pub(super) fn cache_lab_tip(&self) -> &Cache { &self.cache_lab_tip }
    pub(super) fn cache_lab_int(&self) -> &Cache { &self.cache_lab_int }
    pub(super) fn cache_lab_brnch(&self) -> &Cache { &self.cache_lab_brnch }
    pub(super) fn cache_sel_nodes(&self) -> &Cache { &self.cache_sel_nodes }
    pub(super) fn cache_filtered_nodes(&self) -> &Cache { &self.cache_filtered_nodes }

    pub(super) fn clear_cache_edge(&self) { self.cache_edge.clear(); }
    pub(super) fn clear_cache_lab_tip(&self) { self.cache_lab_tip.clear(); }
    pub(super) fn clear_cache_lab_int(&self) { self.cache_lab_int.clear(); }
    pub(super) fn clear_cache_lab_brnch(&self) { self.cache_lab_brnch.clear(); }
    pub(super) fn clear_cache_sel_nodes(&self) { self.cache_sel_nodes.clear(); }
    pub(super) fn clear_cache_filtered_nodes(&self) { self.cache_filtered_nodes.clear(); }

    pub(super) fn clear_caches_all(&self) {
        self.clear_cache_edge();
        self.clear_cache_lab_tip();
        self.clear_cache_lab_int();
        self.clear_cache_lab_brnch();
        self.clear_cache_sel_nodes();
        self.clear_cache_filtered_nodes();
    }

    // Setup ---------------------------------------------------------------------------------------
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

        self.clear_caches_all();
        self.sort(NodeOrd::Unordered);
    }
}
