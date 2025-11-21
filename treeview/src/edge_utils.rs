use crate::*;

impl From<NodeDataCart> for NodeData {
    fn from(nd: NodeDataCart) -> Self {
        Self {
            node_id: nd.node_id,
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
            node_id: nd.node_id,
            edge_idx: nd.edge_idx,
            points: nd.points,
            y_parent: None,
            angle: Some(nd.angle),
            angle_parent: nd.angle_parent,
        }
    }
}

pub fn edge_path_cart(nd: &NodeDataCart, pb: PathBuilder) -> PathBuilder {
    pb.move_to(nd.points.p1).line_to(nd.points.p0)
}

pub fn edge_path_diag_cart(nd: &NodeDataCart, pb: PathBuilder) -> PathBuilder {
    if let Some(y_parent) = nd.y_parent {
        let pt_parent = Point { x: nd.points.p0.x, y: y_parent };
        pb.move_to(nd.points.p1).line_to(pt_parent)
    } else {
        pb
    }
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
        pb.move_to(nd.points.p0).arc(
            nd.angle,
            angle_parent,
            ORIGIN,
            ORIGIN.distance(nd.points.p0),
        )
    } else {
        pb
    }
}

pub fn point_cart(
    w: Float,
    h: Float,
    x_relative: Float,
    y_relative: Float,
) -> Point {
    let x = x_relative * w;
    let y = y_relative * h;
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

pub fn edge_angle(opn_angle: Float, edge: &Edge) -> Float {
    opn_angle * edge.y as Float
}

/// Calculates coordinates of a point given the angle and radius.
///
/// `radius = radius_relative * size_max + offset_absolute`
///
/// `size_max = radius_absolute - offset_absolute`
///
pub fn point_pol(
    angle: Float,
    radius_absolute: Float,
    offset_absolute: Float,
    radius_relative: Float,
) -> Point {
    let (sin, cos) = angle.sin_cos();
    let size_max = radius_absolute - offset_absolute;
    let radius = radius_relative * size_max + offset_absolute;
    let x = radius * cos;
    let y = radius * sin;
    Point { x, y }
}

pub fn edge_point_pol(
    angle: Float,
    radius: Float,
    offset: Float,
    edge: &Edge,
) -> Point {
    point_pol(angle, radius, offset, edge.x0 as Float)
}

pub fn edge_midpoint_pol(
    angle: Float,
    radius: Float,
    offset: Float,
    edge: &Edge,
) -> Point {
    point_pol(angle, radius, offset, edge.x_mid as Float)
}

pub fn node_point_pol(
    angle: Float,
    radius: Float,
    offset: Float,
    edge: &Edge,
) -> Point {
    point_pol(angle, radius, offset, edge.x1 as Float)
}

pub fn edge_points_pol(
    angle: Float,
    radius: Float,
    offset: Float,
    edge: &Edge,
) -> EdgePoints {
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
    NodeDataCart {
        node_id: edge.node_id,
        edge_idx: edge.edge_index,
        points,
        y_parent,
    }
}

pub fn node_data_rad(
    opn_angle: Float,
    rot_angle: Float,
    radius: Float,
    offset: Float,
    edge: &Edge,
) -> NodeDataPol {
    let angle = edge_angle(opn_angle, edge) + rot_angle;
    let mut angle_parent: Option<Float> = None;
    if let Some(y) = edge.y_parent {
        angle_parent = Some(opn_angle * y as Float);
    }
    let points = edge_points_pol(angle, radius, offset, edge);
    NodeDataPol {
        node_id: edge.node_id,
        edge_idx: edge.edge_index,
        points,
        angle,
        angle_parent,
    }
}

pub fn prepare_nodes(
    tre_vs: &RectVals<Float>,
    root_len: Float,
    tre_sty: TreSty,
    opn_angle: Float,
    edges: &[Edge],
    node_idxs: &[usize],
    results: &mut Vec<NodeData>,
) {
    node_idxs
        .par_iter()
        .map(|&idx| match tre_sty {
            TreSty::PhyGrm => {
                node_data_cart(tre_vs.w, tre_vs.h, &edges[idx]).into()
            }
            TreSty::Fan => node_data_rad(
                opn_angle, ZRO, tre_vs.radius_min, root_len, &edges[idx],
            )
            .into(),
        })
        .collect_into_vec(results);
}
