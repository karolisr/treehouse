use crate::iced::*;
use crate::*;

pub fn stroke_rect(rect: Rectangle, stroke: Strk, frame: &mut Frame) {
    frame.stroke(&PathBuilder::new().rectangle(rect).build(), stroke);
}

pub fn stroke_circle(point: Point, stroke: Strk, radius: Float, frame: &mut Frame) {
    frame.stroke(&PathBuilder::new().circle(point, radius).build(), stroke);
}

pub fn fill_circle(point: Point, fill: CnvFill, radius: Float, frame: &mut Frame) {
    frame.fill(&PathBuilder::new().circle(point, radius).build(), fill);
}
