mod draw;
mod state;

use draw::*;
use state::St;

use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

impl Program<TvMsg> for TreeView {
    type State = St;

    fn update(&self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor) -> Option<Action<TvMsg>> {
        // ---------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // ---------------------------------------------------------------
        let tre_opt = self.sel_tre();
        if !self.drawing_enabled || tre_opt.is_none() {
            return None;
        }
        let tst: &TreeState = tre_opt?;
        let edges = tst.edges_srtd_y();
        let tip_edge_idxs = tst.edges_tip_idx();
        let tre_sty = self.tre_style_opt_sel;
        let opn_angle = self.opn_angle;
        // ---------------------------------------------------------------
        if st.text_w_tip.as_mut()?.font_size() != self.lab_size_tip {
            st.text_w_tip.as_mut()?.set_font_size(self.lab_size_tip);
        }
        if st.text_w_int.as_mut()?.font_size() != self.lab_size_int {
            st.text_w_int.as_mut()?.set_font_size(self.lab_size_int);
        }
        if st.text_w_brnch.as_mut()?.font_size() != self.lab_size_brnch {
            st.text_w_brnch.as_mut()?.set_font_size(self.lab_size_brnch);
        }
        // ---------------------------------------------------------------
        if self.stale_vis_rect {
            st.vis_vs =
                RectVals::corners(self.tre_cnv_vis_x0, self.tre_cnv_vis_y0, self.tre_cnv_vis_x1, self.tre_cnv_vis_y1);
        }
        // ---------------------------------------------------------------
        if bnds != st.bnds || self.stale_tre_dims {
            st.bnds = bnds;
            st.cnv_vs = RectVals::cnv(bnds);
            st.tre_vs = RectVals::tre(st.cnv_vs, self.tre_padd);
            st.cnv_rect = st.cnv_vs.into();

            if self.draw_labs_tip && tst.has_tip_labs() {
                st.tip_lab_extra_w = st.calc_tip_lab_extra_w(tst);
            } else {
                st.tip_lab_extra_w = 0e0;
            }

            let tip_w = st.tip_lab_extra_w + self.lab_offset_tip;

            match tre_sty {
                TreSty::PhyGrm => {
                    st.tre_vs = st.tre_vs.padded(
                        0e0,
                        tip_w,
                        -self.lab_offset_brnch + self.lab_size_max,
                        self.lab_size_max / 2e0,
                    );
                }
                TreSty::Fan => {
                    st.tre_vs = st.tre_vs.padded(tip_w, tip_w, tip_w, tip_w);
                }
            }
            st.tre_rect = st.tre_vs.into();
        }

        match tre_sty {
            TreSty::PhyGrm => {
                if self.tre_cnv_w_idx_sel == 1 {
                    st.vis_vs = st.vis_vs.transfer_x_from(&st.cnv_vs);
                }
                if self.tre_cnv_h_idx_sel == 1 {
                    st.vis_vs = st.vis_vs.transfer_y_from(&st.cnv_vs);
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_z_idx_sel == 1 {
                    st.vis_vs = st.cnv_vs;
                }
            }
        }
        st.vis_rect = st.vis_vs.into();
        // ---------------------------------------------------------------
        st.rl = 0e0;
        let root_len_frac = self.root_len_idx_sel as Float / 2e2;
        match tre_sty {
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
        match tre_sty {
            TreSty::PhyGrm => {
                let node_size = st.tre_vs.h / tst.tip_count() as Float;
                st.update_vis_nodes_phygrm(self.tip_labs_vis_max, self.node_labs_vis_max, node_size, tip_edge_idxs);
            }
            TreSty::Fan => {
                st.update_vis_nodes_fan(self.tip_labs_vis_max, self.node_labs_vis_max, opn_angle, edges);
            }
        }
        // ---------------------------------------------------------------
        st.vis_node_idxs
            .par_iter()
            .map(|&idx| match tre_sty {
                TreSty::PhyGrm => node_data_cart(st.tre_vs.w - st.rl, st.tre_vs.h, &edges[idx]).into(),
                TreSty::Fan => node_data_rad(opn_angle, st.tre_vs.radius_min, st.rl, &edges[idx]).into(),
            })
            .collect_into_vec(&mut st.node_data);
        // ---------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // ---------------------------------------------------------------
        if self.draw_labs_tip && tst.has_tip_labs() {
            node_labs(&st.node_data, edges, self.lab_size_tip, true, false, st.text_w_tip.as_mut()?, &mut st.labs_tip);
        }
        // -----------------------------------------------------------
        if self.draw_labs_int && tst.has_int_labs() {
            node_labs(&st.node_data, edges, self.lab_size_int, false, false, st.text_w_int.as_mut()?, &mut st.labs_int);
        }
        // -----------------------------------------------------------
        if self.draw_labs_brnch && tst.has_brlen() {
            node_labs(
                &st.node_data,
                edges,
                self.lab_size_brnch,
                false,
                true,
                st.text_w_brnch.as_mut()?,
                &mut st.labs_brnch,
            );
        }

        if self.stale_vis_rect {
            action = Some(Action::publish(TvMsg::RefreshedVisRect(st.vis_vs)))
        }

        if self.stale_tre_dims {
            action = Some(Action::publish(TvMsg::RefreshedTreeDims(st.cnv_vs)))
        }

        match ev {
            Event::Mouse(mouse_ev) => match mouse_ev {
                MouseEvent::CursorEntered => {
                    st.mouse = None;
                }
                MouseEvent::CursorMoved { position: _ } => {
                    if let Some(mouse) = crsr.position_in(bnds) {
                        st.mouse = Some(mouse);
                        action = Some(Action::request_redraw());
                    } else {
                        st.mouse = None;
                    }
                }
                MouseEvent::CursorLeft => {
                    st.mouse = None;
                }
                MouseEvent::ButtonPressed(_btn) => {}
                MouseEvent::ButtonReleased(_btn) => {}
                _ => {}
            },
            Event::Keyboard(_e) => {}
            Event::Window(WindowEvent::RedrawRequested(_e)) => action = None,
            _ => {}
        }

        action
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
