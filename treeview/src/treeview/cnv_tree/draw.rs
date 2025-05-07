use super::TreeCnv;
use crate::{Float, Label};
use iced::{
    Pixels, Point, Rectangle, Vector,
    alignment::Vertical,
    widget::canvas::{Fill, Frame, Path, Stroke, Text},
    widget::text::Alignment as TextAlignment,
};
use utils::text_width;

pub(crate) fn draw_edges(
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

// pub(crate) fn draw_labels(
//     &self,
//     labels: Vec<Label>,
//     text_size: Float,
//     offset: Point,
//     tree_rect: &Rectangle,
//     clip: &Rectangle,
//     frame: &mut Frame,
// ) {
//     let zero_point = Point { x: 0e0, y: 0e0 };
//     let mut text_w = text_width(text_size, text_size, TREE_LAB_FONT_NAME);
//     let text_size: Pixels = text_size.into();
//     frame.with_clip(*clip, |f| {
//         f.translate(Vector { x: tree_rect.x + offset.x, y: tree_rect.y + offset.y });
//         for Label { mut text, angle } in labels {
//             text.size = text_size;
//             if let Some(mut angle) = angle {
//                 let mut adjust_w = offset.x;
//                 // = Rotate labels on the left side of the circle by 180 degrees ==============
//                 let a = angle % (2e0 * PI);
//                 if a > PI / 2e0 && a < PI * 1.5 {
//                     angle += PI;
//                     match text.align_x {
//                         TextAlignment::Left => {
//                             adjust_w = -text_w.width(&text.content) - offset.x
//                         }
//                         TextAlignment::Center => {}
//                         TextAlignment::Right => {
//                             adjust_w = text_w.width(&text.content) + offset.x
//                         }
//                         _ => {}
//                     }
//                 } // ==========================================================================
//                 f.push_transform();
//                 f.translate(Vector {
//                     x: text.position.x - offset.x + angle.cos() * adjust_w,
//                     y: text.position.y - offset.y + angle.sin() * adjust_w,
//                 });
//                 f.rotate(angle);
//                 text.position = zero_point;
//                 f.fill_text(text);
//                 f.pop_transform();
//             } else {
//                 f.fill_text(text);
//             }
//         }
//     });
// }

// pub(crate) fn draw_node(
//     &self,
//     point: &Point,
//     ps: Float,
//     stroke: Stroke,
//     fill: impl Into<Fill>,
//     tree_rect: &Rectangle,
//     frame: &mut Frame,
// ) {
//     frame.with_save(|f| {
//         f.translate(Vector { x: tree_rect.x, y: tree_rect.y });
//         let path_fill = Path::new(|p| {
//             p.circle(*point, ps);
//         });

//         let path_stroke = Path::new(|p| {
//             p.circle(*point, ps - SF / 2e0);
//         });

//         f.fill(&path_fill, fill);
//         f.stroke(&path_stroke, stroke);
//     });
// }

// pub(crate) fn draw_scale_bar(
//     &self,
//     stroke: Stroke,
//     label_template: &Text,
//     tree_rect: &Rectangle,
//     frame: &mut Frame,
// ) {
//     let mut sb_len = self.tree_height as Float / 4e0;

//     if sb_len > 1e1 {
//         sb_len = sb_len.floor();
//     } else {
//         sb_len = (sb_len * 1e1).floor() / 1e1;
//     }

//     let sb_frac = sb_len / self.tree_height as Float;
//     let sb_len_on_screen = sb_frac * tree_rect.width;
//     let sb_str = format!("{sb_len}");

//     let y = tree_rect.y + tree_rect.height + PADDING;

//     // in the middle...
//     // let p0 = Point { x: tree_rect.width / 2e0 - sb_len_on_screen / 2e0, y };
//     let p0 = Point { x: PADDING, y };
//     let p1 = Point { x: p0.x + sb_len_on_screen, y };

//     let p_lab = Point { x: p0.x + (p1.x - p0.x) / 2e0, y: y + PADDING / 2e0 };

//     let mut l = label_template.clone();
//     l.align_x = TextAlignment::Center;
//     l.align_y = Vertical::Top;
//     l.position = p_lab;
//     l.content = sb_str;
//     l.size = (TEXT_SIZE - SF * 2e0).into();

//     let path = Path::new(|p| {
//         p.move_to(p0);
//         p.line_to(p1);
//     });

//     frame.stroke(&path, stroke);
//     frame.fill_text(l);
// }
