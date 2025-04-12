use super::{
    super::{TreeView, TreeViewMsg},
    NodePoint, TreeViewState,
};
use crate::{
    ColorSimple, NodeType,
    app::{PADDING, SF},
};
use iced::{
    Event, Rectangle, Renderer, Size, Theme, Vector,
    border::Radius,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{Action, Geometry, Path, Program, Stroke, stroke},
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
                        width: bounds.width - PADDING * 2e0,
                        height: self.canvas_h,
                    };

                    state.tree_rect = Rectangle {
                        x: state.clip_rect.x + SF,
                        y: state.clip_rect.y + SF + self.max_label_size / 2e0,
                        width: state.clip_rect.width - SF * 2e0 - self.tip_label_w,
                        height: state.clip_rect.height - SF * 2e0 - self.max_label_size,
                    };

                    state.tip_idx_range = self.visible_tip_range();
                    if let Some(tip_idx_range) = &state.tip_idx_range {
                        state.visible_nodes = self.visible_nodes(
                            state.tree_rect.width,
                            state.tree_rect.height,
                            tip_idx_range,
                        );
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
                let pointer_rect = Rectangle {
                    width: state.clip_rect.width + PADDING * 2e0,
                    ..state.clip_rect
                };
                if cursor.is_over(bounds) && self.drawing_enabled {
                    let mut mouse_pt;
                    if let Some(x) = cursor.position_in(pointer_rect) {
                        mouse_pt = x;
                    } else {
                        return None;
                    }

                    mouse_pt.x -= state.ps;
                    mouse_pt.y -= self.max_label_size;

                    let closest_pt: Option<&NodePoint> =
                        state.visible_nodes.iter().min_by(|&a, &b| {
                            mouse_pt
                                .distance(a.point)
                                .total_cmp(&mouse_pt.distance(b.point))
                        });

                    if let Some(NodePoint { point, edge }) = closest_pt {
                        if mouse_pt.distance(*point) <= SF * 9e0 {
                            state.mouse_hovering_node = true;
                            if state.closest_node_point.is_none()
                                || state.closest_node_point.clone().unwrap().edge.node_id
                                    != edge.node_id
                            {
                                self.pointer_geom_cache.clear();
                                state.closest_node_point = Some(NodePoint {
                                    point: *point,
                                    edge: edge.clone(),
                                });
                                Some(Action::request_redraw())
                            } else {
                                state.closest_node_point = Some(NodePoint {
                                    point: *point,
                                    edge: edge.clone(),
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
                        state.closest_node_point = None;
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
        if state.mouse_hovering_node {
            Interaction::Pointer
        } else {
            Interaction::default()
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        if self.drawing_enabled {
            let mut geoms: Vec<Geometry> = Vec::new();
            let g_edges = self
                .edge_geom_cache
                .draw(renderer, state.clip_rect.size(), |f| {
                    let stroke = Stroke {
                        width: SF,
                        line_cap: stroke::LineCap::Square,
                        line_join: stroke::LineJoin::Round,
                        ..Default::default()
                    };
                    let paths =
                        self.paths_from_chunks(state.tree_rect.width, state.tree_rect.height);
                    self.draw_edges(paths, stroke, f);
                });
            geoms.push(g_edges);

            if let Some(tip_idx_range) = &state.tip_idx_range {
                if self.draw_tip_labels_allowed && self.draw_tip_labels_selection {
                    let g_tip_labels =
                        self.tip_labels_geom_cache
                            .draw(renderer, state.clip_rect.size(), |f| {
                                let labels = self.tip_labels_in_range(
                                    state.tree_rect.width,
                                    state.tree_rect.height,
                                    tip_idx_range.b,
                                    tip_idx_range.e,
                                    &state.tree_label_template,
                                );
                                self.draw_labels(
                                    labels,
                                    self.tip_label_size,
                                    self.tip_label_offset,
                                    state.clip_rect,
                                    f,
                                );
                            });
                    geoms.push(g_tip_labels);
                }

                let g_selected_nodes =
                    self.selected_nodes_geom_cache
                        .draw(renderer, state.clip_rect.size(), |f| {
                            let stroke = Stroke {
                                width: SF,
                                line_cap: stroke::LineCap::Square,
                                line_join: stroke::LineJoin::Round,
                                style: ColorSimple::RED.into(),
                                ..Default::default()
                            };
                            f.with_save(|f| {
                                f.translate(Vector {
                                    x: SF - state.ps / 2e0,
                                    y: SF + self.max_label_size / 2e0 - state.ps / 2e0,
                                });
                                let path = Path::new(|p| {
                                    for NodePoint { point, edge } in &state.visible_nodes {
                                        for node_id in &self.selected_node_ids {
                                            if edge.node_id == *node_id {
                                                p.rounded_rectangle(
                                                    *point,
                                                    Size::new(state.ps, state.ps),
                                                    Radius::new(state.ps),
                                                );
                                            }
                                        }
                                    }
                                });
                                f.fill(&path, ColorSimple::YEL.scale_alpha(0.75));
                                f.stroke(&path, stroke);
                            });
                        });
                geoms.push(g_selected_nodes);

                let g_pointer =
                    self.pointer_geom_cache
                        .draw(renderer, state.clip_rect.size(), |f| {
                            if let Some(NodePoint { point, edge: _ }) = &state.closest_node_point {
                                f.with_save(|f| {
                                    f.translate(Vector {
                                        x: SF - state.ps / 2e0,
                                        y: SF + self.max_label_size / 2e0 - state.ps / 2e0,
                                    });
                                    let path = Path::new(|p| {
                                        p.rounded_rectangle(
                                            *point,
                                            Size::new(state.ps, state.ps),
                                            Radius::new(state.ps),
                                        );
                                    });
                                    f.fill(&path, ColorSimple::RED.scale_alpha(0.75));
                                });
                            }
                        });
                geoms.push(g_pointer);
            }

            if self.draw_int_labels_selection {
                let g_int_labels =
                    self.int_labels_geom_cache
                        .draw(renderer, state.clip_rect.size(), |f| {
                            let labels = self.labels_from_chunks(
                                state.tree_rect.width,
                                state.tree_rect.height,
                                NodeType::Internal,
                                &state.tree_label_template,
                            );
                            self.draw_labels(
                                labels,
                                self.int_label_size,
                                self.int_label_offset,
                                state.clip_rect,
                                f,
                            );
                        });
                geoms.push(g_int_labels);
            }

            geoms
        } else {
            vec![]
        }
    }
}
