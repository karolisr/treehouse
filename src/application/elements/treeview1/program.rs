use crate::{SimpleColor, flatten_tree};

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
        cursor: Cursor,
    ) -> Vec<Geometry> {
        // println!("BEGIN draw");
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // println!("BEGIN cache.draw");
            // state.draw_bg(&state.bounds_global, &blk, frame);
            // state.draw_bg(&state.bounds_full, &grn, frame);
            // state.draw_bg(&state.bounds_tree, &red, frame);
            // state.draw_bg(&state.bounds_tl_sep, &grn, frame);
            // state.draw_bg(&state.bounds_tip_labels, &blu, frame);

            // if cursor.is_over(state.bounds_tl_sep) {
            //     let x = state.bounds_tl_sep.x + state.bounds_tl_sep.width / 2e0;
            //     let path = Path::new(|p| {
            //         p.move_to(Point {
            //             x,
            //             y: state.bounds_tl_sep.y,
            //         });

            //         p.line_to(Point {
            //             x,
            //             y: state.bounds_tl_sep.y + state.bounds_tl_sep.height,
            //         });
            //     });
            //     frame.stroke(
            //         &path,
            //         Stroke::default().with_width(5e0).with_color(Color {
            //             r: 0e0,
            //             g: 0e0,
            //             b: 1e0,
            //             a: 4e-1,
            //         }),
            //     );
            // }

            // #[cfg(debug_assertions)]
            // unsafe {
            //     COUNTER = 0
            // };

            // println!("BEGIN cache.draw draw_tree");
            // state.draw_tree(
            //     self.tree.first_node_id(),
            //     &self.tree,
            //     frame,
            //     &state.bounds_tree,
            // );

            // for b in &state.tip_label_rects {
            //     state.draw_bounds(b, frame);
            // }

            // println!("BEGIN cache.draw draw_tip_labels");
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
            // println!("  END cache.draw");

            // let edges = flatten_tree(&self.tree);
            frame.translate(Vector::new(1e1, 1e1));
            let mut parent_prev = self.edges[0].0;
            let mut y_prev = self.edges[0].5;
            for e in &self.edges {
                // println!(
                //     // Prnt Child PHeight  Height    Y
                //     "{:>5} {:>5} {:>3.5} {:>3.5} {:>3.5}",
                //     e.0, e.1, e.3, e.4, e.5,
                // );

                let parent = e.0;
                let y = e.5;
                let path = Path::new(|p| {
                    p.move_to(Point::new(
                        e.3 as Float * state.bounds_tree.width,
                        e.5 as Float * state.bounds_tree.height,
                    ));
                    p.line_to(Point::new(
                        e.4 as Float * state.bounds_tree.width,
                        e.5 as Float * state.bounds_tree.height,
                    ));

                    if parent == parent_prev {
                        p.move_to(Point::new(
                            e.3 as Float * state.bounds_tree.width,
                            e.5 as Float * state.bounds_tree.height,
                        ));
                        p.line_to(Point::new(
                            e.3 as Float * state.bounds_tree.width,
                            y_prev as Float * state.bounds_tree.height,
                        ));
                    }
                    parent_prev = parent;
                });
                y_prev = y;
                frame.stroke(&path, Stroke::default().with_color(SimpleColor::BLACK));
            }
        });
        // println!("  END draw");
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
                            // if cursor.is_over(state.bounds_tl_sep) {
                            //     self.cache.clear();
                            //     return Some(Action::request_redraw());
                            // }
                            // if cursor.is_over(state.bounds_tip_labels) {
                            //     self.cache.clear();
                            //     return Some(Action::request_redraw());
                            // }
                            None
                        }
                    } else {
                        None
                    }
                }
                iced::mouse::Event::CursorEntered => None,
                iced::mouse::Event::CursorLeft => {
                    self.cache.clear();
                    Some(Action::request_redraw())
                }
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
