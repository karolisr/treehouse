// #[cfg(not(debug_assertions))]
use super::{TreeView, TreeViewMsg, TreeViewState};
use iced::{
    Color, Event, Point, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, Path, Program, Stroke},
};

impl Program<TreeViewMsg> for TreeView {
    type State = TreeViewState;

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

            if cursor.is_over(state.bounds_tl_sep) {
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
            }

            // #[cfg(debug_assertions)]
            // unsafe {
            //     COUNTER = 0
            // };

            // println!("BEGIN cache.draw draw_tree");
            state.draw_tree(
                self.tree.first_node_id(),
                &self.tree,
                frame,
                &state.bounds_tree,
            );

            // for b in &state.tip_label_rects {
            //     state.draw_bounds(b, frame);
            // }

            // println!("BEGIN cache.draw draw_tip_labels");
            state.draw_tip_labels(frame, &cursor);
            // println!("  END cache.draw");
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
    ) -> Option<Action<TreeViewMsg>> {
        if state.height != state.height_prev {
            state.height_prev = state.height;
            return Some(Action::publish(TreeViewMsg::CanvasHeightChanged(
                state.height,
            )));
        }
        match event {
            Event::Mouse(e) => match e {
                iced::mouse::Event::CursorMoved { position } => {
                    if cursor.is_over(state.bounds_full) {
                        self.cache.clear();
                        if state.dragging_tl_sep {
                            state.label_width =
                                state.label_width_prev + state.drag_start_x - position.x;
                        } else {
                            for &r in &state.tip_label_rects {
                                if cursor.is_over(r) {
                                    return Some(Action::request_redraw());
                                }
                            }
                        }
                        Some(Action::request_redraw())
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
                            None
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
                        state.dragging_tl_sep = false;
                        state.drag_start_x = 0e0;
                        state.drag_start_y = 0e0;
                        state.label_width_prev = state.label_width;
                        None
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
                    Some(Action::publish(TreeViewMsg::WindowHeightChanged(
                        state.height_win,
                    )))
                }
                iced::window::Event::RedrawRequested(_instant) => {
                    state.label_height = self.lab_size;
                    state.scale_factor_y_min = self.node_size;
                    state.cache_tree_state(&self.tree, &bounds);
                    state.prepare_tip_label_rects();
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
