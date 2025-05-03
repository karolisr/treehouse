use super::{NodeOrdering, TreeStyle};
use crate::{Float, TreeView, TreeViewMsg, PADDING, SF, TREE_LAB_FONT_NAME};
use iced::{widget::scrollable::{self, AbsoluteOffset}, Rectangle, Task};
use utils::text_width;

impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        println!("TreeView::update({msg:?})");
        match msg {
            TreeViewMsg::Search(s) => {
                self.search_string = s;
                self.filter_nodes();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
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
                self.g_node_found_iter.clear();
                if let Some(pt) = self.found_edge_pt {
                    Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::NextResult => {
                self.found_edge_idx += 1;
                self.update_found_node_point();
                self.g_node_found_iter.clear();
                if let Some(pt) = self.found_edge_pt {
                    Task::done(TreeViewMsg::ScrollTo { x: pt.x, y: pt.y })
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::ScrollTo { x, y } => scrollable::scroll_to(
                "tre",
                match self.sel_tree_style_opt {
                    TreeStyle::Phylogram => AbsoluteOffset {
                        x: x - self.tree_scroll_w / 2e0,
                        y: y - self.tree_scroll_h / 2e0,
                    },
                    TreeStyle::Fan => AbsoluteOffset {
                        x: x - self.tree_scroll_w / 2e0 + self.tip_lab_w,
                        y: y - self.tree_scroll_h / 2e0 + self.tip_lab_w,
                    },
                },
            ),

            TreeViewMsg::AddFoundToSelection => {
                for node_id in &self.found_node_ids {
                    self.sel_node_ids.insert(*node_id);
                }
                self.g_node_sel.clear();
                Task::none()
            }

            TreeViewMsg::RemFoundFromSelection => {
                for node_id in &self.found_node_ids {
                    self.sel_node_ids.remove(node_id);
                }
                self.g_node_sel.clear();
                Task::none()
            }

            TreeViewMsg::OpenFile => Task::none(),

            TreeViewMsg::Init => Task::batch([
                Task::done(TreeViewMsg::TipLabelSizeSelectionChanged(self.sel_tip_lab_size_idx)),
                Task::done(TreeViewMsg::IntLabelSizeSelectionChanged(self.sel_int_lab_size_idx)),
                Task::done(TreeViewMsg::BranchLabelSizeSelectionChanged(
                    self.sel_brnch_lab_size_idx,
                )),
                Task::done(TreeViewMsg::OpnAngleSelectionChanged(self.sel_opn_angle_idx)),
                Task::done(TreeViewMsg::RotAngleSelectionChanged(self.sel_rot_angle_idx)),
            ])
            .chain(if let Some(id) = self.win_id {
                iced::window::get_size(id)
                    .map(|s| TreeViewMsg::WindowResized(s.width * SF, s.height * SF))
            } else {
                Task::none()
            })
            .chain(Task::done(TreeViewMsg::EnableDrawing)),

            TreeViewMsg::EnableDrawing => {
                self.drawing_enabled = true;
                Task::none()
            }

            TreeViewMsg::Refresh => {
                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                    self.ltt.g_bounds.clear();
                }
                self.g_cursor_line.clear();
                self.g_frame.clear();
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();

                self.ltt.g_frame.clear();
                self.ltt.g_ltt.clear();
                self.ltt.g_cursor_line.clear();

                Task::none()
            }

            TreeViewMsg::ScrollToX { sender, x } => {
                if self.sel_tree_style_opt == TreeStyle::Phylogram {
                    match sender {
                        "tre" => {
                            self.tre_cnv_scrolled = true;
                            self.ltt_cnv_scrolled = false;
                            scrollable::scroll_to("ltt", AbsoluteOffset { x, y: self.ltt_cnv_y0 })
                        }
                        "ltt" => {
                            self.ltt_cnv_scrolled = true;
                            self.tre_cnv_scrolled = false;
                            scrollable::scroll_to("tre", AbsoluteOffset { x, y: self.tre_cnv_y0 })
                        }
                        _ => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }

            TreeViewMsg::TreCnvScrolled(vp) => {
                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                    self.ltt.g_bounds.clear();
                }
                self.tre_cnv_x0 = vp.absolute_offset().x;
                self.tre_cnv_y0 = vp.absolute_offset().y;
                self.tre_cnv_y1 = self.tre_cnv_y0 + vp.bounds().height;
                self.g_legend.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();

                if self.sel_tree_style_opt == TreeStyle::Phylogram {
                    self.g_lab_tip.clear();
                    self.g_lab_int.clear();
                    self.g_lab_brnch.clear();
                }

                self.update_visible_nodes();
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

            TreeViewMsg::CursorOnTreCnv { x } => {
                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                    self.ltt.g_bounds.clear();
                }
                self.cursor_x_fraction = None;
                self.ltt.cursor_x_fraction = x;
                self.ltt.g_cursor_line.clear();
                self.g_cursor_line.clear();
                Task::none()
            }

            TreeViewMsg::CursorOnLttCnv { x } => {
                self.cursor_x_fraction = x;
                self.ltt.cursor_x_fraction = x;
                self.ltt.g_cursor_line.clear();
                self.g_cursor_line.clear();
                Task::none()
            }

            TreeViewMsg::WindowResized(w, h) => {
                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                }
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();
                self.window_w = w;
                self.window_h = h;
                self.tree_scroll_w = self.window_w - self.side_with_padding_w;
                self.update_canvas_w();
                if self.tip_brnch_labs_allowed && self.draw_tip_labs {
                    self.update_extra_space_for_labels();
                }
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();
                Task::none()
            }

            TreeViewMsg::SetWinId(id) => {
                self.win_id = Some(id);
                Task::none()
            }

            TreeViewMsg::SelectDeselectNode(node_id) => {
                if self.sel_node_ids.contains(&node_id) {
                    Task::done(TreeViewMsg::DeselectNode(node_id))
                } else {
                    Task::done(TreeViewMsg::SelectNode(node_id))
                }
            }

            TreeViewMsg::SelectNode(node_id) => {
                self.sel_node_ids.insert(node_id);
                self.g_node_sel.clear();
                Task::none()
            }

            TreeViewMsg::DeselectNode(node_id) => {
                self.sel_node_ids.remove(&node_id);
                self.g_node_sel.clear();
                Task::none()
            }

            TreeViewMsg::TreeUpdated(tree) => {
                self.sel_node_ids.clear();
                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                }
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();
                self.tree_orig = tree;
                self.tree_srtd_asc = None;
                self.tree_srtd_desc = None;
                self.tree_srtd_asc_edges_chunked = None;
                self.tree_srtd_desc_edges_chunked = None;
                self.tree_orig_edges_chunked = None;
                self.node_count = self.tree_orig.node_count_all();
                self.tip_count = self.tree_orig.tip_count_all();
                self.int_node_count = self.tree_orig.internal_node_count_all();
                self.has_brlen = self.tree_orig.has_branch_lengths();
                self.has_int_labs = self.tree_orig.has_int_labels();
                self.has_tip_labs = self.tree_orig.has_tip_labels();
                self.tree_height = self.tree_orig.height() as Float;
                self.is_rooted = self.tree_orig.is_rooted();
                let epsilon = self.tree_orig.height() / 1e2;
                self.is_ultrametric = self.tree_orig.is_ultrametric(epsilon);
                self.sort();
                self.ltt.set_data(ltt(&self.tree_edges, self.ltt_bins));
                self.merge_tip_chunks();
                self.update_tallest_tips();
                self.update_extra_space_for_labels();
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();
                if !self.drawing_enabled { Task::done(TreeViewMsg::Init) } else { Task::none() }
            }

            // ------------------------------------------------------------------------------------

            TreeViewMsg::Root(_node_id) => {
                let mut tree = self.tree.clone();
                let rslt = tree.root(node_id);
                match rslt {
                    Ok(_) => Task::done(TreeViewMsg::TreeUpdated(tree)),
                    Err(err) => {
                        println!("{err}");
                        Task::none()
                    }
                }
                Task::none()
            }

            TreeViewMsg::Unroot => {
                self.tree_orig.unroot();
                Task::done(TreeViewMsg::TreeUpdated(self.tree_orig.clone()))
                Task::none()
            }

            // ------------------------------------------------------------------------------------

            TreeViewMsg::TreeWinPaneGridMsg(pane_grid_msg) => {
                self.pane_grid_main.update(pane_grid_msg);
                Task::none()
            }

            TreeViewMsg::TreeStyleOptionChanged(tree_repr_option) => {
                self.sel_tree_style_opt = tree_repr_option;

                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                    self.ltt.g_bounds.clear();
                }
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();

                Task::none()
            }

            TreeViewMsg::OpnAngleSelectionChanged(idx) => {
                self.sel_opn_angle_idx = idx;

                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                    self.ltt.g_bounds.clear();
                }
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();
                self.opn_angle = idx as Float / 360e0 * 2e0 * PI;
                self.update_visible_nodes();
                self.update_found_node_point();

                Task::none()
            }

            TreeViewMsg::RotAngleSelectionChanged(idx) => {
                self.sel_rot_angle_idx = idx;

                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                    self.ltt.g_bounds.clear();
                }
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();
                self.rot_angle = idx as Float / 360e0 * 2e0 * PI;
                self.update_visible_nodes();
                self.update_found_node_point();

                Task::none()
            }

            TreeViewMsg::NodeSizeSelectionChanged(idx) => {
                self.sel_node_size_idx = idx;

                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();

                Task::none()
            }

            TreeViewMsg::BranchLabelVisibilityChanged(state) => {
                self.draw_brnch_labs = state;
                Task::none()
            }

            TreeViewMsg::BranchLabelSizeSelectionChanged(idx) => {
                self.sel_brnch_lab_size_idx = idx;

                self.g_lab_brnch.clear();
                self.brnch_lab_size = self.min_lab_size * idx as Float;

                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
                self.draw_tip_labs = state;

                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                }
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();
                if self.drawing_enabled && self.tip_brnch_labs_allowed && self.draw_tip_labs {
                    self.update_extra_space_for_labels();
                }
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();

                Task::none()
            }

            TreeViewMsg::TipLabelSizeSelectionChanged(idx) => {
                self.sel_tip_lab_size_idx = idx;

                #[cfg(debug_assertions)]
                {
                    self.g_bounds.clear();
                    self.g_palette.clear();
                }
                self.g_legend.clear();
                self.g_edge.clear();
                self.g_lab_tip.clear();
                self.g_lab_int.clear();
                self.g_lab_brnch.clear();
                self.g_node_sel.clear();
                self.g_node_found.clear();
                self.g_node_found_iter.clear();
                self.g_node_hover.clear();
                self.tip_lab_size = self.min_lab_size * idx as Float;
                self.update_extra_space_for_labels();
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();

                Task::none()
            }

            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labs = state;
                Task::none()
            }

            TreeViewMsg::IntLabelSizeSelectionChanged(idx) => {
                self.sel_int_lab_size_idx = idx;

                self.g_lab_int.clear();
                self.int_lab_size = self.min_lab_size * idx as Float;

                Task::none()
            }

            TreeViewMsg::NodeOrderingOptionChanged(node_ordering_option) => {
                if node_ordering_option != self.sel_node_ord_opt {
                    self.sel_node_ord_opt = node_ordering_option;

                    self.g_edge.clear();
                    self.g_lab_tip.clear();
                    self.g_lab_int.clear();
                    self.g_lab_brnch.clear();
                    self.g_node_sel.clear();
                    self.g_node_found.clear();
                    self.g_node_found_iter.clear();
                    self.g_node_hover.clear();
                    self.sort();
                    self.merge_tip_chunks();
                    self.update_visible_nodes();
                    self.update_found_node_point();
                }
                Task::none()
            }

            TreeViewMsg::LegendVisibilityChanged(state) => {
                self.draw_legend = state;
                Task::none()
            }

            TreeViewMsg::LttVisibilityChanged(state) => {
                self.show_ltt = state;

                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();
                Task::done(TreeViewMsg::ScrollToX { sender: "tre", x: self.tre_cnv_x0 })
            }

            TreeViewMsg::CursorLineVisibilityChanged(state) => {
                self.show_cursor_line = state;

                self.g_cursor_line.clear();

                Task::none()
            }

            TreeViewMsg::CanvasWidthSelectionChanged(idx) => {
                self.sel_tre_cnv_w_idx = idx;

                self.update_canvas_w();
                if self.sel_tree_style_opt == TreeStyle::Fan {
                    self.update_canvas_h();
                }
                if self.tip_brnch_labs_allowed && self.draw_tip_labs {
                    self.update_extra_space_for_labels();
                }
                self.update_tip_label_w();
                self.update_rects();
                self.update_visible_nodes();
                self.update_found_node_point();

                Task::none()
            }
        }
    }
}

