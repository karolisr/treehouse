use super::TreeView;
use crate::{ColorSimple, Float};
use iced::{
    Color, Point, Rectangle, Size, Vector,
    widget::canvas::{Frame, LineCap, Path, Stroke},
};
use std::thread::{self, ScopedJoinHandle};

impl TreeView {
    #[allow(dead_code)]
    pub(super) fn draw_bg(
        &self,
        r: (Float, Float, Float, Float),
        lw: Float,
        offset: Float,
        color: &Color,
        frame: &mut Frame,
    ) {
        let top_left = Point {
            x: r.0 + offset,
            y: r.1 + offset,
        };
        let size = Size {
            width: r.2 - offset * 2e0,
            height: r.3 - offset * 2e0,
        };
        frame.fill_rectangle(top_left, size, (*color).scale_alpha(1e-1));
        frame.stroke_rectangle(
            top_left,
            size,
            Stroke::default()
                .with_color((*color).scale_alpha(1e1))
                .with_width(lw)
                .with_line_cap(LineCap::Square),
        );
    }

    pub(super) fn draw_tree(
        &self,
        bounds: &Rectangle,
        lw: Float,
        offset: Float,
        frame: &mut Frame,
    ) {
        frame.translate(Vector {
            x: offset,
            y: offset,
        });

        let bounds: Rectangle = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width - 2e0 * offset,
            height: bounds.height - 2e0 * offset,
        };

        let stroke = Stroke::default()
            .with_color(ColorSimple::BLK)
            .with_width(lw)
            .with_line_cap(LineCap::Square);

        thread::scope(|thread_scope| {
            let mut handles: Vec<ScopedJoinHandle<'_, Path>> = Vec::new();
            for chunk in &self.tree_chunked_edges {
                let path = thread_scope.spawn(move || {
                    Path::new(|p| {
                        for edge in chunk {
                            let x0 = edge.x0 as Float * bounds.width;
                            let x1 = edge.x1 as Float * bounds.width;
                            let y = edge.y as Float * bounds.height;
                            p.move_to(Point::new(x1, y));
                            p.line_to(Point::new(x0, y));
                            if let Some(y_prev) = edge.y_prev {
                                p.line_to(Point::new(x0, y_prev as Float * bounds.height))
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

        frame.translate(Vector {
            x: -offset,
            y: -offset,
        });
    }
}
