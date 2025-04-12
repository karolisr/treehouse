use super::super::TreeView;
use crate::{Edge, Float, NodeType};
use iced::{
    Point,
    widget::canvas::{Path, Text, path::Builder as PathBuilder},
};
use std::{
    ops::{Deref, RangeInclusive},
    thread::{self, ScopedJoinHandle},
};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexRange {
    pub b: usize,
    pub e: usize,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkEdgeRange {
    pub chnk: IndexRange,
    pub edge: IndexRange,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodePoint {
    pub point: Point,
    pub edge: Edge,
}

impl TreeView {
    pub fn visible_tip_range(&self) -> Option<IndexRange> {
        let tip_idx_0: i64 = (self.cnv_y0 / self.node_size) as i64 - 3;
        let tip_idx_1: i64 = (self.cnv_y1 / self.node_size) as i64 + 3;
        let tip_idx_0: usize = tip_idx_0.max(0) as usize;
        let tip_idx_1: usize = tip_idx_1.min(self.tree_tip_edges.len() as i64 - 1) as usize;

        if tip_idx_0 < tip_idx_1 {
            Some(IndexRange {
                b: tip_idx_0,
                e: tip_idx_1,
            })
        } else {
            None
        }
    }

    pub fn visible_nodes(
        &self,
        width: Float,
        height: Float,
        tip_idx_range: &IndexRange,
    ) -> Vec<NodePoint> {
        let ChunkEdgeRange {
            chnk:
                IndexRange {
                    b: chnk_idx_0,
                    e: chnk_idx_1,
                },
            edge:
                IndexRange {
                    b: edge_idx_0,
                    e: edge_idx_1,
                },
        } = self.visible_node_ranges(tip_idx_range);

        let mut points: Vec<NodePoint> = Vec::new();
        if chnk_idx_0 == chnk_idx_1 {
            let chunk = &self.tree_chunked_edges[chnk_idx_0];
            for e in &chunk[edge_idx_0..=edge_idx_1] {
                let point = Point {
                    x: e.x1 as Float * width,
                    y: e.y as Float * height,
                };
                points.push(NodePoint {
                    point,
                    edge: e.clone(),
                });
            }
        } else {
            for chnk_idx in chnk_idx_0..=chnk_idx_1 {
                let edge_range: RangeInclusive<usize>;
                if chnk_idx == chnk_idx_0 {
                    edge_range = edge_idx_0..=self.tree_chunked_edges[chnk_idx].len() - 1;
                } else if chnk_idx == chnk_idx_1 {
                    edge_range = 0..=edge_idx_1
                } else {
                    edge_range = 0..=self.tree_chunked_edges[chnk_idx].len() - 1;
                }

                let chunk = &self.tree_chunked_edges[chnk_idx];

                for e in &chunk[edge_range] {
                    let point = Point {
                        x: e.x1 as Float * width,
                        y: e.y as Float * height,
                    };
                    points.push(NodePoint {
                        point,
                        edge: e.clone(),
                    });
                }
            }
        }
        points
    }

    pub fn visible_node_ranges(&self, tip_idx_range: &IndexRange) -> ChunkEdgeRange {
        let idx_0 = &self.tree_tip_edges[tip_idx_range.b];
        let idx_1 = &self.tree_tip_edges[tip_idx_range.e];

        let chnk_idx_0 = idx_0.chunk_idx;
        let edge_idx_0 = idx_0.edge_idx;

        let chnk_idx_1 = idx_1.chunk_idx;
        let edge_idx_1 = idx_1.edge_idx;

        // println!(
        //     "({:6}:{:6}|{:6}:{:6})",
        //     chnk_idx_0, edge_idx_0, chnk_idx_1, edge_idx_1
        // );

        ChunkEdgeRange {
            chnk: IndexRange {
                b: chnk_idx_0,
                e: chnk_idx_1,
            },
            edge: IndexRange {
                b: edge_idx_0,
                e: edge_idx_1,
            },
        }
    }

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
                        if let Some(y_parent) = edge.y_parent {
                            path_builder.line_to(Point::new(x0, y_parent as Float * height))
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

    pub fn tip_labels_in_range(
        &self,
        width: Float,
        height: Float,
        idx_0: usize,
        idx_1: usize,
        label_template: &Text,
    ) -> Vec<Text> {
        let mut labels: Vec<Text> = Vec::with_capacity(idx_1 - idx_0);
        for edge in &self.tree_tip_edges[idx_0..=idx_1] {
            let x1 = edge.x1 as Float * width;
            let y = edge.y as Float * height;
            let pt_node = Point::new(x1, y);
            if let Some(name) = &edge.name {
                let mut label = label_template.clone();
                label.content = name.deref().into();
                label.position = pt_node;
                labels.push(label);
            }
        }
        labels
    }

    pub fn labels_from_chunks(
        &self,
        width: Float,
        height: Float,
        return_only: NodeType,
        label_template: &Text,
    ) -> Vec<Text> {
        let mut labels: Vec<Text> = Vec::with_capacity(self.tip_count);
        thread::scope(|thread_scope| {
            let mut handles: Vec<ScopedJoinHandle<'_, Vec<Text>>> = Vec::new();
            for chunk in &self.tree_chunked_edges {
                let handle = thread_scope.spawn(move || {
                    let mut labels_l: Vec<Text> = Vec::with_capacity(chunk.len());
                    for edge in chunk {
                        let should_include = match return_only {
                            NodeType::Tip => edge.is_tip,
                            NodeType::Internal => !edge.is_tip,
                            NodeType::FirstNode => !edge.is_tip,
                            NodeType::Root => !edge.is_tip,
                            NodeType::Unset => !edge.is_tip,
                        };
                        if should_include {
                            let x1 = edge.x1 as Float * width;
                            let y = edge.y as Float * height;
                            let pt_node = Point::new(x1, y);
                            if let Some(name) = &edge.name {
                                let mut label = label_template.clone();
                                label.content = name.deref().into();
                                label.position = pt_node;
                                labels_l.push(label);
                            }
                        }
                    }
                    labels_l
                });
                handles.push(handle);
            }
            for j in handles {
                let mut labels_l = j.join().unwrap();
                labels.append(&mut labels_l);
            }
        });
        labels
    }
}
