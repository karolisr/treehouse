use super::super::TreeView;
use crate::{
    Float,
    app::{PADDING, SF, TEXT_SIZE},
};
use iced::{
    Pixels, Point, Radians, Rectangle, Vector,
    alignment::{Horizontal, Vertical},
    widget::canvas::{Frame, Path, Stroke, Text},
};

impl TreeView {
    pub fn draw_scale_bar(
        &self,
        stroke: Stroke,
        label_template: &Text,
        tree_rect: &Rectangle,
        frame: &mut Frame,
    ) {
        let mut sb_len = self.tree_height / 4e0;

        if sb_len > 1e1 {
            sb_len = sb_len.floor();
        } else {
            sb_len = (sb_len * 1e1).floor() / 1e1;
        }

        let sb_frac = sb_len / self.tree_height;
        let sb_len_on_screen = sb_frac * tree_rect.width;
        let sb_str = format!("{sb_len}");

        let y = tree_rect.y + tree_rect.height + PADDING;

        // in the middle...
        // let p0 = Point { x: tree_rect.width / 2e0 - sb_len_on_screen / 2e0, y };
        let p0 = Point { x: PADDING, y };
        let p1 = Point { x: p0.x + sb_len_on_screen, y };

        let p_lab = Point { x: p0.x + (p1.x - p0.x) / 2e0, y: y + PADDING / 2e0 };

        let mut l = label_template.clone();
        l.align_x = Horizontal::Center;
        l.align_y = Vertical::Top;
        l.position = p_lab;
        l.content = sb_str;
        l.size = (TEXT_SIZE - SF * 2e0).into();

        let path = Path::new(|p| {
            p.move_to(p0);
            p.line_to(p1);
        });

        frame.stroke(&path, stroke);
        frame.fill_text(l);
    }

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

    pub fn draw_labels_fan(
        &self,
        labels: Vec<(Text, Radians)>,
        size: Float,
        offset: Point,
        tree_rect: &Rectangle,
        clip: &Rectangle,
        frame: &mut Frame,
    ) {
        let size_pix: Pixels = size.into();
        let pos = Point { x: 0e0, y: 0e0 };
        frame.with_clip(*clip, |f| {
            f.translate(Vector { x: tree_rect.x, y: tree_rect.y });
            for (mut l, a) in labels {
                f.push_transform();
                l.size = size_pix;
                let af = a.0;
                f.translate(Vector {
                    x: l.position.x + af.cos() * (offset.x),
                    y: l.position.y + af.sin() * (offset.x),
                });
                f.rotate(a);
                l.position = pos;
                f.fill_text(l);
                f.pop_transform();
            }
        });
    }
}
