use super::{
    super::{TreeView, TreeViewMsg},
    NodePoint, TreeViewState,
};
use crate::{
    ColorSimple,
    app::{PADDING, SCROLL_TOOL_W, SF, treeview::TreeReprOption},
};
use iced::{
    Event, Point, Rectangle, Renderer, Size, Theme, Vector,
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
                        width: bounds.width - SCROLL_TOOL_W + PADDING,
                        height: self.canvas_h,
                    };

                    state.tree_rect = Rectangle {
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
                    };

                    state.tip_idx_range = self.visible_tip_idx_range();
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

                    mouse_pt.x += state.tree_rect.x - state.ps;
                    mouse_pt.y -= state.tree_rect.y + state.ps;

                    let closest_pt: Option<&NodePoint> =
                        state.visible_nodes.iter().min_by(|&a, &b| {
                            mouse_pt
                                .distance(a.point)
                                .total_cmp(&mouse_pt.distance(b.point))
                        });

                    if let Some(NodePoint { point, edge }) = closest_pt {
                        if mouse_pt.distance(*point) <= state.ps {
                            state.mouse_hovering_node = true;
                            if state.closest_node_point.is_none()
                                || state.closest_node_point.clone().unwrap().edge.node_id
                                    != edge.node_id
                            {
                                self.pointer_geom_cache.clear();
                                state.closest_node_point =
                                    Some(NodePoint { point: *point, edge: edge.clone() });
                                Some(Action::request_redraw())
                            } else {
                                state.closest_node_point =
                                    Some(NodePoint { point: *point, edge: edge.clone() });
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
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        if !self.drawing_enabled {
            return vec![];
        }

        if self.selected_tree_repr_option.is_none() {
            return vec![];
        }

        let tree_repr = self.selected_tree_repr_option.unwrap();

        let fan_size = state.tree_rect.width.min(state.tree_rect.height) as f64 / 2e0;
        let fan_center = Point {
            x: state.tree_rect.width as f64 / 2e0,
            y: state.tree_rect.height as f64 / 2e0,
        };

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
            });
            geoms.push(g_bounds);
        }

        if self.has_brlen && self.draw_legend {
            let g_legend = self
                .legend_geom_cache
                .draw(renderer, state.clip_rect.size(), |f| {
                    let stroke = Stroke {
                        width: SF,
                        line_cap: stroke::LineCap::Square,
                        line_join: stroke::LineJoin::Round,
                        // style: ColorSimple::BLU.into(),
                        ..Default::default()
                    };
                    self.draw_scale_bar(stroke, &state.tree_label_template, &state.tree_rect, f);
                });
            geoms.push(g_legend);
        }

        let g_edges = self
            .edge_geom_cache
            .draw(renderer, state.clip_rect.size(), |f| {
                let stroke = Stroke {
                    width: SF,
                    line_cap: stroke::LineCap::Square,
                    line_join: stroke::LineJoin::Round,
                    ..Default::default()
                };
                let paths = match tree_repr {
                    TreeReprOption::Phylogram => {
                        self.paths_from_chunks(state.tree_rect.width, state.tree_rect.height)
                    }
                    TreeReprOption::Fan => self.paths_from_chunks_fan(fan_size, fan_center),
                };

                self.draw_edges(paths, stroke, &state.tree_rect, f);
            });
        geoms.push(g_edges);

        if let Some(tip_idx_range) = &state.tip_idx_range {
            if self.draw_tip_branch_labels_allowed && self.has_tip_labels && self.draw_tip_labels {
                let g_tip_labels =
                    self.tip_labels_geom_cache
                        .draw(renderer, state.clip_rect.size(), |f| match tree_repr {
                            TreeReprOption::Phylogram => {
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
                                    Point { x: self.tip_label_offset_x, y: 0e0 },
                                    &state.tree_rect,
                                    &state.clip_rect,
                                    f,
                                );
                            }
                            TreeReprOption::Fan => {
                                let labels = self.tip_labels_in_range_fan(
                                    fan_size,
                                    fan_center,
                                    tip_idx_range.b,
                                    tip_idx_range.e,
                                    &state.tree_label_template,
                                );
                                self.draw_labels_fan(
                                    labels,
                                    self.tip_label_size,
                                    Point { x: self.tip_label_offset_x, y: 0e0 },
                                    &state.tree_rect,
                                    &state.clip_rect,
                                    f,
                                )
                            }
                        });

                geoms.push(g_tip_labels);
            }

            if self.has_int_labels && self.draw_int_labels {
                let g_int_labels =
                    self.int_labels_geom_cache
                        .draw(renderer, state.clip_rect.size(), |f| match tree_repr {
                            TreeReprOption::Phylogram => {
                                let labels = self.visible_int_node_labels(
                                    state.tree_rect.width,
                                    state.tree_rect.height,
                                    &state.visible_nodes,
                                    &state.tree_label_template,
                                );
                                self.draw_labels(
                                    labels,
                                    self.int_label_size,
                                    Point { x: self.int_label_offset_x, y: 0e0 },
                                    &state.tree_rect,
                                    &state.clip_rect,
                                    f,
                                );
                            }
                            TreeReprOption::Fan => {
                                let labels = self.visible_int_node_labels_fan(
                                    fan_size,
                                    fan_center,
                                    &state.visible_nodes,
                                    &state.tree_label_template,
                                );

                                self.draw_labels_fan(
                                    labels,
                                    self.int_label_size,
                                    Point { x: self.int_label_offset_x, y: 0e0 },
                                    &state.tree_rect,
                                    &state.clip_rect,
                                    f,
                                )
                            }
                        });
                geoms.push(g_int_labels);
            }

            if self.has_brlen && self.draw_tip_branch_labels_allowed && self.draw_branch_labels {
                let g_branch_labels =
                    self.branch_labels_geom_cache
                        .draw(renderer, state.clip_rect.size(), |f| {
                            let labels = self.visible_branch_labels(
                                state.tree_rect.width,
                                state.tree_rect.height,
                                &state.visible_nodes,
                                &state.tree_label_template,
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
                        let stroke = Stroke {
                            width: SF * 2e0,
                            line_cap: stroke::LineCap::Square,
                            line_join: stroke::LineJoin::Round,
                            style: ColorSimple::RED.into(),
                            ..Default::default()
                        };
                        let ps = state.ps * 0.75;
                        f.with_save(|f| {
                            f.translate(Vector {
                                x: state.tree_rect.x - ps / 2e0,
                                y: state.tree_rect.y - ps / 2e0,
                            });
                            let path = Path::new(|p| {
                                for NodePoint { point, edge } in &state.visible_nodes {
                                    for node_id in &self.selected_node_ids {
                                        if edge.node_id == *node_id {
                                            p.rounded_rectangle(
                                                *point,
                                                Size::new(ps, ps),
                                                Radius::new(ps),
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

            let g_pointer = self
                .pointer_geom_cache
                .draw(renderer, state.clip_rect.size(), |f| {
                    if let Some(NodePoint { point, edge: _ }) = &state.closest_node_point {
                        let stroke = Stroke {
                            width: SF,
                            line_cap: stroke::LineCap::Square,
                            line_join: stroke::LineJoin::Round,
                            style: ColorSimple::BLK.scale_alpha(0.65).into(),
                            ..Default::default()
                        };
                        f.with_save(|f| {
                            f.translate(Vector {
                                x: state.tree_rect.x - state.ps / 2e0,
                                y: state.tree_rect.y - state.ps / 2e0,
                            });
                            let path = Path::new(|p| {
                                p.rounded_rectangle(
                                    *point,
                                    Size::new(state.ps, state.ps),
                                    Radius::new(state.ps),
                                );
                            });
                            f.fill(&path, ColorSimple::BLU.scale_alpha(0.75));
                            f.stroke(&path, stroke);
                        });
                    }
                });
            geoms.push(g_pointer);
        }

        geoms
    }
}
