mod draw;
mod state;

use draw::*;
use state::St;

use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

impl Program<TvMsg> for TreeView {
    type State = St;

    fn update(
        &self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        // ---------------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // ---------------------------------------------------------------------
        let tre_opt = self.sel_tre();
        if !self.drawing_enabled || tre_opt.is_none() {
            return None;
        }
        let tst: &TreeState = tre_opt?;
        let edges = tst.edges_srtd_y();
        let tip_edge_idxs = tst.edges_tip_idx();
        let tre_sty = self.tre_sty_opt_sel;
        let opn_angle = self.opn_angle;
        // ---------------------------------------------------------------------
        if st.text_w_tip.as_mut()?.font_size() != self.lab_size_tip {
            st.text_w_tip.as_mut()?.set_font_size(self.lab_size_tip);
        }
        if st.text_w_int.as_mut()?.font_size() != self.lab_size_int {
            st.text_w_int.as_mut()?.set_font_size(self.lab_size_int);
        }
        if st.text_w_brnch.as_mut()?.font_size() != self.lab_size_brnch {
            st.text_w_brnch.as_mut()?.set_font_size(self.lab_size_brnch);
        } // -------------------------------------------------------------------
        if bnds != st.bnds || self.stale_vis_rect {
            self.clear_cache_bnds();
            self.clear_caches_lab(true, true, true);

            st.bnds = bnds;
            st.cnv_vs = RectVals::cnv(bnds);
            st.cnv_rect = st.cnv_vs.into();

            st.tre_vs = st
                .cnv_vs
                .padded(self.tre_padd_l, self.tre_padd_r, self.tre_padd_t, self.tre_padd_b);

            let mut tip_w: Float = ZRO;
            if self.draw_labs_tip && tst.has_tip_labs() && self.draw_labs_allowed {
                tip_w = calc_tip_w(
                    tre_sty,
                    st.tre_vs,
                    tst.edges_tip_tallest(),
                    self.lab_offset_tip,
                    st.text_w_tip.as_mut()?,
                );
            }

            st.tre_vs =
                calc_tre_vs(tip_w, st.tre_vs, tre_sty, self.lab_offset_brnch, self.lab_size_max);

            st.tre_rect = st.tre_vs.into();
        }
        // ---------------------------------------------------------------------
        if st.is_new || self.stale_vis_rect {
            st.vis_vs = RectVals::corners(
                self.tre_cnv_vis_x0,
                self.tre_cnv_vis_y0,
                self.tre_cnv_vis_x1 - self.tre_padd_r + TRE_PADD,
                self.tre_cnv_vis_y1 - self.tre_padd_b + TRE_PADD,
            );
        }
        match tre_sty {
            TreSty::PhyGrm => {
                if self.tre_cnv_w_idx_sel == 1 {
                    st.vis_vs = st.vis_vs.transfer_x_from(&st.cnv_vs);
                    st.vis_vs = RectVals::corners(
                        st.vis_vs.x0,
                        st.vis_vs.y0,
                        st.vis_vs.x1 - self.tre_padd_r + TRE_PADD,
                        st.vis_vs.y1,
                    );
                }
                if self.tre_cnv_h_idx_sel == 1 {
                    st.vis_vs = st.vis_vs.transfer_y_from(&st.cnv_vs);
                    st.vis_vs = RectVals::corners(
                        st.vis_vs.x0,
                        st.vis_vs.y0,
                        st.vis_vs.x1,
                        st.vis_vs.y1 - self.tre_padd_b + TRE_PADD,
                    );
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_z_idx_sel == 1 {
                    st.vis_vs = st.cnv_vs;
                }
            }
        }
        st.vis_rect = st.vis_vs.into();
        // ---------------------------------------------------------------------
        st.root_len = ZRO;
        match tre_sty {
            TreSty::PhyGrm => {
                if tst.is_rooted() {
                    st.root_len = st.tre_vs.w * self.root_len_frac;
                }
                st.rotation = ZRO;
                st.translation =
                    Vector { x: st.tre_vs.trans.x + st.root_len, y: st.tre_vs.trans.y };
            }
            TreSty::Fan => {
                if tst.is_rooted() {
                    st.root_len = st.tre_vs.radius_min * self.root_len_frac;
                }
                st.rotation = self.rot_angle;
                st.translation = st.tre_vs.cntr;
            }
        } // -------------------------------------------------------------------
        if st.is_new || self.stale_vis_rect {
            match tre_sty {
                TreSty::PhyGrm => {
                    let node_size = st.tre_vs.h / tst.tip_count() as Float;
                    st.update_vis_node_idxs_phygrm(
                        self.tip_labs_vis_max, self.node_labs_vis_max, node_size, tip_edge_idxs,
                    );
                }
                TreSty::Fan => {
                    st.update_vis_node_idxs_fan(
                        self.tip_labs_vis_max, self.node_labs_vis_max, opn_angle, edges,
                    );
                }
            }
        } // -------------------------------------------------------------------
        st.vis_node_idxs
            .par_iter()
            .map(|&idx| match tre_sty {
                TreSty::PhyGrm => {
                    node_data_cart(st.tre_vs.w - st.root_len, st.tre_vs.h, &edges[idx]).into()
                }
                TreSty::Fan => {
                    node_data_rad(opn_angle, st.tre_vs.radius_min, st.root_len, &edges[idx]).into()
                }
            })
            .collect_into_vec(&mut st.vis_nodes);
        // ---------------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // ---------------------------------------------------------------------
        if self.draw_labs_tip && tst.has_tip_labs() && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_tip,
                true,
                false,
                st.text_w_tip.as_mut()?,
                &mut st.labs_tip,
            );
        } // -------------------------------------------------------------------
        if self.draw_labs_int && tst.has_int_labs() && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_int,
                false,
                false,
                st.text_w_int.as_mut()?,
                &mut st.labs_int,
            );
        } // -------------------------------------------------------------------
        if self.draw_labs_brnch && tst.has_brlen() && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_brnch,
                false,
                true,
                st.text_w_brnch.as_mut()?,
                &mut st.labs_brnch,
            );
        } // -------------------------------------------------------------------
        match ev {
            Event::Mouse(mouse_ev) => match mouse_ev {
                MouseEvent::CursorEntered => {
                    self.clear_cache_hovered_node();
                    self.clear_cache_cursor_line();
                    st.mouse = st.mouse_point(crsr);
                    st.hovered_node = None;
                    st.cursor_tracking_point = None;
                }
                MouseEvent::CursorMoved { position: _ } => {
                    self.clear_cache_hovered_node();
                    self.clear_cache_cursor_line();
                    st.mouse = st.mouse_point(crsr);
                    st.hovered_node = st.hovered_node();
                    st.cursor_tracking_point = st.cursor_tracking_point();

                    if let Some(pt) = st.cursor_tracking_point {
                        let crsr_x_rel = match tre_sty {
                            TreSty::PhyGrm => pt.x / (st.tre_vs.w - st.root_len),
                            TreSty::Fan => {
                                (pt.distance(Point::ORIGIN) - st.root_len)
                                    / (st.tre_vs.radius_min - st.root_len)
                            }
                        };

                        if (ZRO - EPSILON..=ONE + EPSILON).contains(&crsr_x_rel) {
                            action =
                                Some(Action::publish(TvMsg::CursorOnTreCnv { x: Some(crsr_x_rel) }))
                        } else {
                            st.cursor_tracking_point = None;
                            action = Some(Action::publish(TvMsg::CursorOnTreCnv { x: None }))
                        }
                    }
                }
                MouseEvent::CursorLeft => {
                    self.clear_cache_hovered_node();
                    self.clear_cache_cursor_line();
                    st.mouse = None;
                    st.hovered_node = None;
                    st.cursor_tracking_point = None;
                }
                MouseEvent::ButtonPressed(btn) => match btn {
                    MouseButton::Left => {
                        if let Some(hevered_node) = &st.hovered_node {
                            let edge = &edges[hevered_node.edge_idx];
                            let node_id = edge.node_id;
                            action = Some(Action::publish(TvMsg::SelectDeselectNode(node_id)));
                        }
                    }
                    MouseButton::Right => {}
                    _ => {}
                },
                MouseEvent::ButtonReleased(_btn) => {}
                _ => {}
            },
            Event::Keyboard(_e) => {}
            Event::Window(WindowEvent::RedrawRequested(_)) => action = None,
            _ => {}
        }
        // ---------------------------------------------------------------------
        if let Some(crsr_x_rel) = self.crsr_x_rel {
            match tre_sty {
                TreSty::PhyGrm => {
                    st.cursor_tracking_point =
                        Some(Point { x: (crsr_x_rel * (st.tre_vs.w - st.root_len)), y: ZRO })
                }
                TreSty::Fan => {
                    st.cursor_tracking_point = Some(Point {
                        x: st.root_len + crsr_x_rel * (st.tre_vs.radius_min - st.root_len),
                        y: ZRO,
                    })
                }
            }
        }
        // ---------------------------------------------------------------------
        st.is_new = false;
        action
    }

    fn mouse_interaction(&self, st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction {
        if st.hovered_node.is_some() { Interaction::Pointer } else { Interaction::default() }
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, _thm: &Theme, bnds: Rectangle, _crsr: Cursor,
    ) -> Vec<Geometry> {
        let tst_opt = self.sel_tre();
        let mut geoms: Vec<Geometry> = Vec::new();
        if let Some(tst_opt) = tst_opt
            && self.drawing_enabled
        {
            // -----------------------------------------------------------------
            let size = bnds.size();
            let tst: &TreeState = tst_opt;
            // draw_bounds(self, st, rndr, bnds, &mut geoms);
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            draw_legend(self, st, tst, rndr, size, &mut geoms);
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            draw_selected_nodes(self, st, tst, rndr, size, &mut geoms);
            draw_filtered_nodes(self, st, tst, rndr, size, &mut geoms);
            draw_hovered_node(self, st, tst, rndr, size, &mut geoms);
            draw_cursor_line(self, st, tst, rndr, size, &mut geoms);
            // -----------------------------------------------------------------
        }
        geoms
    }
}

