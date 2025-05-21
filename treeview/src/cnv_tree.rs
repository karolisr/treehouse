mod draw;
mod state;

use crate::*;
use draw::{draw_edges, draw_labs_brnch, draw_labs_int, draw_labs_tip};
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
        let tree_opt = self.sel_tree();
        if !self.drawing_enabled || tree_opt.is_none() {
            return None;
        }
        let tst: &TreeState = tree_opt?;
        // ----------------------------------------------------------------------------------------
        if bnds != st.bnds {
            st.bnds = bnds;
            st.clip_vs = RectVals::clip(bnds);
            st.tree_vs = RectVals::tree(st.clip_vs, self.tre_padding);
            st.clip_rect = st.clip_vs.into();
            st.tree_rect = st.tree_vs.into();
        }
        // ----------------------------------------------------------------------------------------
        let root_len_frac = self.root_len_idx_sel as Float / 2e2;
        st.rl = 0e0;
        match self.tree_style_opt_sel {
            TreeStyle::Phylogram => {
                if tst.is_rooted() {
                    st.rl = st.tree_vs.w * root_len_frac;
                }
                st.rot = 0e0;
                st.trans = Vector { x: st.tree_vs.trans.x + st.rl, y: st.tree_vs.trans.y };
            }
            TreeStyle::Fan => {
                if tst.is_rooted() {
                    st.rl = st.tree_vs.radius_min * root_len_frac;
                }
                st.rot = self.rot_angle;
                st.trans = st.tree_vs.cntr;
            }
        }
        // ----------------------------------------------------------------------------------------
        match self.tree_style_opt_sel {
            TreeStyle::Phylogram => {
                st.labs_allowed = self.labs_allowed;
                if (self.draw_tip_labs && tst.has_tip_labs())
                    || (self.draw_int_labs && tst.has_int_labs())
                    || (self.draw_brnch_labs && tst.has_brlen())
                {
                    st.update_vis_nodes_phylogram(
                        self.tre_cnv_vis_y0_rel, self.tre_cnv_vis_y1_rel, self.tre_cnv_h,
                        self.tre_padding, self.is_dirty, tst,
                    );
                }
            }
            TreeStyle::Fan => {
                st.update_vis_rect(
                    self.tre_cnv_vis_x0, self.tre_cnv_vis_x1, self.tre_cnv_vis_y0,
                    self.tre_cnv_vis_y1,
                );
                st.update_vis_nodes_fan(
                    self.tre_cnv_vis_x0_rel, self.tre_cnv_vis_x1_rel, self.tre_cnv_vis_y0_rel,
                    self.tre_cnv_vis_y1_rel, self.opn_angle, self.tip_labs_vis_max,
                    self.node_labs_vis_fan_max, self.is_dirty, tst,
                );
                if !st.labs_allowed {
                    if self.labs_allowed {
                        return Some(Action::publish(TvMsg::SetLabsAllowed(false)));
                    }
                } else if !self.labs_allowed {
                    return Some(Action::publish(TvMsg::SetLabsAllowed(true)));
                }
            }
        }
        // ----------------------------------------------------------------------------------------
        if let Some(visible_nodes) = &st.visible_nodes {
            let tree_style = self.tree_style_opt_sel;
            let opn_angle = self.opn_angle;
            visible_nodes
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
        } else {
            st.node_data.clear();
        }
        // ----------------------------------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();

        if st.labs_allowed && self.labs_allowed && !st.node_data.is_empty() {
            // ------------------------------------------------------------------------------------
            if self.draw_tip_labs && tst.has_tip_labs() {
                if st.text_w_tip.as_mut()?.font_size() != self.lab_size_tip {
                    st.text_w_tip.as_mut()?.set_font_size(self.lab_size_tip);
                }
                node_labs(
                    &st.node_data,
                    tst.edges_srtd_y(),
                    self.lab_size_tip,
                    true,
                    false,
                    st.text_w_tip.as_mut()?,
                    &mut st.labs_tip,
                );
            }
            // ------------------------------------------------------------------------------------
            if self.draw_int_labs && tst.has_int_labs() {
                if st.text_w_int.as_mut()?.font_size() != self.lab_size_int {
                    st.text_w_int.as_mut()?.set_font_size(self.lab_size_int);
                }
                node_labs(
                    &st.node_data,
                    tst.edges_srtd_y(),
                    self.lab_size_int,
                    false,
                    false,
                    st.text_w_int.as_mut()?,
                    &mut st.labs_int,
                );
            }
            // ------------------------------------------------------------------------------------
            if self.draw_brnch_labs && tst.has_brlen() {
                if st.text_w_brnch.as_mut()?.font_size() != self.lab_size_brnch {
                    st.text_w_brnch.as_mut()?.set_font_size(self.lab_size_brnch);
                }
                node_labs(
                    &st.node_data,
                    tst.edges_srtd_y(),
                    self.lab_size_brnch,
                    false,
                    true,
                    st.text_w_brnch.as_mut()?,
                    &mut st.labs_brnch,
                );
            }
        }
        // ----------------------------------------------------------------------------------------
        None
    }

    fn mouse_interaction(&self, st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction {
        if st.mouse.is_some() { Interaction::Crosshair } else { Interaction::default() }
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, _thm: &Theme, bnds: Rectangle, _crsr: Cursor,
    ) -> Vec<Geometry> {
        let tst_opt = self.sel_tree();
        let mut geoms: Vec<Geometry> = Vec::new();
        if let Some(tst_opt) = tst_opt
            && self.drawing_enabled
        {
            // ------------------------------------------------------------------------------------
            let size = bnds.size();
            let tst: &TreeState = tst_opt;
            draw::draw_bounds(self, st, &_crsr, rndr, bnds, &mut geoms);
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            // ------------------------------------------------------------------------------------
        }
        geoms
    }
}

#[inline]
fn lab_text(txt: String, pt: Point, size: Float, template: CanvasText) -> CanvasText {
    let mut text = template.clone();
    text.content = txt;
    text.position = pt;
    text.size = size.into();
    text
}

#[inline]
fn node_labs(
    nodes: &[NodeData], edges: &[Edge], size: Float, tips: bool, branch: bool,
    text_w: &mut TextWidth, result: &mut Vec<Label>,
) {
    nodes
        .iter()
        .filter_map(|nd| {
            let edge = &edges[nd.edge_idx];
            if let Some(name) = &edge.name
                && !branch
                && ((tips && edge.is_tip) || (!tips && !edge.is_tip))
            {
                let width = text_w.width(name);
                let text = lab_text(name.to_string(), nd.points.p1, size, TXT_LAB_TMPL);
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
