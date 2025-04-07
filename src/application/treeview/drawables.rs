use super::TreeView;
use crate::{Float, NodeType};
use iced::{
    Point,
    alignment::Vertical,
    widget::canvas::{Path, Text, path::Builder as PathBuilder},
};
use std::{
    ops::Deref,
    thread::{self, ScopedJoinHandle},
};

impl TreeView {
    pub fn paths_from_chunks(&self, width: Float, height: Float) -> Vec<Path> {
        let mut paths: Vec<Path> = Vec::with_capacity(self.node_count);
        thread::scope(|thread_scope| {
            let mut handles: Vec<ScopedJoinHandle<'_, Path>> = Vec::new();
            for chunk in &self.tree_chunked_edges {
                let handle = thread_scope.spawn(move || {
                    let mut path_builder = PathBuilder::new();
                    for edge in chunk {
                        let x0 = edge.x0 as Float * width;
                        let x1 = edge.x1 as Float * width;
                        let y = edge.y as Float * height;
                        let pt_node = Point::new(x1, y);
                        path_builder.move_to(pt_node);
                        path_builder.line_to(Point::new(x0, y));
                        if let Some(y_prev) = edge.y_prev {
                            path_builder.line_to(Point::new(x0, y_prev as Float * height))
                        };
                    }
                    path_builder.build()
                });
                handles.push(handle);
            }
            for j in handles {
                let path = j.join().unwrap();
                paths.push(path);
            }
        });
        paths
    }

    pub fn labels_from_chunks(
        &self,
        width: Float,
        height: Float,
        return_only: NodeType,
    ) -> Vec<Text> {
        // ------------------------------
        let label_temp = Text {
            font: iced::Font {
                family: iced::font::Family::Name("JetBrains Mono"),
                ..Default::default()
            },
            align_y: Vertical::Center,
            ..Default::default()
        };
        // ------------------------------
        let mut tip_labels: Vec<Text> = Vec::with_capacity(self.tip_count);
        thread::scope(|thread_scope| {
            let mut handles: Vec<ScopedJoinHandle<'_, Vec<Text>>> = Vec::new();
            for chunk in &self.tree_chunked_edges {
                let label_temp_l = label_temp.clone();
                let handle = thread_scope.spawn(move || {
                    let mut tip_labels_l: Vec<Text> = Vec::with_capacity(chunk.len());
                    for edge in chunk {
                        let should_include = match return_only {
                            NodeType::Tip => edge.is_tip,
                            NodeType::Internal => !edge.is_tip,
                            NodeType::FirstNode => !edge.is_tip,
                            NodeType::Root => !edge.is_tip,
                            NodeType::Unset => todo!(),
                        };
                        if should_include {
                            let x1 = edge.x1 as Float * width;
                            let y = edge.y as Float * height;
                            let pt_node = Point::new(x1, y);
                            if let Some(name) = &edge.name {
                                let mut label = label_temp_l.clone();
                                label.content = name.deref().into();
                                label.position = pt_node;
                                tip_labels_l.push(label);
                            }
                        }
                    }
                    tip_labels_l
                });
                handles.push(handle);
            }
            for j in handles {
                let mut tip_labels_l = j.join().unwrap();
                tip_labels.append(&mut tip_labels_l);
            }
        });
        tip_labels
    }

    pub fn all_from_chunks(
        &self,
        width: Float,
        height: Float,
    ) -> (Vec<Path>, Vec<Text>, Vec<Text>) {
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
                        let x0 = edge.x0 as Float * width;
                        let x1 = edge.x1 as Float * width;
                        let y = edge.y as Float * height;
                        let pt_node = Point::new(x1, y);

                        path_builder.move_to(pt_node);
                        path_builder.line_to(Point::new(x0, y));
                        if let Some(y_prev) = edge.y_prev {
                            path_builder.line_to(Point::new(x0, y_prev as Float * height))
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
