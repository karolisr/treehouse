use crate::*;

#[inline]
fn point_phylogram(w: Float, h: Float, edge_x: Float, edge_y: Float) -> Point {
    let x = edge_x * w;
    let y = edge_y * h;
    Point { x, y }
}

#[inline]
fn edge_point_phylogram(w: Float, h: Float, edge: &Edge) -> Point {
    point_phylogram(w, h, edge.x0 as Float, edge.y as Float)
}

#[inline]
fn edge_mid_point_phylogram(w: Float, h: Float, edge: &Edge) -> Point {
    point_phylogram(w, h, edge.x_mid as Float, edge.y as Float)
}

#[inline]
fn node_point_phylogram(w: Float, h: Float, edge: &Edge) -> Point {
    point_phylogram(w, h, edge.x1 as Float, edge.y as Float)
}

#[inline]
fn edge_points_phylogram(w: Float, h: Float, edge: &Edge) -> EdgePoints {
    let p0 = edge_point_phylogram(w, h, edge);
    let p_mid = edge_mid_point_phylogram(w, h, edge);
    let p1 = node_point_phylogram(w, h, edge);
    EdgePoints { p0, p_mid, p1 }
}

#[inline]
fn edge_angle(opn_angle: Float, edge: &Edge) -> Float {
    opn_angle * edge.y as Float
}

#[inline]
fn point_rad(angle: Float, size: Float, offset: Float, edge_x: Float) -> Point {
    let (sin, cos) = angle.sin_cos();
    let size = size - offset;
    let x = offset * cos + edge_x * cos * size;
    let y = offset * sin + edge_x * sin * size;
    Point { x, y }
}

#[inline]
fn edge_point_rad(angle: Float, size: Float, offset: Float, edge: &Edge) -> Point {
    point_rad(angle, size, offset, edge.x0 as Float)
}

#[inline]
fn edge_mid_point_rad(angle: Float, size: Float, offset: Float, edge: &Edge) -> Point {
    point_rad(angle, size, offset, edge.x_mid as Float)
}

#[inline]
fn node_point_rad(angle: Float, size: Float, offset: Float, edge: &Edge) -> Point {
    point_rad(angle, size, offset, edge.x1 as Float)
}

#[inline]
fn edge_points_rad(angle: Float, size: Float, offset: Float, edge: &Edge) -> EdgePoints {
    let p0 = edge_point_rad(angle, size, offset, edge);
    let p_mid = edge_mid_point_rad(angle, size, offset, edge);
    let p1 = node_point_rad(angle, size, offset, edge);
    EdgePoints { p0, p_mid, p1 }
}

#[inline]
pub fn node_data_phylogram(w: Float, h: Float, edge: &Edge) -> NodeDataPhylogram {
    let points = edge_points_phylogram(w, h, edge);
    let mut y_parent: Option<Float> = None;
    if let Some(y) = edge.y_parent {
        y_parent = Some(y as Float * h);
    }
    NodeDataPhylogram { edge_idx: edge.edge_idx, points, y_parent }
}

#[inline]
pub fn node_data_rad(opn_angle: Float, size: Float, offset: Float, edge: &Edge) -> NodeDataRad {
    let angle = edge_angle(opn_angle, edge);
    let mut angle_parent: Option<Float> = None;
    if let Some(y) = edge.y_parent {
        angle_parent = Some(opn_angle * y as Float);
    }
    let points = edge_points_rad(angle, size, offset, edge);
    NodeDataRad { edge_idx: edge.edge_idx, points, angle, angle_parent }
}
