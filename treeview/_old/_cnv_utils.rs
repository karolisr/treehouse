// fn edge_point(w: Float, h: Float, edge: &Edge) -> Point {
//     let x = edge.x0 as Float * w;
//     let y = edge.y as Float * h;
//     Point { x, y }
// }

// fn edge_point_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> Point {
//     let x0 = edge.x0 as Float * angle.cos() * size;
//     let y0 = edge.x0 as Float * angle.sin() * size;
//     Point { x: center.x + x0, y: center.y + y0 }
// }

// fn node_point(w: Float, h: Float, edge: &Edge) -> Point {
//     let x = edge.x1 as Float * w;
//     let y = edge.y as Float * h;
//     Point { x, y }
// }

// fn node_point_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> Point {
//     let x1 = edge.x1 as Float * angle.cos() * size;
//     let y1 = edge.x1 as Float * angle.sin() * size;
//     Point { x: center.x + x1, y: center.y + y1 }
// }

// fn edge_points(w: Float, h: Float, edge: &Edge) -> EdgePoints {
//     let p0 = edge_point(w, h, edge);
//     let p1 = node_point(w, h, edge);
//     EdgePoints { p0, p1 }
// }

// fn edge_points_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> EdgePoints {
//     let p0 = edge_point_rad(angle, center, size, edge);
//     let p1 = node_point_rad(angle, center, size, edge);
//     EdgePoints { p0, p1 }
// }

// fn edge_angle(rot_angle: Float, opn_angle: Float, edge: &Edge) -> Float {
//     rot_angle + edge.y as Float * opn_angle
// }

// fn edge_path_phylogram(w: Float, h: Float, edge: &Edge, pb: &mut PathBuilder, draw_root: bool) {
//     let EdgePoints { p0, p1 } = edge_points(w, h, edge);
//     pb.move_to(p1);
//     pb.line_to(p0);
//     if let Some(y_parent) = edge.y_parent {
//         let pt_parent = Point { x: p0.x, y: y_parent as Float * h };
//         pb.line_to(pt_parent)
//     } else if draw_root && edge.parent_node_id.is_none() {
//         let pt_parent = Point { x: 1e1 * -1e0, y: edge.y as Float * h };
//         pb.line_to(pt_parent)
//     }
// }

// fn edge_path_fan(
//     rot_angle: Float, opn_angle: Float, center: Point, size: Float, edge: &Edge,
//     pb: &mut PathBuilder, draw_root: bool,
// ) {
//     let angle = edge_angle(rot_angle, opn_angle, edge);
//     let EdgePoints { p0, p1 } = edge_points_rad(angle, center, size, edge);
//     pb.move_to(p1);
//     pb.line_to(p0);
//     if let Some(y_parent) = edge.y_parent {
//         let angle_parent = rot_angle + y_parent as Float * opn_angle;
//         let p_arc = Arc {
//             center,
//             radius: center.distance(p0),
//             start_angle: Radians(angle),
//             end_angle: Radians(angle_parent),
//         };
//         pb.arc(p_arc);
//     } else if draw_root && edge.parent_node_id.is_none() {
//         let x0 = center.x - (1e1 * 1e0) * angle.cos();
//         let y0 = center.y - (1e1 * 1e0) * angle.sin();
//         let pt_parent = Point { x: x0, y: y0 };
//         pb.line_to(pt_parent)
//     }
// }

// pub(super) fn paths_from_edges(
//     w: Float, h: Float, center: Point, size: Float, draw_root: bool, rot_angle: Float,
//     opn_angle: Float, tree_style: TreeStyle, edges: &Edges,
// ) -> Vec<Path> {
//     let mut paths: Vec<Path> = Vec::with_capacity(edges.len());
//     let mut pb = PathBuilder::new();

//     match tree_style {
//         TreeStyle::Phylogram => {
//             for edge in edges {
//                 edge_path_phylogram(w, h, edge, &mut pb, draw_root)
//             }
//         }
//         TreeStyle::Fan => {
//             for edge in edges {
//                 edge_path_fan(rot_angle, opn_angle, center, size, edge, &mut pb, draw_root)
//             }
//         }
//     }

//     paths.push(pb.build());
//     paths
// }

// pub(super) fn draw_edges(
//     paths: Vec<Path>, stroke: Stroke, translation: Option<Vector>, frame: &mut Frame,
// ) {
//     frame.with_save(|f| {
//         if let Some(translation) = translation {
//             f.translate(translation);
//         }
//         for p in paths {
//             f.stroke(&p, stroke);
//         }
//     })
// }

