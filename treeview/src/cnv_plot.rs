use crate::TvMsg;
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, Program},
};

#[derive(Debug, Default)]
pub(super) struct PlotCnv {}
impl PlotCnv {}

#[derive(Debug, Default)]
pub struct PlotCnvState {}

impl Program<TvMsg> for PlotCnv {
    type State = PlotCnvState;

    fn mouse_interaction(
        &self, _state: &Self::State, _bounds: Rectangle, _cursor: Cursor,
    ) -> Interaction {
        Interaction::default()
    }

    fn update(
        &self, _state: &mut Self::State, _event: &Event, _bounds: Rectangle, _cursor: Cursor,
    ) -> Option<Action<TvMsg>> {
        None
    }

    fn draw(
        &self, _state: &Self::State, _renderer: &Renderer, _theme: &Theme, _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        vec![]
    }
}
