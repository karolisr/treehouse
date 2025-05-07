use iced::{
    Color, Point, Rectangle,
    widget::canvas::{Frame, LineCap, LineDash, LineJoin, Path, Stroke, Style},
};
use utils::Clr;

const STROKE: Stroke = Stroke {
    style: Style::Solid(Color { a: 0.75, ..Clr::RED }),
    width: 1e0,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    line_dash: LineDash { segments: &[1e0, 2e0], offset: 0 },
};

const RADIUS: f32 = 1e1;

pub(crate) fn clip_rect_from_bounds(bounds: Rectangle) -> Rectangle {
    Rectangle { x: 1e0, y: 1e0, width: bounds.width - 2e0, height: bounds.height - 2e0 }
}

pub(crate) fn draw_point(point: Point, frame: &mut Frame) {
    let path = Path::circle(point, RADIUS);
    frame.stroke(&path, STROKE);
}

pub(crate) fn draw_rectangle(rect: Rectangle, frame: &mut Frame) {
    frame.stroke_rectangle(Point { x: rect.x, y: rect.y }, rect.size(), STROKE);
}
