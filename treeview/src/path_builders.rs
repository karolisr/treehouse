use crate::edge_utils::*;
use crate::*;

pub fn path_edges_phygrm(edges: &[Edge], w: Float, h: Float) -> IcedPath {
    let mut pb: PathBuilder = PathBuilder::new();
    let nds = edges.iter().map(|edge| node_data_cart(w, h, edge));
    for nd in nds {
        pb = edge_path_cart(&nd, pb);
        pb = edge_path_vert_cart(&nd, pb);
    }
    pb.build()
}

pub fn path_edges_fan(
    edges: &[Edge],
    opn_angle: Float,
    root_len: Float,
    radius: Float,
) -> IcedPath {
    let mut pb: PathBuilder = PathBuilder::new();
    if opn_angle >= ONE.to_radians() {
        let nds = edges
            .iter()
            .map(|edge| node_data_pol(opn_angle, ZRO, radius, root_len, edge));
        for nd in nds {
            pb = edge_path_pol(&nd, pb);
            pb = edge_path_arc_pol(&nd, pb);
        }
    } else {
        let p0 = Point { x: root_len, y: ZRO };
        let p1 = Point { x: radius, y: ZRO };
        pb = pb.move_to(p0).line_to(p1);
    }
    pb.build()
}

pub fn path_root_edge_phygrm(
    w: Float,
    h: Float,
    root_len: Float,
    root_edge: &Edge,
) -> IcedPath {
    let nd: NodeDataCart = node_data_cart(w, h, root_edge);
    let pt_parent: Point = Point { x: -root_len, y: nd.points.p0.y };
    PathBuilder::new().move_to(pt_parent).line_to(nd.points.p0).build()
}

pub fn path_root_edge_fan(
    radius: Float,
    opn_angle: Float,
    root_len: Float,
    root_edge: &Edge,
) -> IcedPath {
    let nd: NodeDataPol =
        node_data_pol(opn_angle, ZRO, radius, root_len, root_edge);
    PathBuilder::new().move_to(ORIGIN).line_to(nd.points.p0).build()
}

pub fn path_clade_highlight(
    node_id: NodeId,
    tree_state: &TreeState,
    w: Float,
    h: Float,
    radius: Float,
    root_len: Float,
    opn_angle: Float,
    tre_sty: TreSty,
) -> IcedPath {
    match tre_sty {
        TreSty::PhyGrm => {
            path_clade_highlight_phygrm(node_id, tree_state, w, h)
        }
        TreSty::Fan => path_clade_highlight_fan(
            node_id, tree_state, radius, root_len, opn_angle,
        ),
    }
}

pub fn path_clade_highlight_phygrm(
    node_id: NodeId,
    tree_state: &TreeState,
    w: Float,
    h: Float,
) -> IcedPath {
    let bounding_edges_opt = tree_state.bounding_edges_for_clade(node_id);
    let mut pb: PathBuilder = PathBuilder::new();
    if let Some((edges_top, edges_bottom)) = bounding_edges_opt
        && !edges_top.is_empty()
        && !edges_bottom.is_empty()
    {
        if let Some(edge_top_first) = edges_top.first() {
            let y_top = edge_top_first.y as Float * h;
            let top_right = Point { x: w, y: y_top };
            pb = pb.move_to(top_right);
        }

        for edge in &edges_top {
            let nd = node_data_cart(w, h, edge);
            pb = pb.line_to(nd.points.p0);
            let pt_parent = Point { x: nd.points.p0.x, y: nd.y_parent };
            pb = pb.line_to(pt_parent);
        }

        for edge in &edges_bottom {
            let nd = node_data_cart(w, h, edge);
            let pt_parent = Point { x: nd.points.p0.x, y: nd.y_parent };
            pb = pb.line_to(pt_parent);
            pb = pb.line_to(nd.points.p0);
        }

        if let Some(edge_bottom_last) = edges_bottom.last() {
            let y_bottom = edge_bottom_last.y as Float * h;
            let bottom_right = Point { x: w, y: y_bottom };
            pb = pb.line_to(bottom_right);
        }
    }

    pb = pb.close();
    pb.build()
}

pub fn path_clade_highlight_fan(
    node_id: NodeId,
    tree_state: &TreeState,
    radius: Float,
    root_len: Float,
    opn_angle: Float,
) -> IcedPath {
    let bounding_edges_opt = tree_state.bounding_edges_for_clade(node_id);
    let mut pb: PathBuilder = PathBuilder::new();
    if let Some((edges_top, edges_bottom)) = bounding_edges_opt
        && !edges_top.is_empty()
        && !edges_bottom.is_empty()
    {
        let nd = node_data_pol(
            opn_angle,
            ZRO,
            radius,
            root_len,
            edges_top.first().unwrap(),
        );

        let angle_top = nd.angle;
        let top_right = point_pol(angle_top, radius, root_len, ONE);
        pb = pb.move_to(top_right);

        for edge in &edges_top {
            let nd = node_data_pol(opn_angle, ZRO, radius, root_len, edge);
            pb = pb.line_to(nd.points.p0);
            pb = pb.arc(
                nd.angle,
                nd.angle_parent,
                ORIGIN,
                ORIGIN.distance(nd.points.p0),
            );
        }

        for edge in &edges_bottom {
            let nd = node_data_pol(opn_angle, ZRO, radius, root_len, edge);
            pb = pb.arc(
                nd.angle_parent,
                nd.angle,
                ORIGIN,
                ORIGIN.distance(nd.points.p0),
            );
            pb = pb.line_to(nd.points.p1);
        }

        let nd = node_data_pol(
            opn_angle,
            ZRO,
            radius,
            root_len,
            edges_bottom.last().unwrap(),
        );

        let bottom_right = point_pol(nd.angle, radius, root_len, ONE);
        pb = pb.line_to(bottom_right);
        pb.arc(nd.angle, angle_top, ORIGIN, ORIGIN.distance(bottom_right))
            .build()
    } else {
        pb = pb.close();
        pb.build()
    }
}
