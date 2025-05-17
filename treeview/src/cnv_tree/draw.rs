use super::St;
use crate::{Float, STRK1, STRK2, TreeState, TreeView, draw_rect, stroke_edges};
use iced::{Renderer, Size, widget::canvas::Geometry};

pub(super) fn draw_bounds(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    g.push(tv.cache_bnds.draw(rndr, sz, |f| {
        draw_rect(st.clip_rect, STRK1, f);
        draw_rect(st.tree_rect, STRK2, f);
    }));
}

pub(super) fn draw_edges(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    let root_len_opt: Option<Float> = match tst.is_rooted() {
        true => Some(st.tree_vs.w * 0.1),
        false => None,
    };
    g.push(tst.cache_edge().draw(rndr, sz, |f| {
        let edges = &tst.edges();
        stroke_edges(edges, &st.tree_vs, root_len_opt, f);
    }));
}

pub(super) fn draw_labs_tip(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    if tv.tip_brnch_labs_allowed && tv.draw_tip_labs && tst.has_tip_labs() {
        g.push(tst.cache_lab_tip().draw(rndr, sz, |f| {
            // code here...
        }));
    }
}

pub(super) fn draw_labs_int(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    if tv.draw_int_labs && tst.has_int_labs() {
        g.push(tst.cache_lab_int().draw(rndr, sz, |f| {
            // code here...
        }));
    }
}

pub(super) fn draw_labs_brnch(
    tv: &TreeView, st: &St, tst: &TreeState, rndr: &Renderer, sz: Size, g: &mut Vec<Geometry>,
) {
    if tv.tip_brnch_labs_allowed && tv.draw_brnch_labs && tst.has_brlen() {
        g.push(tst.cache_lab_brnch().draw(rndr, sz, |f| {
            // code here...
        }));
    }
}