impl TreeView {
    pub(crate) fn update_found_node_point(&mut self) {
        if self.found_edges.is_empty() {
            self.found_edge_pt = None;
            return;
        }
        let edge = &self.found_edges[self.found_edge_idx];
        self.found_edge_pt = Some(match self.sel_tree_style_opt {
            TreeStyle::Phylogram => {
                node_point(self.tree_rect.width, self.tree_rect.height, edge)
            }
            TreeStyle::Fan => {
                let angle = edge_angle(self.rot_angle, self.opn_angle, edge);
                node_point_rad(angle, self.center, self.size, edge)
            }
        });
    }

    pub(crate) fn filter_nodes(&mut self) {
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

    pub(crate) fn update_visible_nodes(&mut self) {
        self.tip_idx_range = self.visible_tip_idx_range();
        if let Some(tip_idx_range) = &self.tip_idx_range {
            let node_points =
                self.visible_nodes(self.tree_rect.width, self.tree_rect.height, tip_idx_range);
            self.visible_nodes = node_points.points;
            self.center = node_points.center;
            self.size = node_points.size;
        } else {
            self.visible_nodes.clear();
        }
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
                self.ltt.ltt_rect_w =
                    self.tree_scroll_w - SCROLL_TOOL_W + PADDING - SF - PADDING * 2e0;
            }
        };

