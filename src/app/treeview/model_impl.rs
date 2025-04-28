use super::{
    NodeOrderingOption, TreeStyleOption, TreeView,
    treeview_canvas::{edge_angle, node_point, node_point_rad},
};
use crate::{
    Float,
    app::{LTT_H, PADDING, SCROLL_TOOL_W, SF, TREE_LAB_FONT_NAME, TTR_H},
    chunk_edges, flatten_tree, lerp, text_width,
};
use iced::Rectangle;

impl TreeView {
    pub fn update_found_node_point(&mut self) {
        if self.found_edges.is_empty() {
            self.found_edge_pt = None;
            return;
        }
        let edge = &self.found_edges[self.found_edge_idx];
        self.found_edge_pt = Some(match self.sel_tree_style_opt {
            TreeStyleOption::Phylogram => {
                node_point(self.tree_rect.width, self.tree_rect.height, edge)
            }
            TreeStyleOption::Fan => {
                let angle = edge_angle(self.rot_angle, self.opn_angle, edge);
                node_point_rad(angle, self.center, self.size, edge)
            }
        });
    }

    pub fn filter_nodes(&mut self) {
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

    pub fn update_visible_nodes(&mut self) {
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

    pub fn update_rects(&mut self) {
        self.clip_rect = Rectangle {
            x: 0e0,
            y: 0e0,
            width: self.tre_cnv_w - SCROLL_TOOL_W + PADDING,
            height: self.tre_cnv_h,
        };

        self.tree_rect = match self.sel_tree_style_opt {
            TreeStyleOption::Phylogram => Rectangle {
                x: self.clip_rect.x + SF / 2e0 + PADDING,
                y: self.clip_rect.y + SF / 2e0 + self.max_lab_size + self.brnch_lab_offset_y,
                width: self.clip_rect.width - SF - PADDING * 2e0 - self.tip_lab_w,
                height: self.clip_rect.height - SF - self.max_lab_size * 1.5 - SCROLL_TOOL_W,
            },
            TreeStyleOption::Fan => Rectangle {
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
            TreeStyleOption::Phylogram => {
                self.ltt.ltt_rect_x = self.clip_rect.x + SF / 2e0 + PADDING;
                self.ltt.ltt_rect_w = self.clip_rect.width - SF - PADDING * 2e0;
            }
            TreeStyleOption::Fan => {
                self.ltt.ltt_rect_x = self.clip_rect.x + SF / 2e0 + PADDING;
                self.ltt.ltt_rect_w =
                    self.tree_scroll_w - SCROLL_TOOL_W + PADDING - SF - PADDING * 2e0;
            }
        };

        self.ltt.g_bounds.clear();
        self.ltt.g_ltt.clear();
    }

    pub fn update_canvas_w(&mut self) {
        self.min_tre_cnv_w = self.tree_scroll_w;
        self.tre_cnv_w = self.min_tre_cnv_w + (self.sel_tre_cnv_w_idx - 1) as Float * 1e2 * SF;

        if self.sel_tree_style_opt == TreeStyleOption::Phylogram {
            self.ltt_cnv_w = self.tre_cnv_w;
        } else {
            self.ltt_cnv_w = self.min_tre_cnv_w;
        }
    }

    pub fn update_tip_label_w(&mut self) {
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

    pub fn update_canvas_h(&mut self) {
        match self.sel_tree_style_opt {
            TreeStyleOption::Phylogram => {
                self.tre_cnv_h = self.node_size * self.tip_count as Float;
            }
            TreeStyleOption::Fan => {
                if self.sel_tre_cnv_w_idx == self.min_tre_cnv_w_idx {
                    self.tre_cnv_h = self.min_tre_cnv_h;
                } else {
                    self.tre_cnv_h =
                        self.min_tre_cnv_h + self.sel_tre_cnv_w_idx as Float * 1e2 * SF;
                }
            }
        }
    }

    pub fn update_node_size(&mut self) {
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
            TreeStyleOption::Phylogram => {
                self.tip_brnch_labs_allowed =
                    (self.min_tre_cnv_h / self.node_size) as usize <= self.max_tip_labs_to_draw;
            }
            TreeStyleOption::Fan => {
                self.tip_brnch_labs_allowed = self.tip_count <= self.max_tip_labs_to_draw * 10;
            }
        }
    }

    pub fn update_tallest_tips(&mut self) {
        let n: i32 = 10;
        let mut tmp = self.tree_tip_edges.clone();
        let tmp_len_min: usize = 0.max(tmp.len() as i32 - n) as usize;
        tmp.sort_by(|a, b| a.x1.total_cmp(&b.x1));
        self.tallest_tips = tmp[tmp_len_min..tmp.len()].to_vec();
        tmp.sort_by(|a, b| {
            a.name
                .clone()
                .map(|name| name.len())
                .cmp(&b.name.clone().map(|name| name.len()))
        });
        self.tallest_tips
            .append(&mut tmp[tmp_len_min..tmp.len()].to_vec());
    }

    pub fn update_extra_space_for_labels(&mut self) {
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

    pub fn merge_tip_chunks(&mut self) {
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

    pub fn sort(&mut self) {
        match self.sel_node_ord_opt {
            NodeOrderingOption::Unordered => {
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

            NodeOrderingOption::Ascending => match &self.tree_srtd_asc {
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

            NodeOrderingOption::Descending => match &self.tree_srtd_desc {
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
