use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

#[derive(Debug)]
pub struct St {
    pub(crate) mouse: Option<Point>,
    pub(crate) hovered_node: Option<NodeData>,
    pub(crate) cursor_tracking_point: Option<Point>,
    pub(crate) bnds: Rectangle<Float>,
    pub(crate) cnv_vs: RectVals<Float>,
    pub(crate) tre_vs: RectVals<Float>,
    pub(crate) vis_vs: RectVals<Float>,
    pub(crate) cnv_rect: Rectangle<Float>,
    pub(crate) tre_rect: Rectangle<Float>,
    pub(crate) vis_rect: Rectangle<Float>,
    pub(crate) stale_vis_rect: bool,
    pub(crate) vis_node_idxs: Vec<usize>,
    pub(crate) vis_nodes: Vec<NodeData>,
    pub(crate) filtered_nodes: Vec<NodeData>,
    pub(crate) selected_nodes: Vec<NodeData>,
    pub(crate) node_radius: Float,
    pub(crate) root_len: Float,
    pub(crate) rotation: Float,
    pub(crate) translation: Vector,
    pub(crate) text_w_tip: Option<TextWidth<'static>>,
    pub(crate) text_w_int: Option<TextWidth<'static>>,
    pub(crate) text_w_brnch: Option<TextWidth<'static>>,
    pub(crate) labs_tip: Vec<Label>,
    pub(crate) labs_int: Vec<Label>,
    pub(crate) labs_brnch: Vec<Label>,
    pub(crate) is_new: bool,
}

impl Default for St {
    fn default() -> Self {
        Self {
            mouse: None,
            hovered_node: None,
            cursor_tracking_point: None,
            bnds: Default::default(),
            cnv_vs: Default::default(),
            tre_vs: Default::default(),
            vis_vs: Default::default(),
            cnv_rect: Default::default(),
            tre_rect: Default::default(),
            vis_rect: Default::default(),
            stale_vis_rect: false,
            vis_node_idxs: Vec::new(),
            vis_nodes: Vec::new(),
            filtered_nodes: Vec::new(),
            selected_nodes: Vec::new(),
            node_radius: SF * 5e0,
            root_len: ZRO,
            rotation: ZRO,
            translation: Vector { x: ZRO, y: ZRO },
            text_w_tip: Some(text_width(SF * TIP_LAB_SIZE_IDX as Float, FNT_NAME_LAB)),
            text_w_int: Some(text_width(SF * INTERNAL_LAB_SIZE_IDX as Float, FNT_NAME_LAB)),
            text_w_brnch: Some(text_width(SF * BRANCH_LAB_SIZE_IDX as Float, FNT_NAME_LAB)),
            labs_tip: Vec::new(),
            labs_int: Vec::new(),
            labs_brnch: Vec::new(),
            is_new: true, // sometimes canvas state gets recreated losing all the stored state.
        }
    }
}

impl St {
    pub(super) fn mouse_point(&mut self, crsr: Cursor) -> Option<Point<Float>> {
        crsr.position_in(self.bnds).map(|mouse| {
            if self.rotation != ZRO {
                let mouse_dist_from_center =
                    mouse.distance(Point { x: self.tre_vs.cntr.x, y: self.tre_vs.cntr.y });
                let mouse_x_untrans = mouse.x - self.translation.x;
                let mouse_y_untrans = mouse.y - self.translation.y;
                let angle = mouse_y_untrans.atan2(mouse_x_untrans) - self.rotation;
                let (sin, cos) = angle.sin_cos();
                Point { x: cos * mouse_dist_from_center, y: sin * mouse_dist_from_center }
            } else {
                mouse - self.translation
            }
        })
    }

    pub(super) fn hovered_node(&mut self) -> Option<NodeData> {
        let mut rv: Option<NodeData> = None;
        if let Some(mouse) = self.mouse {
            let closest_node = self
                .vis_nodes
                .iter()
                .min_by(|&a, &b| {
                    mouse.distance(a.points.p1).total_cmp(&mouse.distance(b.points.p1))
                })
                .cloned();
            if let Some(closest_node) = closest_node
                && mouse.distance(closest_node.points.p1) <= self.node_radius + SF * TWO * TWO
            {
                rv = Some(closest_node);
            }
        }
        rv
    }

    pub(super) fn cursor_tracking_point(&mut self) -> Option<Point> {
        let mut rv: Option<Point> = None;
        if let Some(mouse) = &self.mouse {
            let mut x = mouse.x;
            let mut y = mouse.y;
            if let Some(hovered_node) = &self.hovered_node {
                x = hovered_node.points.p1.x;
                y = hovered_node.points.p1.y;
            }
            rv = Some(Point { x, y })
        }
        rv
    }

    pub(super) fn update_vis_node_idxs_phygrm(
        &mut self, max_tips: usize, max_nodes: usize, node_size: Float, tip_edge_idxs: &[usize],
    ) {
        self.vis_node_idxs.clear();
        if let Some(tip_idx_range) = self.vis_tip_idx_range_phygrm(
            self.vis_vs.y0 - self.tre_vs.y0,
            self.vis_vs.y1 - self.tre_vs.y0,
            node_size,
            tip_edge_idxs,
        ) && tip_idx_range.end() - tip_idx_range.start() <= max_tips
        {
            let node_idx_range = self.vis_node_idx_range_phygrm(&tip_idx_range, tip_edge_idxs);
            if node_idx_range.end() - node_idx_range.start() <= max_nodes {
                node_idx_range.collect_into(&mut self.vis_node_idxs);
            }
        }
    }

    fn vis_tip_idx_range_phygrm(
        &self, y0: Float, y1: Float, node_size: Float, tip_edge_idxs: &[usize],
    ) -> Option<IndexRange> {
        tip_idx_range_between_y_vals(y0, y1, node_size, tip_edge_idxs)
    }

    fn vis_node_idx_range_phygrm(
        &self, tip_idx_range: &IndexRange, tip_edge_idxs: &[usize],
    ) -> IndexRange {
        node_idx_range_for_tip_idx_range(tip_idx_range, tip_edge_idxs)
    }

    pub(super) fn update_vis_node_idxs_fan(
        &mut self, max_tips: usize, max_nodes: usize, opn: Float, edges: &[Edge],
    ) {
        let mut tip_count: usize = 0;
        self.vis_node_idxs.clear();
        for e in edges {
            let angle = edge_angle(opn, e) + self.rotation;
            let point = node_point_pol(angle, self.tre_vs.radius_min, self.root_len, e);
            if self.vis_rect.contains(point + self.translation) {
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
}
