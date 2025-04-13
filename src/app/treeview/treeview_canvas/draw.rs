use super::super::TreeView;
use crate::Float;
use iced::{
    Pixels, Point, Rectangle, Vector,
    widget::canvas::{Frame, Path, Stroke, Text},
};

impl TreeView {
    pub fn draw_edges(
        &self,
        paths: Vec<Path>,
        stroke: Stroke,
        tree_rect: &Rectangle,
        frame: &mut Frame,
    ) {
        frame.with_save(|f| {
            f.translate(Vector { x: tree_rect.x, y: tree_rect.y });
            for p in paths {
                f.stroke(&p, stroke);
            }
        })
    }

    pub fn draw_labels(
        &self,
        labels: Vec<Text>,
        size: Float,
        offset: Point,
        tree_rect: &Rectangle,
        clip: &Rectangle,
        frame: &mut Frame,
    ) {
        let size_pix: Pixels = size.into();
        frame.with_clip(*clip, |f| {
            f.translate(Vector { x: tree_rect.x + offset.x, y: tree_rect.y + offset.y });
            for mut l in labels {
                l.size = size_pix;
                f.fill_text(l);
            }
        });
    }
}
