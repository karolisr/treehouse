use super::{TreeStateMsg, TreeStyle};
use crate::{
    Float, PADDING, PI, SF, TREE_LAB_FONT_NAME, TreeViewMsg,
    utils::{
        NodePoint, branch_labels, clip_rect_from_bounds, draw_edges, draw_labels, draw_node,
        draw_scale_bar, node_labels, paths_from_chunks,
    },
};
use dendros::Edges;
use iced::{
    Event, Point, Radians, Rectangle, Renderer, Size, Theme,
    alignment::Vertical,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{
        Action, Cache, Geometry, Path, Program, Stroke, Text,
        path::Arc,
        stroke::{LineCap, LineJoin},
    },
    window::Event as WinEvent,
};

#[derive(Debug, Default)]
pub(crate) struct TreeCnv {
    visible_nodes: Vec<NodePoint>,
    pub(crate) drawing_enabled: bool,
    pub(crate) tree_height: Float,

    pub(crate) sel_tree_style_opt: TreeStyle,
    pub(crate) tree_edges_chunked: Vec<Edges>,
    pub(crate) is_rooted: bool,
    pub(crate) opn_angle: Float,
    pub(crate) rot_angle: Float,

    pub(crate) show_cursor_line: bool,

    pub(crate) brnch_lab_offset_y: Float,
    pub(crate) brnch_lab_size: Float,
    pub(crate) cursor_x_fraction: Option<Float>,
    pub(crate) extra_space_for_tip_labs: Float,
    pub(crate) int_lab_offset_x: Float,
    pub(crate) int_lab_size: Float,
    pub(crate) max_lab_size: Float,
    pub(crate) max_node_size: Float,
    pub(crate) max_tip_labs_to_draw: usize,
    pub(crate) min_lab_size: Float,
    pub(crate) min_node_size: Float,
    pub(crate) node_radius: Float,
    pub(crate) node_size: Float,
    pub(crate) tip_lab_offset_x: Float,
    pub(crate) tip_lab_size: Float,
    pub(crate) tip_lab_w: Float,

    pub(crate) g_cursor_line: Cache,
    pub(crate) g_edge: Cache,
    pub(crate) g_frame: Cache,
    pub(crate) g_lab_brnch: Cache,
    pub(crate) g_lab_int: Cache,
    pub(crate) g_lab_tip: Cache,
    pub(crate) g_legend: Cache,
    pub(crate) g_node_found_iter: Cache,
    pub(crate) g_node_found: Cache,
    pub(crate) g_node_hover: Cache,
    pub(crate) g_node_sel: Cache,

    #[cfg(debug_assertions)]
    pub(crate) g_bounds: Cache,
    #[cfg(debug_assertions)]
    pub(crate) g_palette: Cache,
}

pub struct TreeCnvState {
    pub(crate) clip_rect: Rectangle,
    pub(crate) tree_rect: Rectangle,
    pub(crate) cursor_point: Option<Point>,
    pub(crate) center: Point,
    pub(crate) size: Float,
    pub(crate) lab_txt_template: Text,
    pub(crate) closest_node_point: Option<NodePoint>,
    pub(crate) mouse_hovering_node: bool,
    pub(crate) stroke: Stroke<'static>,
}

impl Default for TreeCnvState {
    fn default() -> Self {
        Self {
            lab_txt_template: Text {
                font: iced::Font {
                    family: iced::font::Family::Name(TREE_LAB_FONT_NAME),
                    ..Default::default()
                },
                size: iced::Pixels(2e1),
                align_y: Vertical::Center,
                ..Default::default()
            },
            closest_node_point: None,
            mouse_hovering_node: false,
            stroke: Stroke {
                width: SF,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                ..Default::default()
            },
            cursor_point: None,
            clip_rect: Default::default(),
            tree_rect: Default::default(),
            center: Default::default(),
            size: Default::default(),
        }
    }
}

