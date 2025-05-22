use crate::iced::*;
use crate::*;

#[derive(Debug, Default)]
pub(super) struct PlotCnv {}
impl PlotCnv {}

#[derive(Debug, Default)]
pub struct St {}

impl Program<TvMsg> for PlotCnv {
    type State = St;
    fn mouse_interaction(&self, _st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction { Interaction::default() }
    fn update(&self, _st: &mut St, _ev: &Event, _bnds: Rectangle, _crsr: Cursor) -> Option<Action<TvMsg>> { None }
    fn draw(&self, _st: &St, _rndr: &Renderer, _thm: &Theme, _bnds: Rectangle, _crsr: Cursor) -> Vec<Geometry> {
        vec![]
    }
}
