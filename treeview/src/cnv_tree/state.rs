use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

#[derive(Debug)]
pub struct St {
    pub(crate) tip_lab_extra_w: Float,
    pub(crate) mouse: Option<Point>,
    pub(crate) bnds: Rectangle<Float>,
    pub(crate) cnv_vs: RectVals<Float>,
    pub(crate) tre_vs: RectVals<Float>,
    pub(crate) vis_vs: RectVals<Float>,
    pub(crate) cnv_rect: Rectangle<Float>,
    pub(crate) tre_rect: Rectangle<Float>,
    pub(crate) vis_rect: Rectangle<Float>,
    pub(crate) vis_node_idxs: Vec<usize>,
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
            tip_lab_extra_w: 0e0,
            mouse: None,
            bnds: Default::default(),
            cnv_vs: Default::default(),
            tre_vs: Default::default(),
            vis_vs: Default::default(),
            cnv_rect: Default::default(),
            tre_rect: Default::default(),
            vis_rect: Default::default(),
            vis_node_idxs: Vec::new(),
            node_data: Vec::new(),
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
    #[inline]
    pub(super) fn update_vis_nodes_phygrm(
        &mut self, max_tips: usize, max_nodes: usize, node_size: Float, tip_edge_idxs: &[usize],
    ) {
        self.vis_node_idxs.clear();
        if let Some(tip_idx_range) =
            self.vis_tip_idx_range_phygrm(self.vis_vs.y0, self.vis_vs.y1, node_size, tip_edge_idxs)
            && tip_idx_range.end() - tip_idx_range.start() <= max_tips
        {
            let node_idx_range = self.vis_node_idx_range_phygrm(&tip_idx_range, tip_edge_idxs);
            if node_idx_range.end() - node_idx_range.start() <= max_nodes {
                node_idx_range.collect_into(&mut self.vis_node_idxs);
            }
        }
    }

    #[inline]
    fn vis_tip_idx_range_phygrm(
        &self, y0: Float, y1: Float, node_size: Float, tip_edge_idxs: &[usize],
    ) -> Option<IndexRange> {
        tip_idx_range_between_y_vals(y0, y1, node_size, tip_edge_idxs)
    }

    #[inline]
    fn vis_node_idx_range_phygrm(&self, tip_idx_range: &IndexRange, tip_edge_idxs: &[usize]) -> IndexRange {
        node_idx_range_for_tip_idx_range(tip_idx_range, tip_edge_idxs)
    }

    #[inline]
    pub(super) fn update_vis_nodes_fan(&mut self, max_tips: usize, max_nodes: usize, opn: Float, edges: &[Edge]) {
        let mut tip_count: usize = 0;
        self.vis_node_idxs.clear();
        for e in edges {
            let angle = edge_angle(opn, e) + self.rot;
            let point = node_point_pol(angle, self.tre_vs.radius_min, self.rl, e);
            if self.vis_rect.contains(point + self.trans) {
                self.vis_node_idxs.push(e.edge_idx);
                if e.is_tip {
                    tip_count += 1;
                    if tip_count > max_tips || self.vis_node_idxs.len() > max_nodes {
                        self.vis_node_idxs.clear();
                        break;
                    }
                }
            }
        }
    }

    #[inline]
    pub(super) fn calc_tip_lab_extra_w(&mut self, tst: &TreeState) -> Float {
        let mut max_w: Float = 0e0;
        if let Some(text_w) = &mut self.text_w_tip {
            let mut max_offset: Float = 0e0;
            for edge in tst.edges_tip_tallest() {
                if let Some(name) = &edge.name {
                    let offset = edge.x1 as Float * self.tre_vs.w;
                    if offset >= max_offset {
                        max_offset = offset
                    };
                    let tip_name_w = text_w.width(name);
                    let curr_max_w = tip_name_w + (max_offset + offset) / 2e0 - self.tre_vs.w;
                    if curr_max_w >= max_w {
                        max_w = curr_max_w;
                    }
                }
            }
        }
        max_w
    }

    // pub(super) fn update_vis_rect(&mut self, abs: RectVals<Float>) {
    //     let buffer = 0e0;
    //     let x0 = abs.x0.max(self.tre_vs.x0) - buffer;
    //     let y0 = abs.y0.max(self.tre_vs.y0) - buffer;
    //     let x1 = abs.x1.min(self.tre_vs.x0 + self.tre_vs.w) + buffer;
    //     let y1 = abs.y1.min(self.tre_vs.y0 + self.tre_vs.h) + buffer;
    //     let w = x1 - x0;
    //     let h = y1 - y0;
    //     let top_left = Point { x: x0, y: y0 };
    //     let size = Size { width: w, height: h };
    //     self.vis_rect = Rectangle::new(top_left, size);
    // }
}
