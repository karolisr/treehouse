use super::St;
use crate::*;
use iced::{
    Point, Radians, Renderer, Size, Vector,
    widget::canvas::{Frame, Geometry, Path, path::Arc},
};
use rayon::prelude::*;

pub(super) fn draw_bounds(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tv.cache_bnds.draw(rndr, sz, |f| {
        draw_rect(st.clip_rect, STRK_3_MAG_50, f);
        draw_rect(st.tree_rect, STRK_3_CYA_50, f);
    }));
}

pub(super) fn draw_edges(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, rl: Float,
    g: &mut Vec<Geometry>,
) {
    g.push(tst.cache_edge().draw(rndr, sz, |f| match tv.tree_style_opt_sel {
        TreeStyle::Phylogram => stroke_edges_phylogram(tst.edges(), &st.tree_vs, rl, f),
        TreeStyle::Fan => {
            stroke_edges_fan(tst.edges(), &st.tree_vs, tv.rot_angle, tv.opn_angle, rl, f)
        }
    }));
}

pub(super) fn draw_labs_tip(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, rl: Float,
    vis_nds: &[Edge], g: &mut Vec<Geometry>,
) {
    if tv.tip_brnch_labs_allowed && tv.draw_tip_labs && tst.has_tip_labs() {
        g.push(tst.cache_lab_tip().draw(rndr, sz, |f| match tv.tree_style_opt_sel {
            TreeStyle::Phylogram => {
                let node_data: Vec<NodeDataPhylogram> = vis_nds
                    .iter()
                    .map(|e| node_data(st.tree_vs.w - rl, st.tree_vs.h, e.clone()))
                    .collect();

                let labs = node_labs(NodeData::Phylogram(&node_data), true);
                draw_labels(
                    labs,
                    tv.lab_size_tip,
                    Vector { x: tv.lab_offset_tip + rl, y: 0e0 },
                    Some(st.tree_vs.trans),
                    f,
                );
            }
            TreeStyle::Fan => {}
        }));
    }
}

pub(super) fn draw_labs_int(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, rl: Float,
    vis_nds: &[Edge], g: &mut Vec<Geometry>,
) {
    if tv.draw_int_labs && tst.has_int_labs() {
        g.push(tst.cache_lab_int().draw(rndr, sz, |f| {
            // code here...
        }));
    }
}

pub(super) fn draw_labs_brnch(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, rl: Float,
    vis_nds: &[Edge], g: &mut Vec<Geometry>,
) {
    if tv.tip_brnch_labs_allowed && tv.draw_brnch_labs && tst.has_brlen() {
        g.push(tst.cache_lab_brnch().draw(rndr, sz, |f| {
            // code here...
        }));
    }
}

#[inline]
fn node_labs(node_data: NodeData, tips: bool) -> Vec<Label> {
    match node_data {
        NodeData::Phylogram(nodes) => {
            let mut labels: Vec<Label> = Vec::with_capacity(nodes.len());
            for NodeDataPhylogram { edge, points, y_parent: _ } in nodes {
                if (tips && !edge.is_tip) || (!tips && edge.is_tip) {
                    continue;
                }
                if let Some(name) = &edge.name {
                    let mut text = TXT_LAB_TMPL.clone();
                    text.content = name.to_string();
                    text.position = points.p1;
                    labels.push(Label { text, angle: None });
                }
            }
            labels
        }
        NodeData::Rad(nodes) => {
            let mut labels: Vec<Label> = Vec::with_capacity(nodes.len());
            labels
        }
    }
}

#[inline]
fn draw_labels(
    labels: Vec<Label>, text_size: Float, offset: Vector, translation: Option<Vector>,
    f: &mut Frame,
) {
    let zero_point = Point { x: 0e0, y: 0e0 };
    let mut text_w = text_width(text_size, text_size, FNT_NAME_LAB);
    let text_size: Pixels = text_size.into();
    f.push_transform();
    if let Some(translation) = translation {
        f.translate(offset + translation);
    } else {
        f.translate(offset);
    }
    for Label { mut text, angle } in labels {
        text.size = text_size;
        if let Some(mut angle) = angle {
            let mut adjust_w = 0e0;
            match text.align_x {
                TextAlignment::Left => adjust_w = offset.x,
                TextAlignment::Right => adjust_w = -offset.x,
                _ => {}
            }
            // = Rotate labels on the left side of the circle by 180 degrees ==============
            let a = angle % (2e0 * PI);
            if a > PI / 2e0 && a < PI * 1.5 {
                angle += PI;
                match text.align_x {
                    TextAlignment::Left => adjust_w = -text_w.width(&text.content) - offset.x,
                    TextAlignment::Right => adjust_w = text_w.width(&text.content) + offset.x,
                    _ => {}
                }
            } // ==========================================================================
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
    edges: &[Edge], tree_vs: &RectVals<Float>, root_len: Float, f: &mut Frame,
) {
    let mut w = tree_vs.w;
    w -= root_len;

    let paths: Vec<Path> =
        edges.par_iter().map(|e| node_data(w, tree_vs.h, e.clone()).into()).collect();
    f.with_save(|f| {
        f.translate(tree_vs.trans);
        f.translate(Vector { x: root_len, y: 0e0 });
        for path in &paths {
            f.stroke(path, STRK_1_RED_50);
        }
    })
}

#[inline]
fn stroke_edges_fan(
    edges: &[Edge], tree_vs: &RectVals<Float>, rot_angle: Float, opn_angle: Float, root_len: Float,
    f: &mut Frame,
) {
    let paths: Vec<Path> = edges
        .par_iter()
        .map(|e| node_data_rad(opn_angle, tree_vs.radius_min, root_len, e.clone()).into())
        .collect();

    f.with_save(|f| {
        f.translate(tree_vs.cntr);
        f.rotate(rot_angle);
        for path in &paths {
            f.stroke(path, STRK_1_RED_50);
        }
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
            // else if node_data.edge.parent_node_id.is_none() && root_len > 0e0 {
            //     let pt_parent = Point { x: root_len * -1e0, y: node_data.points.p0.y };
            //     pb.line_to(pt_parent)
            // }
        })
    }
}

impl From<NodeDataRad> for Path {
    #[inline]
    fn from(node_data: NodeDataRad) -> Self {
        Path::new(|pb| {
            pb.move_to(node_data.points.p1);
            pb.line_to(node_data.points.p0);
            if node_data.edge.y_parent.is_some() {
                let p_arc = Arc {
                    center: Point::ORIGIN,
                    radius: Point::ORIGIN.distance(node_data.points.p0),
                    start_angle: Radians(node_data.angle),
                    end_angle: Radians(node_data.angle_parent.expect("angle_parent?")),
                };
                pb.arc(p_arc);
            }
            // else if node_data.edge.parent_node_id.is_none() && root_len > 0e0 {
            //     pb.line_to(Point::ORIGIN)
            // }
        })
    }
}
