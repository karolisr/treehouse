use super::St;
use crate::edge_utils::*;
use crate::iced::*;
use crate::path_utils::*;
use crate::*;

pub(super) fn draw_bounds(
    tc: &TreeCnv, st: &St, rndr: &Renderer, bnds: Rectangle, g: &mut Vec<Geometry>,
) {
    g.push(tc.cache_bnds.draw(rndr, bnds.size(), |f| {
        stroke_rect(st.cnv_rect, STRK_5_BLU_50, f);
        stroke_rect(st.tre_rect, STRK_3_GRN_50, f);
        stroke_rect(st.vis_rect, Strk { line_dash: DASH_010, ..STRK_5_MAG_50 }, f);
    }));
}

pub(super) fn draw_edges(
    tc: &TreeCnv, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_edge().draw(rndr, sz, |f| match tc.tre_sty {
        TreSty::PhyGrm => {
            stroke_edges_phygrm(tst.edges_srtd_y(), &st.tre_vs, st.root_len, tst.edge_root(), f)
        }
        TreSty::Fan => stroke_edges_fan(
            tst.edges_srtd_y(),
            &st.tre_vs,
            tc.rot_angle,
            tc.opn_angle,
            st.root_len,
            tst.edge_root(),
            f,
        ),
    }));
}

pub(super) fn draw_legend(
    tc: &TreeCnv, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    if tst.has_brlen() && tc.draw_legend {
        g.push(tc.cache_legend.draw(rndr, sz, |f| {
            draw_scale_bar(
                tc.tre_sty,
                &st.tre_vs,
                &st.cnv_vs,
                tc.lab_size_brnch,
                -tc.lab_offset_brnch,
                st.root_len,
                tst.tre_height() as Float,
                f,
            );
        }));
    }
}

pub(super) fn draw_cursor_line(
    tc: &TreeCnv, st: &St, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tc.cache_cursor_line.draw(rndr, sz, |f| {
        if let Some(p) = st.cursor_tracking_point
            && tc.draw_cursor_line
        {
            f.push_transform();
            f.translate(st.translation);
            match tc.tre_sty {
                TreSty::PhyGrm => {
                    let p0 = Point { x: p.x, y: ZERO };
                    let p1 = Point { x: p.x, y: st.tre_vs.h };
                    f.stroke(&PathBuilder::new().move_to(p0).line_to(p1).build(), STRK_CRSR_LINE);
                }
                TreSty::Fan => {
                    let r = ORIGIN.distance(p);
                    f.rotate(st.rotation);
                    stroke_circle(ORIGIN, STRK_CRSR_LINE, r, f);
                }
            }
            f.pop_transform();
        }
    }));
}

pub(super) fn draw_labs_tip(
    tc: &TreeCnv, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_lab_tip().draw(rndr, sz, |f| {
        draw_labels(
            &st.labs_tip,
            Vector { x: tc.lab_offset_tip, y: ZERO },
            Some(st.translation),
            st.rotation,
            f,
        );
    }));
}

pub(super) fn draw_labs_int(
    tc: &TreeCnv, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_lab_int().draw(rndr, sz, |f| {
        draw_labels(
            &st.labs_int,
            Vector { x: tc.lab_offset_int, y: ZERO },
            Some(st.translation),
            st.rotation,
            f,
        );
    }));
}

pub(super) fn draw_labs_brnch(
    tc: &TreeCnv, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_lab_brnch().draw(rndr, sz, |f| {
        draw_labels(
            &st.labs_brnch,
            Vector { x: ZERO, y: tc.lab_offset_brnch },
            Some(st.translation),
            st.rotation,
            f,
        );
    }));
}

pub(super) fn draw_hovered_node(
    tc: &TreeCnv, st: &St, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tc.cache_hovered_node.draw(rndr, sz, |f| {
        if let Some(hovered_node) = &st.hovered_node {
            draw_nodes(
                &[hovered_node.points.p1],
                st.node_radius + SF * 4e0,
                STRK_NODE_HOVER,
                FILL_NODE_HOVER,
                Some(st.translation),
                st.rotation,
                f,
            );
        }
    }));
}

pub(super) fn draw_selected_nodes(
    st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_sel_nodes().draw(rndr, sz, |f| {
        let points: Vec<Point> = st.selected_nodes.par_iter().map(|nd| nd.points.p1).collect();
        draw_nodes(
            &points,
            st.node_radius + SF * 3e0,
            STRK_NODE_SELECTED,
            FILL_NODE_SELECTED,
            Some(st.translation),
            st.rotation,
            f,
        );
    }));
}

