use super::St;
use crate::edge_utils::*;
use crate::iced::*;
use crate::path_utils::*;
use crate::*;

pub(super) fn draw_bounds(tv: &TreeView, st: &St, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>) {
    g.push(tv.cache_bnds.draw(rndr, bnds.size(), |f| {
        stroke_rect(st.cnv_rect, STRK_5_BLU_50, f);
        stroke_rect(st.tre_rect, STRK_3_GRN_50, f);
        stroke_rect(st.vis_rect, Strk { line_dash: DASH_010, ..STRK_5_MAG_50 }, f);
    }));
}

pub(super) fn draw_edges(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_edge().draw(rndr, sz, |f| match tv.tre_sty_opt_sel {
        TreSty::PhyGrm => stroke_edges_phygrm(tst.edges_srtd_y(), &st.tre_vs, st.root_len, tst.edge_root(), f),
        TreSty::Fan => stroke_edges_fan(
            tst.edges_srtd_y(),
            &st.tre_vs,
            tv.rot_angle,
            tv.opn_angle,
            st.root_len,
            tst.edge_root(),
            f,
        ),
    }));
}

pub(super) fn draw_legend(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    if tst.has_brlen() && tv.draw_legend {
        g.push(tv.cache_legend.draw(rndr, sz, |f| {
            draw_scale_bar(
                tv.tre_sty_opt_sel,
                &st.tre_vs,
                tv.lab_size_brnch,
                st.root_len,
                tst.tre_height() as Float,
                f,
            );
        }));
    }
}

pub(super) fn draw_labs_tip(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_lab_tip().draw(rndr, sz, |f| {
        draw_labels(&st.labs_tip, Vector { x: tv.lab_offset_tip, y: ZRO }, Some(st.translation), st.rotation, f);
    }));
}

pub(super) fn draw_labs_int(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_lab_int().draw(rndr, sz, |f| {
        draw_labels(&st.labs_int, Vector { x: tv.lab_offset_int, y: ZRO }, Some(st.translation), st.rotation, f);
    }));
}

pub(super) fn draw_labs_brnch(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_lab_brnch().draw(rndr, sz, |f| {
        draw_labels(&st.labs_brnch, Vector { x: ZRO, y: tv.lab_offset_brnch }, Some(st.translation), st.rotation, f);
    }));
}

pub(super) fn draw_selected_nodes(
    _tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_sel_nodes().draw(rndr, sz, |f| {
        let edges = tst.edges_srtd_y();
        let sel_node_ids = tst.sel_node_ids();
        let points: Vec<Point> = st
            .vis_nodes
            .par_iter()
            .filter_map(|nd| {
                let edge = &edges[nd.edge_idx];
                if sel_node_ids.contains(&edge.node_id) { Some(nd.points.p1) } else { None }
            })
            .collect();
        draw_nodes(&points, st.node_radius, Some(st.translation), st.rotation, f);
    }));
}

pub(super) fn draw_hovered_node(
    tv: &TreeView, st: &St, _tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tv.cache_hovered_node.draw(rndr, sz, |f| {
        if let Some(hovered_node) = &st.hovered_node {
            f.push_transform();
            f.translate(st.translation);
            f.rotate(st.rotation);
            fill_circle(hovered_node.points.p1, FILL_NODE_HOVER, st.node_radius, f);
            stroke_circle(hovered_node.points.p1, STRK_1_RED, st.node_radius, f);
            f.pop_transform();
        }
    }));
}

pub(super) fn draw_cursor_line(
    tv: &TreeView, st: &St, _tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tv.cache_cursor_line.draw(rndr, sz, |f| {
        if let Some(p) = st.cursor_tracking_point
            && tv.draw_cursor_line
        {
            f.push_transform();
            f.translate(st.translation);
            match tv.tre_sty_opt_sel {
                TreSty::PhyGrm => {
                    let p0 = Point { x: p.x, y: ZRO };
                    let p1 = Point { x: p.x, y: st.tre_vs.h };
                    f.stroke(&PathBuilder::new().move_to(p0).line_to(p1).build(), STRK_CRSR_LINE);
                }
                TreSty::Fan => {
                    let r = Point::ORIGIN.distance(p);
                    f.rotate(st.rotation);
                    stroke_circle(Point::ORIGIN, STRK_CRSR_LINE, r, f);
                }
            }
            f.pop_transform();
        }
    }));
}

