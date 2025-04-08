use super::{TreeView, TreeViewMsg, TreeViewState};
use crate::{ColorSimple, NodeType, PADDING, SF};
use iced::border::Radius;
use iced::widget::canvas::{Path, stroke};
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{Action, Geometry, Program, Stroke},
    window::Event as WinEvent,
};
use iced::{Point, Size};

impl Program<TreeViewMsg> for TreeView {
    type State = TreeViewState;

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        match event {
            Event::Window(WinEvent::Resized(size)) => Some(Action::publish(
                TreeViewMsg::WindowResized(size.width, size.height),
            )),
            Event::Mouse(MouseEvent::CursorMoved { position: _ }) => {
                if cursor.is_over(bounds) {
                    self.pointer_geom_cache.clear();
                    Some(Action::request_redraw())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        Interaction::default()
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        if self.drawing_enabled {
            let mut geoms: Vec<Geometry> = Vec::new();

            let clipping: Rectangle = Rectangle {
                x: 0e0,
                y: 0e0,
                width: bounds.width - PADDING * 2e0,
                height: self.canvas_h,
            };

            let tree_rect = Rectangle {
                x: clipping.x + SF,
                y: clipping.y + SF + self.max_label_size / 2e0,
                width: clipping.width - SF * 2e0 - self.tip_label_w,
                height: clipping.height - SF * 2e0 - self.max_label_size,
            };

            let g_edges = self.edge_geom_cache.draw(renderer, clipping.size(), |f| {
                let stroke = Stroke {
                    width: SF,
                    line_cap: stroke::LineCap::Square,
                    line_join: stroke::LineJoin::Round,
                    ..Default::default()
                };
                let paths = self.paths_from_chunks(tree_rect.width, tree_rect.height);
                self.draw_edges(paths, stroke, f);
            });
            geoms.push(g_edges);

            if self.draw_tip_labels_allowed && self.draw_tip_labels_selection {
                let g_tip_labels =
                    self.tip_labels_geom_cache
                        .draw(renderer, clipping.size(), |f| {
                            let tip_idx_0: i64 = (self.cnv_y0 / self.node_size) as i64 - 3;
                            let tip_idx_1: i64 = (self.cnv_y1 / self.node_size) as i64 + 3;
                            let tip_idx_0: usize = tip_idx_0.max(0) as usize;
                            let tip_idx_1: usize = tip_idx_1.min(self.tip_count as i64) as usize;

                            if tip_idx_0 < tip_idx_1 {
                                let labels = self.tip_labels_in_range(
                                    tree_rect.width,
                                    tree_rect.height,
                                    tip_idx_0,
                                    tip_idx_1,
                                    &state.tree_label_template,
                                );
                                self.draw_labels(
                                    labels,
                                    self.tip_label_size,
                                    self.tip_label_offset,
                                    clipping,
                                    f,
                                );
                            }
                        });
                geoms.push(g_tip_labels);
            }

            if self.draw_int_labels_selection {
                let g_int_labels =
                    self.int_labels_geom_cache
                        .draw(renderer, clipping.size(), |f| {
                            let labels = self.labels_from_chunks(
                                tree_rect.width,
                                tree_rect.height,
                                NodeType::Internal,
                                &state.tree_label_template,
                            );
                            self.draw_labels(
                                labels,
                                self.int_label_size,
                                self.int_label_offset,
                                clipping,
                                f,
                            );
                        });
                geoms.push(g_int_labels);
            }

            let g_pointer = self
                .pointer_geom_cache
                .draw(renderer, clipping.size(), |f| {
                    if let Some(mouse_point) = cursor.position_in(clipping) {
                        let ps = SF * 2e1;
                        let mx = mouse_point.x;
                        let my = mouse_point.y;
                        let pnt = Point::new(mx - ps, my - ps);
                        let path = Path::new(|p| {
                            p.rounded_rectangle(pnt, Size::new(ps, ps), Radius::new(ps));
                        });
                        f.fill(&path, ColorSimple::RED);
                    }
                });
            geoms.push(g_pointer);

            geoms
        } else {
            vec![]
        }
    }
}