// pub(super) fn all_nodes(
//     w: Float, h: Float, center: Point, size: Float, rot_angle: Float, opn_angle: Float,
//     tree_style: TreeStyle, edges: &Edges,
// ) -> Vec<NodePoint> {
//     let mut points: Vec<NodePoint> = Vec::new();
//     for e in edges {
//         let mut angle: Option<Float> = None;
//         let point: Point;
//         match tree_style {
//             TreeStyle::Phylogram => {
//                 point = node_point(w, h, e);
//             }
//             TreeStyle::Fan => {
//                 let a = edge_angle(rot_angle, opn_angle, e);
//                 point = node_point_rad(a, center, size, e);
//                 angle = Some(a);
//             }
//         }
//         points.push(NodePoint { point, edge: e.clone(), angle });
//     }
//     points
// }

// pub(super) fn node_labels(nodes: &[NodePoint], tips: bool, lab_txt_template: &Text) -> Vec<Label> {
//     let mut labels: Vec<Label> = Vec::with_capacity(nodes.len());
//     for NodePoint { point, edge, angle } in nodes {
//         if (tips && !edge.is_tip) || (!tips && edge.is_tip) {
//             continue;
//         }
//         if let Some(name) = &edge.name {
//             let mut text = lab_txt_template.clone();
//             text.content = name.to_string();
//             text.position = *point;
//             labels.push(Label { text, angle: *angle });
//         }
//     }
//     labels
// }

pub(super) fn branch_labels(
    nodes: &[NodePoint], w: Float, size: Float, lab_txt_template: &Text,
) -> Vec<Label> {
    let mut lab_txt_template = lab_txt_template.clone();
    lab_txt_template.align_x = TextAlignment::Center;
    lab_txt_template.align_y = Vertical::Bottom;
    let mut labels: Vec<Label> = Vec::with_capacity(nodes.len());

    for NodePoint { point, edge, angle } in nodes {
        if edge.parent_node_id.is_none() {
            continue;
        }
        let mut text = lab_txt_template.clone();
        let mut node_point = *point;

        if let Some(angle) = angle {
            let adj = edge.brlen_normalized as Float * size / 2e0;
            node_point.x -= angle.cos() * adj;
            node_point.y -= angle.sin() * adj;
        } else {
            let adj = edge.brlen_normalized as Float * w / 2e0;
            node_point.x -= adj;
        }

        text.position = node_point;
        text.content = format!("{:.3}", edge.brlen);
        labels.push(Label { text, angle: *angle });
    }
    labels
}

// pub(super) fn draw_labels(
//     labels: Vec<Label>, text_size: Float, offset: Vector, translation: Option<Vector>,
//     f: &mut Frame,
// ) {
//     let zero_point = Point { x: 0e0, y: 0e0 };
//     let mut text_w = text_width(text_size, text_size, FNT_NAME_LAB);
//     let text_size: Pixels = text_size.into();
//     f.push_transform();
//     if let Some(translation) = translation {
//         f.translate(offset + translation);
//     } else {
//         f.translate(offset);
//     }
//     for Label { mut text, angle } in labels {
//         text.size = text_size;
//         if let Some(mut angle) = angle {
//             let mut adjust_w = 0e0;
//             match text.align_x {
//                 TextAlignment::Left => adjust_w = offset.x,
//                 TextAlignment::Right => adjust_w = -offset.x,
//                 _ => {}
//             }
//             // = Rotate labels on the left side of the circle by 180 degrees ==============
//             let a = angle % (2e0 * PI);
//             if a > PI / 2e0 && a < PI * 1.5 {
//                 angle += PI;
//                 match text.align_x {
//                     TextAlignment::Left => adjust_w = -text_w.width(&text.content) - offset.x,
//                     TextAlignment::Right => adjust_w = text_w.width(&text.content) + offset.x,
//                     _ => {}
//                 }
//             } // ==========================================================================
//             f.push_transform();
//             // ToDo: offset.y does not work correctly.
//             f.translate(Vector {
//                 x: text.position.x - offset.x + angle.cos() * adjust_w,
//                 y: text.position.y - offset.y + angle.sin() * adjust_w,
//             });
//             f.rotate(angle);
//             text.position = zero_point;
//             f.fill_text(text);
//             f.pop_transform();
//         } else {
//             f.fill_text(text);
//         }
//     }
//     f.pop_transform();
// }
