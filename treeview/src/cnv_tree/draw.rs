use super::St;
use crate::*;

pub(super) fn draw_bounds(
    tv: &TreeView, st: &St, crsr: &Cursor, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>,
) {
    g.push(tv.cache_bnds.draw(rndr, bnds.size(), |f| {
        draw_rect(st.clip_rect, STRK_3_MAG_50, f);
        draw_rect(st.tree_rect, STRK_3_CYA_50, f);

        if let Some(mouse) = st.mouse {
            draw_point(mouse, STRK_3_RED_50, 20.0, f);
        }
        if let Some(mouse) = crsr.position_in(bnds) {
            draw_point(mouse, STRK_1_BLK, 19.0, f);
        }
    }));
}

pub(super) fn draw_edges(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_edge().draw(rndr, sz, |f| match tv.tree_style_opt_sel {
        TreeStyle::Phylogram => {
            stroke_edges_phylogram(tst.edges(), &st.tree_vs, st.rl, tst.edge_root(), f)
        }
        TreeStyle::Fan => stroke_edges_fan(
            tst.edges(),
            &st.tree_vs,
            tv.rot_angle,
            tv.opn_angle,
            st.rl,
            tst.edge_root(),
            f,
        ),
    }));
}

pub(super) fn draw_labs_tip(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    if tv.labs_allowed && tv.draw_tip_labs && tst.has_tip_labs() {
        g.push(tst.cache_lab_tip().draw(rndr, sz, |f| {
            let labs = node_labs(&st.node_data, tst.edges(), tv.lab_size_tip, true, false);
            draw_labels(labs, Vector { x: tv.lab_offset_tip, y: 0e0 }, Some(st.trans), st.rot, f);
        }));
    }
}

pub(super) fn draw_labs_int(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    if tv.labs_allowed && tv.draw_int_labs && tst.has_int_labs() {
        g.push(tst.cache_lab_int().draw(rndr, sz, |f| {
            let labs = node_labs(&st.node_data, tst.edges(), tv.lab_size_int, false, false);
            draw_labels(labs, Vector { x: tv.lab_offset_int, y: 0e0 }, Some(st.trans), st.rot, f);
        }));
    }
}

pub(super) fn draw_labs_brnch(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    if tv.labs_allowed && tv.draw_brnch_labs && tst.has_brlen() {
        g.push(tst.cache_lab_brnch().draw(rndr, sz, |f| {
            let labs = node_labs(&st.node_data, tst.edges(), tv.lab_size_brnch, false, true);
            draw_labels(labs, Vector { x: 0e0, y: tv.lab_offset_brnch }, Some(st.trans), st.rot, f);
        }));
    }
}

// #[inline]
// fn draw_node(
//     point: &Point, ps: Float, stroke: Strk, fill: impl Into<CanvasFill>, tree_vs: &RectVals<Float>,
//     frame: &mut Frame,
// ) {
//     frame.with_save(|f| {
//         f.translate(tree_vs.trans);
//         let path_fill = Path::new(|p| {
//             p.circle(*point, ps);
//         });

//         let path_stroke = Path::new(|p| {
//             p.circle(*point, ps - 1e0 / 2e0);
//         });

//         f.fill(&path_fill, fill);
//         f.stroke(&path_stroke, stroke);
//     });
// }

#[inline]
fn lab_text(txt: String, pt: Point, size: Float, template: CanvasText) -> CanvasText {
    let mut text = template.clone();
    text.content = txt;
    text.position = pt;
    text.size = size.into();
    text
}

#[inline]
fn node_labs(
    nodes: &[NodeData], edges: &[Edge], size: Float, tips: bool, branch: bool,
) -> Vec<Label> {
    let mut text_w: TextWidth = text_width(size, size, FNT_NAME_LAB);
    nodes
        .iter()
        .filter_map(|nd| {
            let edge = &edges[nd.edge_idx];
            if !branch && edge.name.is_some() && ((tips && edge.is_tip) || (!tips && !edge.is_tip))
            {
                let name = edge.name.as_ref().unwrap();
                let width = text_w.width(name);
                let text = lab_text(name.to_string(), nd.points.p1, size, TXT_LAB_TMPL);
                Some(Label { text, width, angle: nd.angle })
            } else if branch && edge.parent_node_id.is_some() {
                let name = format!("{:.3}", edge.brlen);
                let width = text_w.width(&name);
                let text = lab_text(name.to_string(), nd.points.p_mid, size, TXT_LAB_TMPL_BRNCH);
                Some(Label { text, width, angle: nd.angle })
            } else {
                None
            }
        })
        .collect()
}

