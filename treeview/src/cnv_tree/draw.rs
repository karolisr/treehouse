use super::St;
use crate::{STRK1, STRK2, TreeState, TreeView, draw_rect};
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
    g.push(tst.cache_edge().draw(rndr, sz, |f| {
        // code here...
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