fn draw_scale_bar(
    tre_sty: TreSty, tre_vs: &RectVals<Float>, lab_size: Float, root_len: Float, tree_height: Float, f: &mut Frame,
) {
    let mut lab_y_offset = 5e0;
    let mut lab_size = lab_size;
    if lab_size == ONE {
        lab_size = ZRO;
        lab_y_offset = ZRO;
    }

    let w = match tre_sty {
        TreSty::PhyGrm => tre_vs.w - root_len,
        TreSty::Fan => tre_vs.radius_min - root_len,
    };

    let a = tree_height / 3e0;
    let b = a.fract();
    let c = a - b;
    let sb_len = if c > ZRO { (c / 1e1).floor() * 1e1 } else { (a * 1e1).floor() / 1e1 };

    let sb_frac = sb_len / tree_height;
    let sb_len_on_screen = sb_frac * w;

    let x = match tre_sty {
        TreSty::PhyGrm => tre_vs.x0 + root_len,
        TreSty::Fan => tre_vs.x0,
    };

    let y = tre_vs.y1 - lab_size;
    let p0 = Point { x, y };
    let p1 = Point { x: x + sb_len_on_screen, y };
    let p_lab = Point { x: x.midpoint(p1.x), y };

    f.stroke(&PathBuilder::new().move_to(p0).line_to(p1).build(), STRK_2_BLK);
    let text = lab_text(format!("{sb_len}"), p_lab, lab_size, TXT_LAB_TMPL_SCALE_BAR);
    if lab_size >= TWO {
        let lab = Label { text, width: sb_len_on_screen, angle: None };
        draw_labels(&[lab], Vector { x: ZRO, y: lab_y_offset }, None, ZRO, f);
    }
}

fn draw_nodes(points: &[Point], radius: Float, trans: Option<Vector>, rot: Float, f: &mut Frame) {
    f.push_transform();
    if let Some(trans) = trans {
        f.translate(trans);
    }
    f.rotate(rot);
    for &pt in points {
        fill_circle(pt, FILL_NODE_HOVER, radius, f);
        stroke_circle(pt, STRK_1_RED, radius, f);
    }
    f.pop_transform();
}

fn draw_labels(labels: &[Label], offset: Vector, trans: Option<Vector>, rot: Float, f: &mut Frame) {
    f.push_transform();
    if let Some(trans) = trans {
        f.translate(trans);
    }
    f.rotate(rot);
    f.translate(offset);

    for Label { text, width, angle } in labels {
        let mut text = text.clone();
        let mut adjust_h = match text.align_y {
            Vertical::Top => text.size.0 / TWO - 7e0,
            Vertical::Center => ZRO,
            Vertical::Bottom => -text.size.0 / TWO + 7e0,
        };
        if let Some(angle) = angle {
            let mut angle = *angle;
            let mut adjust_w = match text.align_x {
                TextAlignment::Left => offset.x,
                TextAlignment::Right => -offset.x,
                _ => ZRO,
            };
            adjust_h += offset.y;
            // = Rotate labels on the left side of the circle by 180 degrees ============
            let a = (angle + rot) % TAU;
            if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
                angle += PI;
                adjust_w = match text.align_x {
                    TextAlignment::Left => -width - offset.x,
                    TextAlignment::Right => width + offset.x,
                    _ => ZRO,
                };
            }
            // ==========================================================================
            f.push_transform();
            let (sin, cos) = angle.sin_cos();
            f.translate(Vector {
                x: text.position.x - offset.x + cos * adjust_w - sin * adjust_h,
                y: text.position.y - offset.y + sin * adjust_w + cos * adjust_h,
            });
            text.position = Point::ORIGIN;
            f.rotate(angle);
            f.fill_text(text);
            f.pop_transform();
        } else {
            f.push_transform();
            f.translate(Vector { x: ZRO, y: adjust_h });
            f.fill_text(text);
            f.pop_transform();
        }
    }
    f.pop_transform();
}