        self.ltt.g_bounds.clear();
        self.ltt.g_ltt.clear();
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

    pub(crate) fn update_canvas_h(&mut self) {
        match self.sel_tree_style_opt {
            TreeStyle::Phylogram => {
                self.tre_cnv_h = self.node_size * self.tip_count as Float;
            }
            TreeStyle::Fan => {
                if self.sel_tre_cnv_w_idx == self.min_tre_cnv_w_idx {
                    self.tre_cnv_h = self.min_tre_cnv_h;
                } else {
                    self.tre_cnv_h =
                        self.min_tre_cnv_h + self.sel_tre_cnv_w_idx as Float * 1e2 * SF;
                }
            }
        }
    }

    pub(crate) fn update_node_size(&mut self) {
        self.min_tre_cnv_h = self.window_h - PADDING * 5e0 - TTR_H;
        if self.show_ltt {
            self.min_tre_cnv_h -= LTT_H;
        }
        self.tree_scroll_h = self.min_tre_cnv_h;

        self.min_node_size = self.min_tre_cnv_h / self.tip_count as Float;
        self.max_node_size = Float::max(self.max_lab_size * 3e0, self.min_node_size);
        self.max_node_size_idx = self.max_lab_size_idx;

        if self.min_node_size == self.max_node_size {
            self.max_node_size_idx = self.min_node_size_idx
        }

        if self.sel_node_size_idx > self.max_node_size_idx {
            self.sel_node_size_idx = self.max_node_size_idx
        }

        if self.sel_node_size_idx == self.min_node_size_idx {
            self.tre_cnv_y0 = 0e0;
            self.tre_cnv_y1 = self.tre_cnv_y0 + self.min_tre_cnv_h;
        }

        if self.max_node_size_idx > 1 {
            self.node_size = lerp(
                self.min_node_size,
                self.max_node_size,
                (self.sel_node_size_idx - 1) as Float / self.max_node_size_idx as Float,
            )
        } else {
            self.node_size = self.min_node_size
        }

        match self.sel_tree_style_opt {
            TreeStyle::Phylogram => {
                self.tip_brnch_labs_allowed =
                    (self.min_tre_cnv_h / self.node_size) as usize <= self.max_tip_labs_to_draw;
            }
            TreeStyle::Fan => {
                self.tip_brnch_labs_allowed = self.tip_count <= self.max_tip_labs_to_draw * 10;
            }
        }
    }

