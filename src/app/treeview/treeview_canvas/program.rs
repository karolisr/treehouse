use super::{
    super::{TreeStyleOption, TreeView, TreeViewMsg},
    NodePoint, TreeViewState,
};
use crate::{
    Float,
    app::{PADDING, SF, TTR_H},
};
use iced::{
    Event, Point, Radians, Rectangle, Renderer, Theme,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{Action, Geometry, Path, Program, path::Arc},
    window::Event as WinEvent,
};
use std::f32::consts::PI;

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
                    if let Some(pt) = cursor.position_over(bounds) {
                        if pt.x < PADDING * 3e0 + SF
                            || pt.x > PADDING * 3e0 + SF + self.clip_rect.width
                        {
                            state.cursor_point = None;
                            state.mouse_hovering_node = false;
                            state.closest_node_point = None;
                            self.g_node_hover.clear();
                            return Some(Action::publish(TreeViewMsg::CursorOnLttCnv { x: None }));
                        }
                    } else {
                        state.cursor_point = None;
                        state.mouse_hovering_node = false;
                        state.closest_node_point = None;
                        self.g_node_hover.clear();
                        return Some(Action::publish(TreeViewMsg::CursorOnLttCnv { x: None }));
                    }

                    let mut mouse_pt;
                    if let Some(x) = cursor.position_over(bounds) {
                        mouse_pt = x;
                    } else {
                        return None;
                    }

                    mouse_pt.x -= PADDING * 2e0 + self.tree_rect.x;
                    mouse_pt.y -= TTR_H + PADDING * 3e0 + self.tree_rect.y;

                    state.cursor_point =
                        Some(Point { x: mouse_pt.x - SF / 2e0, y: mouse_pt.y - SF / 2e0 });

                    let closest_pt: Option<&NodePoint> =
                        self.visible_nodes.iter().min_by(|&a, &b| {
                            mouse_pt
                                .distance(a.point)
                                .total_cmp(&mouse_pt.distance(b.point))
                        });

                    if let Some(NodePoint { point, edge, angle }) = closest_pt {
                        if mouse_pt.distance(*point) <= self.node_radius {
                            state.mouse_hovering_node = true;
                            state.cursor_point = Some(*point);
                            if state.closest_node_point.is_none()
                                || state.closest_node_point.clone().unwrap().edge.node_id
                                    != edge.node_id
                            {
                                self.g_node_hover.clear();
                                state.closest_node_point = Some(NodePoint {
                                    point: *point,
                                    edge: edge.clone(),
                                    angle: *angle,
                                });
                                self.g_cursor_line.clear();
                                match self.sel_tree_style_opt {
                                    TreeStyleOption::Phylogram => {
                                        let x_frac = point.x / self.tree_rect.width;
                                        if x_frac <= 1e0 {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: Some(x_frac),
                                            }))
                                        } else {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: None,
                                            }))
                                        }
                                    }
                                    TreeStyleOption::Fan => {
                                        let x_frac = self.center.distance(*point) / self.size;
                                        if x_frac <= 1e0 + f32::EPSILON {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: Some(x_frac),
                                            }))
                                        } else {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: None,
                                            }))
                                        }
                                    }
                                }
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
                            self.g_node_hover.clear();
                            self.g_cursor_line.clear();
                            if let Some(cursor_point) = state.cursor_point {
                                match self.sel_tree_style_opt {
                                    TreeStyleOption::Phylogram => {
                                        let x_frac = cursor_point.x / self.tree_rect.width;
                                        if x_frac <= 1e0 {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: Some(x_frac),
                                            }))
                                        } else {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: None,
                                            }))
                                        }
                                    }
                                    TreeStyleOption::Fan => {
                                        let x_frac = self.center.distance(cursor_point) / self.size;
                                        if x_frac <= 1e0 + f32::EPSILON {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: Some(x_frac),
                                            }))
                                        } else {
                                            Some(Action::publish(TreeViewMsg::CursorOnTreCnv {
                                                x: None,
                                            }))
                                        }
                                    }
                                }
                            } else {
                                None
                            }
                        }
                    } else {
                        state.mouse_hovering_node = false;
                        state.closest_node_point = None;
                        self.g_node_hover.clear();
                        None
                    }
                } else {
                    state.cursor_point = None;
                    state.mouse_hovering_node = false;
                    state.closest_node_point = None;
                    self.g_node_hover.clear();
                    None
                }
            }
            Event::Mouse(MouseEvent::CursorLeft) => {
                state.cursor_point = None;
                state.mouse_hovering_node = false;
                state.closest_node_point = None;
                self.g_node_hover.clear();
                Some(Action::publish(TreeViewMsg::CursorOnLttCnv { x: None }))
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
        } else if state.cursor_point.is_some() {
            Interaction::Crosshair
        } else {
            Interaction::default()
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        if !self.drawing_enabled {
            return vec![];
        }

        let mut geoms: Vec<Geometry> = Vec::new();

        let palette = theme.palette();
        let palette_ex = theme.extended_palette();

        let color_text = palette.text;

        #[cfg(debug_assertions)]
        let color_bg_weakest = palette_ex.background.weakest.color;
        #[cfg(debug_assertions)]
        let color_bg_weak = palette_ex.background.weak.color;
        #[cfg(debug_assertions)]
        let color_bg_base = palette_ex.background.base.color;
        #[cfg(debug_assertions)]
        let color_bg_strong = palette_ex.background.strong.color;
        #[cfg(debug_assertions)]
        let color_bg_strongest = palette_ex.background.strongest.color;

        #[cfg(debug_assertions)]
        let color_primary_weak = palette_ex.primary.weak.color;
        let color_primary_base = palette_ex.primary.base.color;
        let color_primary_strong = palette_ex.primary.strong.color;

        #[cfg(debug_assertions)]
        let color_secondary_weak = palette_ex.secondary.weak.color;
        #[cfg(debug_assertions)]
        let color_secondary_base = palette_ex.secondary.base.color;
        let color_secondary_strong = palette_ex.secondary.strong.color;

        #[cfg(debug_assertions)]
        let color_success_base = palette_ex.success.base.color;
        let color_warning_base = palette_ex.warning.base.color;
        let color_danger_base = palette_ex.danger.base.color;

        let mut lab_txt_template = state.lab_txt_template.clone();
        lab_txt_template.color = color_text;
        let stroke = state.stroke.with_color(color_text);

        #[cfg(debug_assertions)]
        if crate::app::DEBUG {
            let g_bounds = self.g_bounds.draw(renderer, bounds.size(), |f| {
                f.fill_rectangle(
                    Point { x: self.clip_rect.x, y: self.clip_rect.y },
                    self.clip_rect.size(),
                    color_text.scale_alpha(0.05),
                );

                f.fill_rectangle(
                    Point { x: self.tree_rect.x, y: self.tree_rect.y },
                    self.tree_rect.size(),
                    color_text.scale_alpha(0.05),
                );

                if let Some(pt) = state.cursor_point {
                    let path = iced::widget::canvas::Path::new(|p| {
                        p.circle(
                            Point { x: pt.x + self.tree_rect.x, y: pt.y + self.tree_rect.y },
                            self.node_radius + SF * 2e0,
                        );
                    });
                    f.stroke(
                        &path,
                        stroke.with_color(color_danger_base).with_width(SF * 2e0),
                    );
                }
            });
            geoms.push(g_bounds);
        }

        // let g_frame = self.g_frame.draw(renderer, bounds.size(), |f| {
        //     f.stroke_rectangle(
        //         Point {
        //             x: self.clip_rect.x + SF / 2e0,
        //             y: self.clip_rect.y + SF / 2e0,
        //         },
        //         Size {
        //             width: self.clip_rect.width,
        //             height: self.clip_rect.height - SF - PADDING,
        //         },
        //         stroke.with_color(color_primary_strong),
        //     );
        // });
        // geoms.push(g_frame);

        if self.has_brlen && self.draw_legend {
            let g_legend = self.g_legend.draw(renderer, bounds.size(), |f| {
                self.draw_scale_bar(stroke, &lab_txt_template, &self.tree_rect, f);
            });
            geoms.push(g_legend);
        }

        let g_edge = self.g_edge.draw(renderer, bounds.size(), |f| {
            let paths = self.paths_from_chunks(
                self.tree_rect.width,
                self.tree_rect.height,
                self.center,
                self.size,
                self.is_rooted,
            );
            self.draw_edges(paths, stroke, &self.tree_rect, f);
        });
        geoms.push(g_edge);

        if self.tip_brnch_labs_allowed && self.has_tip_labs && self.draw_tip_labs {
            let g_lab_tip = self.g_lab_tip.draw(renderer, bounds.size(), |f| {
                let labels = self.node_labels(&self.visible_nodes, true, &lab_txt_template);
                self.draw_labels(
                    labels,
                    self.tip_lab_size,
                    Point { x: self.tip_lab_offset_x, y: 0e0 },
                    &self.tree_rect,
                    &self.clip_rect,
                    f,
                );
            });

            geoms.push(g_lab_tip);
        }

        if self.has_int_labs && self.draw_int_labs {
            let g_lab_int = self.g_lab_int.draw(renderer, bounds.size(), |f| {
                let labels = self.node_labels(&self.visible_nodes, false, &lab_txt_template);
                self.draw_labels(
                    labels,
                    self.int_lab_size,
                    Point { x: self.int_lab_offset_x, y: 0e0 },
                    &self.tree_rect,
                    &self.clip_rect,
                    f,
                );
            });
            geoms.push(g_lab_int);
        }

        if self.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            let g_lab_brnch = self.g_lab_brnch.draw(renderer, bounds.size(), |f| {
                let labels = self.branch_labels(self.size, &self.visible_nodes, &lab_txt_template);
                self.draw_labels(
                    labels,
                    self.brnch_lab_size,
                    Point { x: 0e0, y: self.brnch_lab_offset_y },
                    &self.tree_rect,
                    &self.clip_rect,
                    f,
                );
            });
            geoms.push(g_lab_brnch);
        }

        let g_node_hover = self.g_node_hover.draw(renderer, bounds.size(), |f| {
            if let Some(NodePoint { point, edge: _, angle: _ }) = &state.closest_node_point {
                self.draw_node(
                    point,
                    self.node_radius,
                    stroke,
                    color_secondary_strong.scale_alpha(0.75),
                    &self.tree_rect,
                    f,
                );
            }
        });
        geoms.push(g_node_hover);

        if let Some(pt) = self.found_edge_pt {
            let g_node_found_iter = self.g_node_found_iter.draw(renderer, bounds.size(), |f| {
                let ps = self.node_radius * 1e0;
                self.draw_node(
                    &pt,
                    ps,
                    stroke,
                    color_danger_base.scale_alpha(0.75),
                    &self.tree_rect,
                    f,
                );
            });
            geoms.push(g_node_found_iter);
        }

        let g_node_sel = self.g_node_sel.draw(renderer, bounds.size(), |f| {
            let ps = self.node_radius * 0.75;
            for NodePoint { point, edge, angle: _ } in &self.visible_nodes {
                for node_id in &self.sel_node_ids {
                    if edge.node_id == *node_id {
                        self.draw_node(
                            point,
                            ps,
                            stroke,
                            color_warning_base.scale_alpha(0.75),
                            &self.tree_rect,
                            f,
                        );
                    }
                }
            }
        });
        geoms.push(g_node_sel);

        let g_node_found = self.g_node_found.draw(renderer, bounds.size(), |f| {
            let ps = self.node_radius * 0.5;
            for NodePoint { point, edge, angle: _ } in &self.visible_nodes {
                for node_id in &self.found_node_ids {
                    if edge.node_id == *node_id {
                        self.draw_node(
                            point,
                            ps,
                            stroke,
                            color_primary_base.scale_alpha(0.75),
                            &self.tree_rect,
                            f,
                        );
                    }
                }
            }
        });
        geoms.push(g_node_found);

        if self.show_cursor_line {
            let g_cursor_line = self.g_cursor_line.draw(renderer, bounds.size(), |f| {
                if state.cursor_point.is_some() || self.cursor_x_fraction.is_some() {
                    let x: Float;
                    let radius: Float;
                    if let Some(x_frac) = self.cursor_x_fraction {
                        x = x_frac * self.size;
                        radius = x;
                    } else {
                        let pt = state.cursor_point.unwrap();
                        x = pt.x;
                        radius = self.center.distance(pt);
                    }
                    let path = Path::new(|p| match self.sel_tree_style_opt {
                        TreeStyleOption::Phylogram => {
                            if x <= self.tree_rect.width {
                                p.move_to(Point { x: x + self.tree_rect.x, y: self.tree_rect.y });
                                p.line_to(Point {
                                    x: x + self.tree_rect.x,
                                    y: self.tree_rect.y + self.tree_rect.height,
                                });
                            }
                        }

                        TreeStyleOption::Fan => {
                            if x > 0e0 && radius <= self.size + SF {
                                let center = Point {
                                    x: self.center.x + self.tree_rect.x,
                                    y: self.center.y + self.tree_rect.y,
                                };

                                let arc: Arc = Arc {
                                    center,
                                    radius,
                                    start_angle: Radians(0e0),
                                    end_angle: Radians(2e0 * PI),
                                };
                                p.arc(arc);
                            }
                        }
                    });
                    f.stroke(&path, stroke.with_color(color_primary_strong));
                }
            });
            geoms.push(g_cursor_line);
        }

        #[cfg(debug_assertions)]
        if crate::app::DEBUG {
            let g_palette = self.g_palette.draw(renderer, bounds.size(), |f| {
                let colors_bg = [
                    color_bg_base,
                    color_bg_weakest,
                    color_bg_weak,
                    color_bg_strong,
                    color_bg_strongest,
                ];

                let colors_primary = [
                    color_primary_base,
                    color_primary_weak,
                    color_primary_strong,
                    color_text,
                ];

                let colors_secondary = [
                    color_secondary_base,
                    color_secondary_weak,
                    color_secondary_strong,
                ];

                let colors_other = [color_success_base, color_warning_base, color_danger_base];

                let color_rect_size = SF * 15e0;
                let palette_rect_w = 2e0 * PADDING + color_rect_size * 5e0;
                let palette_rect_h = 2e0 * PADDING + color_rect_size * 4e0;
                let palette_rect_x = self.tree_rect.x + PADDING;
                let palette_rect_y =
                    self.tree_rect.y + self.tree_rect.height - palette_rect_h - PADDING;

                f.fill_rectangle(
                    Point { x: palette_rect_x, y: palette_rect_y },
                    iced::Size { width: palette_rect_w, height: palette_rect_h },
                    color_bg_base,
                );

                f.stroke_rectangle(
                    Point { x: palette_rect_x + SF / 2e0, y: palette_rect_y + SF / 2e0 },
                    iced::Size {
                        width: 2e0 * PADDING + color_rect_size * 5e0 - SF,
                        height: 2e0 * PADDING + color_rect_size * 4e0 - SF,
                    },
                    stroke.with_color(color_text),
                );

                for (i, c) in colors_bg.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_primary.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * 1e0,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_secondary.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * 2e0,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_other.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * 3e0,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }
            });
            geoms.push(g_palette);
        }
        geoms
    }
}
