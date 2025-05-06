use crate::{TreeState, TreeStateMsg, TreeStyle, TreeView, TreeViewMsg};
use iced::Task;

impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        // println!("TreeView -> {msg:?}");
        match msg {
            TreeViewMsg::TreeStateMsg(tree_state_msg) => {
                if let Some(idx) = self.sel_tree_idx {
                    let sel_tree = &mut self.trees[idx];
                    sel_tree.update(tree_state_msg)
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::PaneGridMsg(pane_grid_msg) => {
                self.pane_grid_main.update(pane_grid_msg);
                Task::none()
            }

            TreeViewMsg::SetSidebarLocation(position) => {
                self.sidebar_position = position;
                Task::none()
            }

            TreeViewMsg::TreeLoaded(tree) => {
                self.trees.push(TreeState::default());
                self.sel_tree_idx = Some(self.trees.len() - 1);
                Task::done(TreeViewMsg::TreeStateMsg(TreeStateMsg::Init(
                    tree, self.sel_node_ord_opt,
                )))
            }

            TreeViewMsg::NodeOrdOptChanged(node_ord_opt) => {
                if node_ord_opt != self.sel_node_ord_opt {
                    self.sel_node_ord_opt = node_ord_opt;
                    Task::done(TreeViewMsg::TreeStateMsg(TreeStateMsg::Sort(node_ord_opt)))
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::TreeUpdated => {
                // self.ltt.set_data(ltt(&self.tree_edges, self.ltt_bins));
                // self.update_extra_space_for_labels();
                // self.update_node_size();
                // self.update_tip_label_w();
                // self.update_canvas_h();
                // self.update_rects();
                // self.update_visible_nodes();
                // self.update_found_node_point();
                // if !self.drawing_enabled { Task::done(TreeViewMsg::Init) } else { Task::none() }
                // self.update_visible_nodes();
                // self.update_found_node_point();
                Task::none()
            }

            // ------------------------------------------------------------------------------------

            // TreeViewMsg::Init => Task::batch([
            //     Task::done(TreeViewMsg::TipLabelSizeSelectionChanged(self.sel_tip_lab_size_idx)),
            //     Task::done(TreeViewMsg::IntLabelSizeSelectionChanged(self.sel_int_lab_size_idx)),
            //     Task::done(TreeViewMsg::BranchLabelSizeSelectionChanged(
            //         self.sel_brnch_lab_size_idx,
            //     )),
            //     Task::done(TreeViewMsg::OpnAngleSelectionChanged(self.sel_opn_angle_idx)),
            //     Task::done(TreeViewMsg::RotAngleSelectionChanged(self.sel_rot_angle_idx)),
            // ])
            // .chain(if let Some(id) = self.win_id {
            //     iced::window::get_size(id)
            //         .map(|s| TreeViewMsg::WindowResized(s.width * SF, s.height * SF))
            // } else {
            //     Task::none()
            // })
            // .chain(Task::done(TreeViewMsg::EnableDrawing)),

            // ------------------------------------------------------------------------------------
            TreeViewMsg::ScrollTo { x, y } => iced::widget::scrollable::scroll_to(
                "tre",
                match self.sel_tree_style_opt {
                    TreeStyle::Phylogram => iced::widget::scrollable::AbsoluteOffset {
                        x: x - self.tree_scroll_w / 2e0,
                        y: y - self.tree_scroll_h / 2e0,
                    },
                    TreeStyle::Fan => iced::widget::scrollable::AbsoluteOffset {
                        x: x - self.tree_scroll_w / 2e0, // + self.tip_lab_w
                        y: y - self.tree_scroll_h / 2e0, // + self.tip_lab_w
                    },
                },
            ),

            TreeViewMsg::ScrollToX { sender, x } => {
                if self.sel_tree_style_opt == TreeStyle::Phylogram {
                    match sender {
                        "tre" => {
                            self.tre_cnv_scrolled = true;
                            self.ltt_cnv_scrolled = false;
                            iced::widget::scrollable::scroll_to(
                                "ltt",
                                iced::widget::scrollable::AbsoluteOffset { x, y: self.ltt_cnv_y0 },
                            )
                        }
                        "ltt" => {
                            self.ltt_cnv_scrolled = true;
                            self.tre_cnv_scrolled = false;
                            iced::widget::scrollable::scroll_to(
                                "tre",
                                iced::widget::scrollable::AbsoluteOffset { x, y: self.tre_cnv_y0 },
                            )
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::TreCnvScrolled(vp) => {
                self.tre_cnv_x0 = vp.absolute_offset().x;
                self.tre_cnv_y0 = vp.absolute_offset().y;
                self.tre_cnv_y1 = self.tre_cnv_y0 + vp.bounds().height;
                // self.update_visible_nodes();
                if self.tre_cnv_scrolled && self.tre_cnv_x0 != self.ltt_cnv_x0 {
                    Task::done(TreeViewMsg::ScrollToX { sender: "tre", x: self.tre_cnv_x0 })
                } else {
                    self.tre_cnv_scrolled = true;
                    Task::none()
                }
            }

            TreeViewMsg::LttCnvScrolled(vp) => {
                self.ltt_cnv_x0 = vp.absolute_offset().x;
                self.ltt_cnv_y0 = vp.absolute_offset().y;
                if self.ltt_cnv_scrolled && self.tre_cnv_x0 != self.ltt_cnv_x0 {
                    Task::done(TreeViewMsg::ScrollToX { sender: "ltt", x: self.ltt_cnv_x0 })
                } else {
                    self.ltt_cnv_scrolled = true;
                    Task::none()
                }
            }

            // ------------------------------------------------------------------------------------

            // TreeViewMsg::CursorOnTreCnv { x } => {
            //     self.cursor_x_fraction = None;
            //     self.ltt.cursor_x_fraction = x;
            //     Task::none()
            // }

            // TreeViewMsg::CursorOnLttCnv { x } => {
            //     self.cursor_x_fraction = x;
            //     self.ltt.cursor_x_fraction = x;
            //     Task::none()
            // }

            // ------------------------------------------------------------------------------------

            // TreeViewMsg::SelectDeselectNode(node_id) => {
            //     if self.sel_node_ids.contains(&node_id) {
            //         Task::done(TreeViewMsg::DeselectNode(node_id))
            //     } else {
            //         Task::done(TreeViewMsg::SelectNode(node_id))
            //     }
            // }

            // TreeViewMsg::SelectNode(node_id) => {
            //     self.sel_node_ids.insert(node_id);
            //     Task::none()
            // }

            // TreeViewMsg::DeselectNode(node_id) => {
            //     self.sel_node_ids.remove(&node_id);
            //     Task::none()
            // }

            // ------------------------------------------------------------------------------------

            // TreeViewMsg::Search(s) => {
            //     self.search_string = s;
            //     self.filter_nodes();
            //     self.update_found_node_point();
            //     if let Some(pt) = self.found_edge_pt {
            //         Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
            //     } else {
            //         Task::none()
            //     }
            // }

            // TreeViewMsg::TipOnlySearchSelectionChanged(state) => {
            //     self.tip_only_search = state;
            //     Task::done(TreeViewMsg::Search(self.search_string.clone()))
            // }

            // TreeViewMsg::PrevResult => {
            //     self.found_edge_idx -= 1;
            //     self.update_found_node_point();
            //     if let Some(pt) = self.found_edge_pt {
            //         Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
            //     } else {
            //         Task::none()
            //     }
            // }

            // TreeViewMsg::NextResult => {
            //     self.found_edge_idx += 1;
            //     self.update_found_node_point();
            //     if let Some(pt) = self.found_edge_pt {
            //         Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
            //     } else {
            //         Task::none()
            //     }
            // }

            // TreeViewMsg::AddFoundToSelection => {
            //     for node_id in &self.found_node_ids {
            //         self.sel_node_ids.insert(*node_id);
            //     }
            //     Task::none()
            // }

            // TreeViewMsg::RemFoundFromSelection => {
            //     for node_id in &self.found_node_ids {
            //         self.sel_node_ids.remove(node_id);
            //     }
            //     Task::none()
            // }

            // ------------------------------------------------------------------------------------
            TreeViewMsg::TreeStyleOptionChanged(tree_repr_option) => {
                self.sel_tree_style_opt = tree_repr_option;
                // self.update_node_size();
                // self.update_tip_label_w();
                // self.update_canvas_w();
                // self.update_canvas_h();
                // self.update_rects();
                // self.update_visible_nodes();
                // self.update_found_node_point();

                Task::none()
            }

            TreeViewMsg::OpnAngleSelectionChanged(idx) => {
                self.sel_opn_angle_idx = idx;
                // self.opn_angle = idx as Float / 360e0 * 2e0 * PI;
                // self.update_visible_nodes();
                // self.update_found_node_point();
                Task::none()
            }

            TreeViewMsg::RotAngleSelectionChanged(idx) => {
                self.sel_rot_angle_idx = idx;
                // self.rot_angle = idx as Float / 360e0 * 2e0 * PI;
                // self.update_visible_nodes();
                // self.update_found_node_point();
                Task::none()
            }

            TreeViewMsg::NodeSizeSelectionChanged(idx) => {
                self.sel_node_size_idx = idx;
                // self.update_node_size();
                // self.update_tip_label_w();
                // self.update_canvas_h();
                // self.update_rects();
                // self.update_visible_nodes();
                // self.update_found_node_point();
                Task::none()
            }

            TreeViewMsg::BranchLabelVisibilityChanged(state) => {
                self.draw_brnch_labs = state;
                Task::none()
            }

            TreeViewMsg::BranchLabelSizeSelectionChanged(idx) => {
                self.sel_brnch_lab_size_idx = idx;
                // self.brnch_lab_size = self.min_lab_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
                self.draw_tip_labs = state;
                // if self.drawing_enabled && self.tip_brnch_labs_allowed && self.draw_tip_labs {
                //     self.update_extra_space_for_labels();
                // }
                // self.update_node_size();
                // self.update_tip_label_w();
                // self.update_canvas_h();
                // self.update_rects();
                // self.update_visible_nodes();
                // self.update_found_node_point();
                Task::none()
            }

            TreeViewMsg::TipLabelSizeSelectionChanged(idx) => {
                self.sel_tip_lab_size_idx = idx;
                // self.tip_lab_size = self.min_lab_size * idx as Float;
                // self.update_extra_space_for_labels();
                // self.update_node_size();
                // self.update_tip_label_w();
                // self.update_canvas_h();
                // self.update_rects();
                // self.update_visible_nodes();
                // self.update_found_node_point();
                Task::none()
            }

            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labs = state;
                Task::none()
            }

            TreeViewMsg::IntLabelSizeSelectionChanged(idx) => {
                self.sel_int_lab_size_idx = idx;
                // self.int_lab_size = self.min_lab_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::LegendVisibilityChanged(state) => {
                self.draw_legend = state;
                Task::none()
            }

            TreeViewMsg::LttPlotVisibilityChanged(state) => {
                self.show_ltt = state;
                // self.update_node_size();
                // self.update_tip_label_w();
                // self.update_canvas_h();
                // self.update_rects();
                // self.update_visible_nodes();
                // self.update_found_node_point();
                // Task::done(TreeViewMsg::ScrollToX { sender: "tre", x: self.tre_cnv_x0 })
                Task::none()
            }

            TreeViewMsg::CursorLineVisibilityChanged(state) => {
                self.show_cursor_line = state;
                Task::none()
            }

            TreeViewMsg::CanvasWidthSelectionChanged(idx) => {
                self.sel_tre_cnv_w_idx = idx;
                // self.update_canvas_w();
                // if self.sel_tree_style_opt == TreeStyle::Fan {
                //     self.update_canvas_h();
                // }
                // if self.tip_brnch_labs_allowed && self.draw_tip_labs {
                //     self.update_extra_space_for_labels();
                // }
                // self.update_tip_label_w();
                // self.update_rects();
                // self.update_visible_nodes();
                // self.update_found_node_point();
                Task::none()
            }
        }
    }
}

impl TreeView {
    // pub(crate) fn update_found_node_point(&mut self) {
    //     if self.found_edges.is_empty() {
    //         self.found_edge_pt = None;
    //         return;
    //     }
    //     let edge = &self.found_edges[self.found_edge_idx];
    //     self.found_edge_pt = Some(match self.sel_tree_style_opt {
    //         TreeStyle::Phylogram => node_point(self.tree_rect.width, self.tree_rect.height, edge),
    //         TreeStyle::Fan => {
    //             let angle = edge_angle(self.rot_angle, self.opn_angle, edge);
    //             node_point_rad(angle, self.center, self.size, edge)
    //         }
    //     });
    // }

    // pub(crate) fn update_visible_nodes(&mut self) {
    //     self.tip_idx_range = self.visible_tip_idx_range();
    //     if let Some(tip_idx_range) = &self.tip_idx_range {
    //         let node_points =
    //             self.visible_nodes(self.tree_rect.width, self.tree_rect.height, tip_idx_range);
    //         self.visible_nodes = node_points.points;
    //         self.center = node_points.center;
    //         self.size = node_points.size;
    //     } else {
    //         self.visible_nodes.clear();
    //     }
    // }

    // pub(crate) fn update_rects(&mut self) {
    //     self.clip_rect = Rectangle {
    //         x: 0e0,
    //         y: 0e0,
    //         width: self.tre_cnv_w - SCROLL_TOOL_W + PADDING,
    //         height: self.tre_cnv_h,
    //     };
    //     self.tree_rect = match self.sel_tree_style_opt {
    //         TreeStyle::Phylogram => Rectangle {
    //             x: self.clip_rect.x + SF / 2e0 + PADDING,
    //             y: self.clip_rect.y + SF / 2e0 + self.max_lab_size + self.brnch_lab_offset_y,
    //             width: self.clip_rect.width - SF - PADDING * 2e0 - self.tip_lab_w,
    //             height: self.clip_rect.height - SF - self.max_lab_size * 1.5 - SCROLL_TOOL_W,
    //         },
    //         TreeStyle::Fan => Rectangle {
    //             x: self.clip_rect.x + SF / 2e0 + self.tip_lab_w,
    //             y: self.clip_rect.y + SF / 2e0 + self.tip_lab_w + PADDING,
    //             width: self.clip_rect.width - SF - self.tip_lab_w * 2e0,
    //             height: self.clip_rect.height
    //                 - SF
    //                 - self.tip_lab_w * 2e0
    //                 - SCROLL_TOOL_W
    //                 - PADDING * 2e0,
    //         },
    //     };
    //     match self.sel_tree_style_opt {
    //         TreeStyle::Phylogram => {
    //             self.ltt.ltt_rect_x = self.clip_rect.x + SF / 2e0 + PADDING;
    //             self.ltt.ltt_rect_w = self.clip_rect.width - SF - PADDING * 2e0;
    //         }
    //         TreeStyle::Fan => {
    //             self.ltt.ltt_rect_x = self.clip_rect.x + SF / 2e0 + PADDING;
    //             self.ltt.ltt_rect_w =
    //                 self.tree_scroll_w - SCROLL_TOOL_W + PADDING - SF - PADDING * 2e0;
    //         }
    //     };
    //     self.ltt.g_bounds.clear();
    //     self.ltt.g_ltt.clear();
    // }

    // pub(crate) fn update_canvas_w(&mut self) {
    //     self.min_tre_cnv_w = self.tree_scroll_w;
    //     self.tre_cnv_w = self.min_tre_cnv_w + (self.sel_tre_cnv_w_idx - 1) as Float * 1e2 * SF;
    //     if self.sel_tree_style_opt == TreeStyle::Phylogram {
    //         self.ltt_cnv_w = self.tre_cnv_w;
    //     } else {
    //         self.ltt_cnv_w = self.min_tre_cnv_w;
    //     }
    // }

    // pub(crate) fn update_tip_label_w(&mut self) {
    //     if self.tip_brnch_labs_allowed && self.has_tip_labs && self.draw_tip_labs {
    //         self.tip_lab_w = self.extra_space_for_tip_labs + self.tip_lab_offset_x;
    //         let max_tip_label_w = self.tre_cnv_w / 1.5;
    //         if self.tip_lab_w > max_tip_label_w {
    //             self.tip_lab_w = max_tip_label_w;
    //         }
    //     } else {
    //         self.tip_lab_w = 0e0;
    //     }
    // }

    // pub(crate) fn update_canvas_h(&mut self) {
    //     match self.sel_tree_style_opt {
    //         TreeStyle::Phylogram => {
    //             self.tre_cnv_h = self.node_size * self.tip_count as Float;
    //         }
    //         TreeStyle::Fan => {
    //             if self.sel_tre_cnv_w_idx == self.min_tre_cnv_w_idx {
    //                 self.tre_cnv_h = self.min_tre_cnv_h;
    //             } else {
    //                 self.tre_cnv_h =
    //                     self.min_tre_cnv_h + self.sel_tre_cnv_w_idx as Float * 1e2 * SF;
    //             }
    //         }
    //     }
    // }

    // pub(crate) fn update_node_size(&mut self) {
    //     self.min_tre_cnv_h = self.window_h - PADDING * 5e0 - TTR_H;
    //     if self.show_ltt {
    //         self.min_tre_cnv_h -= LTT_H;
    //     }
    //     self.tree_scroll_h = self.min_tre_cnv_h;
    //     self.min_node_size = self.min_tre_cnv_h / self.tip_count as Float;
    //     self.max_node_size = Float::max(self.max_lab_size * 3e0, self.min_node_size);
    //     self.max_node_size_idx = self.max_lab_size_idx;
    //     if self.min_node_size == self.max_node_size {
    //         self.max_node_size_idx = self.min_node_size_idx
    //     }
    //     if self.sel_node_size_idx > self.max_node_size_idx {
    //         self.sel_node_size_idx = self.max_node_size_idx
    //     }
    //     if self.sel_node_size_idx == self.min_node_size_idx {
    //         self.tre_cnv_y0 = 0e0;
    //         self.tre_cnv_y1 = self.tre_cnv_y0 + self.min_tre_cnv_h;
    //     }
    //     if self.max_node_size_idx > 1 {
    //         self.node_size = lerp(
    //             self.min_node_size,
    //             self.max_node_size,
    //             (self.sel_node_size_idx - 1) as Float / self.max_node_size_idx as Float,
    //         )
    //     } else {
    //         self.node_size = self.min_node_size
    //     }
    //     match self.sel_tree_style_opt {
    //         TreeStyle::Phylogram => {
    //             self.tip_brnch_labs_allowed =
    //                 (self.min_tre_cnv_h / self.node_size) as usize <= self.max_tip_labs_to_draw;
    //         }
    //         TreeStyle::Fan => {
    //             self.tip_brnch_labs_allowed = self.tip_count <= self.max_tip_labs_to_draw * 10;
    //         }
    //     }
    // }

    // pub(crate) fn update_extra_space_for_labels(&mut self) {
    //     let mut text_w = text_width(self.tip_lab_size, self.tip_lab_size, TREE_LAB_FONT_NAME);
    //     let mut max_w: Float = 0e0;
    //     let mut max_offset: Float = 0e0;
    //     for edge in &self.tallest_tips {
    //         if let Some(name) = &edge.name {
    //             let offset = edge.x1 as Float * self.tre_cnv_w;
    //             if offset >= max_offset {
    //                 max_offset = offset;
    //             }
    //             let tip_name_w = text_w.width(name);
    //             let curr_max_w = tip_name_w + (max_offset + offset) / 2e0 - self.tre_cnv_w;
    //             if curr_max_w >= max_w {
    //                 max_w = curr_max_w;
    //             }
    //         }
    //     }
    //     self.extra_space_for_tip_labs = max_w;
    // }
}
