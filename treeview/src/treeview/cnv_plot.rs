use crate::{
    TreeViewMsg,
    utils::{clip_rect_from_bounds, draw_point, draw_rectangle},
};
use iced::{
    Event, Point, Rectangle, Renderer, Theme,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{Action, Cache, Geometry, Program},
};

#[derive(Debug, Default)]
pub(crate) struct PlotCnv {
    g_bounds: Cache,
}

impl PlotCnv {}

#[derive(Debug, Default)]
pub struct PlotCnvState {
    cursor_point: Option<Point>,
    clip_rect: Option<Rectangle>,
}

impl Program<TreeViewMsg> for PlotCnv {
    type State = PlotCnvState;

    fn mouse_interaction(
        &self,
        state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        if state.cursor_point.is_some() { Interaction::Crosshair } else { Interaction::default() }
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        state.clip_rect = Some(clip_rect_from_bounds(bounds));
        if let Some(cursor_pt) = cursor.position_in(bounds) {
            match event {
                Event::Mouse(MouseEvent::CursorMoved { position: _ }) => {
                    self.g_bounds.clear();
                    state.cursor_point = Some(cursor_pt);
                    Some(Action::request_redraw())
                }
                _ => None,
            }
        } else {
            self.g_bounds.clear();
            state.clip_rect = None;
            state.cursor_point = None;
            None
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();
        if let Some(clip_rect) = state.clip_rect {
            let g_bounds = self.g_bounds.draw(renderer, bounds.size(), |frame| {
                draw_rectangle(clip_rect, frame);
                if let Some(cursor_point) = state.cursor_point {
                    draw_point(cursor_point, frame);
                }
            });
            geoms.push(g_bounds);
        }
        geoms
    }
}
