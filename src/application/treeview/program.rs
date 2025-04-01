use super::{TreeView, TreeViewMsg, TreeViewState};
use crate::{ColorSimple, Float};
use crate::{PADDING, SF};
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, Program, Text},
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
        let mut tip_labels: Vec<Text> = Vec::new();
        let mut int_labels: Vec<Text> = Vec::new();

        let r: Rectangle = Rectangle {
            x: 0e0,
            y: 0e0,
            width: bounds.width - PADDING,
            height: self.canvas_h,
        };

        let g_edges = self.edge_geom_cache.draw(renderer, r.size(), |f| {
            if self.drawing_enabled {
                // Hack for approximating label width ---------------------------------------
                let mut label_w: Float = self.max_name_len as Float * self.node_size / 2.25;
                if !self.draw_tip_labels {
                    label_w = 0e0
                }
                let tree_rect = Rectangle {
                    width: r.width - label_w,
                    ..r
                };
                // --------------------------------------------------------------------------
                let (edge_paths_l, tip_labels_l, int_labels_l) =
                    self.generate_drawables(&tree_rect, PADDING);
                self.draw_edges(edge_paths_l, SF, PADDING, f);
                tip_labels = tip_labels_l;
                int_labels = int_labels_l;
            }
        });

        let g_tip_labels = self.tip_labels_geom_cache.draw(renderer, r.size(), |f| {
            if self.drawing_enabled && self.draw_tip_labels {
                self.draw_labels(
                    tip_labels,
                    self.tip_label_size,
                    &ColorSimple::BLU,
                    SF * 5e0,
                    PADDING,
                    r,
                    f,
                );
            }
        });

        let g_int_labels = self.int_labels_geom_cache.draw(renderer, r.size(), |f| {
            if self.drawing_enabled && self.draw_int_labels {
                self.draw_labels(
                    int_labels,
                    self.int_label_size,
                    &ColorSimple::RED,
                    SF * 2e0,
                    PADDING,
                    r,
                    f,
                );
            }
        });

        vec![g_edges, g_tip_labels, g_int_labels]
    }
}
