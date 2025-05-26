use super::St;
use crate::cnv_utils::*;
use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

#[inline]
pub(super) fn draw_bounds(
    tv: &TreeView, st: &St, crsr: &Cursor, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>,
) {
    g.push(tv.cache_bnds.draw(rndr, bnds.size(), |f| {
        draw_rect(st.cnv_rect, STRK_5_BLU_50, f);
        draw_rect(st.tre_rect, STRK_3_GRN_50, f);
        draw_rect(st.vis_rect, STRK_5_RED_50_DASH, f);
        if let Some(mouse) = st.mouse {
            draw_point(mouse + [-1e0, 0e0].into(), STRK_3_RED_50, 20.0, f);
        }
        if let Some(mouse) = crsr.position_in(bnds) {
            draw_point(mouse + [-1e0, 0e0].into(), STRK_1_BLU_50, 19.0, f);
        }
    }));
}

#[inline]
pub(super) fn draw_edges(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_edge().draw(rndr, sz, |f| match tv.tre_style_opt_sel {
        TreSty::PhyGrm => stroke_edges_phygrm(tst.edges_srtd_y(), &st.tre_vs, st.rl, tst.edge_root(), f),
        TreSty::Fan => {
            stroke_edges_fan(tst.edges_srtd_y(), &st.tre_vs, tv.rot_angle, tv.opn_angle, st.rl, tst.edge_root(), f)
        }
    }));
}

#[inline]
pub(super) fn draw_labs_tip(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_lab_tip().draw(rndr, sz, |f| {
        draw_labels(&st.labs_tip, Vector { x: tv.lab_offset_tip, y: 0e0 }, Some(st.trans), st.rot, f);
    }));
}

#[inline]
pub(super) fn draw_labs_int(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_lab_int().draw(rndr, sz, |f| {
        draw_labels(&st.labs_int, Vector { x: tv.lab_offset_int, y: 0e0 }, Some(st.trans), st.rot, f);
    }));
}

#[inline]
pub(super) fn draw_labs_brnch(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_lab_brnch().draw(rndr, sz, |f| {
        draw_labels(&st.labs_brnch, Vector { x: 0e0, y: tv.lab_offset_brnch }, Some(st.trans), st.rot, f);
    }));
}

#[inline]
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
            Vertical::Top => text.size.0 / 2e0 - 7e0,
            Vertical::Center => 0e0,
            Vertical::Bottom => -text.size.0 / 2e0 + 7e0,
        };
        if let Some(angle) = angle {
            let mut angle = *angle;
            let mut adjust_w = match text.align_x {
                TextAlignment::Left => offset.x,
                TextAlignment::Right => -offset.x,
                _ => 0e0,
            };
            adjust_h += offset.y;
            // = Rotate labels on the left side of the circle by 180 degrees ============
            let a = (angle + rot) % TAU;
            if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
                angle += PI;
                adjust_w = match text.align_x {
                    TextAlignment::Left => -width - offset.x,
                    TextAlignment::Right => width + offset.x,
                    _ => 0e0,
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
            f.translate(Vector { x: 0e0, y: adjust_h });
            f.fill_text(text);
            f.pop_transform();
        }
    }
    f.pop_transform();
}

#[inline]
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
        f.translate(Vector { x: root_len, y: 0e0 });
        f.stroke(&pb.build(), STRK_EDGE);
        stroke_root_phygrm(w, tre_vs.h, root_len, root, f);
    })
}

#[inline]
fn stroke_edges_fan(
    edges: &[Edge], tre_vs: &RectVals<Float>, rot_angle: Float, opn_angle: Float, root_len: Float, root: Option<Edge>,
    f: &mut Frame,
) {
    let mut pb: PathBuilder = PathBuilder::new();
    if opn_angle >= 1e0_f32.to_radians() {
        for e in edges {
            let nd = node_data_rad(opn_angle, tre_vs.radius_min, root_len, e);
            pb = edge_path_pol_pb(&nd, pb);
            pb = edge_path_arc_pol_pb(&nd, pb);
        }
    } else {
        let p0 = Point { x: root_len, y: 0e0 };
        let p1 = Point { x: tre_vs.radius_min, y: 0e0 };
        pb = pb.move_to(p0).line_to(p1)
    }

    f.with_save(|f| {
        f.translate(tre_vs.cntr);
        f.rotate(rot_angle);
        f.stroke(&pb.build(), STRK_EDGE);
        stroke_root_fan(tre_vs.radius_min, opn_angle, root_len, root, f);
    })
}

#[inline]
fn stroke_root_phygrm(w: Float, h: Float, root_len: Float, root_edge: Option<Edge>, f: &mut Frame) {
    if let Some(root_edge) = root_edge
        && root_len > 0e0
    {
        let nd = node_data_cart(w, h, &root_edge);
        let pt_parent = Point { x: -root_len, y: nd.points.p0.y };
        f.stroke(&PathBuilder::new().move_to(nd.points.p0).line_to(pt_parent).build(), STRK_DASH);
    };
}

#[inline]
fn stroke_root_fan(radius_min: Float, opn_angle: Float, root_len: Float, root_edge: Option<Edge>, f: &mut Frame) {
    if let Some(root_edge) = root_edge
        && root_len > 0e0
    {
        let nd = node_data_rad(opn_angle, radius_min, root_len, &root_edge);
        f.stroke(&PathBuilder::new().move_to(nd.points.p0).line_to(Point::ORIGIN).build(), STRK_DASH);
    };
}
