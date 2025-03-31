use super::{TreeView, TreeViewMsg, TreeViewState};
#[allow(unused_imports)]
use crate::ColorSimple;
use crate::{Float, SF};
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
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
        _cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        match event {
            Event::Window(WinEvent::RedrawRequested(_)) => {
                state.set_tree_bounds(&bounds);
                Some(Action::capture())
            }
            _ => Some(Action::capture()),
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
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut geometries: Vec<Geometry> = Vec::new();
        if self.drawing_enabled {
            let lw: Float = SF;
            let offset: Float = lw / 2e0;
            #[allow(unused_variables)]
            let g_bg = self.bg_geom_cache.draw(renderer, bounds.size(), |f| {
                // self.draw_bg(
                //     (0e0, 0e0, f.width(), f.height()),
                //     lw,
                //     offset,
                //     &ColorSimple::BLK,
                //     f,
                // );
            });
            geometries.push(g_bg);
            let g_edges = self.edge_geom_cache.draw(renderer, bounds.size(), |f| {
                self.draw_tree(&state.tree_bounds, lw, offset + lw * 5e0, f);
            });
            geometries.push(g_edges);
        }
        geometries
    }
}
