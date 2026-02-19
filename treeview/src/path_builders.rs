use crate::cnv_utils::*;
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

pub fn path_builder_ticks_x(
    w: Float,
    h: Float,
    padding_bottom: Float,
    ticks_x: &[Tick],
    tick_size: Float,
    lab_size: Float,
) -> (PathBuilder, Vec<Label>) {
    let mut pb: PathBuilder = PathBuilder::new();
    let bottom = h + padding_bottom;
    let mut labs_x: Vec<Label> = Vec::with_capacity(ticks_x.len());

    for Tick { relative_position, label } in ticks_x {
        let x = relative_position * w;
        let tick_pt1 = Point { x, y: bottom };
        let tick_pt2 = Point { x, y: bottom + tick_size };
        pb = pb.move_to(tick_pt1);
        pb = pb.line_to(tick_pt2);

        let text = lab_text(
            label.to_string(),
            tick_pt2,
            lab_size,
            TEMPLATE_TXT_LAB_PLOT_AXIS_X,
            false,
        );
        let label = Label { text, width: ZRO, angle: 0.0, aligned_from: None };
        labs_x.push(label);
    }

    (pb, labs_x)
}

pub fn path_builder_ticks_y(
    h: Float,
    padding_left: Float,
    ticks_y: &[Tick],
    tick_size: Float,
    lab_size: Float,
) -> (PathBuilder, Vec<Label>) {
    let mut pb: PathBuilder = PathBuilder::new();
    let left = -padding_left;
    let mut labs_y: Vec<Label> = Vec::with_capacity(ticks_y.len());

    for Tick { relative_position, label } in ticks_y {
        let y = (ONE - relative_position) * h;
        let tick_pt1 = Point { x: left, y };
        let tick_pt2 = Point { x: left - tick_size, y };
        pb = pb.move_to(tick_pt1);
        pb = pb.line_to(tick_pt2);

        let text = lab_text(
            label.to_string(),
            tick_pt2,
            lab_size,
            TEMPLATE_TXT_LAB_PLOT_AXIS_Y,
            false,
        );
        let label = Label { text, width: ZRO, angle: 0.0, aligned_from: None };
        labs_y.push(label);
    }

    (pb, labs_y)
}

pub fn path_builder_x_axis(
    w: Float,
    h: Float,
    axes_padd: Float,
) -> PathBuilder {
    let mut pb: PathBuilder = PathBuilder::new();

    let left = -axes_padd;
    let right = w + axes_padd;
    let top = -axes_padd;
    let bottom = h + axes_padd;

    // x-axis line bottom ------------------------------------------------------
    pb = pb.move_to(Point { x: left, y: bottom });
    pb = pb.line_to(Point { x: right, y: bottom });

    // x-axis line top ---------------------------------------------------------
    pb = pb.move_to(Point { x: left, y: top });
    pb = pb.line_to(Point { x: right, y: top });

    pb
}

pub fn path_builder_y_axis(
    w: Float,
    h: Float,
    axes_padd: Float,
) -> PathBuilder {
    let mut pb: PathBuilder = PathBuilder::new();

    let left = -axes_padd;
    let right = w + axes_padd;
    let top = -axes_padd;
    let bottom = h + axes_padd;

    // y-axis line left --------------------------------------------------------
    pb = pb.move_to(Point { x: left, y: top });
    pb = pb.line_to(Point { x: left, y: bottom });

    // y-axis line right -------------------------------------------------------
    pb = pb.move_to(Point { x: right, y: top });
    pb = pb.line_to(Point { x: right, y: bottom });

    pb
}

pub fn path_builder_ltt(
    data: &PlotData,
    x_axis_scale_type: AxisScaleType,
    y_axis_scale_type: AxisScaleType,
    w: Float,
    h: Float,
) -> PathBuilder {
    let mut first = true;
    let mut pb: PathBuilder = PathBuilder::new();

    let x_min = data.x_min;
    let x_max = data.x_max;
    let y_min = data.y_min;
    let y_max = data.y_max;

    for plot_point in &data.plot_points {
        let x_rel = transformed_relative_value(
            plot_point.x, x_min, x_max, x_axis_scale_type,
        )
        .unwrap_or(0e0)
        .clamp(ZRO, ONE);

        let y_rel = transformed_relative_value(
            plot_point.y, y_min, y_max, y_axis_scale_type,
        )
        .unwrap_or(0e0)
        .clamp(ZRO, ONE);

        let pt = Point { x: x_rel * w, y: (1e0 - y_rel) * h };

        match first {
            true => {
                pb = pb.move_to(pt);
                first = false;
            }
            false => pb = pb.line_to(pt),
        }
    }
    pb
}