    pub(crate) fn update_tallest_tips(&mut self) {
        let n: i32 = 10;
        let mut tmp = self.tree_tip_edges.clone();
        let tmp_len_min: usize = 0.max(tmp.len() as i32 - n) as usize;
        tmp.sort_by(|a, b| a.x1.total_cmp(&b.x1));
        self.tallest_tips = tmp[tmp_len_min..tmp.len()].to_vec();
        tmp.sort_by(|a, b| {
            a.name.clone().map(|name| name.len()).cmp(&b.name.clone().map(|name| name.len()))
        });
        self.tallest_tips.append(&mut tmp[tmp_len_min..tmp.len()].to_vec());
    }

    pub(crate) fn update_extra_space_for_labels(&mut self) {
        let mut text_w = text_width(self.tip_lab_size, self.tip_lab_size, TREE_LAB_FONT_NAME);
        let mut max_w: Float = 0e0;
        let mut max_offset: Float = 0e0;
        for edge in &self.tallest_tips {
            if let Some(name) = &edge.name {
                let offset = edge.x1 as Float * self.tre_cnv_w;
                if offset >= max_offset {
                    max_offset = offset;
                }
                let tip_name_w = text_w.width(name);
                let curr_max_w = tip_name_w + (max_offset + offset) / 2e0 - self.tre_cnv_w;
                if curr_max_w >= max_w {
                    max_w = curr_max_w;
                }
            }
        }
        self.extra_space_for_tip_labs = max_w;
    }

    pub(crate) fn merge_tip_chunks(&mut self) {
        self.tree_tip_edges = Vec::new();
        for (i_c, chunk) in self.tree_edges_chunked.iter().enumerate() {
            for (i_e, edge) in chunk.iter().enumerate() {
                if edge.is_tip {
                    let mut e = edge.clone();
                    e.chunk_idx = i_c;
                    e.edge_idx = i_e;
                    self.tree_tip_edges.push(e);
                }
            }
        }
    }

    pub(crate) fn sort(&mut self) {
        match self.sel_node_ord_opt {
            NodeOrdering::Unordered => {
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

            NodeOrdering::Ascending => match &self.tree_srtd_asc {
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

            NodeOrdering::Descending => match &self.tree_srtd_desc {
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
    }
}
