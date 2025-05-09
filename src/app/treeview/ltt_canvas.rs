use super::TreeViewMsg;
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

    pub ltt_rect_x: Float,
    pub ltt_rect_w: Float,

    pub g_bounds: Cache,
    pub g_ltt: Cache,
    pub g_cursor_line: Cache,
    pub g_frame: Cache,
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
    pub cursor_point: Option<Point>,
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
            cursor_point: None,
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
                    x: self.ltt_rect_x,
                    y: state.clip_rect.y + SF / 2e0 + PADDING,
                    width: self.ltt_rect_w,
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

                    mouse_pt.x -= PADDING * 2e0 + state.ltt_rect.x;
                    mouse_pt.y -= PADDING * 2e0 + state.ltt_rect.y;

                    state.cursor_point =
                        Some(Point { x: mouse_pt.x - SF / 2e0, y: mouse_pt.y - SF / 2e0 });

                    let x_frac = state.cursor_point.unwrap().x / state.ltt_rect.width;

                    if (0e0..=1e0).contains(&x_frac) {
                        Some(Action::publish(TreeViewMsg::CursorOnLttCnv {
                            x: Some(x_frac),
                        }))
                    } else {
                        state.cursor_point = None;
                        Some(Action::publish(TreeViewMsg::CursorOnLttCnv { x: None }))
                    }
                } else {
                    state.cursor_point = None;
                    None
                }
            }
            _ => {
                state.cursor_point = None;
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
        if state.cursor_point.is_some() { Interaction::Crosshair } else { Interaction::default() }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
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
        if crate::app::DEBUG {
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

        // let g_frame = self.g_frame.draw(renderer, bounds.size(), |f| {
        //     let color_secondary_base = palette_ex.secondary.base.color;
        //     f.stroke_rectangle(
        //         Point {
        //             x: state.clip_rect.x + SF / 2e0,
        //             y: state.clip_rect.y + SF / 2e0,
        //         },
        //         iced::Size {
        //             width: state.clip_rect.width,
        //             height: state.clip_rect.height - SF - PADDING,
        //         },
        //         stroke.with_color(color_secondary_base),
        //     );
        // });
        // geoms.push(g_frame);

        let g_cursor_line = self.g_cursor_line.draw(renderer, bounds.size(), |f| {
            if let Some(x_frac) = self.cursor_x_fraction {
                let x = state.ltt_rect.x + state.ltt_rect.width * x_frac;
                if x >= state.ltt_rect.x && x <= state.ltt_rect.x + state.ltt_rect.width + SF {
                    let path = Path::new(|p| {
                        p.move_to(Point { x, y: state.ltt_rect.y });
                        p.line_to(Point { x, y: state.ltt_rect.y + state.ltt_rect.height });
                    });
                    f.stroke(&path, stroke.with_color(color_primary_strong));
                }
            }
        });
        geoms.push(g_cursor_line);

        let g_ltt = self.g_ltt.draw(renderer, bounds.size(), |f| {
            if let Some(pts) = &self.ltt_points {
                let mut max_count: usize = 0;
                for LttPoint { time: _, count } in pts {
                    max_count = max_count.max(*count)
                }
                let path = Path::new(|p| {
                    let y_max = state.ltt_rect.y + state.ltt_rect.height;

                    let calc_y = |count: usize| {
                        y_max
                            - (((count as Float).log10() / (max_count as Float).log10())
                                * state.ltt_rect.height)
                    };

                    p.move_to(Point { x: 0e0, y: calc_y(1) });
                    for LttPoint { time, count } in pts {
                        let x = *time as Float * state.ltt_rect.width + state.ltt_rect.x;
                        let pt = Point { x, y: calc_y(*count) };
                        p.line_to(pt);
                    }
                });

                f.stroke(&path, stroke);
            }
        });
        geoms.push(g_ltt);

        geoms
    }
}
