impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::PaneDragged(drag_event) => {
                if let Some(pgs) = &mut self.panes {
                    match drag_event {
                        DragEvent::Picked { pane: _pane_idx } => Task::none(),
                        DragEvent::Dropped { pane: pane_idx, target } => {
                            pgs.drop(pane_idx, target);
                            Task::none()
                        }
                        DragEvent::Canceled { pane: _pane_idx } => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::CursorOnTreCnv { x } => {
                if let Some(tree) = self.tree_mut() {
                    tree.tre_cnv.cursor_x_fraction = None;
                    tree.ltt_cnv.cursor_x_fraction = x;
                }
                Task::none()
            }

            TreeViewMsg::CursorOnLttCnv { x } => {
                if let Some(tree) = self.tree_mut() {
                    tree.tre_cnv.cursor_x_fraction = x;
                    tree.ltt_cnv.cursor_x_fraction = x;
                }
                Task::none()
            }

            TreeViewMsg::LegendVisibilityChanged(draw_legend) => {
                self.draw_legend = draw_legend;
                Task::none()
            }

            TreeViewMsg::CursorLineVisibilityChanged(show_cursor_line) => {
                self.show_cursor_line = show_cursor_line;
                if let Some(tree) = self.tree_mut() {
                    tree.tre_cnv.show_cursor_line = show_cursor_line;
                }
                Task::none()
            }

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