pub(super) fn draw_filtered_nodes(
    tc: &TreeCnv, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_filtered_nodes().draw(rndr, sz, |f| {
        let points: Vec<Point> = st.filtered_nodes.par_iter().map(|nd| nd.points.p1).collect();
        draw_nodes(
            &points,
            st.node_radius + SF * 2e0,
            STRK_NODE_FILTERED,
            FILL_NODE_FILTERED,
            Some(st.translation),
            st.rotation,
            f,
        );

        if let Some(edge) = tst.current_found_edge() {
            let pt = match tc.tre_sty {
                TreSty::PhyGrm => node_point_cart(st.tre_vs.w - st.root_len, st.tre_vs.h, &edge),
                TreSty::Fan => {
                    let angle = edge_angle(tc.opn_angle, &edge);
                    node_point_pol(angle, st.tre_vs.radius_min, st.root_len, &edge)
                }
            };
            draw_nodes(
                &[pt],
                st.node_radius + SF * 1e0,
                STRK_NODE_CURRENT,
                FILL_NODE_CURRENT,
                Some(st.translation),
                st.rotation,
                f,
            );
        }
    }));
}

fn draw_scale_bar(
    tre_sty: TreSty, tre_vs: &RectVals<Float>, cnv_vs: &RectVals<Float>, lab_size: Float,
    lab_y_offset: Float, root_len: Float, tre_height: Float, f: &mut Frame,
) {
    let stroke = STRK_2_BLK;
    let w = match tre_sty {
        TreSty::PhyGrm => tre_vs.w - root_len,
        TreSty::Fan => tre_vs.radius_min - root_len,
    };

    let a = tre_height / 3.25;
    let b = a.fract();
    let c = a - b;
    let sb_len = if c > ZERO { (c / TEN).floor() * TEN } else { (a * TEN).floor() / TEN };
    let sb_frac = sb_len / tre_height;
    let sb_len_on_screen = sb_frac * w;

    let x = match tre_sty {
        TreSty::PhyGrm => tre_vs.x0 + root_len + TEN + TEN,
        TreSty::Fan => tre_vs.x0 + TEN,
    };

    let y = cnv_vs.y1 - stroke.width / TWO - lab_size - lab_y_offset - TEN - TEN;
    let p0 = Point { x, y };
    let p1 = Point { x: x + sb_len_on_screen, y };
    let p_lab = Point { x: x.midpoint(p1.x), y };

    f.stroke(&PathBuilder::new().move_to(p0).line_to(p1).build(), stroke);
    let text = lab_text(format!("{sb_len}"), p_lab, lab_size, TEMPLATE_TXT_LAB_SCALEBAR);
    let lab = Label { text, width: sb_len_on_screen, angle: None };
    draw_labels(&[lab], Vector { x: ZERO, y: lab_y_offset }, None, ZERO, f);
}

fn draw_nodes(
    points: &[Point], radius: Float, stroke: Strk, fill: CnvFill, trans: Option<Vector>,
    rot: Float, f: &mut Frame,
) {
    f.push_transform();
    if let Some(trans) = trans {
        f.translate(trans);
    }
    f.rotate(rot);
    for &pt in points {
        fill_circle(pt, fill, radius, f);
        stroke_circle(pt, stroke, radius, f);
    }
    f.pop_transform();
}

fn stroke_edges_phygrm(
    edges: &[Edge], tre_vs: &RectVals<Float>, root_len: Float, root: Option<Edge>, f: &mut Frame,
) {
    let w = tre_vs.w - root_len;

    let mut pb: PathBuilder = PathBuilder::new();
    for e in edges {
        let nd = node_data_cart(w, tre_vs.h, e);
        pb = edge_path_cart(&nd, pb);
        pb = edge_path_vert_cart(&nd, pb);
    }

    f.with_save(|f| {
        f.translate(tre_vs.trans);
        f.translate(Vector { x: root_len, y: ZERO });
        f.stroke(&pb.build(), STRK_EDGE);
        stroke_root_phygrm(w, tre_vs.h, root_len, root, f);
    })
}

