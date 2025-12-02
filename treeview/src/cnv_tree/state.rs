use crate::edge_utils::*;
use crate::*;

#[derive(Debug)]
pub struct St {
    pub(crate) previous_click: Option<MouseClick>,
    pub(crate) mouse: Option<Point>,
    pub(crate) modifs: Modifiers,
    pub(crate) hovered_node: Option<(NodeId, NodeData)>,
    pub(crate) cursor_tracking_point: Option<Point>,
    pub(crate) bnds: Rectangle<Float>,
    pub(crate) cnv_vs: RectVals<Float>,
    pub(crate) tre_vs: RectVals<Float>,
    pub(crate) vis_vs: RectVals<Float>,
    pub(crate) cnv_rect: Rectangle<Float>,
    pub(crate) tre_rect: Rectangle<Float>,
    pub(crate) vis_rect: Rectangle<Float>,
    pub(crate) stale_vis_rect: bool,
    pub(crate) vis_edge_idxs: Vec<usize>,
    pub(crate) vis_nodes: Vec<NodeData>,
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
    pub(crate) mouse_angle: Option<Float>,
    pub(crate) mouse_zone: Option<Zone>,
    pub(crate) tip_lab_w_rect: Option<Rectangle<Float>>,
    pub(crate) tip_lab_w_ring: Option<Float>,
    pub(crate) mouse_is_over_tip_w_resize_area: bool,
    pub(crate) tip_lab_w_is_being_resized: bool,
    pub(crate) is_new: bool,
    pub(crate) tre_sty: TreSty,
    pub(crate) opn_angle: Float,
}

impl Default for St {
    fn default() -> Self {
        Self {
            previous_click: None,
            mouse: None,
            modifs: Modifiers::empty(),
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
            vis_edge_idxs: Vec::new(),
            vis_nodes: Vec::new(),
            selected_nodes: Vec::new(),
            node_radius: SF * 3e0,
            root_len: ZRO,
            rotation: ZRO,
            translation: Vector { x: ZRO, y: ZRO },
            text_w_tip: Some(text_width(
                SF * TIP_LAB_SIZE_IDX as Float,
                FNT_NAME_LAB,
            )),
            text_w_int: Some(text_width(
                SF * INTERNAL_LAB_SIZE_IDX as Float,
                FNT_NAME_LAB,
            )),
            text_w_brnch: Some(text_width(
                SF * BRANCH_LAB_SIZE_IDX as Float,
                FNT_NAME_LAB,
            )),
            labs_tip: Vec::new(),
            labs_int: Vec::new(),
            labs_brnch: Vec::new(),
            mouse_angle: None,
            mouse_zone: None,
            tip_lab_w_rect: None,
            tip_lab_w_ring: None,
            mouse_is_over_tip_w_resize_area: false,
            tip_lab_w_is_being_resized: false,
            is_new: true, // sometimes canvas state gets recreated losing all the stored state.
            opn_angle: ZRO,
            tre_sty: TreSty::PhyGrm,
        }
    }
}

impl St {
    pub(super) fn update_vis_nodes(&mut self, edges: &[Edge]) {
        prepare_nodes(
            &self.tre_vs, self.root_len, self.tre_sty, self.opn_angle, edges,
            &self.vis_edge_idxs, &mut self.vis_nodes,
        );
    }

    pub(super) fn update_selected_nodes(
        &mut self,
        edges: &[Edge],
        sel_edge_idxs: &[usize],
    ) {
        prepare_nodes(
            &self.tre_vs, self.root_len, self.tre_sty, self.opn_angle, edges,
            sel_edge_idxs, &mut self.selected_nodes,
        );
    }

    pub(super) fn mouse_angle(&mut self, crsr: Cursor) -> Option<Float> {
        crsr.position_in(self.bnds).map(|mouse| {
            if self.rotation == ZRO {
                let x = (mouse.x - self.vis_vs.cntr_x) / self.vis_vs.w;
                let y = (mouse.y - self.vis_vs.cntr_y) / self.vis_vs.h;
                y.atan2(x) + PI
            } else {
                let mouse_x_untrans = mouse.x - self.translation.x;
                let mouse_y_untrans = mouse.y - self.translation.y;
                mouse_y_untrans.atan2(mouse_x_untrans) + PI
            }
        })
    }

