use super::TreeView;
use crate::{ColorSimple, Float};
use iced::{
    Color, Pixels, Point, Rectangle, Size, Vector,
    alignment::Vertical,
    widget::canvas::{Frame, LineCap, Path, Stroke, Text, path::Builder as PathBuilder},
};
use std::{
    ops::Deref,
    thread::{self, ScopedJoinHandle},
};

impl TreeView {
    #[allow(dead_code)]
    pub(super) fn draw_bg_rect(
        &self,
        r: Rectangle,
        lw: Float,
        offset: Float,
        color: &Color,
        frame: &mut Frame,
    ) {
        self.draw_bg_xywh((r.x, r.y, r.width, r.height), lw, offset, color, frame);
    }

    #[allow(dead_code)]
    pub(super) fn draw_bg_xywh(
        &self,
        xywh: (Float, Float, Float, Float),
        lw: Float,
        offset: Float,
        color: &Color,
        frame: &mut Frame,
    ) {
        let top_left = Point {
            x: xywh.0 + offset,
            y: xywh.1 + offset,
        };
        let size = Size {
            width: xywh.2 - offset * 2e0,
            height: xywh.3 - offset * 2e0,
        };
        frame.fill_rectangle(top_left, size, (*color).scale_alpha(1e-1));
        frame.stroke_rectangle(
            top_left,
            size,
            Stroke::default()
                .with_color((*color).scale_alpha(1e1))
                .with_width(lw)
                .with_line_cap(LineCap::Square),
        );
    }

    pub(super) fn draw_edges(
        &self,
        edge_paths: Vec<Path>,
        lw: Float,
        padding: Float,
        frame: &mut Frame,
    ) {
        let stroke = Stroke::default()
            .with_color(ColorSimple::BLK)
            .with_width(lw)
            .with_line_cap(LineCap::Square);

        frame.push_transform();

        frame.translate(Vector {
            x: padding * 2e0,
            y: padding * 2e0,
        });

        for p in edge_paths {
            frame.stroke(&p, stroke);
        }

        frame.pop_transform();
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn draw_labels(
        &self,
        labels: Vec<Text>,
        size: Float,
        color: &Color,
        label_offset: Float,
        padding: Float,
        clip: Rectangle,
        frame: &mut Frame,
    ) {
        let size_pix: Pixels = size.into();
        frame.with_clip(clip, |f| {
            f.push_transform();

            f.translate(Vector {
                x: padding * 2e0 + label_offset,
                y: padding * 2e0,
            });

            for mut l in labels {
                l.color = *color;
                l.size = size_pix;
                f.fill_text(l);
            }
            f.pop_transform();
        });
    }

    pub(super) fn generate_drawables(
        &self,
        bounds: &Rectangle,
        padding: Float,
    ) -> (Vec<Path>, Vec<Text>, Vec<Text>) {
        let bounds: Rectangle = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width - 4e0 * padding,
            height: bounds.height - 4e0 * padding,
        };
        let mut edge_paths: Vec<Path> = Vec::with_capacity(self.node_count);
        let mut tip_labels: Vec<Text> = Vec::with_capacity(self.tip_count);
        let mut int_labels: Vec<Text> = Vec::with_capacity(self.int_node_count);
        thread::scope(|thread_scope| {
            #[allow(clippy::type_complexity)]
            let mut handles: Vec<ScopedJoinHandle<'_, (Path, Vec<Text>, Vec<Text>)>> = Vec::new();
            for chunk in &self.tree_chunked_edges {
                let handle = thread_scope.spawn(move || {
                    let mut path_builder = PathBuilder::new();
                    let mut tip_labels_l: Vec<Text> = Vec::with_capacity(chunk.len());
                    let mut int_labels_l: Vec<Text> = Vec::with_capacity(chunk.len());
                    for edge in chunk {
                        let x0 = edge.x0 as Float * bounds.width;
                        let x1 = edge.x1 as Float * bounds.width;
                        let y = edge.y as Float * bounds.height;
                        let pt_node = Point::new(x1, y);
                        path_builder.move_to(pt_node);
                        path_builder.line_to(Point::new(x0, y));

                        if let Some(y_prev) = edge.y_prev {
                            path_builder.line_to(Point::new(x0, y_prev as Float * bounds.height))
                        };

                        if let Some(name) = &edge.name {
                            let label = Text {
                                content: name.deref().into(),
                                position: pt_node,
                                align_y: Vertical::Center,
                                ..Default::default()
                            };
                            if edge.is_tip {
                                tip_labels_l.push(label);
                            } else {
                                int_labels_l.push(label);
                            }
                        }
                    }
                    (path_builder.build(), tip_labels_l, int_labels_l)
                });
                handles.push(handle);
            }

            for j in handles {
                let (path, mut tip_labels_l, mut int_labels_l) = j.join().unwrap();
                edge_paths.push(path);
                tip_labels.append(&mut tip_labels_l);
                int_labels.append(&mut int_labels_l);
            }
        });

        (edge_paths, tip_labels, int_labels)
    }
}
