mod draw;
mod state;

use crate::*;
use draw::{draw_bounds, draw_edges, draw_labs_brnch, draw_labs_int, draw_labs_tip};
use state::St;

impl Program<TvMsg> for TreeView {
    type State = St;

    fn update(
        &self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        match ev {
            Event::Mouse(mouse_ev) => match mouse_ev {
                MouseEvent::CursorEntered => {
                    // println!("CursorEntered");
                    st.mouse = None;
                    return None;
                }
                MouseEvent::CursorMoved { position: _ } => {
                    if let Some(mouse) = crsr.position_in(bnds) {
                        // println!("{mouse}");
                        st.mouse = Some(mouse);
                    } else {
                        st.mouse = None;
                    }
                    self.cache_bnds.clear();
                    return Some(Action::request_redraw());
                }
                MouseEvent::CursorLeft => {
                    // println!("CursorLeft");
                    st.mouse = None;
                    return None;
                }
                // MouseEvent::ButtonPressed(_btn) => {
                //     // println!("{_btn:?}");
                //     return None;
                // }
                // MouseEvent::ButtonReleased(_btn) => {
                //     // println!("{_btn:?}");
                //     return None;
                // }
                _ => {
                    return None;
                }
            },
            // Event::Keyboard(_e) => {
            //     // println!("{_e:?}");
            //     return None;
            // }
            Event::Window(_e) => {}
            _ => {
                return None;
            }
        }
        // ----------------------------------------------------------------------------------------
        let tree_opt = self.get_sel_tree();
        if !self.drawing_enabled || tree_opt.is_none() {
            return None;
        }
        let tst: &TreeState = tree_opt.unwrap();
        // ----------------------------------------------------------------------------------------
        if bnds != st.bnds {
            st.bnds = bnds;
            st.clip_vs = RectVals::clip(bnds);
            st.tree_vs = RectVals::tree(st.clip_vs, 5e1);
            st.clip_rect = st.clip_vs.into();
            st.tree_rect = st.tree_vs.into();
        }
        // ----------------------------------------------------------------------------------------
        if self.labs_allowed && (self.draw_tip_labs || self.draw_int_labs || self.draw_brnch_labs) {
            st.update_visible_nodes(
                self.is_dirty, tst, self.tre_cnv_h, self.tre_cnv_vis_y0_rel,
                self.tre_cnv_vis_y1_rel,
            );
        } else {
            st.visible_nodes = None;
        }
        // ----------------------------------------------------------------------------------------
        let root_len_frac = self.root_len_idx_sel as Float / 2e2;
        st.rl = match tst.is_rooted() {
            true => match self.tree_style_opt_sel {
                TreeStyle::Phylogram => st.tree_vs.w * root_len_frac,
                TreeStyle::Fan => st.tree_vs.radius_min * root_len_frac,
            },
            false => 0e0,
        };
        st.trans = match self.tree_style_opt_sel {
            TreeStyle::Phylogram => Vector { x: st.tree_vs.trans.x + st.rl, y: st.tree_vs.trans.y },
            TreeStyle::Fan => st.tree_vs.cntr,
        };
        // ----------------------------------------------------------------------------------------
        st.rot = match self.tree_style_opt_sel {
            TreeStyle::Phylogram => 0e0,
            TreeStyle::Fan => self.rot_angle,
        };
        // ----------------------------------------------------------------------------------------
        if let Some(vis_nds) = &st.visible_nodes {
            let tree_style = self.tree_style_opt_sel;
            let opn_angle = self.opn_angle;
            vis_nds
                .par_iter()
                .map(|e| match tree_style {
                    TreeStyle::Phylogram => {
                        node_data_phylogram(st.tree_vs.w - st.rl, st.tree_vs.h, e).into()
                    }
                    TreeStyle::Fan => {
                        node_data_rad(opn_angle, st.tree_vs.radius_min, st.rl, e).into()
                    }
                })
                .collect_into_vec(&mut st.node_data);
        }
        // ----------------------------------------------------------------------------------------
        None
    }

    fn mouse_interaction(&self, st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction {
        if st.mouse.is_some() { Interaction::Crosshair } else { Interaction::default() }
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, _thm: &Theme, bnds: Rectangle, crsr: Cursor,
    ) -> Vec<Geometry> {
        let tst_opt = self.get_sel_tree();
        let mut geoms: Vec<Geometry> = Vec::new();
        if let Some(tst_opt) = tst_opt
            && self.drawing_enabled
        {
            // ------------------------------------------------------------------------------------
            let size = bnds.size();
            let tst: &TreeState = tst_opt;
            draw_bounds(self, st, &crsr, rndr, bnds, &mut geoms);
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            // ------------------------------------------------------------------------------------
        }
        geoms
    }
}
