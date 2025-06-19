use crate::*;

impl From<NodeDataCart> for NodeData {
    fn from(nd: NodeDataCart) -> Self {
        Self {
            edge_idx: nd.edge_idx,
            points: nd.points,
            y_parent: nd.y_parent,
            angle: None,
            angle_parent: None,
        }
    }
}

impl From<NodeDataPol> for NodeData {
    fn from(nd: NodeDataPol) -> Self {
        Self {
            edge_idx: nd.edge_idx,
            points: nd.points,
            angle: Some(nd.angle),
            y_parent: None,
            angle_parent: nd.angle_parent,
        }
    }
}

pub fn edge_path_cart(nd: &NodeDataCart, pb: PathBuilder) -> PathBuilder {
    pb.move_to(nd.points.p1).line_to(nd.points.p0)
}

pub fn edge_path_pol(nd: &NodeDataPol, pb: PathBuilder) -> PathBuilder {
    pb.move_to(nd.points.p1).line_to(nd.points.p0)
}

pub fn edge_path_vert_cart(nd: &NodeDataCart, pb: PathBuilder) -> PathBuilder {
    if let Some(y_parent) = nd.y_parent {
        let pt_parent = Point { x: nd.points.p0.x, y: y_parent };
        pb.move_to(nd.points.p0).line_to(pt_parent)
    } else {
        pb
    }
}

pub fn edge_path_arc_pol(nd: &NodeDataPol, pb: PathBuilder) -> PathBuilder {
    if let Some(angle_parent) = nd.angle_parent {
        pb.move_to(nd.points.p0).arc_approx_line(
            nd.angle,
            angle_parent,
            ORIGIN,
            ORIGIN.distance(nd.points.p0),
        )
    } else {
        pb
    }
}

pub fn tip_idx_range_between_y_vals(
    y0: Float, y1: Float, node_size: Float, tips: &[usize],
) -> Option<IndexRange> {
    if node_size <= ZRO {
        return None;
    }
    let i0: i64 = (y0 / node_size) as i64;
    let i1: i64 = (y1 / node_size) as i64;
    if i1.abs() < i0.abs() {
        return None;
    }
    let mut tip_idx_0: usize = i0.max(0) as usize;
    let mut tip_idx_1: usize = i1.abs().min(tips.len() as i64 - 1) as usize;
    if tip_idx_0 == tip_idx_1 {
        if tip_idx_0 > 0 {
            tip_idx_0 -= 1;
        } else if tip_idx_1 < tips.len().max(1) - 1 {
            tip_idx_1 += 1;
        }
    }
    if tip_idx_0 < tip_idx_1 { Some(IndexRange::new(tip_idx_0, tip_idx_1)) } else { None }
}

pub fn node_idx_range_for_tip_idx_range(tip_idx_range: &IndexRange, tips: &[usize]) -> IndexRange {
    let idx_node_0 = tips[*tip_idx_range.start()];
    let idx_node_1 = tips[*tip_idx_range.end()];
    IndexRange::new(idx_node_0, idx_node_1)
}

pub fn point_cart(w: Float, h: Float, edge_x: Float, edge_y: Float) -> Point {
    let x = edge_x * w;
    let y = edge_y * h;
    Point { x, y }
}

pub fn edge_point_cart(w: Float, h: Float, edge: &Edge) -> Point {
    point_cart(w, h, edge.x0 as Float, edge.y as Float)
}

pub fn edge_midpoint_cart(w: Float, h: Float, edge: &Edge) -> Point {
    point_cart(w, h, edge.x_mid as Float, edge.y as Float)
}

pub fn node_point_cart(w: Float, h: Float, edge: &Edge) -> Point {
    point_cart(w, h, edge.x1 as Float, edge.y as Float)
}

pub fn edge_points_cart(w: Float, h: Float, edge: &Edge) -> EdgePoints {
    let p0 = edge_point_cart(w, h, edge);
    let p_mid = edge_midpoint_cart(w, h, edge);
    let p1 = node_point_cart(w, h, edge);
    EdgePoints { p0, p_mid, p1 }
}

pub fn edge_angle(opn_angle: Float, edge: &Edge) -> Float { opn_angle * edge.y as Float }

pub fn point_pol(angle: Float, radius: Float, offset: Float, edge_x: Float) -> Point {
    let (sin, cos) = angle.sin_cos();
    let size = radius - offset;
    let x = offset * cos + edge_x * cos * size;
    let y = offset * sin + edge_x * sin * size;
    Point { x, y }
}

pub fn edge_point_pol(angle: Float, radius: Float, offset: Float, edge: &Edge) -> Point {
    point_pol(angle, radius, offset, edge.x0 as Float)
}

pub fn edge_midpoint_pol(angle: Float, radius: Float, offset: Float, edge: &Edge) -> Point {
    point_pol(angle, radius, offset, edge.x_mid as Float)
}

pub fn node_point_pol(angle: Float, radius: Float, offset: Float, edge: &Edge) -> Point {
    point_pol(angle, radius, offset, edge.x1 as Float)
}

pub fn edge_points_pol(angle: Float, radius: Float, offset: Float, edge: &Edge) -> EdgePoints {
    let p0 = edge_point_pol(angle, radius, offset, edge);
    let p_mid = edge_midpoint_pol(angle, radius, offset, edge);
    let p1 = node_point_pol(angle, radius, offset, edge);
    EdgePoints { p0, p_mid, p1 }
}

pub fn node_data_cart(w: Float, h: Float, edge: &Edge) -> NodeDataCart {
    let points = edge_points_cart(w, h, edge);
    let mut y_parent: Option<Float> = None;
    if let Some(y) = edge.y_parent {
        y_parent = Some(y as Float * h);
    }
    NodeDataCart { edge_idx: edge.edge_idx, points, y_parent }
}

pub fn node_data_rad(
    opn_angle: Float, rot_angle: Float, radius: Float, offset: Float, edge: &Edge,
) -> NodeDataPol {
    let angle = edge_angle(opn_angle, edge) + rot_angle;
    let mut angle_parent: Option<Float> = None;
    if let Some(y) = edge.y_parent {
        angle_parent = Some(opn_angle * y as Float);
    }
    let points = edge_points_pol(angle, radius, offset, edge);
    NodeDataPol { edge_idx: edge.edge_idx, points, angle, angle_parent }
}
