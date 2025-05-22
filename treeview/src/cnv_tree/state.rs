use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

#[derive(Debug)]
pub struct St {
    // prev_x0_rel: Float,
    // prev_x1_rel: Float,
    // prev_y0_rel: Float,
    // prev_y1_rel: Float,
    // prev_node_idx_range: IndexRange,
    pub(crate) mouse: Option<Point>,
    pub(crate) bnds: Rectangle<Float>,
    pub(crate) clip_vs: RectVals<Float>,
    pub(crate) tre_vs: RectVals<Float>,
    pub(crate) clip_rect: Rectangle<Float>,
    pub(crate) tre_rect: Rectangle<Float>,
    pub(crate) vis_rect: Rectangle<Float>,

    // pub(crate) labs_allowed: bool,
    pub(crate) visible_nodes: Option<Vec<Edge>>,
    pub(crate) node_data: Vec<NodeData>,
    pub(crate) rl: Float,
    pub(crate) rot: Float,
    pub(crate) trans: Vector,
    pub(crate) text_w_tip: Option<TextWidth<'static>>,
    pub(crate) text_w_int: Option<TextWidth<'static>>,
    pub(crate) text_w_brnch: Option<TextWidth<'static>>,
    pub(crate) labs_tip: Vec<Label>,
    pub(crate) labs_int: Vec<Label>,
    pub(crate) labs_brnch: Vec<Label>,
}

impl Default for St {
    fn default() -> Self {
        Self {
            // prev_x0_rel: 0e0,
            // prev_x1_rel: 0e0,
            // prev_y0_rel: 0e0,
            // prev_y1_rel: 0e0,
            // prev_node_idx_range: IndexRange::new(0, 0),
            mouse: None,
            bnds: Default::default(),
            clip_vs: Default::default(),
            tre_vs: Default::default(),
            clip_rect: Default::default(),
            tre_rect: Default::default(),
            vis_rect: Default::default(),

            // labs_allowed: false,
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
    pub(super) fn update_vis_rect(&mut self, abs: RectVals<Float>) {
        let buffer = 0e0;
        let x0 = abs.x0.max(self.tre_vs.x0) - buffer;
        let y0 = abs.y0.max(self.tre_vs.y0) - buffer;
        let x1 = abs.x1.min(self.tre_vs.x0 + self.tre_vs.w) + buffer;
        let y1 = abs.y1.min(self.tre_vs.y0 + self.tre_vs.h) + buffer;
        let w = x1 - x0;
        let h = y1 - y0;
        let top_left = Point { x: x0, y: y0 };
        let size = Size { width: w, height: h };
        self.vis_rect = Rectangle::new(top_left, size);
    }

    pub(super) fn update_vis_nodes_fan(
        &mut self, rel: RectVals<Float>, opn_angle: Float, tip_max: usize, node_max: usize, tst: &TreeState,
    ) {
        // if self.prev_y0_rel != rel.y0
        //     || self.prev_x0_rel != rel.x0
        //     || self.prev_y1_rel != rel.y1
        //     || self.prev_x1_rel != rel.x1
        // {
        //     self.prev_x0_rel = rel.x0;
        //     self.prev_y0_rel = rel.y0;
        //     self.prev_x1_rel = rel.x1;
        //     self.prev_y1_rel = rel.y1;

        let mut vis_nodes: Vec<Edge> = Vec::new();
        let mut tip_count: usize = 0;
        for e in tst.edges_srtd_y() {
            let angle = edge_angle(opn_angle, e) + self.rot;
            let point = node_point_pol(angle, self.tre_vs.radius_min, self.rl, e);
            if self.vis_rect.contains(point + self.trans) {
                vis_nodes.push(e.clone());
                if e.is_tip {
                    tip_count += 1;
                    if tip_count > tip_max {
                        // self.labs_allowed = false;
                        self.visible_nodes = None;
                        return;
                    }
                }
            }
        }
        if vis_nodes.is_empty() {
            // self.labs_allowed = true;
            self.visible_nodes = None;
        } else if vis_nodes.len() > node_max {
            // self.labs_allowed = false;
            self.visible_nodes = None;
        } else {
            // self.labs_allowed = true;
            self.visible_nodes = Some(vis_nodes);
        }
        // }
    }

    pub(super) fn update_vis_nodes_phygrm(
        &mut self, rel: RectVals<Float>, tre_cnv_h: Float, tre_padding: Float, tst: &TreeState,
    ) {
        // let h_ratio_1 = (tre_cnv_h + tre_padding * 2e0) / self.clip_vs.h;
        // let h_ratio_2 = self.clip_vs.h / (tre_cnv_h + tre_padding * 2e0);

        // let h_ratio_min = h_ratio_1.min(h_ratio_2);
        // let h_ratio_max = h_ratio_1.max(h_ratio_2);

        // let node_size = self.tre_vs.h / tst.tip_count() as Float;
        // let y0 = (rel.y0 * h_ratio_min) * self.tre_vs.h - tre_padding; // - node_size * 3e0;
        // let y1 = (rel.y1 * h_ratio_max) * self.tre_vs.h - tre_padding; // + node_size * 3e0;

        let node_size = self.tre_vs.h / tst.tip_count() as Float;
        let y0 = rel.y0 * self.tre_vs.h - tre_padding;
        let y1 = rel.y1 * self.tre_vs.h - tre_padding;

        // if self.prev_y0_rel != rel.y0 || self.prev_y1_rel != rel.y1 {
        if let Some(idx_range) = self.vis_node_idx_range_phygrm(y0, y1, node_size, tst) {
            // if idx_range != self.prev_node_idx_range {
            // self.prev_node_idx_range = idx_range.clone();
            let visible_nodes = &tst.edges_srtd_y()[idx_range];
            self.visible_nodes = Some(visible_nodes.to_vec());
            // }
        } else {
            self.visible_nodes = None;
        }
        // self.prev_y0_rel = rel.y0;
        // self.prev_y1_rel = rel.y1;
        // }
    }

    fn vis_tip_idx_range_phygrm(&self, y0: Float, y1: Float, node_size: Float, tst: &TreeState) -> Option<IndexRange> {
        tip_idx_range_between_y_vals(y0, y1, node_size, tst.edges_tip_idx())
    }

    fn vis_node_idx_range_phygrm(&self, y0: Float, y1: Float, node_size: Float, tst: &TreeState) -> Option<IndexRange> {
        self.vis_tip_idx_range_phygrm(y0, y1, node_size, tst)
            .map(|visible_tip_range| node_idx_range_for_tip_idx_range(&visible_tip_range, tst.edges_tip_idx()))
    }
}
