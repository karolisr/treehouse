use crate::iced::*;
use crate::*;

pub fn draw_rect(rect: Rectangle, stroke: Strk, frame: &mut Frame) {
    frame.stroke_rectangle(Point { x: rect.x, y: rect.y }, rect.size(), stroke);
}

pub fn draw_point(point: Point, stroke: Strk, radius: Float, frame: &mut Frame) {
    let path = Path::circle(point, radius);
    frame.stroke(&path, stroke);
}
