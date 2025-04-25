use super::TreeViewMsg;
use crate::Float;
#[cfg(debug_assertions)]
use crate::app::DEBUG;
use crate::app::{PADDING, SCROLL_TOOL_W, SF};
use dendros::LttPoint;
use iced::Point;
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{
        Action, Cache, Geometry, Path, Program, Stroke,
        stroke::{LineCap, LineJoin},
    },
    window::Event as WinEvent,
};

#[derive(Debug, Default)]
pub struct Ltt {
    pub g_bounds: Cache,
    pub g_ltt: Cache,
    pub g_crosshairs: Cache,
    pub crosshairs: Option<Point>,
    pub ltt_points: Option<Vec<LttPoint>>,
}

impl Ltt {
    pub fn set_data(&mut self, ltt_points: Vec<LttPoint>) {
        self.ltt_points = Some(ltt_points);
        self.g_ltt.clear();
    }
}

#[derive(Debug)]
pub struct LttState {
    pub clip_rect: Rectangle,
    pub ltt_rect: Rectangle,
    pub stroke: Stroke<'static>,
}

impl Default for LttState {
    fn default() -> Self {
        Self {
            clip_rect: Default::default(),
            ltt_rect: Default::default(),
            stroke: Stroke {
                width: SF,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                ..Default::default()
            },
        }
    }
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
        self.g_crosshairs.clear();
        match event {
            Event::Window(WinEvent::RedrawRequested(_)) => {
                state.clip_rect = Rectangle {
                    x: 0e0,
                    y: 0e0,
                    width: bounds.width - SCROLL_TOOL_W + PADDING,
                    height: bounds.height,
                };

                state.ltt_rect = Rectangle {
                    x: state.clip_rect.x + SF / 2e0 + PADDING,
                    y: state.clip_rect.y + SF / 2e0,
                    width: state.clip_rect.width - SF - PADDING * 2e0,
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
        let palette_ex = theme.extended_palette();

        let color_text = palette.text;

        let color_bg_weakest = palette_ex.background.weakest.color;
        let color_bg_weak = palette_ex.background.weak.color;
        let color_bg_base = palette_ex.background.base.color;
        let color_bg_strong = palette_ex.background.strong.color;
        let color_bg_strongest = palette_ex.background.strongest.color;

        let color_primary_weak = palette_ex.primary.weak.color;
        let color_primary_base = palette_ex.primary.base.color;
        let color_primary_strong = palette_ex.primary.strong.color;

        let color_secondary_weak = palette_ex.secondary.weak.color;
        let color_secondary_base = palette_ex.secondary.base.color;
        let color_secondary_strong = palette_ex.secondary.strong.color;

        let color_success_base = palette_ex.success.base.color;
        let color_warning_base = palette_ex.warning.base.color;
        let color_danger_base = palette_ex.danger.base.color;

        let stroke = state.stroke.with_color(color_text);

        #[cfg(debug_assertions)]
        if DEBUG {
            let g_bounds = self.g_bounds.draw(renderer, bounds.size(), |f| {
                f.fill_rectangle(
                    Point { x: state.clip_rect.x, y: state.clip_rect.y },
                    state.clip_rect.size(),
                    color_text.scale_alpha(0.05),
                );

                f.fill_rectangle(
                    Point { x: state.ltt_rect.x, y: state.ltt_rect.y },
                    state.ltt_rect.size(),
                    color_text.scale_alpha(0.05),
                );
            });
            geoms.push(g_bounds);
        }

        let g_cross = self
            .g_crosshairs
            .draw(renderer, state.clip_rect.size(), |f| {
                if let Some(ch) = self.crosshairs {
                    let path = Path::new(|p| {
                        p.move_to(Point { x: ch.x + state.ltt_rect.x, y: 0e0 });
                        p.line_to(Point { x: ch.x + state.ltt_rect.x, y: f.height() });
                    });
                    f.stroke(&path, stroke.with_color(color_primary_strong));
                }
            });
        geoms.push(g_cross);

        let g_ltt = self.g_ltt.draw(renderer, state.clip_rect.size(), |f| {
            if let Some(pts) = &self.ltt_points {
                let mut max_count: usize = 0;
                for LttPoint { time: _, count } in pts {
                    max_count = max_count.max(*count)
                }

                let path = Path::new(|p| {
                    for LttPoint { time, count } in pts {
                        let x = *time as Float * state.ltt_rect.width + state.ltt_rect.x;
                        let y = state.ltt_rect.height
                            - (((*count as Float).log10() / (max_count as Float).log10())
                                * state.ltt_rect.height);
                        let pt = Point { x, y };
                        p.circle(pt, SF);
                    }
                });
                f.fill(&path, color_text);
            }
        });
        geoms.push(g_ltt);

        geoms
    }
}
