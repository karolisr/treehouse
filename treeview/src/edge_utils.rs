use crate::iced::*;
use crate::*;

impl From<NodeDataCart> for NodeData {
    #[inline]
    fn from(nd: NodeDataCart) -> Self {
        Self { edge_idx: nd.edge_idx, points: nd.points, y_parent: nd.y_parent, angle: None, angle_parent: None }
    }
}

impl From<NodeDataPol> for NodeData {
    #[inline]
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

// #[inline]
// pub fn edge_path_cart(nd: &NodeDataCart) -> Path { edge_path_cart_pb(nd, PathBuilder::new()).build() }
#[inline]
pub fn edge_path_cart_pb(nd: &NodeDataCart, pb: PathBuilder) -> PathBuilder {
    pb.move_to(nd.points.p1).line_to(nd.points.p0)
}

// #[inline]
// pub fn edge_path_pol(nd: &NodeDataPol) -> Path { edge_path_pol_pb(nd, PathBuilder::new()).build() }
#[inline]
pub fn edge_path_pol_pb(nd: &NodeDataPol, pb: PathBuilder) -> PathBuilder {
    pb.move_to(nd.points.p1).line_to(nd.points.p0)
}

// #[inline]
// pub fn edge_path_vert_cart(nd: &NodeDataCart) -> Path { edge_path_vert_cart_pb(nd, PathBuilder::new()).build() }
#[inline]
pub fn edge_path_vert_cart_pb(nd: &NodeDataCart, pb: PathBuilder) -> PathBuilder {
    if let Some(y_parent) = nd.y_parent {
        let pt_parent = Point { x: nd.points.p0.x, y: y_parent };
        pb.move_to(nd.points.p0).line_to(pt_parent)
    } else {
        pb
    }
}

// #[inline]
// pub fn edge_path_arc_pol(nd: &NodeDataPol) -> Path { edge_path_arc_pol_pb(nd, PathBuilder::new()).build() }
#[inline]
pub fn edge_path_arc_pol_pb(nd: &NodeDataPol, pb: PathBuilder) -> PathBuilder {
    if let Some(angle_parent) = nd.angle_parent {
        pb.move_to(nd.points.p0).arc(nd.angle, angle_parent, Point::ORIGIN, Point::ORIGIN.distance(nd.points.p0))
    } else {
        pb
    }
}

#[inline]
pub fn tip_idx_range_between_y_vals(y0: Float, y1: Float, node_size: Float, tips: &[usize]) -> Option<IndexRange> {
    if node_size <= 0e0 {
        return None;
    }
    let i0: i64 = (y0 / node_size) as i64;
    let i1: i64 = (y1 / node_size) as i64;
    if i1.abs() <= i0.abs() {
        return None;
    }
    let idx_tip_0: usize = i0.max(0) as usize;
    let idx_tip_1: usize = i1.abs().min(tips.len() as i64 - 1) as usize;
    if idx_tip_0 < idx_tip_1 { Some(IndexRange::new(idx_tip_0, idx_tip_1)) } else { None }
}

#[inline]
pub fn node_idx_range_for_tip_idx_range(tip_idx_range: &IndexRange, tips: &[usize]) -> IndexRange {
    let idx_node_0 = tips[*tip_idx_range.start()];
    let idx_node_1 = tips[*tip_idx_range.end()];
    IndexRange::new(idx_node_0, idx_node_1)
}

#[inline]
pub fn point_cart(w: Float, h: Float, edge_x: Float, edge_y: Float) -> Point {
    let x = edge_x * w;
    let y = edge_y * h;
    Point { x, y }
}

#[inline]
pub fn edge_point_cart(w: Float, h: Float, edge: &Edge) -> Point { point_cart(w, h, edge.x0 as Float, edge.y as Float) }

#[inline]
pub fn edge_midpoint_cart(w: Float, h: Float, edge: &Edge) -> Point {
    point_cart(w, h, edge.x_mid as Float, edge.y as Float)
}

#[inline]
pub fn node_point_cart(w: Float, h: Float, edge: &Edge) -> Point { point_cart(w, h, edge.x1 as Float, edge.y as Float) }

#[inline]
pub fn edge_points_cart(w: Float, h: Float, edge: &Edge) -> EdgePoints {
    let p0 = edge_point_cart(w, h, edge);
    let p_mid = edge_midpoint_cart(w, h, edge);
    let p1 = node_point_cart(w, h, edge);
    EdgePoints { p0, p_mid, p1 }
}

#[inline]
pub fn edge_angle(opn_angle: Float, edge: &Edge) -> Float { opn_angle * edge.y as Float }

#[inline]
pub fn point_pol(angle: Float, size: Float, offset: Float, edge_x: Float) -> Point {
    let (sin, cos) = angle.sin_cos();
    let size = size - offset;
    let x = offset * cos + edge_x * cos * size;
    let y = offset * sin + edge_x * sin * size;
    Point { x, y }
}

#[inline]
pub fn edge_point_pol(angle: Float, size: Float, offset: Float, edge: &Edge) -> Point {
    point_pol(angle, size, offset, edge.x0 as Float)
}

#[inline]
pub fn edge_midpoint_pol(angle: Float, size: Float, offset: Float, edge: &Edge) -> Point {
    point_pol(angle, size, offset, edge.x_mid as Float)
}

#[inline]
pub fn node_point_pol(angle: Float, size: Float, offset: Float, edge: &Edge) -> Point {
    point_pol(angle, size, offset, edge.x1 as Float)
}

#[inline]
pub fn edge_points_pol(angle: Float, size: Float, offset: Float, edge: &Edge) -> EdgePoints {
    let p0 = edge_point_pol(angle, size, offset, edge);
    let p_mid = edge_midpoint_pol(angle, size, offset, edge);
    let p1 = node_point_pol(angle, size, offset, edge);
    EdgePoints { p0, p_mid, p1 }
}

#[inline]
pub fn node_data_cart(w: Float, h: Float, edge: &Edge) -> NodeDataCart {
    let points = edge_points_cart(w, h, edge);
    let mut y_parent: Option<Float> = None;
    if let Some(y) = edge.y_parent {
        y_parent = Some(y as Float * h);
    }
    NodeDataCart { edge_idx: edge.edge_idx, points, y_parent }
}

#[inline]
pub fn node_data_rad(opn_angle: Float, size: Float, offset: Float, edge: &Edge) -> NodeDataPol {
    let angle = edge_angle(opn_angle, edge);
    let mut angle_parent: Option<Float> = None;
    if let Some(y) = edge.y_parent {
        angle_parent = Some(opn_angle * y as Float);
    }
    let points = edge_points_pol(angle, size, offset, edge);
    NodeDataPol { edge_idx: edge.edge_idx, points, angle, angle_parent }
}
