use crate::CLR;
use iced::Color;
use std::thread;
use std::thread::ScopedJoinHandle;

// #[cfg(not(debug_assertions))]
use super::{TreeView1, TreeView1Msg, TreeView1State};
use iced::{
    Event, Point, Rectangle, Renderer, Theme, Vector,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, Path, Program, Stroke},
};

type Float = f32;

impl Program<TreeView1Msg> for TreeView1 {
    type State = TreeView1State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // state.draw_bg(&state.bounds_global, &SimpleColor::BLACK, frame);
            // state.draw_bg(&state.bounds_full, &SimpleColor::GREEN, frame);
            // state.draw_bg(&state.bounds_tree, &SimpleColor::RED, frame);
            // state.draw_bg(&state.bounds_tl_sep, &SimpleColor::GREEN, frame);
            // state.draw_bg(&state.bounds_tip_labels, &SimpleColor::BLUE, frame);

            let x = state.bounds_tl_sep.x + state.bounds_tl_sep.width / 2e0;
            let path = Path::new(|p| {
                p.move_to(Point {
                    x,
                    y: state.bounds_tl_sep.y,
                });

                p.line_to(Point {
                    x,
                    y: state.bounds_tl_sep.y + state.bounds_tl_sep.height,
                });
            });
            frame.stroke(
                &path,
                Stroke::default().with_width(5e0).with_color(Color {
                    r: 0e0,
                    g: 0e0,
                    b: 1e0,
                    a: 4e-1,
                }),
            );

            // #[cfg(debug_assertions)]
            // unsafe {
            //     COUNTER = 0
            // };

            // state.draw_tree(
            //     self.tree.first_node_id(),
            //     &self.tree,
            //     frame,
            //     &state.bounds_tree,
            // );

            // for b in &state.tip_label_rects {
            //     state.draw_bounds(b, frame);
            // }

            // prepare_tip_label_rects --------------------------
            // let mut tip_label_rects: Vec<Rectangle> = Vec::new();
            // for i in 0..self.tip_count {
            //     let label_bounds: Rectangle = Rectangle {
            //         x: state.bounds_tip_labels.x,
            //         y: state.bounds_tip_labels.y
            //             + state.scale_factor_y / 4e0
            //             + state.scale_factor_y * i as Float,
            //         width: state.label_width,
            //         height: state.scale_factor_y / 2e0,
            //     };
            //     tip_label_rects.push(label_bounds);
            // } // ------------------------------------------------

            // let tip_names = self
            //     .tree
            //     .tip_node_ids_all()
            //     .iter()
            //     .map(|&id| self.tree.name(id))
            //     .collect();

            // state.draw_tip_labels(frame, &cursor, tip_label_rects, tip_names);

            frame.translate(Vector::new(1e1, 1e1));
            let stroke = Stroke::default().with_color(CLR::BLACK);
            thread::scope(|thread_scope| {
                let mut handles: Vec<ScopedJoinHandle<'_, Path>> = Vec::new();
                for chunk in &self.edges_chunks {
                    let path = thread_scope.spawn(move || {
                        Path::new(|p| {
                            for e in chunk {
                                let x0 = e.x0 as Float * state.bounds_tree.width;
                                let x1 = e.x1 as Float * state.bounds_tree.width;
                                let y = e.y as Float * state.bounds_tree.height;
                                p.move_to(Point::new(x1, y));
                                p.line_to(Point::new(x0, y));
                                if let Some(y_prev) = e.y_prev {
                                    p.line_to(Point::new(
                                        x0,
                                        y_prev as Float * state.bounds_tree.height,
                                    ))
                                };
                            }
                        })
                    });
                    handles.push(path);
                }
                for j in handles {
                    frame.stroke(&j.join().unwrap(), stroke);
                }
            });
        });
        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<TreeView1Msg>> {
        match event {
            Event::Mouse(e) => match e {
                iced::mouse::Event::CursorMoved { position } => {
                    if cursor.is_over(state.bounds_full) {
                        if state.dragging_tl_sep {
                            state.label_width =
                                state.label_width_prev + state.drag_start_x - position.x;
                            self.cache.clear();
                            Some(Action::request_redraw())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                iced::mouse::Event::CursorEntered => None,
                iced::mouse::Event::CursorLeft => None,
                iced::mouse::Event::ButtonPressed(button) => match button {
                    iced::mouse::Button::Left => {
                        if cursor.is_over(state.bounds_tl_sep) {
                            state.dragging_tl_sep = true;
                            state.drag_start_x = cursor.position().unwrap_or_default().x;
                            state.drag_start_y = cursor.position().unwrap_or_default().y;
                            Some(Action::request_redraw())
                        } else {
                            state.dragging_tl_sep = false;
                            None
                        }
                    }
                    iced::mouse::Button::Right => None,
                    // iced::mouse::Button::Middle => None,
                    // iced::mouse::Button::Back => None,
                    // iced::mouse::Button::Forward => None,
                    // iced::mouse::Button::Other(_) => None,
                    _ => None,
                },
                iced::mouse::Event::ButtonReleased(button) => match button {
                    iced::mouse::Button::Left => {
                        if state.dragging_tl_sep {
                            state.dragging_tl_sep = false;
                            state.drag_start_x = 0e0;
                            state.drag_start_y = 0e0;
                            state.label_width_prev = state.label_width;
                            self.cache.clear();
                            Some(Action::request_redraw())
                        } else {
                            None
                        }
                    }
                    iced::mouse::Button::Right => None,
                    // iced::mouse::Button::Middle => None,
                    // iced::mouse::Button::Back => None,
                    // iced::mouse::Button::Forward => None,
                    // iced::mouse::Button::Other(_) => None,
                    _ => None,
                },
                // iced::mouse::Event::WheelScrolled { delta: _ } => None,
                _ => None,
            },
            Event::Keyboard(_e) => None,
            Event::Window(e) => match e {
                // iced::window::Event::Opened { position: _, size: _, } => None,
                // iced::window::Event::Closed => None,
                // iced::window::Event::Moved(_point) => None,
                iced::window::Event::Resized(size) => {
                    state.height_win = size.height;
                    Some(Action::publish(TreeView1Msg::WindowHeightChanged(
                        state.height_win,
                    )))
                }

                iced::window::Event::RedrawRequested(_instant) => {
                    state.label_height = self.lab_size;
                    state.scale_factor_y_min = self.node_size;
                    state.cache_tree_state(self, &bounds);

                    if state.height != state.height_prev {
                        state.height_prev = state.height;
                        return Some(Action::publish(TreeView1Msg::CanvasHeightChanged(
                            state.height,
                        )));
                    }

                    None
                }
                // iced::window::Event::CloseRequested => None,
                // iced::window::Event::Focused => None,
                // iced::window::Event::Unfocused => None,
                // iced::window::Event::FileHovered(_path_buf) => None,
                // iced::window::Event::FileDropped(_path_buf) => None,
                // iced::window::Event::FilesHoveredLeft => None,
                _ => None,
            },
            // Event::Touch(_e) => None,
            // Event::InputMethod(_e) => None,
            _ => None,
        }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        _bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if cursor.is_over(state.bounds_tl_sep) {
            Interaction::ResizingHorizontally
        } else {
            Interaction::default()
        }
    }
}
