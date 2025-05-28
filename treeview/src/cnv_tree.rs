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
        // -------------------------------------------------------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // -------------------------------------------------------------------------------------------------------------
        let tre_opt = self.sel_tre();
        if !self.drawing_enabled || tre_opt.is_none() {
            return None;
        }
        let tst: &TreeState = tre_opt?;
        let edges = tst.edges_srtd_y();
        let tip_edge_idxs = tst.edges_tip_idx();
        let tre_sty = self.tre_style_opt_sel;
        let opn_angle = self.opn_angle;
        // -------------------------------------------------------------------------------------------------------------
        if st.text_w_tip.as_mut()?.font_size() != self.lab_size_tip {
            st.text_w_tip.as_mut()?.set_font_size(self.lab_size_tip);
        }
        if st.text_w_int.as_mut()?.font_size() != self.lab_size_int {
            st.text_w_int.as_mut()?.set_font_size(self.lab_size_int);
        }
        if st.text_w_brnch.as_mut()?.font_size() != self.lab_size_brnch {
            st.text_w_brnch.as_mut()?.set_font_size(self.lab_size_brnch);
        } // -----------------------------------------------------------------------------------------------------------
        if bnds != st.bnds || self.stale_vis_rect || self.stale_tre_dims {
            st.bnds = bnds;
            st.cnv_vs = RectVals::cnv(bnds);
            st.tre_vs = RectVals::tre(st.cnv_vs, self.tre_padd);
            st.cnv_rect = st.cnv_vs.into();

            let mut tip_w: Float;
            if self.draw_labs_tip && tst.has_tip_labs() {
                tip_w = self.lab_offset_tip + st.calc_tip_lab_extra_w(tst);
            } else {
                tip_w = self.lab_offset_tip;
            }

            match tre_sty {
                TreSty::PhyGrm => {
                    tip_w = tip_w.min(st.tre_vs.w / 2e0);
                    st.tre_vs = st.tre_vs.padded(
                        0e0,
                        tip_w,
                        -self.lab_offset_brnch + self.lab_size_max,
                        self.lab_size_max / 2e0,
                    );
                }
                TreSty::Fan => {
                    tip_w = tip_w.min(st.tre_vs.radius_min / 2e0);
                    st.tre_vs = st.tre_vs.padded(tip_w, tip_w, tip_w, tip_w);
                }
            }
            st.tre_rect = st.tre_vs.into();
        }
        // -------------------------------------------------------------------------------------------------------------
        if self.stale_vis_rect || st.is_new {
            st.vis_vs =
                RectVals::corners(self.tre_cnv_vis_x0, self.tre_cnv_vis_y0, self.tre_cnv_vis_x1, self.tre_cnv_vis_y1);
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
        // -----------------------------------------------------------------------------------------------------------
        st.root_len = 0e0;
        let root_len_frac = self.root_len_idx_sel as Float / 2e2;
        match tre_sty {
            TreSty::PhyGrm => {
                if tst.is_rooted() {
                    st.root_len = st.tre_vs.w * root_len_frac;
                }
                st.rotation = 0e0;
                st.translation = Vector { x: st.tre_vs.trans.x + st.root_len, y: st.tre_vs.trans.y };
            }
            TreSty::Fan => {
                if tst.is_rooted() {
                    st.root_len = st.tre_vs.radius_min * root_len_frac;
                }
                st.rotation = self.rot_angle;
                st.translation = st.tre_vs.cntr;
            }
        } // -----------------------------------------------------------------------------------------------------------
        if self.stale_vis_rect || st.is_new {
            match tre_sty {
                TreSty::PhyGrm => {
                    let node_size = st.tre_vs.h / tst.tip_count() as Float;
                    st.update_vis_node_idxs_phygrm(
                        self.tip_labs_vis_max, self.node_labs_vis_max, node_size, tip_edge_idxs,
                    );
                }
                TreSty::Fan => {
                    st.update_vis_node_idxs_fan(self.tip_labs_vis_max, self.node_labs_vis_max, opn_angle, edges);
                }
            }
        } // -----------------------------------------------------------------------------------------------------------
        st.vis_node_idxs
            .par_iter()
            .map(|&idx| match tre_sty {
                TreSty::PhyGrm => node_data_cart(st.tre_vs.w - st.root_len, st.tre_vs.h, &edges[idx]).into(),
                TreSty::Fan => node_data_rad(opn_angle, st.tre_vs.radius_min, st.root_len, &edges[idx]).into(),
            })
            .collect_into_vec(&mut st.vis_nodes);
        // -------------------------------------------------------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // -------------------------------------------------------------------------------------------------------------
        if self.draw_labs_tip && tst.has_tip_labs() {
            node_labs(&st.vis_nodes, edges, self.lab_size_tip, true, false, st.text_w_tip.as_mut()?, &mut st.labs_tip);
        } // -----------------------------------------------------------------------------------------------------------
        if self.draw_labs_int && tst.has_int_labs() {
            node_labs(&st.vis_nodes, edges, self.lab_size_int, false, false, st.text_w_int.as_mut()?, &mut st.labs_int);
        } // -----------------------------------------------------------------------------------------------------------
        if self.draw_labs_brnch && tst.has_brlen() {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_brnch,
                false,
                true,
                st.text_w_brnch.as_mut()?,
                &mut st.labs_brnch,
            );
        } // -----------------------------------------------------------------------------------------------------------
        if self.stale_vis_rect {
            self.clear_cache_bnds();
            self.clear_caches_lab(true, true, true);
            action = Some(Action::publish(TvMsg::RefreshedVisRect(st.vis_vs)))
        }
        if self.stale_tre_dims {
            action = Some(Action::publish(TvMsg::RefreshedTreeDims(st.cnv_vs)))
        } // -----------------------------------------------------------------------------------------------------------
        match ev {
            Event::Mouse(mouse_ev) => match mouse_ev {
                MouseEvent::CursorEntered => {
                    self.clear_cache_node_hover();
                    st.mouse = st.update_mouse_pos(crsr);
                    st.hovered_node = None;
                }
                MouseEvent::CursorMoved { position: _ } => {
                    self.clear_cache_node_hover();
                    st.mouse = st.update_mouse_pos(crsr);
                    action = Some(Action::request_redraw());
                    if let Some(mouse) = st.mouse {
                        let closest_node = st
                            .vis_nodes
                            .iter()
                            .min_by(|&a, &b| mouse.distance(a.points.p1).total_cmp(&mouse.distance(b.points.p1)))
                            .cloned();
                        if let Some(closest_node) = closest_node {
                            if mouse.distance(closest_node.points.p1) <= st.node_radius_hover {
                                st.hovered_node = Some(closest_node)
                            } else {
                                st.hovered_node = None
                            }
                        }
                    } else {
                        st.hovered_node = None;
                    }
                }
                MouseEvent::CursorLeft => {
                    self.clear_cache_node_hover();
                    st.mouse = None;
                    st.hovered_node = None;
                }
                MouseEvent::ButtonPressed(_btn) => {}
                MouseEvent::ButtonReleased(_btn) => {}
                _ => {}
            },
            Event::Keyboard(_e) => {}
            Event::Window(WindowEvent::RedrawRequested(_e)) => {
                // action = None
            }
            _ => {}
        }
        // -------------------------------------------------------------------------------------------------------------
        st.is_new = false;
        action
    }

    fn mouse_interaction(&self, st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction {
        if st.hovered_node.is_some() { Interaction::Pointer } else { Interaction::default() }
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
            // draw_bounds(self, st, rndr, bnds, &mut geoms);
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            draw_node_hover(self, st, tst, rndr, size, &mut geoms);
            // -----------------------------------------------------------
        }
        geoms
    }
}
