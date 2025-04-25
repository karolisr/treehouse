use std::f32::consts::PI;

use super::{
    super::{TreeView, TreeViewMsg},
    NodePoint, TreeViewState,
};
#[cfg(debug_assertions)]
use crate::app::DEBUG;
use crate::{
    Float,
    app::{PADDING, SCROLL_TOOL_W, SF},
};
use iced::{
    Event, Point, Radians, Rectangle, Renderer, Size, Theme,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{Action, Geometry, Path, Program, path::Arc},
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
                /////////////////////////////////////////////////////////////////////////////////
                if self.drawing_enabled {
                    state.clip_rect = Rectangle {
                        x: 0e0,
                        y: 0e0,
                        width: bounds.width - SCROLL_TOOL_W + PADDING,
                        height: self.tre_cnv_h,
                    };

                    state.tree_rect = match self.sel_tree_style_opt {
                        crate::app::treeview::TreeStyleOption::Phylogram => Rectangle {
                            x: state.clip_rect.x + SF / 2e0 + state.node_radius,
                            y: state.clip_rect.y
                                + SF / 2e0
                                + self.max_lab_size
                                + self.brnch_lab_offset_y,
                            width: state.clip_rect.width
                                - SF
                                - state.node_radius * 2e0
                                - self.tip_lab_w,
                            height: state.clip_rect.height
                                - SF
                                - self.max_lab_size * 1.5
                                - SCROLL_TOOL_W,
                        },
                        crate::app::treeview::TreeStyleOption::Fan => Rectangle {
                            x: state.clip_rect.x + SF / 2e0 + self.tip_lab_w,
                            y: state.clip_rect.y + SF / 2e0 + self.tip_lab_w,
                            width: state.clip_rect.width - SF - self.tip_lab_w * 2e0,
                            height: state.clip_rect.height
                                - SF
                                - self.tip_lab_w * 2e0
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
                /////////////////////////////////////////////////////////////////////////////////
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
                    self.g_bounds.clear();

                    let mut mouse_pt;
                    if let Some(x) = cursor.position_over(bounds) {
                        mouse_pt = x;
                    } else {
                        return None;
                    }

                    mouse_pt.x -= PADDING + state.tree_rect.x;
                    mouse_pt.y -= PADDING + state.tree_rect.y;

                    state.crosshairs =
                        Some(Point { x: mouse_pt.x - SF / 2e0, y: mouse_pt.y - SF / 2e0 });

                    let closest_pt: Option<&NodePoint> =
                        state.visible_nodes.iter().min_by(|&a, &b| {
                            mouse_pt
                                .distance(a.point)
                                .total_cmp(&mouse_pt.distance(b.point))
                        });

                    if let Some(NodePoint { point, edge, angle }) = closest_pt {
                        if mouse_pt.distance(*point) <= state.node_radius {
                            state.mouse_hovering_node = true;
                            state.crosshairs = Some(*point);
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
                                // Some(Action::request_redraw())
                                self.g_crosshairs.clear();
                                match self.sel_tree_style_opt {
                                    crate::app::treeview::TreeStyleOption::Phylogram => {
                                        Some(Action::publish(TreeViewMsg::CursorPosition {
                                            x: point.x,
                                            y: point.y,
                                        }))
                                    }
                                    crate::app::treeview::TreeStyleOption::Fan => {
                                        Some(Action::publish(TreeViewMsg::CursorPosition {
                                            x: state.center.distance(*point) / state.size
                                                // * (state.clip_rect.width - SF - PADDING * 2e0),
                                                * (self.ltt_cnv_w - SCROLL_TOOL_W + PADDING - SF - PADDING * 2e0),
                                            y: 0e0,
                                        }))
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
                            // Some(Action::request_redraw())
                            self.g_crosshairs.clear();
                            if let Some(ch) = state.crosshairs {
                                match self.sel_tree_style_opt {
                                    crate::app::treeview::TreeStyleOption::Phylogram => {
                                        Some(Action::publish(TreeViewMsg::CursorPosition {
                                            x: ch.x,
                                            y: ch.y,
                                        }))
                                    }
                                    crate::app::treeview::TreeStyleOption::Fan => {
                                        Some(Action::publish(TreeViewMsg::CursorPosition {
                                            x: state.center.distance(ch) / state.size
                                                // * (state.clip_rect.width - SF - PADDING * 2e0),
                                                * (self.ltt_cnv_w - SCROLL_TOOL_W + PADDING - SF - PADDING * 2e0),
                                            y: 0e0,
                                        }))
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
                    state.crosshairs = None;
                    state.mouse_hovering_node = false;
                    state.closest_node_point = None;
                    self.g_node_hover.clear();
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
        if state.mouse_hovering_node { Interaction::Pointer } else { Interaction::Crosshair }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        #[cfg(not(debug_assertions))] _bounds: Rectangle,
        #[cfg(debug_assertions)] bounds: Rectangle,
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
        let color_bg_weak = palette_ex.background.weak.color;
        let color_bg_base = palette_ex.background.base.color;
        let color_bg_strong = palette_ex.background.strong.color;
        let color_bg_strongest = palette_ex.background.strongest.color;

        let color_primary_weak = palette_ex.primary.weak.color;
        let color_primary_base = palette_ex.primary.base.color;
        let color_primary_strong = palette_ex.primary.strong.color;

        let color_secondary_weak = palette_ex.secondary.weak.color;
        let color_secondary_base = palette_ex.secondary.base.color;
        let color_secondary_strong = palette_ex.secondary.strong.color;

        let color_success_base = palette_ex.success.base.color;
        let color_warning_base = palette_ex.warning.base.color;
        let color_danger_base = palette_ex.danger.base.color;

        let mut lab_txt_template = state.lab_txt_template.clone();
        lab_txt_template.color = color_text;
        let stroke = state.stroke.with_color(color_text);

        #[cfg(debug_assertions)]
        if DEBUG {
            let g_bounds = self.g_bounds.draw(renderer, bounds.size(), |f| {
                f.fill_rectangle(
                    Point { x: state.clip_rect.x, y: state.clip_rect.y },
                    state.clip_rect.size(),
                    color_text.scale_alpha(0.05),
                );

                f.fill_rectangle(
                    Point { x: state.tree_rect.x, y: state.tree_rect.y },
                    state.tree_rect.size(),
                    color_text.scale_alpha(0.05),
                );

                if let Some(pt) = state.crosshairs {
                    let path = iced::widget::canvas::Path::new(|p| {
                        p.circle(
                            Point { x: pt.x + state.tree_rect.x, y: pt.y + state.tree_rect.y },
                            state.node_radius + SF * 2e0,
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

        if self.has_brlen && self.draw_legend {
            let g_legend = self.g_legend.draw(renderer, state.clip_rect.size(), |f| {
                self.draw_scale_bar(stroke, &lab_txt_template, &state.tree_rect, f);
            });
            geoms.push(g_legend);
        }

        let g_edge = self.g_edge.draw(renderer, state.clip_rect.size(), |f| {
            let paths = self.paths_from_chunks(
                state.tree_rect.width,
                state.tree_rect.height,
                state.center,
                state.size,
            );
            self.draw_edges(paths, stroke, &state.tree_rect, f);
        });
        geoms.push(g_edge);

        if self.tip_brnch_labs_allowed && self.has_tip_labs && self.draw_tip_labs {
            let g_lab_tip = self.g_lab_tip.draw(renderer, state.clip_rect.size(), |f| {
                let labels = self.node_labels(&state.visible_nodes, true, &lab_txt_template);
                self.draw_labels(
                    labels,
                    self.tip_lab_size,
                    Point { x: self.tip_lab_offset_x, y: 0e0 },
                    &state.tree_rect,
                    &state.clip_rect,
                    f,
                );
            });

            geoms.push(g_lab_tip);
        }

        if self.has_int_labs && self.draw_int_labs {
            let g_lab_int = self.g_lab_int.draw(renderer, state.clip_rect.size(), |f| {
                let labels = self.node_labels(&state.visible_nodes, false, &lab_txt_template);
                self.draw_labels(
                    labels,
                    self.int_lab_size,
                    Point { x: self.int_lab_offset_x, y: 0e0 },
                    &state.tree_rect,
                    &state.clip_rect,
                    f,
                );
            });
            geoms.push(g_lab_int);
        }

        if self.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            let g_lab_brnch = self
                .g_lab_brnch
                .draw(renderer, state.clip_rect.size(), |f| {
                    let labels =
                        self.branch_labels(state.size, &state.visible_nodes, &lab_txt_template);
                    self.draw_labels(
                        labels,
                        self.brnch_lab_size,
                        Point { x: 0e0, y: self.brnch_lab_offset_y },
                        &state.tree_rect,
                        &state.clip_rect,
                        f,
                    );
                });
            geoms.push(g_lab_brnch);
        }

        let g_node_sel = self.g_node_sel.draw(renderer, state.clip_rect.size(), |f| {
            let ps = state.node_radius * 0.75;
            for NodePoint { point, edge, angle: _ } in &state.visible_nodes {
                for node_id in &self.sel_node_ids {
                    if edge.node_id == *node_id {
                        self.draw_node(
                            point,
                            ps,
                            stroke,
                            color_warning_base.scale_alpha(0.75),
                            &state.tree_rect,
                            f,
                        );
                    }
                }
            }
        });
        geoms.push(g_node_sel);

        let g_node_hover = self
            .g_node_hover
            .draw(renderer, state.clip_rect.size(), |f| {
                if let Some(NodePoint { point, edge: _, angle: _ }) = &state.closest_node_point {
                    self.draw_node(
                        point,
                        state.node_radius,
                        stroke,
                        color_secondary_strong.scale_alpha(0.75),
                        &state.tree_rect,
                        f,
                    );
                }
            });
        geoms.push(g_node_hover);

        let g_cross = self
            .g_crosshairs
            .draw(renderer, state.clip_rect.size(), |f| {
                if let Some(ch) = state.crosshairs {
                    let path = Path::new(|p| match self.sel_tree_style_opt {
                        crate::app::treeview::TreeStyleOption::Phylogram => {
                            p.move_to(Point { x: 0e0, y: ch.y + state.tree_rect.y });
                            p.line_to(Point { x: f.width(), y: ch.y + state.tree_rect.y });
                            p.move_to(Point { x: ch.x + state.tree_rect.x, y: 0e0 });
                            p.line_to(Point { x: ch.x + state.tree_rect.x, y: f.height() });
                        }
                        crate::app::treeview::TreeStyleOption::Fan => {
                            let arc: Arc = Arc {
                                center: state.center,
                                radius: state.center.distance(ch),
                                start_angle: Radians(0e0),
                                end_angle: Radians(2e0 * PI),
                            };
                            p.arc(arc);
                        }
                    });
                    f.stroke(&path, stroke.with_color(color_primary_strong));
                }
            });
        geoms.push(g_cross);

        #[cfg(debug_assertions)]
        if DEBUG {
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
                let palette_rect_x = state.tree_rect.x + PADDING;
                let palette_rect_y =
                    state.tree_rect.y + state.tree_rect.height - palette_rect_h - PADDING;

                f.fill_rectangle(
                    Point { x: palette_rect_x, y: palette_rect_y },
                    Size { width: palette_rect_w, height: palette_rect_h },
                    color_bg_base,
                );

                f.stroke_rectangle(
                    Point { x: palette_rect_x + SF / 2e0, y: palette_rect_y + SF / 2e0 },
                    Size {
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
                        Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_primary.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * 1e0,
                        },
                        Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_secondary.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * 2e0,
                        },
                        Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_other.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * 3e0,
                        },
                        Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }
            });
            geoms.push(g_palette);
        }
        geoms
    }
}
