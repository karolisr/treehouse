use super::{TreeView, TreeViewMsg, TreeViewState};
use crate::Float;
use crate::{PADDING, SF};
use iced::widget::canvas::stroke;
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Event as MouseEvent, Interaction},
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
            Event::Window(WinEvent::Resized(size)) => {
                // println!("{size:?}");
                Some(Action::publish(TreeViewMsg::WindowResized(
                    size.width,
                    size.height,
                )))
            }
            Event::Mouse(MouseEvent::CursorMoved { position: _ }) => {
                // println!("{position}");
                None
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
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        if self.drawing_enabled {
            let mut geoms: Vec<Geometry> = Vec::new();

            let r: Rectangle = Rectangle {
                x: 0e0,
                y: 0e0,
                width: bounds.width - PADDING,
                height: self.canvas_h,
            };

            // Hack for approximating label width ---------------------------------------
            let mut label_w: Float = self.max_name_len as Float * self.tip_label_size / 3e0;
            if !self.draw_tip_labels {
                label_w = 0e0
            }
            let tree_rect = Rectangle {
                width: r.width - label_w,
                ..r
            };
            // --------------------------------------------------------------------------

            let g_edges = self.edge_geom_cache.draw(renderer, r.size(), |f| {
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
                let g_tip_labels = self.tip_labels_geom_cache.draw(renderer, r.size(), |f| {
                    let labels = self.tip_labels_from_chunks(tree_rect.width, tree_rect.height);
                    self.draw_labels(labels, self.tip_label_size, r, f);
                });
                geoms.push(g_tip_labels);
            }

            if self.draw_int_labels {
                let g_int_labels = self.int_labels_geom_cache.draw(renderer, r.size(), |f| {
                    let labels = self.int_labels_from_chunks(tree_rect.width, tree_rect.height);
                    self.draw_labels(labels, self.int_label_size, r, f);
                });
                geoms.push(g_int_labels);
            }

            geoms
        } else {
            vec![]
        }
    }
}
