use super::TreeViewMsg;
#[cfg(debug_assertions)]
use crate::app::DEBUG;
use crate::app::{PADDING, SCROLL_TOOL_W, SF};
#[cfg(debug_assertions)]
use iced::Point;
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Cache, Geometry, Program},
    window::Event as WinEvent,
};

#[derive(Debug, Default)]
pub struct Ltt {
    pub g_bounds: Cache,
}

#[derive(Debug, Default)]
pub struct LttState {
    pub clip_rect: Rectangle,
    pub ltt_rect: Rectangle,
}

impl Program<TreeViewMsg> for Ltt {
    type State = LttState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        self.g_bounds.clear();
        match event {
            Event::Window(WinEvent::RedrawRequested(_)) => {
                state.clip_rect = Rectangle {
                    x: 0e0,
                    y: 0e0,
                    width: bounds.width - SCROLL_TOOL_W + PADDING,
                    height: bounds.height,
                };

                state.ltt_rect = Rectangle {
                    x: state.clip_rect.x + SF / 2e0,
                    y: state.clip_rect.y + SF / 2e0,
                    width: state.clip_rect.width - SF,
                    height: state.clip_rect.height - SF,
                };
                None
            }
            _ => None,
        }
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
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        #[cfg(not(debug_assertions))] _bounds: Rectangle,
        #[cfg(debug_assertions)] bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        #[allow(unused_mut)]
        let mut geoms: Vec<Geometry> = Vec::new();

        let palette = theme.palette();
        // let palette_ex = theme.extended_palette();

        let color_text = palette.text;

        // let color_bg_weakest = palette_ex.background.weakest.color;
        // let color_bg_weak = palette_ex.background.weak.color;
        // let color_bg_base = palette_ex.background.base.color;
        // let color_bg_strong = palette_ex.background.strong.color;
        // let color_bg_strongest = palette_ex.background.strongest.color;

        // let color_primary_weak = palette_ex.primary.weak.color;
        // let color_primary_base = palette_ex.primary.base.color;
        // let color_primary_strong = palette_ex.primary.strong.color;

        // let color_secondary_weak = palette_ex.secondary.weak.color;
        // let color_secondary_base = palette_ex.secondary.base.color;
        // let color_secondary_strong = palette_ex.secondary.strong.color;

        // let color_success_base = palette_ex.success.base.color;
        // let color_warning_base = palette_ex.warning.base.color;
        // let color_danger_base = palette_ex.danger.base.color;

        #[cfg(debug_assertions)]
        if DEBUG {
            let g_bounds = self.g_bounds.draw(renderer, bounds.size(), |f| {
                f.fill_rectangle(
                    Point { x: state.clip_rect.x, y: state.clip_rect.y },
                    state.clip_rect.size(),
                    color_text.scale_alpha(0.05),
                );

                // f.fill_rectangle(
                //     Point { x: state.ltt_rect.x, y: state.ltt_rect.y },
                //     state.ltt_rect.size(),
                //     color_text.scale_alpha(0.05),
                // );
            });
            geoms.push(g_bounds);
        }
        geoms
    }
}
