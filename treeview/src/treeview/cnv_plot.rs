use crate::TreeViewMsg;
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Cache, Geometry, Program},
};

#[derive(Debug, Default)]
pub(crate) struct PlotCnv {
    g_bounds: Cache,
}

impl PlotCnv {}

#[derive(Debug)]
pub struct PlotCnvState {}

impl Default for PlotCnvState {
    fn default() -> Self {
        Self {}
    }
}

impl Program<TreeViewMsg> for PlotCnv {
    type State = PlotCnvState;

    fn update(
        &self,
        _state: &mut Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        None
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
        _renderer: &Renderer,
        _theme: &Theme,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        vec![]
    }
}
