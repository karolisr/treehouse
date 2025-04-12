use super::{TreeView, TreeViewMsg};
use crate::{Float, app::SF};
use iced::Task;

impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::OpenFile => Task::none(),

            TreeViewMsg::TreeViewScrolled(vp) => {
                self.cnv_y0 = vp.absolute_offset().y;
                self.cnv_y1 = self.cnv_y0 + vp.bounds().height;
                self.tip_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.draw_tip_labels_selection = state;
                self.update_node_size();
                Task::none()
            }

            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labels_selection = state;
                Task::none()
            }

            TreeViewMsg::TipLabelSizeSelectionChanged(idx) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.selected_tip_label_size_idx = idx;
                self.tip_label_size = self.min_label_size * idx as Float;
                self.update_node_size();
                Task::none()
            }

            TreeViewMsg::IntLabelSizeSelectionChanged(idx) => {
                self.int_labels_geom_cache.clear();
                self.selected_int_label_size_idx = idx;
                self.int_label_size = self.min_label_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::NodeSizeSelectionChanged(idx) => {
                self.selected_node_size_idx = idx;
                self.update_node_size();
                Task::none()
            }

            TreeViewMsg::NodeOrderingOptionChanged(node_ordering_option) => {
                if node_ordering_option != self.selected_node_ordering_option.unwrap() {
                    self.edge_geom_cache.clear();
                    self.tip_labels_geom_cache.clear();
                    self.selected_nodes_geom_cache.clear();
                    self.pointer_geom_cache.clear();
                    self.int_labels_geom_cache.clear();
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
                self.window_w = w;
                self.window_h = h;
                self.update_node_size();
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
                self.selected_node_ids.clear();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.selected_nodes_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.tree_original = tree;
                self.tree_srtd_asc = None;
                self.tree_srtd_desc = None;
                self.tree_srtd_asc_chunked_edges = None;
                self.tree_srtd_desc_chunked_edges = None;
                self.tree_original_chunked_edges = None;
                self.node_count = self.tree_original.node_count_all();
                self.tip_count = self.tree_original.tip_count_all();
                self.int_node_count = self.tree_original.internal_node_count_all();
                self.sort();
                self.merge_tip_chunks();
                self.tip_labels_w_scale_factor = self.calc_tip_labels_w_scale_factor();
                self.update_node_size();
                self.drawing_enabled = true;
                Task::none()
            }
        }
    }
}
