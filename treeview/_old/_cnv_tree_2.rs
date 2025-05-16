use crate::{
    Clr, Float, RectVals, STROKE_TMPL, TXT_LAB_TMPL, TreeState, TreeView, TvMsg,
    cnv_utils::{
        branch_labels, draw_edges, draw_labels, draw_rectangle, node_labels, paths_from_edges,
    },
};
use iced::{
    Event, Rectangle, Renderer, Theme, Vector,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, Program, Stroke},
};

#[derive(Debug)]
pub struct St {
    strk_edge: Stroke<'static>,
    strk1: Stroke<'static>,
    strk2: Stroke<'static>,
    strk3: Stroke<'static>,
    bnds: Rectangle,
    clip_vs: RectVals<Float>,
    tree_vs: RectVals<Float>,
}

impl Default for St {
    fn default() -> Self {
        Self {
            strk_edge: STROKE_TMPL,
            strk1: Stroke { width: 3e0, style: Clr::RED.scale_alpha(0.5).into(), ..STROKE_TMPL },
            strk2: Stroke { width: 3e0, style: Clr::GRN.scale_alpha(0.5).into(), ..STROKE_TMPL },
            strk3: Stroke { width: 3e0, style: Clr::BLU.scale_alpha(0.5).into(), ..STROKE_TMPL },
            bnds: Default::default(),
            clip_vs: Default::default(),
            tree_vs: Default::default(),
        }
    }
}

impl Program<TvMsg> for TreeView {
    type State = St;

    fn mouse_interaction(&self, st: &St, bnds: Rectangle, crsr: Cursor) -> Interaction {
        Interaction::default()
    }

    fn update(
        &self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        if bnds != st.bnds {
            st.bnds = bnds;
            st.clip_vs = RectVals::clip(bnds);
            st.tree_vs = RectVals::tree(st.clip_vs, 1e1);
        }
        None
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, thm: &Theme, bnds: Rectangle, crsr: Cursor,
    ) -> Vec<Geometry> {
        let tree_opt = self.get_sel_tree();
        if !self.drawing_enabled || tree_opt.is_none() {
            return vec![];
        }

        let tree: &TreeState = tree_opt.unwrap();
        let mut geoms: Vec<Geometry> = Vec::new();

        let g_bounds = self.cache_bounds.draw(rndr, bnds.size(), |f| {
            draw_rectangle(st.clip_vs.into(), st.strk1, f);
            draw_rectangle(st.tree_vs.into(), st.strk2, f);
        });
        geoms.push(g_bounds);

        let g_edge = tree.cache_edge().draw(rndr, bnds.size(), |f| {
            let paths = paths_from_edges(
                st.tree_vs.w,
                st.tree_vs.h,
                st.tree_vs.cntr_untrans,
                st.tree_vs.radius_min,
                tree.is_rooted(),
                self.rot_angle,
                self.opn_angle,
                self.sel_tree_style_opt,
                tree.edges(),
            );
            draw_edges(paths, st.strk_edge, Some(st.tree_vs.trans), f);
        });
        geoms.push(g_edge);

        if self.tip_brnch_labs_allowed && tree.has_tip_labs() && self.draw_tip_labs {
            let g_lab_tip = tree.cache_lab_tip().draw(rndr, bnds.size(), |f| {
                let labels = node_labels(&[], true, &TXT_LAB_TMPL);
                draw_labels(
                    labels,
                    self.tip_lab_size,
                    Vector { x: self.tip_lab_offset, y: 0e0 },
                    Some(st.tree_vs.trans),
                    f,
                );
            });
            geoms.push(g_lab_tip);
        }

        if tree.has_int_labs() && self.draw_int_labs {
            let g_lab_int = tree.cache_lab_int().draw(rndr, bnds.size(), |f| {
                draw_labels(
                    node_labels(&[], false, &TXT_LAB_TMPL),
                    self.int_lab_size,
                    Vector { x: self.int_lab_offset, y: 0e0 },
                    Some(st.tree_vs.trans),
                    f,
                );
            });
            geoms.push(g_lab_int);
        }

        if tree.has_brlen() && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            let g_lab_brnch = tree.cache_lab_brnch().draw(rndr, bnds.size(), |f| {
                draw_labels(
                    branch_labels(&[], st.tree_vs.w, st.tree_vs.radius_min, &TXT_LAB_TMPL),
                    self.brnch_lab_size,
                    Vector { x: 0e0, y: -self.brnch_lab_offset },
                    Some(st.tree_vs.trans),
                    f,
                );
            });
            geoms.push(g_lab_brnch);
        }

        geoms
    }
}
