mod draw;

use crate::*;
use draw::{draw_bounds, draw_edges, draw_labs_brnch, draw_labs_int, draw_labs_tip};
use iced::{
    Event, Rectangle, Renderer, Theme,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, Program},
};

#[derive(Debug, Default)]
pub struct St {
    bnds: Rectangle<Float>,
    clip_vs: RectVals<Float>,
    tree_vs: RectVals<Float>,
    clip_rect: Rectangle<Float>,
    tree_rect: Rectangle<Float>,
}

// impl Default for St {
//     fn default() -> Self {
//         Self {
//             bnds: Default::default(),
//             clip_vs: Default::default(),
//             tree_vs: Default::default(),
//             clip_rect: Default::default(),
//             tree_rect: Default::default(),
//         }
//     }
// }

impl Program<TvMsg> for TreeView {
    type State = St;

    fn mouse_interaction(&self, st: &St, bnds: Rectangle, crsr: Cursor) -> Interaction {
        Interaction::default()
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, thm: &Theme, bnds: Rectangle, crsr: Cursor,
    ) -> Vec<Geometry> {
        let tst_opt = self.get_sel_tree();
        let mut geoms: Vec<Geometry> = Vec::new();
        if self.drawing_enabled && tst_opt.is_some() {
            let tst: &TreeState = tst_opt.unwrap();
            let size = bnds.size();

            let root_len_frac = self.root_len_idx_sel as Float / 2e2;
            let rl: Float = match tst.is_rooted() {
                true => match self.tree_style_opt_sel {
                    TreeStyle::Phylogram => st.tree_vs.w * root_len_frac,
                    TreeStyle::Fan => st.tree_vs.radius_min * root_len_frac,
                },
                false => 0e0,
            };

            draw_bounds(self, st, tst, rndr, size, &mut geoms);
            draw_edges(self, st, tst, rndr, size, rl, &mut geoms);
            if let Some(vis_nds) = self.visible_nodes() {
                draw_labs_tip(self, st, tst, rndr, size, rl, vis_nds, &mut geoms);
                draw_labs_int(self, st, tst, rndr, size, rl, vis_nds, &mut geoms);
                draw_labs_brnch(self, st, tst, rndr, size, rl, vis_nds, &mut geoms);
            }
        }
        geoms
    }

    fn update(
        &self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        let tree_opt = self.get_sel_tree();
        if !self.drawing_enabled || tree_opt.is_none() {
            return None;
        }

        if bnds != st.bnds {
            st.bnds = bnds;
            st.clip_vs = RectVals::clip(bnds);
            st.tree_vs = RectVals::tree(st.clip_vs, 1e1);
            st.clip_rect = st.clip_vs.into();
            st.tree_rect = st.tree_vs.into();
        }

        let tree: &TreeState = tree_opt.unwrap();
        None
    }
}
