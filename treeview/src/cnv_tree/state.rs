use crate::*;

#[derive(Debug)]
pub struct St {
    prev_y0_rel: Float,
    prev_y1_rel: Float,
    prev_node_idx_range: IndexRange,

    pub mouse: Option<Point>,

    pub bnds: Rectangle<Float>,
    pub clip_vs: RectVals<Float>,
    pub tree_vs: RectVals<Float>,
    pub clip_rect: Rectangle<Float>,
    pub tree_rect: Rectangle<Float>,

    pub visible_nodes: Option<Vec<Edge>>,
    pub node_data: Vec<NodeData>,

    pub rl: Float,
    pub rot: Float,
    pub trans: Vector,
}

impl Default for St {
    fn default() -> Self {
        Self {
            prev_y0_rel: 0e0,
            prev_y1_rel: 0e0,
            prev_node_idx_range: IndexRange::new(0, 0),

            mouse: None,

            bnds: Default::default(),
            clip_vs: Default::default(),
            tree_vs: Default::default(),
            clip_rect: Default::default(),
            tree_rect: Default::default(),

            visible_nodes: None,
            node_data: vec![],

            rl: 0e0,
            rot: 0e0,
            trans: Vector { x: 0e0, y: 0e0 },
        }
    }
}

impl St {
    pub(crate) fn update_visible_nodes(
        &mut self, is_dirty: bool, tst: &TreeState, tre_cnv_h: Float, y0_rel: Float, y1_rel: Float,
    ) {
        let h_ratio_1 = tre_cnv_h / self.clip_vs.h;
        let h_ratio_2 = self.clip_vs.h / tre_cnv_h;

        let h_ratio_min = h_ratio_1.min(h_ratio_2);
        let h_ratio_max = h_ratio_1.max(h_ratio_2);

        let node_size = self.tree_vs.h / tst.tip_count() as Float;
        let y0 = (y0_rel * h_ratio_min) * self.tree_vs.h; // + node_size * 5e0;
        let y1 = (y1_rel * h_ratio_max) * self.tree_vs.h; // - node_size * 5e0;

        if self.prev_y0_rel != y0_rel || self.prev_y1_rel != y1_rel || is_dirty {
            if let Some(idx_range) = tst.visible_node_idx_range(y0, y1, node_size) {
                if idx_range != self.prev_node_idx_range || is_dirty {
                    self.prev_node_idx_range = idx_range.clone();
                    let visible_nodes = &tst.edges()[idx_range];
                    self.visible_nodes = Some(visible_nodes.to_vec());
                }
            } else {
                self.visible_nodes = None;
            }

            self.prev_y0_rel = y0_rel;
            self.prev_y1_rel = y1_rel;
        }
    }
}
