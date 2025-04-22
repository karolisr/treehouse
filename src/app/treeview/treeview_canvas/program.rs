use super::{
    super::{TreeView, TreeViewMsg},
    NodePoint, TreeViewState,
};
use crate::{
    ColorSimple,
    app::{PADDING, SCROLL_TOOL_W, SF},
};
use iced::{
    Event, Point, Rectangle, Renderer, Theme,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{Action, Geometry, Program},
    window::Event as WinEvent,
};

impl Program<TreeViewMsg> for TreeView {
    type State = TreeViewState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        match event {
            Event::Window(WinEvent::RedrawRequested(_)) => {
                if self.drawing_enabled {
                    state.clip_rect = Rectangle {
                        x: 0e0,
                        y: 0e0,
                        width: bounds.width - SCROLL_TOOL_W + PADDING,
                        height: self.canvas_h,
                    };

                    state.tree_rect = match self.selected_tree_repr_option {
                        crate::app::treeview::TreeReprOption::Phylogram => Rectangle {
                            x: state.clip_rect.x + SF,
                            y: state.clip_rect.y
                                + SF
                                + self.max_label_size
                                + self.branch_label_offset_y,
                            width: state.clip_rect.width - SF * 2e0 - self.tip_label_w,
                            height: state.clip_rect.height
                                - SF * 2e0
                                - self.max_label_size * 1.5
                                - SCROLL_TOOL_W,
                        },
                        crate::app::treeview::TreeReprOption::Fan => Rectangle {
                            x: state.clip_rect.x + SF + self.tip_label_w,
                            y: state.clip_rect.y + SF + self.tip_label_w,
                            width: state.clip_rect.width - SF * 2e0 - self.tip_label_w * 2e0,
                            height: state.clip_rect.height
                                - SF * 2e0
                                - self.tip_label_w * 2e0
                                - SCROLL_TOOL_W,
                        },
                    };

                    state.tip_idx_range = self.visible_tip_idx_range();
                    if let Some(tip_idx_range) = &state.tip_idx_range {
                        let x = self.visible_nodes(
                            state.tree_rect.width,
                            state.tree_rect.height,
                            tip_idx_range,
                        );
                        state.visible_nodes = x.points;
                        state.center = x.center;
                        state.size = x.size;
                    } else {
                        state.visible_nodes.clear();
                    }
                }
                None
            }
            Event::Window(WinEvent::Resized(size)) => Some(Action::publish(
                TreeViewMsg::WindowResized(size.width, size.height),
            )),
            Event::Mouse(MouseEvent::ButtonPressed(button)) => match button {
                iced::mouse::Button::Left => {
                    if state.mouse_hovering_node {
                        if let Some(hovered_node) = &state.closest_node_point {
                            return Some(Action::publish(TreeViewMsg::SelectDeselectNode(
                                hovered_node.edge.node_id,
                            )));
                        }
                    }
                    None
                }
                iced::mouse::Button::Right => None,
                iced::mouse::Button::Middle => None,
                iced::mouse::Button::Back => None,
                iced::mouse::Button::Forward => None,
                iced::mouse::Button::Other(_) => None,
            },
            Event::Mouse(MouseEvent::CursorMoved { position: _ }) => {
                if cursor.is_over(bounds) && self.drawing_enabled {
                    #[cfg(debug_assertions)]
                    self.debug_geom_cache.clear();

                    let mut mouse_pt;
                    if let Some(x) = cursor.position_over(bounds) {
                        mouse_pt = x;
                    } else {
                        return None;
                    }

                    mouse_pt.x -= PADDING + state.tree_rect.x;
                    mouse_pt.y -= PADDING + state.tree_rect.y;

                    let closest_pt: Option<&NodePoint> =
                        state.visible_nodes.iter().min_by(|&a, &b| {
                            mouse_pt
                                .distance(a.point)
                                .total_cmp(&mouse_pt.distance(b.point))
                        });

                    if let Some(NodePoint { point, edge, angle }) = closest_pt {
                        if mouse_pt.distance(*point) <= state.node_radius {
                            state.mouse_hovering_node = true;
                            if state.closest_node_point.is_none()
                                || state.closest_node_point.clone().unwrap().edge.node_id
                                    != edge.node_id
                            {
                                self.pointer_geom_cache.clear();
                                state.closest_node_point = Some(NodePoint {
                                    point: *point,
                                    edge: edge.clone(),
                                    angle: *angle,
                                });
                                Some(Action::request_redraw())
                            } else {
                                state.closest_node_point = Some(NodePoint {
                                    point: *point,
                                    edge: edge.clone(),
                                    angle: *angle,
                                });
                                None
                            }
                        } else {
                            state.mouse_hovering_node = false;
                            state.closest_node_point = None;
                            self.pointer_geom_cache.clear();
                            Some(Action::request_redraw())
                        }
                    } else {
                        state.mouse_hovering_node = false;
                        state.closest_node_point = None;
                        self.pointer_geom_cache.clear();
                        None
                    }
                } else {
                    state.mouse_hovering_node = false;
                    state.closest_node_point = None;
                    self.pointer_geom_cache.clear();
                    None
                }
            }
            _ => None,
        }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        if state.mouse_hovering_node { Interaction::Pointer } else { Interaction::default() }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        #[cfg(not(debug_assertions))] _bounds: Rectangle,
        #[cfg(debug_assertions)] bounds: Rectangle,
        #[cfg(not(debug_assertions))] _cursor: Cursor,
        #[cfg(debug_assertions)] cursor: Cursor,
    ) -> Vec<Geometry> {
        if !self.drawing_enabled {
            return vec![];
        }

        let mut geoms: Vec<Geometry> = Vec::new();

        #[cfg(debug_assertions)]
        {
            let g_bounds = self.debug_geom_cache.draw(renderer, bounds.size(), |f| {
                f.fill_rectangle(
                    Point { x: state.clip_rect.x, y: state.clip_rect.y },
                    state.clip_rect.size(),
                    ColorSimple::CYA.scale_alpha(0.125),
                );

                f.fill_rectangle(
                    Point { x: state.tree_rect.x, y: state.tree_rect.y },
                    state.tree_rect.size(),
                    ColorSimple::MAG.scale_alpha(0.125),
                );

                if let Some(pt) = cursor.position_over(state.clip_rect) {
                    let path = iced::widget::canvas::Path::new(|p| {
                        p.circle(
                            Point { x: pt.x - PADDING, y: pt.y - PADDING },
                            state.node_radius + SF * 2e0,
                        );
                    });
                    f.stroke(&path, state.stroke.with_color(ColorSimple::RED));
                }
            });
            geoms.push(g_bounds);
        }

        if self.has_brlen && self.draw_legend {
            let g_legend = self
                .legend_geom_cache
                .draw(renderer, state.clip_rect.size(), |f| {
                    self.draw_scale_bar(
                        state.stroke,
                        &state.tree_label_text_template,
                        &state.tree_rect,
                        f,
                    );
                });
            geoms.push(g_legend);
        }

        let g_edges = self
            .edge_geom_cache
            .draw(renderer, state.clip_rect.size(), |f| {
                let paths = self.paths_from_chunks(
                    state.tree_rect.width,
                    state.tree_rect.height,
                    state.center,
                    state.size,
                );
                self.draw_edges(paths, state.stroke, &state.tree_rect, f);
            });
        geoms.push(g_edges);

        if self.draw_tip_branch_labels_allowed && self.has_tip_labels && self.draw_tip_labels {
            let g_tip_labels =
                self.tip_labels_geom_cache
                    .draw(renderer, state.clip_rect.size(), |f| {
                        let labels = self.node_labels(
                            &state.visible_nodes,
                            true,
                            &state.tree_label_text_template,
                        );
                        self.draw_labels(
                            labels,
                            self.tip_label_size,
                            Point { x: self.tip_label_offset_x, y: 0e0 },
                            &state.tree_rect,
                            &state.clip_rect,
                            f,
                        );
                    });

            geoms.push(g_tip_labels);
        }

        if self.has_int_labels && self.draw_int_labels {
            let g_int_labels =
                self.int_labels_geom_cache
                    .draw(renderer, state.clip_rect.size(), |f| {
                        let labels = self.node_labels(
                            &state.visible_nodes,
                            false,
                            &state.tree_label_text_template,
                        );
                        self.draw_labels(
                            labels,
                            self.int_label_size,
                            Point { x: self.int_label_offset_x, y: 0e0 },
                            &state.tree_rect,
                            &state.clip_rect,
                            f,
                        );
                    });
            geoms.push(g_int_labels);
        }

        if self.has_brlen && self.draw_tip_branch_labels_allowed && self.draw_branch_labels {
            let g_branch_labels =
                self.branch_labels_geom_cache
                    .draw(renderer, state.clip_rect.size(), |f| {
                        let labels = self.branch_labels(
                            state.size,
                            &state.visible_nodes,
                            &state.tree_label_text_template,
                        );
                        self.draw_labels(
                            labels,
                            self.branch_label_size,
                            Point { x: 0e0, y: self.branch_label_offset_y },
                            &state.tree_rect,
                            &state.clip_rect,
                            f,
                        );
                    });
            geoms.push(g_branch_labels);
        }

        let g_selected_nodes =
            self.selected_nodes_geom_cache
                .draw(renderer, state.clip_rect.size(), |f| {
                    let ps = state.node_radius * 0.75;
                    for NodePoint { point, edge, angle: _ } in &state.visible_nodes {
                        for node_id in &self.selected_node_ids {
                            if edge.node_id == *node_id {
                                self.draw_node(
                                    point,
                                    ps,
                                    state.stroke,
                                    ColorSimple::YEL.scale_alpha(0.75),
                                    &state.tree_rect,
                                    f,
                                );
                            }
                        }
                    }
                });
        geoms.push(g_selected_nodes);

        let g_pointer = self
            .pointer_geom_cache
            .draw(renderer, state.clip_rect.size(), |f| {
                if let Some(NodePoint { point, edge: _, angle: _ }) = &state.closest_node_point {
                    self.draw_node(
                        point,
                        state.node_radius,
                        state.stroke,
                        ColorSimple::BLU.scale_alpha(0.75),
                        &state.tree_rect,
                        f,
                    );
                }
            });
        geoms.push(g_pointer);

        geoms
    }
}
