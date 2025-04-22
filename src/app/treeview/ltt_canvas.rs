use crate::ColorSimple;

use super::TreeViewMsg;
use iced::{
    Event, Point, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Cache, Geometry, Program},
};

#[derive(Debug, Default)]
pub struct Ltt {
    pub ltt_geom_cache: Cache,
}

#[derive(Debug, Default)]
pub struct LttState {}

impl Program<TreeViewMsg> for Ltt {
    type State = LttState;

    fn update(
        &self,
        _state: &mut Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        self.ltt_geom_cache.clear();
        None
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if cursor.is_over(bounds) { Interaction::Crosshair } else { Interaction::default() }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();
        let g_bounds = self.ltt_geom_cache.draw(renderer, bounds.size(), |f| {
            f.fill_rectangle(
                Point { x: 0e0, y: 0e0 },
                bounds.size(),
                ColorSimple::GRN.scale_alpha(0.125),
            );
        });
        geoms.push(g_bounds);
        geoms
    }
}
