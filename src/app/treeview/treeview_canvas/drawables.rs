use super::super::{TreeStyleOption, TreeView};
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
    ops::RangeInclusive,
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
    pub angle: Option<Float>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodePoints {
    pub points: Vec<NodePoint>,
    pub center: Point,
    pub size: Float,
}

#[derive(Debug, Clone, Default)]
pub struct Label {
    pub text: Text,
    pub angle: Option<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EdgePoints {
    pub pt_0: Point,
    pub pt_1: Point,
}

impl TreeView {
    pub fn paths_from_chunks(&self, w: Float, h: Float, center: Point, size: Float) -> Vec<Path> {
        let rot_angle = self.rot_angle;
        let max_angle = self.opn_angle;
        let repr: TreeStyleOption = self.sel_tree_style_opt;
        let mut paths: Vec<Path> = Vec::with_capacity(self.node_count);
        thread::scope(|thread_scope| {
            let mut handles: Vec<ScopedJoinHandle<'_, Path>> = Vec::new();
            for chunk in &self.tree_edges_chunked {
                let handle = thread_scope.spawn(move || {
                    let mut pb = PathBuilder::new();
                    for edge in chunk {
                        match repr {
                            TreeStyleOption::Phylogram => edge_path_phylogram(w, h, edge, &mut pb),
                            TreeStyleOption::Fan => {
                                edge_path_fan(rot_angle, max_angle, center, size, edge, &mut pb)
                            }
                        }
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

    pub fn node_labels(
        &self,
        nodes: &Vec<NodePoint>,
        tips: bool,
        label_text_template: &Text,
    ) -> Vec<Label> {
        let mut labels: Vec<Label> = Vec::with_capacity(nodes.len());
        for NodePoint { point, edge, angle } in nodes {
            if (tips && !edge.is_tip) || (!tips && edge.is_tip) {
                continue;
            }
            if let Some(name) = &edge.name {
                let mut text = label_text_template.clone();
                text.content = name.to_string();
                text.position = *point;
                labels.push(Label { text, angle: *angle });
            }
        }
        labels
    }

    pub fn branch_labels(
        &self,
        size: Float,
        visible_nodes: &Vec<NodePoint>,
        label_text_template: &Text,
    ) -> Vec<Label> {
        let mut label_text_template = label_text_template.clone();
        label_text_template.align_x = Horizontal::Center;
        label_text_template.align_y = Vertical::Bottom;
        let mut labels: Vec<Label> = Vec::with_capacity(visible_nodes.len());
        for NodePoint { point, edge, angle } in visible_nodes {
            if edge.parent_node_id.is_none() {
                continue;
            }
            let mut text = label_text_template.clone();
            let mut node_point = *point;

            let adj = edge.brlen_normalized as Float * size / 2e0;
            if let Some(angle) = angle {
                node_point.x -= angle.cos() * adj;
                node_point.y -= angle.sin() * adj;
            } else {
                node_point.x -= adj;
            }

            text.position = node_point;
            text.content = format!("{:.3}", edge.brlen);
            labels.push(Label { text, angle: *angle });
        }
        labels
    }

    pub fn visible_tip_idx_range(&self) -> Option<IndexRange> {
        match self.sel_tree_style_opt {
            TreeStyleOption::Phylogram => {
                let tip_idx_0: i64 = (self.tre_cnv_y0 / self.node_size) as i64 - 3;
                let tip_idx_1: i64 = (self.tre_cnv_y1 / self.node_size) as i64 + 3;
                let tip_idx_0: usize = tip_idx_0.max(0) as usize;
                let tip_idx_1: usize = tip_idx_1.min(self.tree_tip_edges.len() as i64 - 1) as usize;
                if tip_idx_0 < tip_idx_1 {
                    Some(IndexRange { b: tip_idx_0, e: tip_idx_1 })
                } else {
                    None
                }
            }
            TreeStyleOption::Fan => Some(IndexRange { b: 0, e: self.tree_tip_edges.len() - 1 }),
        }
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

    pub fn visible_nodes(&self, w: Float, h: Float, tip_idx_range: &IndexRange) -> NodePoints {
        let ChunkEdgeRange {
            chnk: IndexRange { b: chnk_idx_0, e: chnk_idx_1 },
            edge: IndexRange { b: edge_idx_0, e: edge_idx_1 },
        } = self.visible_node_ranges(tip_idx_range);
        let tree_repr = self.sel_tree_style_opt;
        let size: Float = match tree_repr {
            TreeStyleOption::Phylogram => w,
            TreeStyleOption::Fan => w.min(h) / 2e0,
        };
        let center = Point { x: w / 2e0, y: h / 2e0 };
        let mut points: Vec<NodePoint> = Vec::new();
        if chnk_idx_0 == chnk_idx_1 {
            let chunk = &self.tree_edges_chunked[chnk_idx_0];
            for e in &chunk[edge_idx_0..=edge_idx_1] {
                let mut angle: Option<Float> = None;
                let point: Point;
                match tree_repr {
                    TreeStyleOption::Phylogram => {
                        point = node_point(w, h, e);
                    }
                    TreeStyleOption::Fan => {
                        let a = edge_angle(self.rot_angle, self.opn_angle, e);
                        point = node_point_rad(a, center, size, e);
                        angle = Some(a);
                    }
                }
                points.push(NodePoint { point, edge: e.clone(), angle });
            }
        } else {
            for chnk_idx in chnk_idx_0..=chnk_idx_1 {
                let edge_range: RangeInclusive<usize>;
                if chnk_idx == chnk_idx_0 {
                    edge_range = edge_idx_0..=self.tree_edges_chunked[chnk_idx].len() - 1;
                } else if chnk_idx == chnk_idx_1 {
                    edge_range = 0..=edge_idx_1
                } else {
                    edge_range = 0..=self.tree_edges_chunked[chnk_idx].len() - 1;
                }

                let chunk = &self.tree_edges_chunked[chnk_idx];

                for e in &chunk[edge_range] {
                    let mut angle: Option<Float> = None;
                    let point: Point;
                    match tree_repr {
                        TreeStyleOption::Phylogram => {
                            point = node_point(w, h, e);
                        }
                        TreeStyleOption::Fan => {
                            let a = edge_angle(self.rot_angle, self.opn_angle, e);
                            point = node_point_rad(a, center, size, e);
                            angle = Some(a);
                        }
                    }
                    points.push(NodePoint { point, edge: e.clone(), angle });
                }
            }
        }

        NodePoints { points, center, size }
    }
}

#[inline]
fn edge_path_phylogram(w: Float, h: Float, edge: &Edge, pb: &mut PathBuilder) {
    let EdgePoints { pt_0, pt_1 } = edge_points(w, h, edge);
    pb.move_to(pt_1);
    pb.line_to(pt_0);
    if let Some(y_parent) = edge.y_parent {
        let pt_parent = Point { x: pt_0.x, y: y_parent as Float * h };
        pb.line_to(pt_parent)
    };
}

#[inline]
fn edge_path_fan(
    rot_angle: Float,
    opn_angle: Float,
    center: Point,
    size: Float,
    edge: &Edge,
    pb: &mut PathBuilder,
) {
    let angle = edge_angle(rot_angle, opn_angle, edge);
    let EdgePoints { pt_0, pt_1 } = edge_points_rad(angle, center, size, edge);
    pb.move_to(pt_1);
    pb.line_to(pt_0);
    if let Some(y_parent) = edge.y_parent {
        let angle_parent = rot_angle + y_parent as Float * opn_angle;
        let p_arc = Arc {
            center,
            radius: center.distance(pt_0),
            start_angle: Radians(angle),
            end_angle: Radians(angle_parent),
        };
        pb.arc(p_arc);
    };
}

#[inline]
fn edge_point(w: Float, h: Float, edge: &Edge) -> Point {
    let x = edge.x0 as Float * w;
    let y = edge.y as Float * h;
    Point { x, y }
}

#[inline]
fn edge_point_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> Point {
    let x0 = edge.x0 as Float * angle.cos() * size;
    let y0 = edge.x0 as Float * angle.sin() * size;
    Point { x: center.x + x0, y: center.y + y0 }
}

#[inline]
fn node_point(w: Float, h: Float, edge: &Edge) -> Point {
    let x = edge.x1 as Float * w;
    let y = edge.y as Float * h;
    Point { x, y }
}

#[inline]
fn node_point_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> Point {
    let x1 = edge.x1 as Float * angle.cos() * size;
    let y1 = edge.x1 as Float * angle.sin() * size;
    Point { x: center.x + x1, y: center.y + y1 }
}

#[inline]
fn edge_points(w: Float, h: Float, edge: &Edge) -> EdgePoints {
    let pt_0 = edge_point(w, h, edge);
    let pt_1 = node_point(w, h, edge);
    EdgePoints { pt_0, pt_1 }
}

#[inline]
fn edge_points_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> EdgePoints {
    let pt_0 = edge_point_rad(angle, center, size, edge);
    let pt_1 = node_point_rad(angle, center, size, edge);
    EdgePoints { pt_0, pt_1 }
}

#[inline]
fn edge_angle(rot_angle: Float, opn_angle: Float, edge: &Edge) -> Float {
    rot_angle + edge.y as Float * opn_angle
}
