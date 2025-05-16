use crate::*;
use dendros::{Edge, Edges};
use iced::{
    Pixels, Point, Radians, Rectangle, Vector,
    alignment::Vertical,
    widget::{
        canvas::{
            Frame, Path, Stroke, Text,
            path::{Arc, Builder as PathBuilder},
        },
        text::Alignment as TextAlignment,
    },
};

pub fn draw_rect(rect: Rectangle, stroke: Stroke, frame: &mut Frame) {
    frame.stroke_rectangle(Point { x: rect.x, y: rect.y }, rect.size(), stroke);
}

pub fn draw_point(point: Point, stroke: Stroke, radius: Float, frame: &mut Frame) {
    let path = Path::circle(point, radius);
    frame.stroke(&path, stroke);
}

pub fn tip_idx_range_vis(
    y0: Float, y1: Float, node_size: Float, edges_tip: &[Edge], tree_style: TreeStyle,
) -> Option<IndexRange> {
    match tree_style {
        TreeStyle::Phylogram => {
            let i0: i64 = (y0 / node_size) as i64 - 3;
            let i1: i64 = (y1 / node_size) as i64 + 3;
            let i0: usize = i0.max(0) as usize;
            let i1: usize = i1.min(edges_tip.len() as i64 - 1) as usize;
            if i0 < i1 { Some(IndexRange { b: i0, e: i1 }) } else { None }
        }
        TreeStyle::Fan => Some(IndexRange { b: 0, e: edges_tip.len() - 1 }),
    }
}

pub fn node_idx_range_vis(tip_idx_range: &IndexRange, edges_tip: &[Edge]) -> IndexRange {
    let it0 = &edges_tip[tip_idx_range.b];
    let it1 = &edges_tip[tip_idx_range.e];
    let in0 = it0.edge_idx;
    let in1 = it1.edge_idx;
    IndexRange { b: in0, e: in1 }
}
