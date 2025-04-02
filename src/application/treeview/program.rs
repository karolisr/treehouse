use super::{TreeView, TreeViewMsg, TreeViewState};
use crate::{PADDING, SF};
use iced::widget::canvas::stroke;
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, Program, Stroke},
    window::Event as WinEvent,
};

impl Program<TreeViewMsg> for TreeView {
    type State = TreeViewState;

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        match event {
            Event::Window(WinEvent::Resized(size)) => Some(Action::publish(
                TreeViewMsg::WindowResized(size.width, size.height),
            )),
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
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
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

            if self.draw_tip_labels {
                let g_tip_labels =
                    self.tip_labels_geom_cache
                        .draw(renderer, clipping.size(), |f| {
                            let labels =
                                self.tip_labels_from_chunks(tree_rect.width, tree_rect.height);
                            self.draw_labels(
                                labels,
                                self.tip_label_size,
                                self.tip_label_offset,
                                clipping,
                                f,
                            );
                        });

                geoms.push(g_tip_labels);
            }

            if self.draw_int_labels {
                let g_int_labels =
                    self.int_labels_geom_cache
                        .draw(renderer, clipping.size(), |f| {
                            let labels =
                                self.int_labels_from_chunks(tree_rect.width, tree_rect.height);
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

            geoms
        } else {
            vec![]
        }
    }
}