fn stroke_edges_fan(
    edges: &[Edge], tre_vs: &RectVals<Float>, rot_angle: Float, opn_angle: Float, root_len: Float,
    root: Option<Edge>, f: &mut Frame,
) {
    let mut pb: PathBuilder = PathBuilder::new();
    if opn_angle >= ONE.to_radians() {
        for e in edges {
            let nd = node_data_rad(opn_angle, ZERO, tre_vs.radius_min, root_len, e);
            pb = edge_path_pol(&nd, pb);
            pb = edge_path_arc_pol(&nd, pb);
        }
    } else {
        let p0 = Point { x: root_len, y: ZERO };
        let p1 = Point { x: tre_vs.radius_min, y: ZERO };
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
        && root_len > ZERO
    {
        let nd = node_data_cart(w, h, &root_edge);
        let pt_parent = Point { x: -root_len, y: nd.points.p0.y };
        f.stroke(&PathBuilder::new().move_to(pt_parent).line_to(nd.points.p0).build(), STRK_ROOT);
    };
}

fn stroke_root_fan(
    radius_min: Float, opn_angle: Float, root_len: Float, root_edge: Option<Edge>, f: &mut Frame,
) {
    if let Some(root_edge) = root_edge
        && root_len > ZERO
    {
        let nd = node_data_rad(opn_angle, ZERO, radius_min, root_len, &root_edge);
        f.stroke(&PathBuilder::new().move_to(ORIGIN).line_to(nd.points.p0).build(), STRK_ROOT);
    };
}

pub(super) fn node_labs(
    nodes: &[NodeData], edges: &[Edge], size: Float, tips: bool, branch: bool,
    text_w: &mut TextWidth, results: &mut Vec<Label>,
) {
    nodes
        .iter()
        .filter_map(|nd| {
            let edge = &edges[nd.edge_idx];
            if let Some(name) = &edge.name
                && !branch
                && ((tips && edge.is_tip) || (!tips && !edge.is_tip))
            {
                let mut txt_lab_tmpl: CnvText = TEMPLATE_TXT_LAB_TIP;
                if !tips {
                    txt_lab_tmpl = TEMPLATE_TXT_LAB_INTERNAL;
                }
                let width = text_w.width(name);
                let text = lab_text(name.to_string(), nd.points.p1, size, txt_lab_tmpl);
                Some(Label { text, width, angle: nd.angle })
            } else if branch && edge.parent_node_id.is_some() {
                let name = format!("{:.3}", edge.brlen);
                let width = text_w.width(&name);
                let text =
                    lab_text(name.to_string(), nd.points.p_mid, size, TEMPLATE_TXT_LAB_BRANCH);
                Some(Label { text, width, angle: nd.angle })
            } else {
                None
            }
        })
        .collect_into(results);
}

pub(super) fn draw_palette(
    tv: &TreeCnv, st: &St, thm: &Theme, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    let palette = thm.palette();
    let palette_ex = thm.extended_palette();
    let color_text = palette.text;
    let color_bg_weakest = palette_ex.background.weakest.color;
    let color_bg_weak = palette_ex.background.weak.color;
    let color_bg_base = palette_ex.background.base.color;
    let color_bg_strong = palette_ex.background.strong.color;
    let color_bg_strongest = palette_ex.background.strongest.color;
    let color_primary_weak = palette_ex.primary.weak.color;
    let color_primary_base = palette_ex.primary.base.color;
    let color_primary_strong = palette_ex.primary.strong.color;
    let color_secondary_weak = palette_ex.secondary.weak.color;
    let color_secondary_base = palette_ex.secondary.base.color;
    let color_secondary_strong = palette_ex.secondary.strong.color;
    let color_success_base = palette_ex.success.base.color;
    let color_warning_base = palette_ex.warning.base.color;
    let color_danger_base = palette_ex.danger.base.color;

    g.push(tv.cache_palette.draw(rndr, sz, |f| {
        let colors_bg =
            [color_bg_base, color_bg_weakest, color_bg_weak, color_bg_strong, color_bg_strongest];
        let colors_primary =
            [color_primary_base, color_primary_weak, color_primary_strong, color_text];
        let colors_secondary = [color_secondary_base, color_secondary_weak, color_secondary_strong];
        let colors_other = [color_success_base, color_warning_base, color_danger_base];
        let color_rect_size = TXT_SIZE;
        let palette_rect_w = TWO * PADDING + color_rect_size * 5e0;
        let palette_rect_h = TWO * PADDING + color_rect_size * 4e0;
        let palette_rect_x = st.cnv_vs.x0 + PADDING * 5e0;
        let palette_rect_y = st.cnv_vs.y0 + st.cnv_vs.h - palette_rect_h - PADDING * 5e0;

        f.fill_rectangle(
            Point { x: palette_rect_x, y: palette_rect_y },
            iced::Size { width: palette_rect_w, height: palette_rect_h },
            color_bg_base,
        );

        f.stroke_rectangle(
            Point { x: palette_rect_x + SF / TWO, y: palette_rect_y + SF / TWO },
            iced::Size {
                width: TWO * PADDING + color_rect_size * 5e0 - SF,
                height: TWO * PADDING + color_rect_size * 4e0 - SF,
            },
            STRK_1_BLK,
        );

        for (i, c) in colors_bg.iter().enumerate() {
            f.fill_rectangle(
                Point {
                    x: palette_rect_x + PADDING + color_rect_size * i as Float,
                    y: palette_rect_y + PADDING,
                },
                iced::Size { width: color_rect_size, height: color_rect_size },
                *c,
            );
        }

        for (i, c) in colors_primary.iter().enumerate() {
            f.fill_rectangle(
                Point {
                    x: palette_rect_x + PADDING + color_rect_size * i as Float,
                    y: palette_rect_y + PADDING + color_rect_size * 1e0,
                },
                iced::Size { width: color_rect_size, height: color_rect_size },
                *c,
            );
        }

        for (i, c) in colors_secondary.iter().enumerate() {
            f.fill_rectangle(
                Point {
                    x: palette_rect_x + PADDING + color_rect_size * i as Float,
                    y: palette_rect_y + PADDING + color_rect_size * 2e0,
                },
                iced::Size { width: color_rect_size, height: color_rect_size },
                *c,
            );
        }

        for (i, c) in colors_other.iter().enumerate() {
            f.fill_rectangle(
                Point {
                    x: palette_rect_x + PADDING + color_rect_size * i as Float,
                    y: palette_rect_y + PADDING + color_rect_size * 3e0,
                },
                iced::Size { width: color_rect_size, height: color_rect_size },
                *c,
            );
        }
    }));
}
