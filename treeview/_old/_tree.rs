#[derive(Default)]
pub(crate) struct TreeState {
    tip_idx_range: Option<IndexRange>,
    found_edge_pt: Option<Point>,
    tip_only_search: bool,
    search_string: String,
    found_edge_idx: usize,
    found_edges: Edges,
    found_node_ids: HashSet<NodeId>,
}

impl TreeState {
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
