use super::St;
use crate::cnv_utils::*;
use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

pub(super) fn draw_bounds(
    tv: &TreeView, st: &St, crsr: &Cursor, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>,
) {
    g.push(tv.cache_bnds.draw(rndr, bnds.size(), |f| {
        draw_rect(st.clip_rect, STRK_5_BLU_50, f);
        draw_rect(st.tre_rect, STRK_3_GRN_50, f);
        // draw_rect(st.vis_rect, STRK_DASH, f);
        // if let Some(mouse) = st.mouse {
        //     draw_point(mouse, STRK_2, 20.0, f);
        // }
        // if let Some(mouse) = crsr.position_in(bnds) {
        //     draw_point(mouse, STRK_2, 19.0, f);
        // }
    }));
}

pub(super) fn draw_edges(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_edge().draw(rndr, sz, |f| match tv.tre_style_opt_sel {
        TreSty::PhyGrm => stroke_edges_phygrm(tst.edges_srtd_y(), &st.tre_vs, st.rl, tst.edge_root(), f),
        TreSty::Fan => {
            stroke_edges_fan(tst.edges_srtd_y(), &st.tre_vs, tv.rot_angle, tv.opn_angle, st.rl, tst.edge_root(), f)
        }
    }));
}

pub(super) fn draw_labs_tip(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_lab_tip().draw(rndr, sz, |f| {
        draw_labels(&st.labs_tip, Vector { x: tv.lab_offset_tip, y: 0e0 }, Some(st.trans), st.rot, f);
    }));
}

pub(super) fn draw_labs_int(tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>) {
    g.push(tst.cache_lab_int().draw(rndr, sz, |f| {
        draw_labels(&st.labs_int, Vector { x: tv.lab_offset_int, y: 0e0 }, Some(st.trans), st.rot, f);
    }));
}

pub(super) fn draw_labs_brnch(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_lab_brnch().draw(rndr, sz, |f| {
        draw_labels(&st.labs_brnch, Vector { x: 0e0, y: tv.lab_offset_brnch }, Some(st.trans), st.rot, f);
    }));
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
        if let Some(angle) = angle {
            let mut angle = *angle;
            let mut adjust_w = 0e0;
            match text.align_x {
                TextAlignment::Left => adjust_w = offset.x,
                TextAlignment::Right => adjust_w = -offset.x,
                _ => {}
            }
            // = Rotate labels on the left side of the circle by 180 degrees ============
            let a = (angle + rot) % TAU;
            if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
                angle += PI;
                match text.align_x {
                    TextAlignment::Left => adjust_w = -width - offset.x,
                    TextAlignment::Right => adjust_w = width + offset.x,
                    _ => {}
                }
            } // ========================================================================
            f.push_transform();
            // ToDo: offset.y does not work correctly.
            f.translate(Vector {
                x: text.position.x - offset.x + angle.cos() * adjust_w,
                y: text.position.y - offset.y + angle.sin() * adjust_w,
            });
            text.position = Point::ORIGIN;
            f.rotate(angle);
            f.fill_text(text);
            f.pop_transform();
        } else {
            f.fill_text(text);
        }
    }
    f.pop_transform();
}

fn stroke_edges_phygrm(edges: &[Edge], tre_vs: &RectVals<Float>, root_len: Float, root: Option<Edge>, f: &mut Frame) {
    let mut w = tre_vs.w;
    w -= root_len;
    let paths: Vec<Path> = edges
        .par_iter()
        .flat_map(|e|
        // node_data_cart(w, tre_vs.h, e).into()
        {
            let nd = node_data_cart(w, tre_vs.h, e);
            let ph = edge_path_cart(&nd);
            let pv = edge_path_vert_cart(&nd);
            [ph, pv]
        })
        // .flatten()
        .collect();
    f.with_save(|f| {
        f.translate(tre_vs.trans);
        f.translate(Vector { x: root_len, y: 0e0 });
        for path in &paths {
            f.stroke(path, STRK_EDGE);
        }
        if let Some(root) = root
            && root_len > 0e0
        {
            let nd = node_data_cart(w, tre_vs.h, &root);
            let root_path = Path::new(|pb| {
                let pt_parent = Point { x: -root_len, y: nd.points.p0.y };
                pb.move_to(nd.points.p0);
                pb.line_to(pt_parent)
            });
            f.stroke(&root_path, STRK_DASH);
        };
    })
}

fn stroke_edges_fan(
    edges: &[Edge], tre_vs: &RectVals<Float>, rot_angle: Float, opn_angle: Float, root_len: Float, root: Option<Edge>,
    f: &mut Frame,
) {
    let paths: Vec<Path> = edges
        .par_iter()
        .flat_map(|e|
            // node_data_rad(opn_angle, tre_vs.radius_min, root_len, e).into()
        {
            let nd = node_data_rad(opn_angle, tre_vs.radius_min, root_len, e);
            let ph = edge_path_pol(&nd);
            let pv = edge_path_arc_pol(&nd);
            [ph, pv]
        })
        // .flatten()
        .collect();

    f.with_save(|f| {
        f.translate(tre_vs.cntr);
        f.rotate(rot_angle);
        for path in &paths {
            f.stroke(path, STRK_EDGE);
        }
        if let Some(root) = root
            && root_len > 0e0
        {
            let nd = node_data_rad(opn_angle, tre_vs.radius_min, root_len, &root);
            let root_path = Path::new(|pb| {
                pb.move_to(nd.points.p0);
                pb.line_to(Point::ORIGIN)
            });
            f.stroke(&root_path, STRK_DASH);
        };
    })
}
