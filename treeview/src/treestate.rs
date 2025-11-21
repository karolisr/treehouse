use crate::CladeLabel;
use crate::CladeLabelType;
use crate::SortOrd;
use crate::TreNodeOrd;

use dendros::IndexRange;
use riced::CnvCache;
use riced::Color;

use dendros::Edge;
use dendros::Node;
use dendros::NodeId;
use dendros::Tree;
use dendros::TreeFloat;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use std::collections::HashMap;
use std::collections::HashSet;

fn normalize_value<T>(
    min: impl Into<T>,
    max: impl Into<T>,
    value: impl Into<T>,
) -> T
where
    T: std::ops::Sub<Output = T>
        + Copy
        + std::ops::Add<
            <<T as std::ops::Sub>::Output as std::ops::Mul<T>>::Output,
            Output = T,
        > + std::ops::Mul
        + std::ops::Div<Output = T>,
{
    let min = min.into();
    let max = max.into();
    let value = value.into();

    (value - min) / (max - min)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeSortField {
    NodeId,
    NodeLabel,
    Selected,
}

#[derive(Default, Debug)]
pub(super) struct TreeState {
    id: usize,
    sel_edge_idxs: Vec<usize>,
    sel_node_ids: HashSet<NodeId>,
    labeled_clades: HashMap<NodeId, CladeLabel>,
    node_ord_opt: TreNodeOrd,

    t_orig: Tree,
    t_srtd_asc: Option<Tree>,
    t_srtd_desc: Option<Tree>,

    edge_root: Option<Edge>,
    edges_tip: Vec<Edge>,
    edges_tip_tallest: Vec<Edge>,

    // --- Canvas Geometry Caches ----------------------------------------------
    cache_cnv_edge: CnvCache,
    cache_cnv_lab_tip: CnvCache,
    cache_cnv_lab_int: CnvCache,
    cache_cnv_lab_brnch: CnvCache,
    cache_cnv_sel_nodes: CnvCache,
    cache_cnv_filtered_nodes: CnvCache,
    cache_cnv_clade_labels: CnvCache,

    // --- Caches of Edges Sorted by the Fields in the Edge Struct -------------
    cache_edges_node_id_asc: Option<Vec<Edge>>,
    cache_edges_node_id_desc: Option<Vec<Edge>>,
    cache_edges_node_label_asc: Option<Vec<Edge>>,
    cache_edges_node_label_desc: Option<Vec<Edge>>,
    cache_edges_selected_asc: Option<Vec<Edge>>,
    cache_edges_selected_desc: Option<Vec<Edge>>,

    // --- Caches of Values for Memoized Accessor Functions --------------------
    cache_tip_count: Option<usize>,
    cache_node_count: Option<usize>,
    cache_max_first_node_to_tip_distance: Option<TreeFloat>,
    cache_has_tip_labs: Option<bool>,
    cache_has_int_labs: Option<bool>,
    cache_has_brlen: Option<bool>,
    cache_is_ultrametric: Option<Option<bool>>,
    cache_is_rooted: Option<bool>,

    // --- Search & Filter -----------------------------------------------------
    vec_idx_to_found_edge_idxs: usize,
    found_edge_idxs: Vec<usize>,
    found_node_ids: HashSet<NodeId>,
    tmp_found_node_id: Option<NodeId>,
    search_query: Option<String>,
    tip_only_search: Option<bool>,

    // --- Subtree View --------------------------------------------------------
    subtree_view_node_id: Option<NodeId>,
    subtree_view_tip_edge_idx_range: Option<IndexRange>,
    subtree_view_edges: Option<Vec<Edge>>,
    subtree_view_cache_tip_count: Option<usize>,
    subtree_view_cache_node_count: Option<usize>,
    subtree_view_cache_max_first_node_to_tip_distance: Option<TreeFloat>,
    subtree_view_edges_tip: Vec<Edge>,
    subtree_view_edges_tip_tallest: Vec<Edge>,
    subtree_view_sel_edge_idxs: Vec<usize>,
}

impl TreeState {
    // --- Setup ---------------------------------------------------------------

    pub(super) fn new(id: usize) -> Self {
        Self { id, ..Default::default() }
    }

    pub(super) fn init(&mut self, tre: Tree) {
        self.t_orig = tre;
        self.t_srtd_asc = None;
        self.t_srtd_desc = None;

        self.edges_tip = vec![];

        self.cache_has_brlen = None;
        self.cache_has_int_labs = None;
        self.cache_has_tip_labs = None;
        self.cache_is_rooted = None;
        self.cache_is_ultrametric = None;
        self.cache_node_count = None;
        self.cache_tip_count = None;
        self.cache_max_first_node_to_tip_distance = None;

        self.cache_has_brlen = Some(self.has_brlen());
        self.cache_has_int_labs = Some(self.has_int_labs());
        self.cache_has_tip_labs = Some(self.has_tip_labels());
        self.cache_is_rooted = Some(self.is_rooted());
        self.cache_is_ultrametric = Some(self.is_ultrametric());
        self.cache_node_count = Some(self.node_count());
        self.cache_tip_count = Some(self.tip_count());
        self.cache_max_first_node_to_tip_distance =
            Some(self.max_first_node_to_tip_distance());

        self.close_subtree_view();
        self.clear_caches_of_edges_sorted_by_field();
        self.clear_caches_cnv();

        self.sort_asc();
        self.sort(self.node_ord_opt);
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Accessors -----------------------------------------------------------

    pub(super) fn id(&self) -> usize {
        self.id
    }

    pub(super) fn tree(&self) -> &Tree {
        match self.node_ord_opt {
            TreNodeOrd::Unordered => &self.t_orig,
            TreNodeOrd::Ascending => match &self.t_srtd_asc {
                Some(t_srtd_asc) => t_srtd_asc,
                None => &self.t_orig,
            },
            TreNodeOrd::Descending => match &self.t_srtd_desc {
                Some(t_srtd_desc) => t_srtd_desc,
                None => &self.t_orig,
            },
        }
    }

    pub(super) fn edges_tip(&self) -> &Vec<Edge> {
        if self.is_subtree_view_active() {
            self.edges_tip_for_subtree_view()
        } else {
            self.edges_tip_tree()
        }
    }

    pub(super) fn edges_tip_tree(&self) -> &Vec<Edge> {
        &self.edges_tip
    }

    pub(super) fn edges_tip_for_subtree_view(&self) -> &Vec<Edge> {
        &self.subtree_view_edges_tip
    }

    pub(super) fn edges_tip_tallest(&self) -> &Vec<Edge> {
        if self.is_subtree_view_active() {
            self.edges_tip_tallest_for_subtree_view()
        } else {
            self.edges_tip_tallest_tree()
        }
    }

    pub(super) fn edges_tip_tallest_tree(&self) -> &Vec<Edge> {
        &self.edges_tip_tallest
    }

    pub(super) fn edges_tip_tallest_for_subtree_view(&self) -> &Vec<Edge> {
        &self.subtree_view_edges_tip_tallest
    }

    pub(super) fn edge_root(&self) -> Option<Edge> {
        if self.is_subtree_view_active() { None } else { self.edge_root_tree() }
    }

    fn edge_root_tree(&self) -> Option<Edge> {
        self.edge_root.clone()
    }

    // --- Memoized Accessors --------------------------------------------------

    pub(super) fn tip_count(&self) -> usize {
        if self.is_subtree_view_active() {
            self.tip_count_for_subtree_view().unwrap()
        } else {
            self.tip_count_tree()
        }
    }

    pub(super) fn tip_count_tree(&self) -> usize {
        if let Some(cached) = self.cache_tip_count {
            cached
        } else {
            self.tree().tip_count_all()
        }
    }

    pub(super) fn node_count(&self) -> usize {
        if self.is_subtree_view_active() {
            self.node_count_for_subtree_view().unwrap()
        } else {
            self.node_count_tree()
        }
    }

    pub(super) fn node_count_tree(&self) -> usize {
        if let Some(cached) = self.cache_node_count {
            cached
        } else {
            self.tree().node_count_all()
        }
    }

    pub(super) fn max_first_node_to_tip_distance(&self) -> TreeFloat {
        if self.is_subtree_view_active() {
            self.max_first_node_to_tip_distance_for_subtree_view().unwrap()
        } else {
            self.max_first_node_to_tip_distance_tree()
        }
    }

    pub(super) fn max_first_node_to_tip_distance_tree(&self) -> TreeFloat {
        if let Some(cached) = self.cache_max_first_node_to_tip_distance {
            cached
        } else {
            self.tree().max_first_node_to_tip_distance()
        }
    }

    pub(super) fn has_tip_labels(&self) -> bool {
        if let Some(cached) = self.cache_has_tip_labs {
            cached
        } else {
            self.tree().has_tip_labels()
        }
    }

    pub(super) fn has_int_labs(&self) -> bool {
        if let Some(cached) = self.cache_has_int_labs {
            cached
        } else {
            self.tree().has_internal_node_labels()
        }
    }

    pub(super) fn has_brlen(&self) -> bool {
        if let Some(cached) = self.cache_has_brlen {
            cached
        } else {
            self.tree().has_branch_lengths()
        }
    }

    pub(super) fn is_ultrametric(&self) -> Option<bool> {
        if let Some(cached) = self.cache_is_ultrametric {
            cached
        } else {
            let epsilon = self.tree().max_first_node_to_tip_distance() / 1e2;
            self.tree().is_ultrametric(epsilon)
        }
    }

    pub(super) fn is_rooted(&self) -> bool {
        if self.is_subtree_view_active() { true } else { self.is_rooted_tree() }
    }

    pub(super) fn is_rooted_tree(&self) -> bool {
        if let Some(cached) = self.cache_is_rooted {
            cached
        } else {
            self.tree().is_rooted()
        }
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Utilities -----------------------------------------------------------

    pub(super) fn edges(&self) -> Option<&Vec<Edge>> {
        if self.is_subtree_view_active() {
            self.edges_for_subtree_view()
        } else {
            self.edges_for_tree()
        }
    }

    fn edges_for_subtree_view(&self) -> Option<&Vec<Edge>> {
        self.subtree_view_edges.as_ref()
    }

    fn edges_for_tree(&self) -> Option<&Vec<Edge>> {
        self.tree().edges()
    }

    pub(super) fn bounding_edges_for_clade(
        &self,
        node_id: NodeId,
    ) -> Option<(Vec<Edge>, Vec<Edge>)> {
        if self.is_subtree_view_active() {
            self.bounding_edges_for_clade_for_subtree_view(node_id)
        } else {
            self.bounding_edges_for_clade_tree(node_id)
        }
    }

    fn bounding_edges_for_clade_tree(
        &self,
        node_id: NodeId,
    ) -> Option<(Vec<Edge>, Vec<Edge>)> {
        self.tree().bounding_edges_for_clade(node_id)
    }

    pub(super) fn is_tip(&self, node_id: NodeId) -> bool {
        self.tree().is_tip(node_id)
    }

    pub(super) fn node_ids_srtd_asc(&self) -> Vec<NodeId> {
        if let Some(t) = &self.t_srtd_asc { t.node_ids_all() } else { vec![] }
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Subtree View --------------------------------------------------------

    pub(super) fn close_subtree_view(&mut self) {
        let current_found_node_id = self.current_found_node_id();
        self.subtree_view_node_id = None;
        self.subtree_view_tip_edge_idx_range = None;
        self.subtree_view_edges = None;
        self.subtree_view_cache_tip_count = None;
        self.subtree_view_cache_node_count = None;
        self.subtree_view_cache_max_first_node_to_tip_distance = None;
        self.subtree_view_edges_tip = Vec::new();
        self.subtree_view_edges_tip_tallest = Vec::new();
        self.subtree_view_sel_edge_idxs = Vec::new();
        self.clear_caches_of_edges_sorted_by_field();

        if let Some(search_query) = &self.search_query.clone()
            && let Some(tip_only_search) = self.tip_only_search
        {
            self.filter_nodes(search_query, tip_only_search);
        }

        self.update_filter_results(current_found_node_id);
    }

    pub(super) fn set_subtree_view(&mut self, node_id: NodeId) {
        let current_found_node_id = self.current_found_node_id();
        self.close_subtree_view();
        self.clear_caches_of_edges_sorted_by_field();
        self.clear_caches_cnv();

        let idx_range_opt =
            self.tree().bounding_tip_edge_index_range_for_clade(node_id);

        if let Some(idx_range) = idx_range_opt
            && let Some(edges_within_tip_index_range) =
                self.tree().edges_within_tip_index_range(idx_range.clone())
        {
            let subtree_node_ids = self.tree().descending_node_ids(node_id);

            let subtree_edge_indexes =
                subtree_node_ids.iter().filter_map(|&subtree_node_id| {
                    self.tree().edge_index_for_node_id(subtree_node_id)
                });

            let offset = *idx_range.clone().start();

            let mut subtree_edges: Vec<Edge> = subtree_edge_indexes
                .map(|edge_idx| {
                    edges_within_tip_index_range[edge_idx - offset].clone()
                })
                .collect();

            self.subtree_view_tip_edge_idx_range = Some(idx_range);

            let min_x = subtree_edges
                .par_iter()
                .map(|edge| edge.x0)
                .reduce(|| 1.0, TreeFloat::min);

            let max_x = subtree_edges
                .par_iter()
                .map(|edge| edge.x1)
                .reduce(|| 0.0, TreeFloat::max);

            let min_y = subtree_edges
                .par_iter()
                .map(|edge| edge.y)
                .reduce(|| 1.0, TreeFloat::min);

            let max_y = subtree_edges
                .par_iter()
                .map(|edge| edge.y)
                .reduce(|| 0.0, TreeFloat::max);

            subtree_edges.iter_mut().enumerate().for_each(|(i, edge)| {
                edge.edge_index = i;
                edge.x0 = normalize_value(min_x, max_x, edge.x0);
                edge.x_mid = normalize_value(min_x, max_x, edge.x_mid);
                edge.x1 = normalize_value(min_x, max_x, edge.x1);

                edge.y = normalize_value(min_y, max_y, edge.y);
                if let Some(y_parent) = edge.y_parent {
                    edge.y_parent =
                        Some(normalize_value(min_y, max_y, y_parent));
                }
            });

            self.subtree_view_edges = Some(subtree_edges);

            self.subtree_view_cache_tip_count =
                Some(self.tree().tip_node_count_recursive(node_id));

            self.subtree_view_cache_node_count =
                Some(self.tree().child_node_count_recursive(node_id));

            self.subtree_view_cache_max_first_node_to_tip_distance =
                Some(self.tree().max_node_to_tip_distance(node_id));

            self.subtree_view_node_id = Some(node_id);

            self.subtree_view_edges_tip =
                self.edges_tip_prep_for_subtree_view();

            self.subtree_view_edges_tip_tallest =
                self.edges_tip_tallest_prep_for_subtree_view();

            self.subtree_view_sel_edge_idxs =
                self.sel_edge_idxs_prep_for_subtree_view();

            self.update_filter_results(current_found_node_id);
        }
    }

    fn edges_tip_prep_for_subtree_view(&mut self) -> Vec<Edge> {
        let mut rv_tip = Vec::new();
        if let Some(edges) = self.edges_for_subtree_view() {
            for edge in edges {
                if edge.is_tip {
                    rv_tip.push(edge.clone());
                }
            }
        }
        rv_tip
    }

    fn sel_edge_idxs_prep_for_subtree_view(&self) -> Vec<usize> {
        let sel_node_ids = &self.sel_node_ids;
        let mut rv_edge_idx: Vec<usize> = Vec::new();
        if let Some(edges) = self.edges_for_subtree_view() {
            rv_edge_idx = edges
                .par_iter()
                .filter_map(|edge| {
                    if sel_node_ids.contains(&edge.node_id) {
                        Some(edge.edge_index)
                    } else {
                        None
                    }
                })
                .collect();
        }
        rv_edge_idx
    }

    fn edges_tip_tallest_prep_for_subtree_view(&self) -> Vec<Edge> {
        let n: i32 = 10;
        let mut rv: Vec<Edge> = Vec::new();
        let mut edges_tip = self.edges_tip_for_subtree_view().clone();
        let idx2 = edges_tip.len();
        let idx1: usize = 0.max(idx2 as i32 - n) as usize;
        // ---------------------------------------------------------------------
        edges_tip.sort_by(|a, b| a.x1.total_cmp(&b.x1));
        rv.append(&mut edges_tip[idx1..idx2].to_vec());
        // ---------------------------------------------------------------------
        edges_tip.sort_by(|a, b| {
            a.label
                .clone()
                .map(|name| name.len())
                .cmp(&b.label.clone().map(|name| name.len()))
        });
        rv.append(&mut edges_tip[idx1..idx2].to_vec());
        // ---------------------------------------------------------------------
        rv
    }

    fn sel_edge_idxs_for_subtree_view(&self) -> &Vec<usize> {
        &self.subtree_view_sel_edge_idxs
    }

    pub(super) fn is_subtree_view_active(&self) -> bool {
        self.subtree_view_node_id.is_some()
    }

    pub(super) fn is_valid_potential_subtree_view_node(
        &self,
        node_id: NodeId,
    ) -> bool {
        if let Some(subtree_view_node_id) = self.subtree_view_node_id()
            && node_id == subtree_view_node_id
        {
            false
        } else {
            self.tree().first_node_id().unwrap() != node_id
                && !self.is_tip(node_id)
        }
    }

    fn subtree_view_node_id(&self) -> Option<NodeId> {
        self.subtree_view_node_id
    }

    fn tip_count_for_subtree_view(&self) -> Option<usize> {
        self.subtree_view_cache_tip_count
    }

    fn node_count_for_subtree_view(&self) -> Option<usize> {
        self.subtree_view_cache_node_count
    }

    fn max_first_node_to_tip_distance_for_subtree_view(
        &self,
    ) -> Option<TreeFloat> {
        self.subtree_view_cache_max_first_node_to_tip_distance
    }

    fn bounding_edges_for_clade_for_subtree_view(
        &self,
        node_id: NodeId,
    ) -> Option<(Vec<Edge>, Vec<Edge>)> {
        let (node_ids_top, node_ids_bottom) =
            self.tree().bounding_node_ids_for_clade(node_id)?;

        let edges = self.edges_for_subtree_view()?;

        let mut edges_top: Vec<Edge> = Vec::new();
        let mut edges_bottom: Vec<Edge> = Vec::new();

        node_ids_top.iter().for_each(|&node_id_top| {
            edges.iter().for_each(|edge| {
                if edge.node_id == node_id_top {
                    edges_top.push(edge.clone());
                }
            });
        });

        node_ids_bottom.iter().for_each(|&node_id_bottom| {
            edges.iter().for_each(|edge| {
                if edge.node_id == node_id_bottom {
                    edges_bottom.push(edge.clone());
                }
            });
        });

        Some((edges_top, edges_bottom))
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Rooting -------------------------------------------------------------

    pub(super) fn is_valid_potential_outgroup_node(
        &self,
        node_id: NodeId,
    ) -> bool {
        self.tree().is_valid_potential_outgroup_node(node_id)
    }

    pub(super) fn root(&mut self, node_id: NodeId) -> Option<NodeId> {
        self.tmp_found_node_id = self.current_found_node_id();
        let mut tre = self.tree().clone();
        let rslt = tre.root(node_id);
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
        if let Ok(yanked_node) = tre.unroot() {
            if let Some(yanked_node_id) = yanked_node.node_id()
                && self.sel_node_ids.contains(&yanked_node_id)
            {
                self.deselect_node(yanked_node_id);
            }
            self.init(tre);
            Some(yanked_node)
        } else {
            None
        }
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Sorting -------------------------------------------------------------

    pub(super) fn sort(&mut self, node_ord_opt: TreNodeOrd) {
        let current_found_node_id;
        if let Some(node_id) = self.tmp_found_node_id {
            current_found_node_id = Some(node_id);
        } else {
            current_found_node_id = self.current_found_node_id();
        }
        self.tmp_found_node_id = None;
        self.node_ord_opt = node_ord_opt;

        match node_ord_opt {
            TreNodeOrd::Unordered => {}
            TreNodeOrd::Ascending => self.sort_asc(),
            TreNodeOrd::Descending => self.sort_desc(),
        };

        self.edges_tip = self.edges_tip_prep_tree();
        self.edges_tip_tallest = self.edges_tip_tallest_prep_tree();
        self.edge_root = self.edge_root_prep();
        self.update_filter_results(current_found_node_id);
        self.sel_edge_idxs = self.sel_edge_idxs_prep_tree();
        // ---------------------------------------------------------------------
        // Drop clade label for nodes that may not exist anymore.
        let mut node_ids_to_drop: Vec<NodeId> = Vec::new();
        for &node_id in self.labeled_clades.keys() {
            if !self.tree().node_exists(Some(node_id)) {
                node_ids_to_drop.push(node_id);
            }
        }
        for node_id in node_ids_to_drop {
            self.remove_clade_label(node_id);
        }
        // ---------------------------------------------------------------------
        self.clear_caches_of_edges_sorted_by_field();
        self.clear_caches_cnv();

        if let Some(subtree_view_node_id) = self.subtree_view_node_id() {
            self.set_subtree_view(subtree_view_node_id);
            self.update_filter_results(current_found_node_id);
        }
    }

    fn sort_asc(&mut self) {
        if self.t_srtd_asc.is_none() {
            self.t_srtd_asc = Some(Tree::sorted_clone(&self.t_orig, false));
        }
    }

    fn sort_desc(&mut self) {
        if self.t_srtd_desc.is_none() {
            self.t_srtd_desc = Some(Tree::sorted_clone(&self.t_orig, true));
        }
    }

    fn update_filter_results(&mut self, current_found_node_id: Option<NodeId>) {
        let found_node_ids_old = self.found_node_ids.to_owned();
        self.found_node_ids.clear();
        self.found_edge_idxs.clear();
        self.vec_idx_to_found_edge_idxs = 0;
        if let Some(current_found_node_id) = current_found_node_id
            && let Some(edges) = self.edges()
        {
            let mut idx: usize = 0;
            let mut found_node_ids: HashSet<NodeId> = HashSet::new();
            let mut found_edge_idxs: Vec<usize> = Vec::new();
            let mut vec_idx_to_found_edge_idxs: usize = idx;
            for e in edges {
                if found_node_ids_old.contains(&e.node_id) {
                    _ = found_node_ids.insert(e.node_id);
                    found_edge_idxs.push(e.edge_index);
                    if e.node_id == current_found_node_id {
                        vec_idx_to_found_edge_idxs = idx;
                    }
                    idx += 1;
                }
            }
            self.found_node_ids = found_node_ids;
            self.found_edge_idxs = found_edge_idxs;
            self.vec_idx_to_found_edge_idxs = vec_idx_to_found_edge_idxs;
        }
    }

    fn edges_tip_prep_tree(&mut self) -> Vec<Edge> {
        let mut rv_tip = Vec::new();
        if let Some(edges) = self.edges_for_tree() {
            for edge in edges {
                if edge.is_tip {
                    rv_tip.push(edge.clone());
                }
            }
        }
        rv_tip
    }

    fn edges_tip_tallest_prep_tree(&self) -> Vec<Edge> {
        let n: i32 = 10;
        let mut rv: Vec<Edge> = Vec::new();
        let mut edges_tip = self.edges_tip_tree().clone();
        let idx2 = edges_tip.len();
        let idx1: usize = 0.max(idx2 as i32 - n) as usize;
        // ---------------------------------------------------------------------
        edges_tip.sort_by(|a, b| a.x1.total_cmp(&b.x1));
        rv.append(&mut edges_tip[idx1..idx2].to_vec());
        // ---------------------------------------------------------------------
        edges_tip.sort_by(|a, b| {
            a.label
                .clone()
                .map(|name| name.len())
                .cmp(&b.label.clone().map(|name| name.len()))
        });
        rv.append(&mut edges_tip[idx1..idx2].to_vec());
        // ---------------------------------------------------------------------
        rv
    }

    fn edge_root_prep(&mut self) -> Option<Edge> {
        if self.is_rooted()
            && let Some(edges) = self.edges_for_tree()
        {
            Some(
                edges
                    .iter()
                    .find(|j| j.parent_node_id.is_none())
                    .expect("Should have root!")
                    .clone(),
            )
        } else {
            None
        }
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Selection -----------------------------------------------------------

    pub(super) fn sel_node_ids(&self) -> &HashSet<NodeId> {
        &self.sel_node_ids
    }

    pub(super) fn sel_edge_idxs(&self) -> &Vec<usize> {
        if self.is_subtree_view_active() {
            self.sel_edge_idxs_for_subtree_view()
        } else {
            self.sel_edge_idxs_tree()
        }
    }

    fn sel_edge_idxs_tree(&self) -> &Vec<usize> {
        &self.sel_edge_idxs
    }

    pub(super) fn select_deselect_node(&mut self, node_id: NodeId) {
        if self.sel_node_ids.contains(&node_id) {
            self.deselect_node(node_id);
        } else {
            self.select_node(node_id);
        }
    }

    pub(super) fn select_node(&mut self, node_id: NodeId) {
        _ = self.sel_node_ids.insert(node_id);
        self.sel_edge_idxs = self.sel_edge_idxs_prep_tree();
        self.clear_cache_cnv_sel_nodes();
        self.cache_edges_selected_asc = None;
        self.cache_edges_selected_desc = None;

        if self.is_subtree_view_active() {
            self.subtree_view_sel_edge_idxs =
                self.sel_edge_idxs_prep_for_subtree_view();
        }
    }

    pub(super) fn deselect_node(&mut self, node_id: NodeId) {
        _ = self.sel_node_ids.remove(&node_id);
        self.sel_edge_idxs = self.sel_edge_idxs_prep_tree();
        self.clear_cache_cnv_sel_nodes();
        self.cache_edges_selected_asc = None;
        self.cache_edges_selected_desc = None;

        if self.is_subtree_view_active() {
            self.subtree_view_sel_edge_idxs =
                self.sel_edge_idxs_prep_for_subtree_view();
        }
    }

    pub(super) fn select_deselect_node_exclusive(&mut self, node_id: NodeId) {
        let selected = self.sel_node_ids.clone();
        let n_selected = selected.len();
        selected.iter().for_each(|id| {
            if *id != node_id {
                _ = self.sel_node_ids.remove(id);
            }
        });
        self.select_deselect_node(node_id);
        if n_selected > 1 {
            self.select_node(node_id);
        }
    }

    fn sel_edge_idxs_prep_tree(&self) -> Vec<usize> {
        let sel_node_ids = &self.sel_node_ids;
        let mut rv_edge_idx: Vec<usize> = Vec::new();
        if let Some(edges) = self.edges_for_tree() {
            rv_edge_idx = edges
                .par_iter()
                .filter_map(|edge| {
                    if sel_node_ids.contains(&edge.node_id) {
                        Some(edge.edge_index)
                    } else {
                        None
                    }
                })
                .collect();
        }
        rv_edge_idx
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Search & Filter -----------------------------------------------------

    pub(super) fn found_edge_idxs(&self) -> &Vec<usize> {
        &self.found_edge_idxs
    }

    pub(super) fn found_node_ids(&self) -> &HashSet<NodeId> {
        &self.found_node_ids
    }

    pub(super) fn found_edge_idx(&self) -> usize {
        self.vec_idx_to_found_edge_idxs
    }

    pub(super) fn current_found_edge(&self) -> Option<Edge> {
        if !self.found_edge_idxs.is_empty()
            && let Some(edges) = self.edges()
        {
            Some(
                edges[self.found_edge_idxs[self.vec_idx_to_found_edge_idxs]]
                    .clone(),
            )
        } else {
            None
        }
    }

    pub(super) fn current_found_node_id(&self) -> Option<NodeId> {
        self.current_found_edge().map(|e| e.node_id)
    }

    pub(super) fn prev_result(&mut self) {
        self.clear_cache_cnv_filtered_nodes();
        if self.vec_idx_to_found_edge_idxs > 0 {
            self.vec_idx_to_found_edge_idxs -= 1;
        }
    }

    pub(super) fn next_result(&mut self) {
        self.clear_cache_cnv_filtered_nodes();
        if self.vec_idx_to_found_edge_idxs < self.found_edge_idxs.len() - 1 {
            self.vec_idx_to_found_edge_idxs += 1;
        }
    }

    pub(super) fn add_found_to_sel(&mut self) {
        let max_capacity = self.sel_node_ids.len() + self.found_node_ids.len();
        let mut sel_node_ids: HashSet<NodeId> =
            HashSet::with_capacity(max_capacity);
        self.sel_node_ids.union(&self.found_node_ids).for_each(|id| {
            _ = sel_node_ids.insert(*id);
        });
        self.sel_node_ids = sel_node_ids;
        self.sel_edge_idxs = self.sel_edge_idxs_prep_tree();
        self.clear_cache_cnv_sel_nodes();
    }

    pub(super) fn rem_found_from_sel(&mut self) {
        let mut sel_node_ids: HashSet<NodeId> =
            HashSet::with_capacity(self.sel_node_ids.len());
        self.sel_node_ids.difference(&self.found_node_ids).for_each(|id| {
            _ = sel_node_ids.insert(*id);
        });
        self.sel_node_ids = sel_node_ids;
        self.sel_edge_idxs = self.sel_edge_idxs_prep_tree();
        self.clear_cache_cnv_sel_nodes();
    }

    pub(super) fn clear_filter_results(&mut self) {
        self.found_node_ids.clear();
        self.found_edge_idxs.clear();
        self.clear_cache_cnv_filtered_nodes();
        self.vec_idx_to_found_edge_idxs = 0;
    }

    pub(super) fn filter_nodes(&mut self, query: &str, tips_only: bool) {
        self.clear_filter_results();

        if query.is_empty() {
            self.search_query = None;
            self.tip_only_search = None;
            return;
        };

        self.search_query = Some(query.to_string());
        self.tip_only_search = Some(tips_only);

        let edges_to_search = match tips_only {
            true => self.edges_tip(),
            false => {
                if let Some(edges) = self.edges() {
                    edges
                } else {
                    return;
                }
            }
        };

        let mut found_node_ids: HashSet<NodeId> = HashSet::new();
        let mut found_edge_idxs: Vec<usize> = Vec::new();

        for e in edges_to_search {
            if let Some(n) = &e.label
                && let Some(_) = n.to_lowercase().find(&query.to_lowercase())
            {
                _ = found_node_ids.insert(e.node_id);
                found_edge_idxs.push(e.edge_index);
            }
        }

        self.found_node_ids = found_node_ids;
        self.found_edge_idxs = found_edge_idxs;
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Clade Labels --------------------------------------------------------

    pub(super) fn add_remove_clade_label(
        &mut self,
        node_id: NodeId,
        color: Color,
        label: impl Into<String>,
        label_type: CladeLabelType,
    ) {
        if self.clade_has_label(node_id) {
            self.remove_clade_label(node_id);
        } else {
            self.add_clade_label(node_id, color, label, label_type);
        }
    }

    pub(super) fn add_clade_label(
        &mut self,
        node_id: NodeId,
        color: Color,
        label: impl Into<String>,
        label_type: CladeLabelType,
    ) {
        let clade_label: CladeLabel =
            CladeLabel { node_id, color, label: label.into(), label_type };
        _ = self.labeled_clades.insert(node_id, clade_label);
    }

    pub(super) fn remove_clade_label(&mut self, node_id: NodeId) {
        _ = self.labeled_clades.remove(&node_id);
    }

    pub(super) fn clade_has_label(&self, node_id: NodeId) -> bool {
        self.labeled_clades.contains_key(&node_id)
    }

    pub(super) fn labeled_clades(&self) -> &HashMap<NodeId, CladeLabel> {
        &self.labeled_clades
    }

    pub(super) fn has_clade_labels(&self) -> bool {
        !self.labeled_clades().is_empty()
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Caches of Edges Sorted by the Fields in the Edge Struct -------------

    pub(super) fn edges_sorted_by_field(
        &self,
        sort_column: EdgeSortField,
        sort_direction: SortOrd,
    ) -> Option<&Vec<Edge>> {
        match sort_direction {
            SortOrd::Ascending => match sort_column {
                EdgeSortField::NodeId => self.cache_edges_node_id_asc.as_ref(),
                EdgeSortField::NodeLabel => {
                    self.cache_edges_node_label_asc.as_ref()
                }
                EdgeSortField::Selected => {
                    self.cache_edges_selected_asc.as_ref()
                }
            },

            SortOrd::Descending => match sort_column {
                EdgeSortField::NodeId => self.cache_edges_node_id_desc.as_ref(),
                EdgeSortField::NodeLabel => {
                    self.cache_edges_node_label_desc.as_ref()
                }
                EdgeSortField::Selected => {
                    self.cache_edges_selected_desc.as_ref()
                }
            },
        }
    }

    pub(super) fn populate_cache_of_edges_sorted_by_field(
        &mut self,
        sort_column: EdgeSortField,
        sort_direction: SortOrd,
    ) {
        let needs_population = match sort_direction {
            SortOrd::Ascending => match sort_column {
                EdgeSortField::NodeId => self.cache_edges_node_id_asc.is_none(),
                EdgeSortField::NodeLabel => {
                    self.cache_edges_node_label_asc.is_none()
                }
                EdgeSortField::Selected => {
                    self.cache_edges_selected_asc.is_none()
                }
            },

            SortOrd::Descending => match sort_column {
                EdgeSortField::NodeId => {
                    self.cache_edges_node_id_desc.is_none()
                }
                EdgeSortField::NodeLabel => {
                    self.cache_edges_node_label_desc.is_none()
                }
                EdgeSortField::Selected => {
                    self.cache_edges_selected_desc.is_none()
                }
            },
        };

        if needs_population && let Some(edges) = self.edges() {
            let mut sorted_edges = edges.clone();
            let sel_node_ids = &self.sel_node_ids;

            match sort_column {
                EdgeSortField::NodeId => {
                    sorted_edges.sort_by(|a, b| {
                        let ord = a.node_id.cmp(&b.node_id);
                        match sort_direction {
                            SortOrd::Ascending => ord,
                            SortOrd::Descending => ord.reverse(),
                        }
                    });
                }
                EdgeSortField::NodeLabel => {
                    sorted_edges.sort_by(|a, b| {
                        let label_a = a.label.as_deref().unwrap_or("");
                        let label_b = b.label.as_deref().unwrap_or("");
                        let ord = label_a.cmp(label_b);
                        match sort_direction {
                            SortOrd::Ascending => ord,
                            SortOrd::Descending => ord.reverse(),
                        }
                    });
                }
                EdgeSortField::Selected => {
                    sorted_edges.sort_by(|a, b| {
                        let sel_a = sel_node_ids.contains(&a.node_id);
                        let sel_b = sel_node_ids.contains(&b.node_id);
                        let ord = sel_a.cmp(&sel_b);
                        match sort_direction {
                            SortOrd::Ascending => ord,
                            SortOrd::Descending => ord.reverse(),
                        }
                    });
                }
            }

            match (sort_column, sort_direction) {
                (EdgeSortField::NodeId, SortOrd::Ascending) => {
                    self.cache_edges_node_id_asc = Some(sorted_edges);
                }
                (EdgeSortField::NodeId, SortOrd::Descending) => {
                    self.cache_edges_node_id_desc = Some(sorted_edges);
                }
                (EdgeSortField::NodeLabel, SortOrd::Ascending) => {
                    self.cache_edges_node_label_asc = Some(sorted_edges);
                }
                (EdgeSortField::NodeLabel, SortOrd::Descending) => {
                    self.cache_edges_node_label_desc = Some(sorted_edges);
                }
                (EdgeSortField::Selected, SortOrd::Ascending) => {
                    self.cache_edges_selected_asc = Some(sorted_edges);
                }
                (EdgeSortField::Selected, SortOrd::Descending) => {
                    self.cache_edges_selected_desc = Some(sorted_edges);
                }
            }
        }
    }

    fn clear_caches_of_edges_sorted_by_field(&mut self) {
        self.cache_edges_node_id_asc = None;
        self.cache_edges_node_id_desc = None;
        self.cache_edges_node_label_asc = None;
        self.cache_edges_node_label_desc = None;
        self.cache_edges_selected_asc = None;
        self.cache_edges_selected_desc = None;
    }

    // -------------------------------------------------------------------------

    // =========================================================================

    // --- Cached Geometries ---------------------------------------------------

    pub(super) fn cache_cnv_edge(&self) -> &CnvCache {
        &self.cache_cnv_edge
    }

    pub(super) fn cache_cnv_lab_tip(&self) -> &CnvCache {
        &self.cache_cnv_lab_tip
    }

    pub(super) fn cache_cnv_lab_int(&self) -> &CnvCache {
        &self.cache_cnv_lab_int
    }

    pub(super) fn cache_cnv_lab_brnch(&self) -> &CnvCache {
        &self.cache_cnv_lab_brnch
    }

    pub(super) fn cache_cnv_sel_nodes(&self) -> &CnvCache {
        &self.cache_cnv_sel_nodes
    }

    pub(super) fn cache_cnv_filtered_nodes(&self) -> &CnvCache {
        &self.cache_cnv_filtered_nodes
    }

    pub(super) fn cache_cnv_clade_labels(&self) -> &CnvCache {
        &self.cache_cnv_clade_labels
    }

    // -------------------------------------------------------------------------

    pub(super) fn clear_cache_cnv_edge(&self) {
        self.cache_cnv_edge.clear();
    }

    pub(super) fn clear_cache_cnv_lab_tip(&self) {
        self.cache_cnv_lab_tip.clear();
    }

    pub(super) fn clear_cache_cnv_lab_int(&self) {
        self.cache_cnv_lab_int.clear();
    }

    pub(super) fn clear_cache_cnv_lab_brnch(&self) {
        self.cache_cnv_lab_brnch.clear();
    }

    pub(super) fn clear_cache_cnv_sel_nodes(&self) {
        self.cache_cnv_sel_nodes.clear();
    }

    pub(super) fn clear_cache_cnv_filtered_nodes(&self) {
        self.cache_cnv_filtered_nodes.clear();
    }

    pub(super) fn clear_cache_cnv_clade_labels(&self) {
        self.cache_cnv_clade_labels.clear();
    }

    pub(super) fn clear_caches_cnv(&self) {
        self.clear_cache_cnv_edge();
        self.clear_cache_cnv_lab_tip();
        self.clear_cache_cnv_lab_int();
        self.clear_cache_cnv_lab_brnch();
        self.clear_cache_cnv_sel_nodes();
        self.clear_cache_cnv_filtered_nodes();
        self.clear_cache_cnv_clade_labels();
    }

    // -------------------------------------------------------------------------

    // =========================================================================
}
