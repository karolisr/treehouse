use crate::*;

#[derive(Debug)]
pub struct St {
    prev_x0_rel: Float,
    prev_x1_rel: Float,
    prev_y0_rel: Float,
    prev_y1_rel: Float,
    prev_node_idx_range: IndexRange,

    pub mouse: Option<Point>,

    pub bnds: Rectangle<Float>,
    pub clip_vs: RectVals<Float>,
    pub tree_vs: RectVals<Float>,
    pub clip_rect: Rectangle<Float>,
    pub tree_rect: Rectangle<Float>,

    pub vis_rect: Rectangle<Float>,
    pub vis_pts: Vec<Point>,
    pub labs_allowed: bool,

    pub visible_nodes: Option<Vec<Edge>>,
    pub node_data: Vec<NodeData>,

    pub rl: Float,
    pub rot: Float,
    pub trans: Vector,

    pub text_w_tip: Option<TextWidth<'static>>,
    pub text_w_int: Option<TextWidth<'static>>,
    pub text_w_brnch: Option<TextWidth<'static>>,
    pub labs_tip: Vec<Label>,
    pub labs_int: Vec<Label>,
    pub labs_brnch: Vec<Label>,
}

impl Default for St {
    fn default() -> Self {
        Self {
            prev_x0_rel: 0e0,
            prev_x1_rel: 0e0,
            prev_y0_rel: 0e0,
            prev_y1_rel: 0e0,
            prev_node_idx_range: IndexRange::new(0, 0),

            mouse: None,

            bnds: Default::default(),
            clip_vs: Default::default(),
            tree_vs: Default::default(),
            clip_rect: Default::default(),
            tree_rect: Default::default(),

            vis_rect: Default::default(),
            vis_pts: Vec::new(),

            labs_allowed: false,

            visible_nodes: None,
            node_data: vec![],

            rl: 0e0,
            rot: 0e0,
            trans: Vector { x: 0e0, y: 0e0 },

            text_w_tip: Some(text_width(TIP_LAB_SIZE as Float, FNT_NAME_LAB)),
            text_w_int: Some(text_width(INT_LAB_SIZE as Float, FNT_NAME_LAB)),
            text_w_brnch: Some(text_width(BRNCH_LAB_SIZE as Float, FNT_NAME_LAB)),

            labs_tip: Vec::new(),
            labs_int: Vec::new(),
            labs_brnch: Vec::new(),
        }
    }
}

impl St {
    pub(super) fn update_vis_rect(&mut self, x0: Float, x1: Float, y0: Float, y1: Float) {
        let buffer = 7e0;
        let x0 = x0.max(self.tree_vs.x) - buffer;
        let y0 = y0.max(self.tree_vs.y) - buffer;
        let x1 = x1.min(self.tree_vs.x + self.tree_vs.w) + buffer;
        let y1 = y1.min(self.tree_vs.y + self.tree_vs.h) + buffer;
        let w = x1 - x0;
        let h = y1 - y0;
        let top_left = Point { x: x0, y: y0 };
        let size = Size { width: w, height: h };
        self.vis_rect = Rectangle::new(top_left, size);
    }

    pub(super) fn update_vis_nodes_fan(
        &mut self, x0_rel: Float, x1_rel: Float, y0_rel: Float, y1_rel: Float, opn_angle: Float,
        tip_max: usize, node_max: usize, is_dirty: bool, tst: &TreeState,
    ) {
        if self.prev_y0_rel != y0_rel
            || self.prev_x0_rel != x0_rel
            || self.prev_y1_rel != y1_rel
            || self.prev_x1_rel != x1_rel
            || is_dirty
        {
            self.prev_x0_rel = x0_rel;
            self.prev_y0_rel = y0_rel;
            self.prev_x1_rel = x1_rel;
            self.prev_y1_rel = y1_rel;

            let mut vis_nodes: Vec<Edge> = Vec::new();
            let mut tip_count: usize = 0;
            for e in tst.edges_srtd_y() {
                let angle = edge_angle(opn_angle, e) + self.rot;
                let point = node_point_rad(angle, self.tree_vs.radius_min, self.rl, e);
                if self.vis_rect.contains(point + self.trans) {
                    vis_nodes.push(e.clone());
                    if e.is_tip {
                        tip_count += 1;
                        if tip_count > tip_max {
                            self.labs_allowed = false;
                            self.visible_nodes = None;
                            return;
                        }
                    }
                }
            }
            if vis_nodes.is_empty() {
                self.labs_allowed = true;
                self.visible_nodes = None;
            } else if vis_nodes.len() > node_max {
                self.labs_allowed = false;
                self.visible_nodes = None;
            } else {
                self.labs_allowed = true;
                self.visible_nodes = Some(vis_nodes);
            }
        }
    }

    pub(super) fn update_vis_nodes_phylogram(
        &mut self, y0_rel: Float, y1_rel: Float, tre_cnv_h: Float, tre_padding: Float,
        is_dirty: bool, tst: &TreeState,
    ) {
        let h_ratio_1 = (tre_cnv_h + tre_padding * 2e0) / self.clip_vs.h;
        let h_ratio_2 = self.clip_vs.h / (tre_cnv_h + tre_padding * 2e0);

        let h_ratio_min = h_ratio_1.min(h_ratio_2);
        let h_ratio_max = h_ratio_1.max(h_ratio_2);

        let node_size = self.tree_vs.h / tst.tip_count() as Float;
        let y0 = (y0_rel * h_ratio_min) * self.tree_vs.h - tre_padding - node_size * 3e0;
        let y1 = (y1_rel * h_ratio_max) * self.tree_vs.h - tre_padding + node_size * 3e0;

        if self.prev_y0_rel != y0_rel || self.prev_y1_rel != y1_rel || is_dirty {
            if let Some(idx_range) = self.vis_node_idx_range_phylogram(y0, y1, node_size, tst) {
                if idx_range != self.prev_node_idx_range || is_dirty {
                    self.prev_node_idx_range = idx_range.clone();
                    let visible_nodes = &tst.edges_srtd_y()[idx_range];
                    self.visible_nodes = Some(visible_nodes.to_vec());
                }
            } else {
                self.visible_nodes = None;
            }
            self.prev_y0_rel = y0_rel;
            self.prev_y1_rel = y1_rel;
        }
    }

    fn vis_tip_idx_range_phylogram(
        &self, y0: Float, y1: Float, node_size: Float, tst: &TreeState,
    ) -> Option<IndexRange> {
        tip_idx_range_between_y_vals(y0, y1, node_size, tst.edges_tip_idx())
    }

    fn vis_node_idx_range_phylogram(
        &self, y0: Float, y1: Float, node_size: Float, tst: &TreeState,
    ) -> Option<IndexRange> {
        self.vis_tip_idx_range_phylogram(y0, y1, node_size, tst).map(|visible_tip_range| {
            node_idx_range_for_tip_idx_range(&visible_tip_range, tst.edges_tip_idx())
        })
    }
}
