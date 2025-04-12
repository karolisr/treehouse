use super::{NodeOrderingOption, TreeView};
use crate::{
    Float,
    app::{PADDING, SF, TREE_LAB_FONT_NAME},
    flatten_tree, lerp, text_width,
};

impl TreeView {
    fn update_tip_label_w(&mut self) {
        if self.draw_tip_labels_allowed && self.draw_tip_labels_selection {
            self.tip_label_w =
                self.tip_labels_w_scale_factor * self.tip_label_size + self.tip_label_offset;
        } else {
            self.tip_label_w = 0e0;
        }
    }

    fn update_canvas_h(&mut self) {
        self.canvas_h = self.node_size * self.tip_count as Float;
    }

    pub fn update_node_size(&mut self) {
        self.available_vertical_space = self.window_h - PADDING * 2e0 - SF * 2e0;
        self.min_node_size = self.available_vertical_space / self.tip_count as Float;
        self.max_node_size = Float::max(self.max_label_size, self.min_node_size);
        self.max_node_size_idx = self.max_label_size_idx;

        if self.min_node_size == self.max_node_size {
            self.max_node_size_idx = self.min_node_size_idx
        }

        if self.selected_node_size_idx > self.max_node_size_idx {
            self.selected_node_size_idx = self.max_node_size_idx
        }

        if self.selected_node_size_idx == self.min_node_size_idx {
            self.cnv_y0 = 0e0;
            self.cnv_y1 = self.cnv_y0 + self.available_vertical_space;
        }

        if self.max_node_size_idx > 1 {
            self.node_size = lerp(
                self.min_node_size,
                self.max_node_size,
                (self.selected_node_size_idx - 1) as Float / self.max_node_size_idx as Float,
            )
        } else {
            self.node_size = self.min_node_size
        }

        self.draw_tip_labels_allowed = (self.available_vertical_space / self.node_size) as usize
            <= self.max_count_of_tip_labels_to_draw;

        self.update_tip_label_w();
        self.update_canvas_h();
    }

    pub fn calc_tip_labels_w_scale_factor(&mut self) -> Float {
        let mut max_tip_height: f64 = 0e0;
        let mut max_tip_height_name: &str = "";
        let mut max_tip_height_name_len: usize = 0;
        for edge in &self.tree_tip_edges {
            if edge.x1 >= max_tip_height - max_tip_height / 1e1 {
                max_tip_height = edge.x1;
                if let Some(name) = &edge.name {
                    if name.len() > max_tip_height_name_len {
                        max_tip_height_name = name;
                        max_tip_height_name_len = name.len();
                    }
                }
            }
        }
        text_width(max_tip_height_name, 1e0, 1e0, TREE_LAB_FONT_NAME)
    }

    pub fn merge_tip_chunks(&mut self) {
        self.tree_tip_edges = Vec::new();
        for (i_c, chunk) in self.tree_chunked_edges.iter().enumerate() {
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
        match self.selected_node_ordering_option.unwrap() {
            NodeOrderingOption::Unordered => {
                self.tree = self.tree_original.clone();
                self.tree_chunked_edges = match &self.tree_original_chunked_edges {
                    Some(chunked_edges) => chunked_edges.clone(),
                    None => {
                        self.tree_original_chunked_edges =
                            Some(flatten_tree(&self.tree, self.threads));
                        self.tree_original_chunked_edges.clone().unwrap()
                    }
                };
            }
            NodeOrderingOption::Ascending => match &self.tree_srtd_asc {
                Some(tree_srtd_asc) => {
                    self.tree = tree_srtd_asc.clone();
                    self.tree_chunked_edges = self.tree_srtd_asc_chunked_edges.clone().unwrap();
                }
                None => {
                    let mut tmp = self.tree_original.clone();
                    tmp.sort(false);
                    self.tree_srtd_asc = Some(tmp);
                    self.tree = self.tree_srtd_asc.clone().unwrap();
                    self.tree_srtd_asc_chunked_edges = Some(flatten_tree(&self.tree, self.threads));
                    self.tree_chunked_edges = self.tree_srtd_asc_chunked_edges.clone().unwrap();
                }
            },

            NodeOrderingOption::Descending => match &self.tree_srtd_desc {
                Some(tree_srtd_desc) => {
                    self.tree = tree_srtd_desc.clone();
                    self.tree_chunked_edges = self.tree_srtd_desc_chunked_edges.clone().unwrap();
                }
                None => {
                    let mut tmp = self.tree_original.clone();
                    tmp.sort(true);
                    self.tree_srtd_desc = Some(tmp);
                    self.tree = self.tree_srtd_desc.clone().unwrap();
                    self.tree_srtd_desc_chunked_edges =
                        Some(flatten_tree(&self.tree, self.threads));
                    self.tree_chunked_edges = self.tree_srtd_desc_chunked_edges.clone().unwrap();
                }
            },
        };
    }
}