fn stroke_edges_phygrm(edges: &[Edge], tre_vs: &RectVals<Float>, root_len: Float, root: Option<Edge>, f: &mut Frame) {
    let w = tre_vs.w - root_len;

    let mut pb: PathBuilder = PathBuilder::new();
    for e in edges {
        let nd = node_data_cart(w, tre_vs.h, e);
        pb = edge_path_cart_pb(&nd, pb);
        pb = edge_path_vert_cart_pb(&nd, pb);
    }

    f.with_save(|f| {
        f.translate(tre_vs.trans);
        f.translate(Vector { x: root_len, y: ZRO });
        f.stroke(&pb.build(), STRK_EDGE);
        stroke_root_phygrm(w, tre_vs.h, root_len, root, f);
    })
}

fn stroke_edges_fan(
    edges: &[Edge], tre_vs: &RectVals<Float>, rot_angle: Float, opn_angle: Float, root_len: Float, root: Option<Edge>,
    f: &mut Frame,
) {
    let mut pb: PathBuilder = PathBuilder::new();
    if opn_angle >= ONE.to_radians() {
        for e in edges {
            let nd = node_data_rad(opn_angle, tre_vs.radius_min, root_len, e);
            pb = edge_path_pol_pb(&nd, pb);
            pb = edge_path_arc_pol_pb(&nd, pb);
        }
    } else {
        let p0 = Point { x: root_len, y: ZRO };
        let p1 = Point { x: tre_vs.radius_min, y: ZRO };
        pb = pb.move_to(p0).line_to(p1)
    }

    f.with_save(|f| {
        f.translate(tre_vs.cntr);
        f.rotate(rot_angle);
        f.stroke(&pb.build(), STRK_EDGE);
        stroke_root_fan(tre_vs.radius_min, opn_angle, root_len, root, f);
    })
}

fn stroke_root_phygrm(w: Float, h: Float, root_len: Float, root_edge: Option<Edge>, f: &mut Frame) {
    if let Some(root_edge) = root_edge
        && root_len > ZRO
    {
        let nd = node_data_cart(w, h, &root_edge);
        let pt_parent = Point { x: -root_len, y: nd.points.p0.y };
        f.stroke(&PathBuilder::new().move_to(nd.points.p0).line_to(pt_parent).build(), STRK_ROOT);
    };
}

fn stroke_root_fan(radius_min: Float, opn_angle: Float, root_len: Float, root_edge: Option<Edge>, f: &mut Frame) {
    if let Some(root_edge) = root_edge
        && root_len > ZRO
    {
        let nd = node_data_rad(opn_angle, radius_min, root_len, &root_edge);
        f.stroke(&PathBuilder::new().move_to(nd.points.p0).line_to(Point::ORIGIN).build(), STRK_ROOT);
    };
}

fn lab_text(txt: String, pt: Point, size: Float, template: CnvText) -> CnvText {
    let mut text = template.clone();
    text.content = txt;
    text.position = pt;
    text.size = size.into();
    text
}

pub(super) fn node_labs(
    nodes: &[NodeData], edges: &[Edge], size: Float, tips: bool, branch: bool, text_w: &mut TextWidth,
    result: &mut Vec<Label>,
) {
    nodes
        .iter()
        .filter_map(|nd| {
            let edge = &edges[nd.edge_idx];
            if let Some(name) = &edge.name
                && !branch
                && ((tips && edge.is_tip) || (!tips && !edge.is_tip))
            {
                let mut txt_lab_tmpl: CnvText = TXT_LAB_TMPL;
                if !tips {
                    txt_lab_tmpl = TXT_LAB_TMPL_INT;
                }
                let width = text_w.width(name);
                let text = lab_text(name.to_string(), nd.points.p1, size, txt_lab_tmpl);
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
        .collect_into(result);
}
