use crate::*;
use iced::{
    Point, Rectangle,
    widget::canvas::{Frame, Path, Stroke},
};

#[inline]
pub fn draw_rect(rect: Rectangle, stroke: Stroke, frame: &mut Frame) {
    frame.stroke_rectangle(Point { x: rect.x, y: rect.y }, rect.size(), stroke);
}

#[inline]
pub fn draw_point(point: Point, stroke: Stroke, radius: Float, frame: &mut Frame) {
    let path = Path::circle(point, radius);
    frame.stroke(&path, stroke);
}
