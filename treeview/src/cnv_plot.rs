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
pub struct St {}

impl Program<TvMsg> for PlotCnv {
    type State = St;

    fn mouse_interaction(&self, st: &St, bnds: Rectangle, crsr: Cursor) -> Interaction {
        Interaction::default()
    }

    fn update(
        &self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        None
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, thm: &Theme, bnds: Rectangle, crsr: Cursor,
    ) -> Vec<Geometry> {
        vec![]
    }
}
