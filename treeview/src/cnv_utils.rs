use crate::iced::*;
use crate::*;

#[inline]
pub fn draw_rect(rect: Rectangle, stroke: Strk, frame: &mut Frame) {
    frame.stroke(&PathBuilder::new().rectangle(rect).build(), stroke);
}

#[inline]
pub fn draw_point(point: Point, stroke: Strk, radius: Float, frame: &mut Frame) {
    frame.stroke(&PathBuilder::new().circle(point, radius).build(), stroke);
}
