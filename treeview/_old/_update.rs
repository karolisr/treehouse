impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::Search(s) => {
                self.search_string = s;
                self.filter_nodes();
                self.update_found_node_point();
                if let Some(pt) = self.found_edge_pt {
                    Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::TipOnlySearchSelectionChanged(state) => {
                self.tip_only_search = state;
                Task::done(TreeViewMsg::Search(self.search_string.clone()))
            }

            TreeViewMsg::PrevResult => {
                self.found_edge_idx -= 1;
                self.update_found_node_point();
                if let Some(pt) = self.found_edge_pt {
                    Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::NextResult => {
                self.found_edge_idx += 1;
                self.update_found_node_point();
                if let Some(pt) = self.found_edge_pt {
                    Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::AddFoundToSelection => {
                for node_id in &self.found_node_ids {
                    self.sel_node_ids.insert(*node_id);
                }
                Task::none()
            }

            TreeViewMsg::RemFoundFromSelection => {
                for node_id in &self.found_node_ids {
                    self.sel_node_ids.remove(node_id);
                }
                Task::none()
            }
        }
    }
}

pub(crate) fn update_found_node_point(&mut self) {
    if self.found_edges.is_empty() {
        self.found_edge_pt = None;
        return;
    }
    let edge = &self.found_edges[self.found_edge_idx];
    self.found_edge_pt = Some(match self.sel_tree_style_opt {
        TreeStyle::Phylogram => node_point(self.tree_rect.width, self.tree_rect.height, edge),
        TreeStyle::Fan => {
            let angle = edge_angle(self.rot_angle, self.opn_angle, edge);
            node_point_rad(angle, self.center, self.size, edge)
        }
    });
}