#[inline]
fn draw_labels(
    labels: Vec<Label>, offset: Vector, trans: Option<Vector>, rot: Float, f: &mut Frame,
) {
    let zero_point = Point { x: 0e0, y: 0e0 };

    f.push_transform();
    if let Some(trans) = trans {
        f.translate(trans);
    }
    f.rotate(rot);
    f.translate(offset);

    for Label { mut text, width, angle } in labels {
        if let Some(mut angle) = angle {
            let mut adjust_w = 0e0;
            match text.align_x {
                TextAlignment::Left => adjust_w = offset.x,
                TextAlignment::Right => adjust_w = -offset.x,
                _ => {}
            }
            // = Rotate labels on the left side of the circle by 180 degrees ======================
            let a = (angle + rot) % TAU;
            if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
                angle += PI;
                match text.align_x {
                    TextAlignment::Left => adjust_w = -width - offset.x,
                    TextAlignment::Right => adjust_w = width + offset.x,
                    _ => {}
                }
            } // ==================================================================================
            f.push_transform();
            // ToDo: offset.y does not work correctly.
            f.translate(Vector {
                x: text.position.x - offset.x + angle.cos() * adjust_w,
                y: text.position.y - offset.y + angle.sin() * adjust_w,
            });
            f.rotate(angle);
            text.position = zero_point;
            f.fill_text(text);
            f.pop_transform();
        } else {
            f.fill_text(text);
        }
    }
    f.pop_transform();
}

#[inline]
fn stroke_edges_phylogram(
    edges: &[Edge], tree_vs: &RectVals<Float>, root_len: Float, root: Option<Edge>, f: &mut Frame,
) {
    let mut w = tree_vs.w;
    w -= root_len;

    let paths: Vec<Path> =
        edges.par_iter().map(|e| node_data_phylogram(w, tree_vs.h, e).into()).collect();
    f.with_save(|f| {
        f.translate(tree_vs.trans);
        f.translate(Vector { x: root_len, y: 0e0 });
        for path in &paths {
            f.stroke(path, STRK_1_RED_50);
        }
        if let Some(root) = root {
            if root_len > 0e0 {
                let nd = node_data_phylogram(w, tree_vs.h, &root);
                let root_path = Path::new(|pb| {
                    let pt_parent = Point { x: root_len * -1e0, y: nd.points.p0.y };
                    pb.move_to(nd.points.p0);
                    pb.line_to(pt_parent)
                });
                f.stroke(&root_path, STRK_3_BLU_50);
            };
        };
    })
}

#[inline]
fn stroke_edges_fan(
    edges: &[Edge], tree_vs: &RectVals<Float>, rot_angle: Float, opn_angle: Float, root_len: Float,
    root: Option<Edge>, f: &mut Frame,
) {
    let paths: Vec<Path> = edges
        .par_iter()
        .map(|e| node_data_rad(opn_angle, tree_vs.radius_min, root_len, e).into())
        .collect();

    f.with_save(|f| {
        f.translate(tree_vs.cntr);
        f.rotate(rot_angle);
        for path in &paths {
            f.stroke(path, STRK_1_RED_50);
        }
        if let Some(root) = root {
            if root_len > 0e0 {
                let nd = node_data_rad(opn_angle, tree_vs.radius_min, root_len, &root);
                let root_path = Path::new(|pb| {
                    pb.move_to(nd.points.p0);
                    pb.line_to(Point::ORIGIN)
                });
                f.stroke(&root_path, STRK_3_BLU_50);
            };
        };
    })
}

impl From<NodeDataPhylogram> for Path {
    #[inline]
    fn from(node_data: NodeDataPhylogram) -> Self {
        Path::new(|pb| {
            pb.move_to(node_data.points.p1);
            pb.line_to(node_data.points.p0);
            if let Some(y_parent) = node_data.y_parent {
                let pt_parent = Point { x: node_data.points.p0.x, y: y_parent };
                pb.line_to(pt_parent)
            }
        })
    }
}

impl From<NodeDataRad> for Path {
    #[inline]
    fn from(node_data: NodeDataRad) -> Self {
        Path::new(|pb| {
            pb.move_to(node_data.points.p1);
            pb.line_to(node_data.points.p0);
            if node_data.angle_parent.is_some() {
                let p_arc = Arc {
                    center: Point::ORIGIN,
                    radius: Point::ORIGIN.distance(node_data.points.p0),
                    start_angle: Radians(node_data.angle),
                    end_angle: Radians(node_data.angle_parent.expect("angle_parent?")),
                };
                pb.arc(p_arc);
            }
        })
    }
}
