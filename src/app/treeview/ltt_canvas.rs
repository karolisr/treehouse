use super::TreeViewMsg;
#[cfg(debug_assertions)]
use crate::app::DEBUG;
use crate::{
    Float, LttPoint,
    app::{PADDING, SCROLL_TOOL_W, SF},
};
use iced::{
    Event, Point, Rectangle, Renderer, Theme,
    mouse::{Cursor, Event as MouseEvent, Interaction},
    widget::canvas::{
        Action, Cache, Geometry, Path, Program, Stroke,
        stroke::{LineCap, LineJoin},
    },
    window::Event as WinEvent,
};

#[derive(Debug, Default)]
pub struct Ltt {
    pub ltt_points: Option<Vec<LttPoint>>,

    pub cursor_x_fraction: Option<Float>,

    pub tree_rect_x: Float,
    pub tree_rect_w: Float,

    pub g_bounds: Cache,
    pub g_ltt: Cache,
    pub g_crosshairs: Cache,
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
    pub crosshairs: Option<Point>,
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
            crosshairs: None,
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
        cursor: Cursor,
    ) -> Option<Action<TreeViewMsg>> {
        match event {
            Event::Window(WinEvent::RedrawRequested(_)) => {
                state.clip_rect = Rectangle {
                    x: 0e0,
                    y: 0e0,
                    width: bounds.width - SCROLL_TOOL_W + PADDING,
                    height: bounds.height,
                };

                state.ltt_rect = Rectangle {
                    x: self.tree_rect_x,
                    y: state.clip_rect.y + SF / 2e0 + PADDING,
                    width: self.tree_rect_w,
                    height: state.clip_rect.height - SF - SCROLL_TOOL_W,
                };
                None
            }
            Event::Mouse(MouseEvent::CursorMoved { position: _ }) => {
                if cursor.is_over(bounds) {
                    let mut mouse_pt;
                    if let Some(x) = cursor.position_over(bounds) {
                        mouse_pt = x;
                    } else {
                        return None;
                    }

                    mouse_pt.x -= PADDING + state.ltt_rect.x;
                    mouse_pt.y -= PADDING + state.ltt_rect.y;

                    state.crosshairs =
                        Some(Point { x: mouse_pt.x - SF / 2e0, y: mouse_pt.y - SF / 2e0 });

                    let x_frac = state.crosshairs.unwrap().x / state.ltt_rect.width;

                    if (0e0..=1e0).contains(&x_frac) {
                        Some(Action::publish(TreeViewMsg::CursorOnLttCnv {
                            x: Some(x_frac),
                        }))
                    } else {
                        state.crosshairs = None;
                        Some(Action::publish(TreeViewMsg::CursorOnLttCnv { x: None }))
                    }
                } else {
                    state.crosshairs = None;
                    None
                }
            }
            _ => {
                state.crosshairs = None;
                None
            }
        }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        if state.crosshairs.is_some() { Interaction::Crosshair } else { Interaction::default() }
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
        let color_primary_strong = palette_ex.primary.strong.color;
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
                if let Some(x_frac) = self.cursor_x_fraction {
                    let x = state.ltt_rect.x + state.ltt_rect.width * x_frac;
                    if x >= state.ltt_rect.x && x <= state.ltt_rect.x + state.ltt_rect.width {
                        let path = Path::new(|p| {
                            p.move_to(Point { x, y: state.ltt_rect.y });
                            p.line_to(Point { x, y: state.ltt_rect.y + state.ltt_rect.height });
                        });
                        f.stroke(&path, stroke.with_color(color_primary_strong));
                    }
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
                        let y = state.ltt_rect.y + state.ltt_rect.height
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