pub(crate) fn calc_tre_vs(
    tip_w: Float, tre_vs: RectVals<Float>, tre_sty: TreSty, lab_offset_brnch: Float,
    lab_size_max: Float,
) -> RectVals<Float> {
    match tre_sty {
        TreSty::PhyGrm => {
            tre_vs.padded(ZRO, tip_w, -lab_offset_brnch + lab_size_max, lab_size_max / TWO)
        }
        TreSty::Fan => tre_vs.padded(tip_w, tip_w, tip_w, tip_w),
    }
}

pub(crate) fn calc_tip_w(
    tre_sty: TreSty, tre_vs: RectVals<Float>, edges_tip_tallest: &[Edge], lab_offset_tip: Float,
    text_w_tip: &mut TextWidth,
) -> Float {
    let tre_vs_w = match tre_sty {
        TreSty::PhyGrm => tre_vs.w,
        TreSty::Fan => tre_vs.radius_min,
    };
    let tip_w: Float =
        lab_offset_tip + calc_tip_lab_extra_w(tre_vs_w, edges_tip_tallest, text_w_tip);
    tip_w.min(tre_vs_w / TWO)
}

fn calc_tip_lab_extra_w(
    tre_vs_w: Float, edges_tip_tallest: &[Edge], text_w_tip: &mut TextWidth,
) -> Float {
    let mut max_w: Float = ZRO;
    let mut max_offset: Float = ZRO;
    for edge in edges_tip_tallest {
        if let Some(name) = &edge.name {
            let offset = edge.x1 as Float * tre_vs_w;
            if offset >= max_offset {
                max_offset = offset
            };
            let tip_name_w = text_w_tip.width(name);
            let curr_max_w = tip_name_w + (max_offset + offset) / TWO - tre_vs_w;
            if curr_max_w >= max_w {
                max_w = curr_max_w;
            }
        }
    }
    max_w
}
