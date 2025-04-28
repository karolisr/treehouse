use super::{TreeStyleOption, TreeView, TreeViewMsg};
use crate::{Float, PI, app::SF, ltt};
use iced::{
    Task,
    widget::scrollable::{self, AbsoluteOffset},
};

impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::Search(s) => {
                self.search_string = s;
                self.filter_nodes();
                self.g_node_filt.clear();
                Task::none()
            }

            TreeViewMsg::AddFoundToSelection => {
                for node_id in &self.filtered_node_ids {
                    self.sel_node_ids.insert(*node_id);
                }
                self.g_node_sel.clear();
                Task::none()
            }

            TreeViewMsg::RemFoundFromSelection => {
                for node_id in &self.filtered_node_ids {
                    self.sel_node_ids.remove(node_id);
                }
                self.g_node_sel.clear();
                Task::none()
            }

            TreeViewMsg::OpenFile => Task::none(),

            TreeViewMsg::Init => Task::batch([
                Task::done(TreeViewMsg::TipLabelSizeSelectionChanged(
                    self.sel_tip_lab_size_idx,
                )),
                Task::done(TreeViewMsg::IntLabelSizeSelectionChanged(
                    self.sel_int_lab_size_idx,
                )),
                Task::done(TreeViewMsg::BranchLabelSizeSelectionChanged(
                    self.sel_brnch_lab_size_idx,
                )),
                Task::done(TreeViewMsg::OpnAngleSelectionChanged(
                    self.sel_opn_angle_idx,
                )),
                Task::done(TreeViewMsg::RotAngleSelectionChanged(
                    self.sel_rot_angle_idx,
                )),
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
                self.g_node_filt.clear();
                self.g_node_hover.clear();

                self.ltt.g_frame.clear();
                self.ltt.g_ltt.clear();
                self.ltt.g_cursor_line.clear();

                Task::none()
            }

            TreeViewMsg::TreeReprOptionChanged(tree_repr_option) => {
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
                self.g_node_filt.clear();
                self.g_node_hover.clear();
                self.sel_tree_style_opt = tree_repr_option;
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible();
                Task::none()
            }

            TreeViewMsg::OpnAngleSelectionChanged(idx) => {
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
                self.g_node_filt.clear();
                self.g_node_hover.clear();
                self.sel_opn_angle_idx = idx;
                self.opn_angle = idx as Float / 360e0 * 2e0 * PI;
                self.update_visible();
                Task::none()
            }

            TreeViewMsg::RotAngleSelectionChanged(idx) => {
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
                self.g_node_filt.clear();
                self.g_node_hover.clear();
                self.sel_rot_angle_idx = idx;
                self.rot_angle = idx as Float / 360e0 * 2e0 * PI;
                self.update_visible();
                Task::none()
            }

            TreeViewMsg::ScrollToX { sender, x } => {
                if self.sel_tree_style_opt == TreeStyleOption::Phylogram {
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
                self.g_node_filt.clear();
                self.g_node_hover.clear();

                if self.sel_tree_style_opt == TreeStyleOption::Phylogram {
                    self.g_lab_tip.clear();
                    self.g_lab_int.clear();
                    self.g_lab_brnch.clear();
                }

                self.update_visible();
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

            TreeViewMsg::NodeSizeSelectionChanged(idx) => {
                self.sel_node_size_idx = idx;
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible();
                Task::none()
            }

            TreeViewMsg::BranchLabelVisibilityChanged(state) => {
                self.draw_brnch_labs = state;
                Task::none()
            }

            TreeViewMsg::BranchLabelSizeSelectionChanged(idx) => {
                self.g_lab_brnch.clear();
                self.sel_brnch_lab_size_idx = idx;
                self.brnch_lab_size = self.min_lab_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
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
                self.g_node_filt.clear();
                self.g_node_hover.clear();
                self.draw_tip_labs = state;
                if self.drawing_enabled && self.tip_brnch_labs_allowed && self.draw_tip_labs {
                    self.update_extra_space_for_labels();
                }
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible();
                Task::none()
            }

            TreeViewMsg::TipLabelSizeSelectionChanged(idx) => {
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
                self.g_node_filt.clear();
                self.g_node_hover.clear();
                self.sel_tip_lab_size_idx = idx;
                self.tip_lab_size = self.min_lab_size * idx as Float;
                self.update_extra_space_for_labels();
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.update_rects();
                self.update_visible();
                Task::none()
            }

            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labs = state;
                Task::none()
            }

            TreeViewMsg::IntLabelSizeSelectionChanged(idx) => {
                self.g_lab_int.clear();
                self.sel_int_lab_size_idx = idx;
                self.int_lab_size = self.min_lab_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::NodeOrderingOptionChanged(node_ordering_option) => {
                if node_ordering_option != self.sel_node_ord_opt {
                    self.g_edge.clear();
                    self.g_lab_tip.clear();
                    self.g_lab_int.clear();
                    self.g_lab_brnch.clear();
                    self.g_node_sel.clear();
                    self.g_node_filt.clear();
                    self.g_node_hover.clear();
                    self.sel_node_ord_opt = node_ordering_option;
                    self.sort();
                    self.merge_tip_chunks();
                    self.update_visible();
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
                self.update_visible();
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
                if self.sel_tree_style_opt == TreeStyleOption::Fan {
                    self.update_canvas_h();
                }
                if self.tip_brnch_labs_allowed && self.draw_tip_labs {
                    self.update_extra_space_for_labels();
                }
                self.update_tip_label_w();
                self.update_rects();
                self.update_visible();
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
                self.g_node_filt.clear();
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
                self.update_visible();
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

            TreeViewMsg::Root(node_id) => {
                let mut tree = self.tree.clone();
                let rslt = tree.root(node_id);
                match rslt {
                    Ok(_) => Task::done(TreeViewMsg::TreeUpdated(tree)),
                    Err(err) => {
                        println!("{err}");
                        Task::none()
                    }
                }
            }

            TreeViewMsg::Unroot => {
                self.tree_orig.unroot();
                Task::done(TreeViewMsg::TreeUpdated(self.tree_orig.clone()))
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
                self.g_node_filt.clear();
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
                self.update_visible();
                if !self.drawing_enabled { Task::done(TreeViewMsg::Init) } else { Task::none() }
            }
        }
    }
}
