use super::TreeView;
use crate::Float;
use iced::{
    Pixels, Rectangle,
    widget::canvas::{Frame, Path, Stroke, Text},
};

impl TreeView {
    pub fn draw_edges(&self, paths: Vec<Path>, stroke: Stroke, frame: &mut Frame) {
        for p in paths {
            frame.stroke(&p, stroke);
        }
    }

    pub fn draw_labels(&self, labels: Vec<Text>, size: Float, clip: Rectangle, frame: &mut Frame) {
        let size_pix: Pixels = size.into();
        frame.with_clip(clip, |f| {
            for mut l in labels {
                l.size = size_pix;
                f.fill_text(l);
            }
        });
    }
}