impl Program<TreeViewMsg> for TreeCnv {
    type State = TreeCnvState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        state.clip_rect = clip_rect_from_bounds(bounds);
        state.tree_rect = state.clip_rect;
        state.size = state.tree_rect.width / 2e0;
        state.center = state.tree_rect.center();
        match event {
            Event::Mouse(MouseEvent::ButtonPressed(button)) => match button {
                iced::mouse::Button::Left => {
                    if state.mouse_hovering_node {
                        if let Some(hovered_node) = &state.closest_node_point {
                            return Some(Action::publish(TreeViewMsg::TreeStateMsg(
                                TreeStateMsg::SelectDeselectNode(hovered_node.edge.node_id),
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
                    if let Some(pt) = cursor.position_in(bounds) {
                        if pt.x < 0e0 || pt.x > 0e0 + state.clip_rect.width {
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

                    let cursor_point;
                    if let Some(x) = cursor.position_in(bounds) {
                        cursor_point = x;
                    } else {
                        return None;
                    }

                    state.cursor_point = Some(cursor_point);

                    let closest_pt: Option<&NodePoint> =
                        self.visible_nodes.iter().min_by(|&a, &b| {
                            cursor_point
                                .distance(a.point)
                                .total_cmp(&cursor_point.distance(b.point))
                        });

                    if let Some(NodePoint { point, edge, angle }) = closest_pt {
                        if cursor_point.distance(*point) <= self.node_radius {
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
                                    TreeStyle::Phylogram => {
                                        let x_frac = point.x / state.tree_rect.width;
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
                                    TreeStyle::Fan => {
                                        let x_frac = state.center.distance(*point) / state.size;
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
                                    TreeStyle::Phylogram => {
                                        let x_frac = cursor_point.x / state.tree_rect.width;
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
                                    TreeStyle::Fan => {
                                        let x_frac =
                                            state.center.distance(cursor_point) / state.size;
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

        // #[cfg(debug_assertions)]
        // let color_bg_weakest = palette_ex.background.weakest.color;
        // #[cfg(debug_assertions)]
        // let color_bg_weak = palette_ex.background.weak.color;
        // #[cfg(debug_assertions)]
        // let color_bg_base = palette_ex.background.base.color;
        // #[cfg(debug_assertions)]
        // let color_bg_strong = palette_ex.background.strong.color;
        // #[cfg(debug_assertions)]
        // let color_bg_strongest = palette_ex.background.strongest.color;

        // #[cfg(debug_assertions)]
        // let color_primary_weak = palette_ex.primary.weak.color;
        // let color_primary_base = palette_ex.primary.base.color;
        let color_primary_strong = palette_ex.primary.strong.color;

        // #[cfg(debug_assertions)]
        // let color_secondary_weak = palette_ex.secondary.weak.color;
        // #[cfg(debug_assertions)]
        // let color_secondary_base = palette_ex.secondary.base.color;
        let color_secondary_strong = palette_ex.secondary.strong.color;

        // #[cfg(debug_assertions)]
        // let color_success_base = palette_ex.success.base.color;
        // let color_warning_base = palette_ex.warning.base.color;
        // let color_danger_base = palette_ex.danger.base.color;

        let mut lab_txt_template = state.lab_txt_template.clone();
        lab_txt_template.color = color_text;
        let stroke = state.stroke.with_color(color_text);

        // if self.has_brlen && self.draw_legend {
        let g_legend = self.g_legend.draw(renderer, bounds.size(), |f| {
            draw_scale_bar(self.tree_height, stroke, &lab_txt_template, &state.tree_rect, f);
        });
        geoms.push(g_legend);
        // }

        let g_edge = self.g_edge.draw(renderer, bounds.size(), |f| {
            let paths = paths_from_chunks(
                state.tree_rect.width, state.tree_rect.height, state.center, state.size,
                self.is_rooted, self.rot_angle, self.opn_angle, self.sel_tree_style_opt,
                &self.tree_edges_chunked,
            );
            draw_edges(paths, stroke, &state.tree_rect, f);
        });
        geoms.push(g_edge);

        // if self.tip_brnch_labs_allowed && self.has_tip_labs && self.draw_tip_labs {
        // let g_lab_tip = self.g_lab_tip.draw(renderer, bounds.size(), |f| {
        //     let labels = node_labels(&self.visible_nodes, true, &lab_txt_template);
        //     draw_labels(
        //         labels,
        //         self.tip_lab_size,
        //         Point { x: self.tip_lab_offset_x, y: 0e0 },
        //         &state.tree_rect,
        //         &state.clip_rect,
        //         f,
        //     );
        // });

        // geoms.push(g_lab_tip);
        // }

        // if self.has_int_labs && self.draw_int_labs {
        // let g_lab_int = self.g_lab_int.draw(renderer, bounds.size(), |f| {
        //     let labels = node_labels(&self.visible_nodes, false, &lab_txt_template);
        //     draw_labels(
        //         labels,
        //         self.int_lab_size,
        //         Point { x: self.int_lab_offset_x, y: 0e0 },
        //         &state.tree_rect,
        //         &state.clip_rect,
        //         f,
        //     );
        // });
        // geoms.push(g_lab_int);
        // }

        // if self.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
        // let g_lab_brnch = self.g_lab_brnch.draw(renderer, bounds.size(), |f| {
        //     let labels = branch_labels(state.size, &self.visible_nodes, &lab_txt_template);
        //     draw_labels(
        //         labels,
        //         self.brnch_lab_size,
        //         Point { x: 0e0, y: self.brnch_lab_offset_y },
        //         &state.tree_rect,
        //         &state.clip_rect,
        //         f,
        //     );
        // });
        // geoms.push(g_lab_brnch);
        // }

        let g_node_hover = self.g_node_hover.draw(renderer, bounds.size(), |f| {
            if let Some(NodePoint { point, edge: _, angle: _ }) = &state.closest_node_point {
                draw_node(
                    point,
                    self.node_radius,
                    stroke,
                    color_secondary_strong.scale_alpha(0.75),
                    &state.tree_rect,
                    f,
                );
            }
        });
        geoms.push(g_node_hover);

        // if let Some(pt) = self.found_edge_pt {
        //     let g_node_found_iter = self.g_node_found_iter.draw(renderer, bounds.size(), |f| {
        //         let ps = self.node_radius * 1e0;
        //         draw_node(
        //             &pt,
        //             ps,
        //             stroke,
        //             color_danger_base.scale_alpha(0.75),
        //             &state.tree_rect,
        //             f,
        //         );
        //     });
        //     geoms.push(g_node_found_iter);
        // }

        // let g_node_sel = self.g_node_sel.draw(renderer, bounds.size(), |f| {
        //     let ps = self.node_radius * 0.75;
        //     for NodePoint { point, edge, angle: _ } in &self.visible_nodes {
        //         for node_id in &self.sel_node_ids {
        //             if edge.node_id == *node_id {
        //                 draw_node(
        //                     point,
        //                     ps,
        //                     stroke,
        //                     color_warning_base.scale_alpha(0.75),
        //                     &state.tree_rect,
        //                     f,
        //                 );
        //             }
        //         }
        //     }
        // });
        // geoms.push(g_node_sel);

        // let g_node_found = self.g_node_found.draw(renderer, bounds.size(), |f| {
        //     let ps = self.node_radius * 0.5;
        //     for NodePoint { point, edge, angle: _ } in &self.visible_nodes {
        //         for node_id in &self.found_node_ids {
        //             if edge.node_id == *node_id {
        //                 draw_node(
        //                     point,
        //                     ps,
        //                     stroke,
        //                     color_primary_base.scale_alpha(0.75),
        //                     &state.tree_rect,
        //                     f,
        //                 );
        //             }
        //         }
        //     }
        // });
        // geoms.push(g_node_found);

        // if self.show_cursor_line {
        let g_cursor_line = self.g_cursor_line.draw(renderer, bounds.size(), |f| {
            if state.cursor_point.is_some() || self.cursor_x_fraction.is_some() {
                let x: Float;
                let radius: Float;
                if let Some(x_frac) = self.cursor_x_fraction {
                    x = x_frac * state.size;
                    radius = x;
                } else {
                    let pt = state.cursor_point.unwrap();
                    x = pt.x;
                    radius = state.center.distance(pt);
                }
                let path = Path::new(|p| match self.sel_tree_style_opt {
                    TreeStyle::Phylogram => {
                        if x <= state.tree_rect.width {
                            p.move_to(Point { x: x + state.tree_rect.x, y: state.tree_rect.y });
                            p.line_to(Point {
                                x: x + state.tree_rect.x,
                                y: state.tree_rect.y + state.tree_rect.height,
                            });
                        }
                    }

                    TreeStyle::Fan => {
                        if x > 0e0 && radius <= state.size + SF {
                            let center = Point {
                                x: state.center.x + state.tree_rect.x,
                                y: state.center.y + state.tree_rect.y,
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
        // }

        // #[cfg(debug_assertions)]
        // {
        //     let g_palette = self.g_palette.draw(renderer, bounds.size(), |f| {
        //         let colors_bg = [
        //             color_bg_base, color_bg_weakest, color_bg_weak, color_bg_strong,
        //             color_bg_strongest,
        //         ];

        //         let colors_primary =
        //             [color_primary_base, color_primary_weak, color_primary_strong, color_text];

        //         let colors_secondary =
        //             [color_secondary_base, color_secondary_weak, color_secondary_strong];

        //         let colors_other = [color_success_base, color_warning_base, color_danger_base];

        //         let color_rect_size = SF * 15e0;
        //         let palette_rect_w = 2e0 * PADDING + color_rect_size * 5e0;
        //         let palette_rect_h = 2e0 * PADDING + color_rect_size * 4e0;
        //         let palette_rect_x = state.tree_rect.x + PADDING;
        //         let palette_rect_y =
        //             state.tree_rect.y + state.tree_rect.height - palette_rect_h - PADDING;

        //         f.fill_rectangle(
        //             Point { x: palette_rect_x, y: palette_rect_y },
        //             iced::Size { width: palette_rect_w, height: palette_rect_h },
        //             color_bg_base,
        //         );

        //         f.stroke_rectangle(
        //             Point { x: palette_rect_x + SF / 2e0, y: palette_rect_y + SF / 2e0 },
        //             iced::Size {
        //                 width: 2e0 * PADDING + color_rect_size * 5e0 - SF,
        //                 height: 2e0 * PADDING + color_rect_size * 4e0 - SF,
        //             },
        //             stroke.with_color(color_text),
        //         );

        //         for (i, c) in colors_bg.iter().enumerate() {
        //             f.fill_rectangle(
        //                 Point {
        //                     x: palette_rect_x + PADDING + color_rect_size * i as Float,
        //                     y: palette_rect_y + PADDING,
        //                 },
        //                 iced::Size { width: color_rect_size, height: color_rect_size },
        //                 *c,
        //             );
        //         }

        //         for (i, c) in colors_primary.iter().enumerate() {
        //             f.fill_rectangle(
        //                 Point {
        //                     x: palette_rect_x + PADDING + color_rect_size * i as Float,
        //                     y: palette_rect_y + PADDING + color_rect_size * 1e0,
        //                 },
        //                 iced::Size { width: color_rect_size, height: color_rect_size },
        //                 *c,
        //             );
        //         }

        //         for (i, c) in colors_secondary.iter().enumerate() {
        //             f.fill_rectangle(
        //                 Point {
        //                     x: palette_rect_x + PADDING + color_rect_size * i as Float,
        //                     y: palette_rect_y + PADDING + color_rect_size * 2e0,
        //                 },
        //                 iced::Size { width: color_rect_size, height: color_rect_size },
        //                 *c,
        //             );
        //         }

        //         for (i, c) in colors_other.iter().enumerate() {
        //             f.fill_rectangle(
        //                 Point {
        //                     x: palette_rect_x + PADDING + color_rect_size * i as Float,
        //                     y: palette_rect_y + PADDING + color_rect_size * 3e0,
        //                 },
        //                 iced::Size { width: color_rect_size, height: color_rect_size },
        //                 *c,
        //             );
        //         }
        //     });
        //     geoms.push(g_palette);
        // }
        geoms
    }
}
