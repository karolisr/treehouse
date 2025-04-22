use super::super::TreeView;
use crate::{Edge, Float};
use iced::{
    Point, Radians,
    alignment::{Horizontal, Vertical},
    widget::canvas::{
        Path, Text,
        path::{Arc, Builder as PathBuilder},
    },
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
    pub fn paths_from_chunks(&self, tree_rect_width: Float, tree_rect_height: Float) -> Vec<Path> {
        let mut paths: Vec<Path> = Vec::with_capacity(self.node_count);
        thread::scope(|thread_scope| {
            let mut handles: Vec<ScopedJoinHandle<'_, Path>> = Vec::new();
            for chunk in &self.tree_chunked_edges {
                let handle = thread_scope.spawn(move || {
                    let mut pb = PathBuilder::new();
                    for edge in chunk {
                        let x0 = edge.x0 as Float * tree_rect_width;
                        let x1 = edge.x1 as Float * tree_rect_width;
                        let y = edge.y as Float * tree_rect_height;
                        let pt_node = Point::new(x1, y);
                        pb.move_to(pt_node);
                        pb.line_to(Point::new(x0, y));
                        if let Some(y_parent) = edge.y_parent {
                            pb.line_to(Point::new(x0, y_parent as Float * tree_rect_height))
                        };
                    }
                    pb.build()
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

    pub fn paths_from_chunks_fan(&self, size: f64, center: Point<f64>) -> Vec<Path> {
        let mut paths: Vec<Path> = Vec::with_capacity(self.node_count);
        let c_x = center.x;
        let c_y = center.y;
        let center_f32 = Point { x: c_x as Float, y: c_y as Float };
        let rot_angle = self.rot_angle;
        let max_angle = self.opn_angle;
        thread::scope(|thread_scope| {
            let mut handles: Vec<ScopedJoinHandle<'_, Path>> = Vec::new();
            for chunk in &self.tree_chunked_edges {
                let handle = thread_scope.spawn(move || {
                    let mut pb = PathBuilder::new();
                    for edge in chunk {
                        let angle = rot_angle + edge.y * max_angle;
                        let x0 = edge.x0 * angle.cos() * size;
                        let y0 = edge.x0 * angle.sin() * size;
                        let x1 = edge.x1 * angle.cos() * size;
                        let y1 = edge.x1 * angle.sin() * size;
                        let pt_node = Point { x: (c_x + x1) as Float, y: (c_y + y1) as Float };
                        let pt_0 = Point { x: (c_x + x0) as Float, y: (c_y + y0) as Float };
                        pb.move_to(pt_node);
                        pb.line_to(pt_0);
                        if let Some(y_parent) = edge.y_parent {
                            let angle_parent = rot_angle + y_parent * max_angle;
                            let p_arc = Arc {
                                center: center_f32,
                                radius: center_f32.distance(pt_0),
                                start_angle: Radians(angle as f32),
                                end_angle: Radians(angle_parent as f32),
                            };
                            pb.arc(p_arc);
                        };
                    }
                    pb.build()
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
        tree_rect_width: Float,
        tree_rect_height: Float,
        idx_0: usize,
        idx_1: usize,
        label_template: &Text,
    ) -> Vec<Text> {
        let mut labels: Vec<Text> = Vec::with_capacity(idx_1 - idx_0);
        for edge in &self.tree_tip_edges[idx_0..=idx_1] {
            if let Some(name) = &edge.name {
                let x1 = edge.x1 as Float * tree_rect_width;
                let y = edge.y as Float * tree_rect_height;
                let pt_node = Point::new(x1, y);
                let mut label = label_template.clone();
                label.content = name.deref().into();
                label.position = pt_node;
                labels.push(label);
            }
        }
        labels
    }

    pub fn tip_labels_in_range_fan(
        &self,
        size: f64,
        center: Point<f64>,
        idx_0: usize,
        idx_1: usize,
        label_template: &Text,
    ) -> Vec<(Text, Radians)> {
        let mut labels: Vec<(Text, Radians)> = Vec::with_capacity(idx_1 - idx_0);
        let c_x = center.x;
        let c_y = center.y;
        for edge in &self.tree_tip_edges[idx_0..=idx_1] {
            if let Some(name) = &edge.name {
                let angle = self.rot_angle + edge.y * self.opn_angle;
                let x1 = edge.x1 * angle.cos() * size;
                let y1 = edge.x1 * angle.sin() * size;
                let pt_node = Point { x: (c_x + x1) as Float, y: (c_y + y1) as Float };
                let mut label = label_template.clone();
                label.content = name.deref().into();
                label.position = pt_node;
                labels.push((label, Radians(angle as f32)));
            }
        }
        labels
    }

    pub fn visible_int_node_labels(
        &self,
        _tree_rect_width: Float,
        _tree_rect_height: Float,
        visible_nodes: &Vec<NodePoint>,
        label_template: &Text,
    ) -> Vec<Text> {
        let mut labels: Vec<Text> = Vec::with_capacity(visible_nodes.len());
        for NodePoint { point, edge } in visible_nodes {
            if edge.is_tip {
                continue;
            }
            if let Some(name) = &edge.name {
                let mut label = label_template.clone();
                label.content = name.deref().into();
                label.position = *point;
                labels.push(label);
            }
        }
        labels
    }

    pub fn visible_int_node_labels_fan(
        &self,
        size: f64,
        center: Point<f64>,
        visible_nodes: &Vec<NodePoint>,
        label_template: &Text,
    ) -> Vec<(Text, Radians)> {
        let mut labels: Vec<(Text, Radians)> = Vec::with_capacity(visible_nodes.len());
        let c_x = center.x;
        let c_y = center.y;
        for NodePoint { point: _, edge } in visible_nodes {
            if edge.is_tip {
                continue;
            }
            if let Some(name) = &edge.name {
                let angle = self.rot_angle + edge.y * self.opn_angle;
                let x1 = edge.x1 * angle.cos() * size;
                let y1 = edge.x1 * angle.sin() * size;
                let pt_node = Point { x: (c_x + x1) as Float, y: (c_y + y1) as Float };
                let mut label = label_template.clone();
                label.content = name.deref().into();
                label.position = pt_node;
                labels.push((label, Radians(angle as f32)));
            }
        }
        labels
    }

    pub fn visible_branch_labels(
        &self,
        tree_rect_width: Float,
        _tree_rect_height: Float,
        visible_nodes: &Vec<NodePoint>,
        label_template: &Text,
    ) -> Vec<Text> {
        let mut labels: Vec<Text> = Vec::with_capacity(visible_nodes.len());
        for NodePoint { point, edge } in visible_nodes {
            if edge.parent_node_id.is_none() {
                continue;
            }
            let mut pt_node = *point;
            pt_node.x -= edge.brlen_normalized as Float * tree_rect_width / 2e0;
            let mut label = label_template.clone();
            label.content = format!("{:.3}", edge.brlen);
            label.position = pt_node;
            label.align_x = Horizontal::Center;
            label.align_y = Vertical::Bottom;
            labels.push(label);
        }
        labels
    }

    pub fn visible_tip_idx_range(&self) -> Option<IndexRange> {
        let tip_idx_0: i64 = (self.cnv_y0 / self.node_size) as i64 - 3;
        let tip_idx_1: i64 = (self.cnv_y1 / self.node_size) as i64 + 3;
        let tip_idx_0: usize = tip_idx_0.max(0) as usize;
        let tip_idx_1: usize = tip_idx_1.min(self.tree_tip_edges.len() as i64 - 1) as usize;

        if tip_idx_0 < tip_idx_1 { Some(IndexRange { b: tip_idx_0, e: tip_idx_1 }) } else { None }
    }

    pub fn visible_node_ranges(&self, tip_idx_range: &IndexRange) -> ChunkEdgeRange {
        let idx_0 = &self.tree_tip_edges[tip_idx_range.b];
        let idx_1 = &self.tree_tip_edges[tip_idx_range.e];

        let chnk_idx_0 = idx_0.chunk_idx;
        let edge_idx_0 = idx_0.edge_idx;

        let chnk_idx_1 = idx_1.chunk_idx;
        let edge_idx_1 = idx_1.edge_idx;

        ChunkEdgeRange {
            chnk: IndexRange { b: chnk_idx_0, e: chnk_idx_1 },
            edge: IndexRange { b: edge_idx_0, e: edge_idx_1 },
        }
    }

    pub fn visible_nodes(
        &self,
        width: Float,
        height: Float,
        tip_idx_range: &IndexRange,
    ) -> Vec<NodePoint> {
        let ChunkEdgeRange {
            chnk: IndexRange { b: chnk_idx_0, e: chnk_idx_1 },
            edge: IndexRange { b: edge_idx_0, e: edge_idx_1 },
        } = self.visible_node_ranges(tip_idx_range);

        let mut points: Vec<NodePoint> = Vec::new();
        if chnk_idx_0 == chnk_idx_1 {
            let chunk = &self.tree_chunked_edges[chnk_idx_0];
            for e in &chunk[edge_idx_0..=edge_idx_1] {
                let point = Point { x: e.x1 as Float * width, y: e.y as Float * height };
                points.push(NodePoint { point, edge: e.clone() });
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
                    let point = Point { x: e.x1 as Float * width, y: e.y as Float * height };
                    points.push(NodePoint { point, edge: e.clone() });
                }
            }
        }
        points
    }
}