    pub(super) fn mouse_angle_to_zone(&mut self) -> Option<Zone> {
        match self.mouse_angle?.to_degrees() {
            a if (015.0..075.0).contains(&a) => Some(Zone::TopLeft),
            a if (075.0..105.0).contains(&a) => Some(Zone::Top),
            a if (105.0..165.0).contains(&a) => Some(Zone::TopRight),
            a if (165.0..195.0).contains(&a) => Some(Zone::Right),
            a if (195.0..255.0).contains(&a) => Some(Zone::BottomRight),
            a if (255.0..285.0).contains(&a) => Some(Zone::Bottom),
            a if (285.0..345.0).contains(&a) => Some(Zone::BottomLeft),
            _ => Some(Zone::Left),
        }
    }

    pub(super) fn mouse_point(&mut self, crsr: Cursor) -> Option<Point<Float>> {
        crsr.position_in(self.bnds).map(|mouse| {
            if self.rotation != ZRO {
                let mouse_dist_from_center = mouse.distance(Point {
                    x: self.tre_vs.cntr.x,
                    y: self.tre_vs.cntr.y,
                });
                let mouse_x_untrans = mouse.x - self.translation.x;
                let mouse_y_untrans = mouse.y - self.translation.y;
                let angle =
                    mouse_y_untrans.atan2(mouse_x_untrans) - self.rotation;
                let (sin, cos) = angle.sin_cos();
                Point {
                    x: cos * mouse_dist_from_center,
                    y: sin * mouse_dist_from_center,
                }
            } else {
                mouse - self.translation
            }
        })
    }

    pub(super) fn hovered_node(
        &mut self,
        edges: &[Edge],
    ) -> Option<(NodeId, NodeData)> {
        let mouse = &self.mouse?;
        let closest_node = self
            .vis_nodes
            .iter()
            .min_by(|&a, &b| {
                mouse
                    .distance(a.points.p1)
                    .total_cmp(&mouse.distance(b.points.p1))
            })
            .cloned();
        if let Some(closest_node) = closest_node
            && mouse.distance(closest_node.points.p1)
                <= self.node_radius + SF * 5e0
        {
            Some((edges[closest_node.edge_idx].node_id, closest_node))
        } else {
            None
        }
    }

    pub(super) fn is_mouse_over_tip_w_resize_area(&mut self) -> bool {
        if let Some(mouse) = self.mouse {
            if let Some(tip_lab_w_rect) = self.tip_lab_w_rect {
                tip_lab_w_rect.contains(mouse + self.translation)
            } else if let Some(tip_lab_w_ring) = self.tip_lab_w_ring {
                let d = mouse.distance(ORIGIN);
                d > tip_lab_w_ring && d < tip_lab_w_ring + PADDING
            } else {
                false
            }
        } else {
            false
        }
    }

    pub(super) fn calc_tip_lab_w(&mut self) -> Float {
        if let Some(mouse) = self.mouse {
            if self.tip_lab_w_rect.is_some() {
                self.bnds.width - mouse.x - self.translation.x + PADDING / TWO
            } else if self.tip_lab_w_ring.is_some() {
                self.cnv_vs.radius_min - mouse.distance(ORIGIN) + PADDING / TWO
            } else {
                ZRO
            }
        } else {
            ZRO
        }
    }

    pub(super) fn cursor_tracking_point(&mut self) -> Option<Point> {
        let mouse = &self.mouse?;
        if let Some((_, hovered_node)) = &self.hovered_node {
            Some(Point {
                x: hovered_node.points.p1.x,
                y: hovered_node.points.p1.y,
            })
        } else {
            Some(Point { x: mouse.x, y: mouse.y })
        }
    }

    pub(super) fn update_vis_edge_idxs(&mut self, edges: &[Edge]) {
        self.vis_edge_idxs.clear();
        let vis_rect_expanded = self.vis_rect.expand(SF * 500.0);
        for edge in edges {
            let point = match self.tre_sty {
                TreSty::PhyGrm => {
                    node_point_cart(self.tre_vs.w, self.tre_vs.h, edge)
                }
                TreSty::Fan => node_point_pol(
                    edge_angle(self.opn_angle, edge) + self.rotation,
                    self.tre_vs.radius_min,
                    self.root_len,
                    edge,
                ),
            };

            if vis_rect_expanded.contains(point + self.translation) {
                self.vis_edge_idxs.push(edge.edge_index);
            }
        }
    }
}
