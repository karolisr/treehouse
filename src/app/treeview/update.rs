use super::{TreeView, TreeViewMsg};
use crate::{Float, app::SF};
use iced::Task;

impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::OpenFile => Task::none(),

            TreeViewMsg::Init => Task::batch([
                Task::done(TreeViewMsg::TipLabelSizeSelectionChanged(
                    self.selected_tip_label_size_idx,
                )),
                Task::done(TreeViewMsg::IntLabelSizeSelectionChanged(
                    self.selected_int_label_size_idx,
                )),
                Task::done(TreeViewMsg::BranchLabelSizeSelectionChanged(
                    self.selected_branch_label_size_idx,
                )),
            ]),

            TreeViewMsg::TreeViewScrolled(vp) => {
                self.cnv_y0 = vp.absolute_offset().y;
                self.cnv_y1 = self.cnv_y0 + vp.bounds().height;
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.branch_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::NodeSizeSelectionChanged(idx) => {
                self.selected_node_size_idx = idx;
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                Task::none()
            }

            TreeViewMsg::BranchLabelVisibilityChanged(state) => {
                self.draw_branch_labels = state;
                Task::none()
            }

            TreeViewMsg::BranchLabelSizeSelectionChanged(idx) => {
                self.branch_labels_geom_cache.clear();
                self.selected_branch_label_size_idx = idx;
                self.branch_label_size = self.min_label_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
                #[cfg(debug_assertions)]
                self.debug_geom_cache.clear();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.branch_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.draw_tip_labels = state;
                if self.drawing_enabled
                    && self.draw_tip_branch_labels_allowed
                    && self.draw_tip_labels
                {
                    self.update_extra_space_for_labels();
                }
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                Task::none()
            }

            TreeViewMsg::TipLabelSizeSelectionChanged(idx) => {
                #[cfg(debug_assertions)]
                self.debug_geom_cache.clear();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.branch_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.selected_tip_label_size_idx = idx;
                self.tip_label_size = self.min_label_size * idx as Float;
                self.update_extra_space_for_labels();
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                Task::none()
            }

            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labels = state;
                Task::none()
            }

            TreeViewMsg::IntLabelSizeSelectionChanged(idx) => {
                self.int_labels_geom_cache.clear();
                self.selected_int_label_size_idx = idx;
                self.int_label_size = self.min_label_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::NodeOrderingOptionChanged(node_ordering_option) => {
                if node_ordering_option != self.selected_node_ordering_option.unwrap() {
                    self.edge_geom_cache.clear();
                    self.tip_labels_geom_cache.clear();
                    self.int_labels_geom_cache.clear();
                    self.branch_labels_geom_cache.clear();
                    self.selected_nodes_geom_cache.clear();
                    self.pointer_geom_cache.clear();
                    self.selected_node_ordering_option = Some(node_ordering_option);
                    self.sort();
                    self.merge_tip_chunks();
                }
                Task::none()
            }

            TreeViewMsg::SetWinId(id) => {
                self.win_id = Some(id);
                iced::window::get_size(id)
                    .map(|s| TreeViewMsg::WindowResized(s.width * SF, s.height * SF))
            }

            TreeViewMsg::WindowResized(w, h) => {
                #[cfg(debug_assertions)]
                self.debug_geom_cache.clear();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.branch_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.window_w = w;
                self.window_h = h;
                self.scroll_w = self.window_w - self.not_scroll_w;
                self.update_canvas_w();
                if self.draw_tip_branch_labels_allowed && self.draw_tip_labels {
                    self.update_extra_space_for_labels();
                }
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                Task::none()
            }

            TreeViewMsg::CanvasWidthSelectionChanged(idx) => {
                self.selected_canvas_w_idx = idx;
                self.update_canvas_w();

                if self.draw_tip_branch_labels_allowed && self.draw_tip_labels {
                    self.update_extra_space_for_labels();
                }

                self.update_tip_label_w();

                Task::none()
            }

            TreeViewMsg::SelectDeselectNode(node_id) => {
                if self.selected_node_ids.contains(&node_id) {
                    Task::done(TreeViewMsg::DeselectNode(node_id))
                } else {
                    Task::done(TreeViewMsg::SelectNode(node_id))
                }
            }

            TreeViewMsg::SelectNode(node_id) => {
                self.selected_node_ids.insert(node_id);
                self.selected_nodes_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::DeselectNode(node_id) => {
                self.selected_node_ids.remove(&node_id);
                self.selected_nodes_geom_cache.clear();
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
                self.tree_original.unroot();
                Task::done(TreeViewMsg::TreeUpdated(self.tree_original.clone()))
            }

            TreeViewMsg::TreeUpdated(tree) => {
                self.drawing_enabled = false;
                #[cfg(debug_assertions)]
                self.debug_geom_cache.clear();
                self.selected_node_ids.clear();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.branch_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.tree_original = tree;
                self.tree_srtd_asc = None;
                self.tree_srtd_desc = None;
                self.tree_srtd_asc_chunked_edges = None;
                self.tree_srtd_desc_chunked_edges = None;
                self.tree_original_chunked_edges = None;
                self.node_count = self.tree_original.node_count_all();
                self.tip_count = self.tree_original.tip_count_all();
                self.int_node_count = self.tree_original.internal_node_count_all();
                self.has_brlen = self.tree_original.has_branch_lengths();
                self.tree_height = self.tree_original.height() as Float;
                self.is_rooted = self.tree_original.is_rooted();
                let epsilon = self.tree_original.height() / 1e2;
                self.is_ultrametric = self.tree_original.is_ultrametric(epsilon);
                self.sort();
                self.merge_tip_chunks();
                self.update_tallest_tips();
                self.update_extra_space_for_labels();
                self.update_node_size();
                self.update_tip_label_w();
                self.update_canvas_h();
                self.drawing_enabled = true;
                Task::none()
            }
        }
    }
}
