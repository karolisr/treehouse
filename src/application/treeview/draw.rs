use super::TreeView;
use crate::{Float, SF};
use iced::{
    Pixels, Rectangle, Vector,
    widget::canvas::{Frame, Path, Stroke, Text},
};

impl TreeView {
    pub fn draw_edges(&self, paths: Vec<Path>, stroke: Stroke, frame: &mut Frame) {
        frame.with_save(|f| {
            f.translate(Vector {
                x: SF,
                y: SF + self.max_label_size / 2e0,
            });
            for p in paths {
                f.stroke(&p, stroke);
            }
        })
    }

    pub fn draw_labels(
        &self,
        labels: Vec<Text>,
        size: Float,
        offset: Float,
        clip: Rectangle,
        frame: &mut Frame,
    ) {
        let size_pix: Pixels = size.into();
        frame.with_clip(clip, |f| {
            f.translate(Vector {
                x: SF + offset,
                y: SF + self.max_label_size / 2e0,
            });
            for mut l in labels {
                l.size = size_pix;
                f.fill_text(l);
            }
        });
    }
}
