mod draw;
mod state;

use crate::edge_utils::*;
use crate::iced::*;
use crate::*;
use draw::*;
use state::St;

impl Program<TvMsg> for TreeView {
    type State = St;

    fn update(&self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor) -> Option<Action<TvMsg>> {
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
        // ---------------------------------------------------------------
        let tre_opt = self.sel_tre();
        if !self.drawing_enabled || tre_opt.is_none() {
            return None;
        }
        let tst: &TreeState = tre_opt?;
        // ---------------------------------------------------------------
        if bnds != st.bnds {
            st.bnds = bnds;
            st.clip_vs = RectVals::clip(bnds);
            st.tre_vs = RectVals::tre(st.clip_vs, self.tre_padd());
            st.clip_rect = st.clip_vs.into();
            st.tre_rect = st.tre_vs.into();
        }
        // ---------------------------------------------------------------

        st.rl = 0e0;
        let root_len_frac = self.root_len_idx_sel as Float / 2e2;

        match self.tre_style_opt_sel {
            TreSty::PhyGrm => {
                if tst.is_rooted() {
                    st.rl = st.tre_vs.w * root_len_frac;
                }
                st.rot = 0e0;
                st.trans = Vector { x: st.tre_vs.trans.x + st.rl, y: st.tre_vs.trans.y };
            }

            TreSty::Fan => {
                if tst.is_rooted() {
                    st.rl = st.tre_vs.radius_min * root_len_frac;
                }
                st.rot = self.rot_angle;
                st.trans = st.tre_vs.cntr;
            }
        }

        // ---------------------------------------------------------------

        st.update_vis_rect(self.tre_cnv_vis());

        match self.tre_style_opt_sel {
            TreSty::PhyGrm => {
                // st.labs_allowed = self.labs_allowed;
                if (self.draw_tip_labs && tst.has_tip_labs())
                    || (self.draw_int_labs && tst.has_int_labs())
                    || (self.draw_brnch_labs && tst.has_brlen())
                {
                    st.update_vis_nodes_phygrm(self.tre_cnv_vis_rel(), self.tre_cnv_h(), self.tre_padd(), tst);
                }
            }
            TreSty::Fan => {
                st.update_vis_nodes_fan(
                    self.tre_cnv_vis_rel(),
                    self.opn_angle,
                    self.tip_labs_vis_max,
                    self.node_labs_vis_fan_max,
                    tst,
                );
                // if !st.labs_allowed {
                //     if self.labs_allowed {
                //         return Some(Action::publish(TvMsg::SetLabsAllowed(false)));
                //     }
                // } else if !self.labs_allowed {
                //     return Some(Action::publish(TvMsg::SetLabsAllowed(true)));
                // }
            }
        }
        // ---------------------------------------------------------------
        if let Some(visible_nodes) = &st.visible_nodes {
            let tre_sty = self.tre_style_opt_sel;
            let opn_angle = self.opn_angle;
            visible_nodes
                .par_iter()
                .map(|e| match tre_sty {
                    TreSty::PhyGrm => node_data_cart(st.tre_vs.w - st.rl, st.tre_vs.h, e).into(),
                    TreSty::Fan => node_data_rad(opn_angle, st.tre_vs.radius_min, st.rl, e).into(),
                })
                .collect_into_vec(&mut st.node_data);
        } else {
            st.node_data.clear();
        }
        // ---------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // ---------------------------------------------------------------
        if
        // st.labs_allowed &&
        self.labs_allowed && !st.node_data.is_empty() {
            // -----------------------------------------------------------
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
            // -----------------------------------------------------------
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
            // -----------------------------------------------------------
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
        // ---------------------------------------------------------------
        None
    }

    fn mouse_interaction(&self, st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction {
        if st.mouse.is_some() { Interaction::Crosshair } else { Interaction::default() }
    }

    fn draw(&self, st: &St, rndr: &Renderer, _thm: &Theme, bnds: Rectangle, _crsr: Cursor) -> Vec<Geometry> {
        let tst_opt = self.sel_tre();
        let mut geoms: Vec<Geometry> = Vec::new();
        if let Some(tst_opt) = tst_opt
            && self.drawing_enabled
        {
            // -----------------------------------------------------------
            let size = bnds.size();
            let tst: &TreeState = tst_opt;
            draw_bounds(self, st, &_crsr, rndr, bnds, &mut geoms);
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            // -----------------------------------------------------------
        }
        geoms
    }
}

fn lab_text(txt: String, pt: Point, size: Float, template: CnvText) -> CnvText {
    let mut text = template.clone();
    text.content = txt;
    text.position = pt;
    text.size = size.into();
    text
}

fn node_labs(
    nodes: &[NodeData], edges: &[Edge], size: Float, tips: bool, branch: bool, text_w: &mut TextWidth,
    result: &mut Vec<Label>,
) {
    nodes
        .iter()
        .filter_map(|nd| {
            let edge = &edges[nd.edge_idx];
            if let Some(name) = &edge.name
                && !branch
                && ((tips && edge.is_tip) || (!tips && !edge.is_tip))
            {
                let mut txt_lab_tmpl: CnvText = TXT_LAB_TMPL;
                if !tips {
                    txt_lab_tmpl = TXT_LAB_TMPL_INT;
                }
                let width = text_w.width(name);
                let text = lab_text(name.to_string(), nd.points.p1, size, txt_lab_tmpl);
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
