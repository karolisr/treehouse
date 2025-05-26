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

pub(crate) fn update_rects(&mut self) {
    self.clip_rect = Rectangle {
        x: 0e0,
        y: 0e0,
        width: self.tre_cnv_w - SCROLL_TOOL_W + PADDING,
        height: self.tre_cnv_h,
    };
    self.tree_rect = match self.sel_tree_style_opt {
        TreeStyle::Phylogram => Rectangle {
            x: self.clip_rect.x + SF / 2e0 + PADDING,
            y: self.clip_rect.y + SF / 2e0 + self.max_lab_size + self.brnch_lab_offset_y,
            width: self.clip_rect.width - SF - PADDING * 2e0 - self.tip_lab_w,
            height: self.clip_rect.height - SF - self.max_lab_size * 1.5 - SCROLL_TOOL_W,
        },
        TreeStyle::Fan => Rectangle {
            x: self.clip_rect.x + SF / 2e0 + self.tip_lab_w,
            y: self.clip_rect.y + SF / 2e0 + self.tip_lab_w + PADDING,
            width: self.clip_rect.width - SF - self.tip_lab_w * 2e0,
            height: self.clip_rect.height
                - SF
                - self.tip_lab_w * 2e0
                - SCROLL_TOOL_W
                - PADDING * 2e0,
        },
    };
    match self.sel_tree_style_opt {
        TreeStyle::Phylogram => {
            self.ltt.ltt_rect_x = self.clip_rect.x + SF / 2e0 + PADDING;
            self.ltt.ltt_rect_w = self.clip_rect.width - SF - PADDING * 2e0;
        }
        TreeStyle::Fan => {
            self.ltt.ltt_rect_x = self.clip_rect.x + SF / 2e0 + PADDING;
            self.ltt.ltt_rect_w = self.tree_scroll_w - SCROLL_TOOL_W + PADDING - SF - PADDING * 2e0;
        }
    };
}

pub(crate) fn update_canvas_w(&mut self) {
    self.min_tre_cnv_w = self.tree_scroll_w;
    self.tre_cnv_w = self.min_tre_cnv_w + (self.sel_tre_cnv_w_idx - 1) as Float * 1e2 * SF;
    if self.sel_tree_style_opt == TreeStyle::Phylogram {
        self.ltt_cnv_w = self.tre_cnv_w;
    } else {
        self.ltt_cnv_w = self.min_tre_cnv_w;
    }
}

pub(crate) fn update_tip_label_w(&mut self) {
    if self.tip_brnch_labs_allowed && self.has_tip_labs && self.draw_tip_labs {
        self.tip_lab_w = self.extra_space_for_tip_labs + self.tip_lab_offset_x;
        let max_tip_label_w = self.tre_cnv_w / 1.5;
        if self.tip_lab_w > max_tip_label_w {
            self.tip_lab_w = max_tip_label_w;
        }
    } else {
        self.tip_lab_w = 0e0;
    }
}
