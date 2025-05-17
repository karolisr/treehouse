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

fn edge_point(w: Float, h: Float, edge: &Edge) -> Point {
    let x = edge.x0 as Float * w;
    let y = edge.y as Float * h;
    Point { x, y }
}

fn node_point(w: Float, h: Float, edge: &Edge) -> Point {
    let x = edge.x1 as Float * w;
    let y = edge.y as Float * h;
    Point { x, y }
}

fn edge_points(w: Float, h: Float, edge: &Edge) -> EdgePoints {
    let p0 = edge_point(w, h, edge);
    let p1 = node_point(w, h, edge);
    EdgePoints { p0, p1 }
}

fn edge_path_phylogram(w: Float, h: Float, edge: &Edge, pb: &mut PathBuilder, root_len: Float) {
    let EdgePoints { p0, p1 } = edge_points(w, h, edge);
    pb.move_to(p1);
    pb.line_to(p0);
    if let Some(y_parent) = edge.y_parent {
        let pt_parent = Point { x: p0.x, y: y_parent as Float * h };
        pb.line_to(pt_parent)
    } else if edge.parent_node_id.is_none() && root_len > 0e0 {
        let pt_parent = Point { x: root_len * -1e0, y: edge.y as Float * h };
        pb.line_to(pt_parent)
    }
}

pub fn stroke_edges(
    edges: &[Edge], tree_vs: &RectVals<Float>, root_len_opt: Option<Float>, f: &mut Frame,
) {
    let mut pb = PathBuilder::new();
    let mut w = tree_vs.w;
    let mut rl = 0e0;

    if let Some(root_len) = root_len_opt {
        w -= root_len;
        rl = root_len
    }

    for e in edges {
        edge_path_phylogram(w, tree_vs.h, e, &mut pb, rl)
    }

    let path: Path = pb.build();

    f.with_save(|f| {
        f.translate(tree_vs.trans);
        f.translate(Vector { x: rl, y: 0e0 });
        f.stroke(&path, STRK_EDGE);
    })
}
