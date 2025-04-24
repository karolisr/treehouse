use super::{NodeOrderingOption, TreeStyleOption, TreeView};
use crate::{
    Float,
    app::{LTT_H, PADDING, SF, TREE_LAB_FONT_NAME},
    flatten_tree, lerp, text_width,
};

impl TreeView {
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
        self.min_tre_cnv_h = self.window_h - PADDING * 2e0;
        if self.show_ltt {
            self.min_tre_cnv_h -= LTT_H + PADDING;
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
        match self.sel_node_ord_opt {
            NodeOrderingOption::Unordered => {
                self.tree = self.tree_orig.clone();
                self.tree_chunked_edges = match &self.tree_orig_chunked_edges {
                    Some(chunked_edges) => chunked_edges.clone(),
                    None => {
                        self.tree_orig_chunked_edges = Some(flatten_tree(&self.tree, self.threads));
                        self.tree_orig_chunked_edges.clone().unwrap()
                    }
                };
            }

            NodeOrderingOption::Ascending => match &self.tree_srtd_asc {
                Some(tree_srtd_asc) => {
                    self.tree = tree_srtd_asc.clone();
                    self.tree_chunked_edges = self.tree_srtd_asc_chunked_edges.clone().unwrap();
                }
                None => {
                    let mut tmp = self.tree_orig.clone();
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
                    let mut tmp = self.tree_orig.clone();
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
