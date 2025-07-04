use super::St;
use super::draw::*;
use crate::edge_utils::*;
use crate::*;

fn prepare_nodes(
    vs: &RectVals<Float>, root_len: Float, tre_sty: TreSty, opn_angle: Float, edges: &[Edge],
    node_idxs: &[usize], results: &mut Vec<NodeData>,
) {
    node_idxs
        .par_iter()
        .map(|&idx| match tre_sty {
            TreSty::PhyGrm => node_data_cart(vs.w, vs.h, &edges[idx]).into(),
            TreSty::Fan => {
                node_data_rad(opn_angle, ZRO, vs.radius_min, root_len, &edges[idx]).into()
            }
        })
        .collect_into_vec(results);
}

impl Program<TvMsg> for TreeCnv {
    type State = St;
    fn update(
        &self, st: &mut St, ev: &Event, bnds: Rectangle, crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        // -----------------------------------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // -----------------------------------------------------------------------------------------
        let tree_state_opt = self.tree_state.as_deref();
        if !self.drawing_enabled || tree_state_opt.is_none() {
            return None;
        }
        let tst: &TreeState = tree_state_opt?;
        let edges = tst.edges_srtd_y()?;
        let tip_edge_idxs = tst.edges_tip_idx();
        let tre_sty = self.tre_sty;
        let opn_angle = self.opn_angle;
        // -----------------------------------------------------------------------------------------
        if st.text_w_tip.as_mut()?.font_size() != self.lab_size_tip {
            st.text_w_tip.as_mut()?.set_font_size(self.lab_size_tip);
        }
        if st.text_w_int.as_mut()?.font_size() != self.lab_size_int {
            st.text_w_int.as_mut()?.set_font_size(self.lab_size_int);
        }
        if st.text_w_brnch.as_mut()?.font_size() != self.lab_size_brnch {
            st.text_w_brnch.as_mut()?.set_font_size(self.lab_size_brnch);
        } // ---------------------------------------------------------------------------------------
        st.stale_vis_rect = false;
        let vis_x0 = self.vis_x0;
        let vis_y0 = self.vis_y0;
        let vis_x1 = self.vis_x1 - self.padd_r + TREE_PADDING;
        let vis_y1 = self.vis_y1 - self.padd_b + TREE_PADDING;
        if vis_x0 != st.vis_vs.x0
            || vis_y0 != st.vis_vs.y0
            || vis_x1 != st.vis_vs.x1
            || vis_y1 != st.vis_vs.y1
        {
            st.vis_vs = RectVals::corners(vis_x0, vis_y0, vis_x1, vis_y1);
            st.vis_rect = st.vis_vs.clone().into();
            st.stale_vis_rect = true;
        } // ---------------------------------------------------------------------------------------
        if bnds != st.bnds || st.stale_vis_rect || st.is_new || self.stale_tre_rect {
            st.cnv_vs = RectVals::cnv(bnds);
            st.cnv_rect = st.cnv_vs.clone().into();
            (st.tre_vs, st.root_len) = self.calc_tre_vs(
                &st.cnv_vs,
                tst.edges_tip_tallest(),
                tst.is_rooted(),
                tst.has_clade_labels(),
                st.text_w_tip.as_mut()?,
            );
            st.tre_rect = st.tre_vs.clone().into();
            st.bnds = bnds;
            // -------------------------------------------------------------------------------------
            self.clear_cache_bnds();
            tst.clear_cache_lab_tip();
            tst.clear_cache_lab_int();
            tst.clear_cache_lab_brnch();
        }
        // -----------------------------------------------------------------------------------------
        match tre_sty {
            TreSty::PhyGrm => {
                st.rotation = ZRO;
                st.translation = Vector { x: st.tre_vs.trans.x, y: st.tre_vs.trans.y };
            }
            TreSty::Fan => {
                st.rotation = self.rot_angle;
                st.translation = st.tre_vs.cntr;
            }
        }
        // -----------------------------------------------------------------------------------------
        if st.stale_vis_rect || st.is_new || self.stale_tre_rect {
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
        } // ---------------------------------------------------------------------------------------
        prepare_nodes(
            &st.tre_vs, st.root_len, tre_sty, opn_angle, edges, &st.vis_node_idxs,
            &mut st.vis_nodes,
        );
        // -----------------------------------------------------------------------------------------
        prepare_nodes(
            &st.tre_vs,
            st.root_len,
            tre_sty,
            opn_angle,
            edges,
            tst.found_edge_idxs(),
            &mut st.filtered_nodes,
        );
        // -----------------------------------------------------------------------------------------
        prepare_nodes(
            &st.tre_vs,
            st.root_len,
            tre_sty,
            opn_angle,
            edges,
            tst.sel_edge_idxs(),
            &mut st.selected_nodes,
        );
        // -----------------------------------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // -----------------------------------------------------------------------------------------
        if tst.has_tip_labs() && self.draw_labs_tip && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_tip,
                true,
                false,
                match self.trim_tip_labs {
                    true => Some(self.trim_tip_labs_to_nchar as usize),
                    false => None,
                },
                st.text_w_tip.as_mut()?,
                &mut st.labs_tip,
            );
        } // ---------------------------------------------------------------------------------------
        if tst.has_int_labs() && self.draw_labs_int && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_int,
                false,
                false,
                None,
                st.text_w_int.as_mut()?,
                &mut st.labs_int,
            );
        } // ---------------------------------------------------------------------------------------
        if tst.has_brlen() && self.draw_labs_brnch && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_brnch,
                false,
                true,
                None,
                st.text_w_brnch.as_mut()?,
                &mut st.labs_brnch,
            );
        } // ---------------------------------------------------------------------------------------
        match ev {
            Event::Mouse(mouse_ev) => {
                self.clear_cache_hovered_node();
                self.clear_cache_cursor_line();
                match mouse_ev {
                    MouseEvent::CursorEntered => {
                        st.mouse = st.mouse_point(crsr);
                        st.hovered_node = None;
                        st.cursor_tracking_point = None;
                    }
                    MouseEvent::CursorMoved { position: _ } => {
                        st.mouse = st.mouse_point(crsr);
                        st.hovered_node = st.hovered_node();
                        st.cursor_tracking_point = st.cursor_tracking_point();

                        if let Some(pt) = st.cursor_tracking_point {
                            let crsr_x_rel = match tre_sty {
                                TreSty::PhyGrm => pt.x / st.tre_vs.w,
                                TreSty::Fan => {
                                    (pt.distance(ORIGIN) - st.root_len)
                                        / (st.tre_vs.radius_min - st.root_len)
                                }
                            };

                            if (ZRO - EPSILON..=ONE + EPSILON).contains(&crsr_x_rel) {
                                action = Some(Action::publish(TvMsg::CursorOnTreCnv {
                                    x: Some(crsr_x_rel),
                                }))
                            } else {
                                st.cursor_tracking_point = None;
                                action = Some(Action::publish(TvMsg::CursorOnTreCnv { x: None }))
                            }
                        }
                    }
                    MouseEvent::CursorLeft => {
                        st.mouse = None;
                        st.hovered_node = None;
                        st.cursor_tracking_point = None;
                    }
                    MouseEvent::ButtonPressed(btn) => match btn {
                        MouseButton::Left => {
                            if let Some(hovered_node) = &st.hovered_node {
                                let edge = &edges[hovered_node.edge_idx];
                                let node_id = edge.node_id;
                                // -----------------------------------------------------------------
                                // if let Some(modifiers) = st.modifiers
                                //     && modifiers == Modifiers::SHIFT
                                // {
                                //     action =
                                //         Some(Action::publish(TvMsg::SelectDeselectNode(node_id)));
                                // } else {
                                //     action = Some(Action::publish(
                                //         TvMsg::SelectDeselectNodeExclusive(node_id),
                                //     ));
                                // }
                                // -----------------------------------------------------------------
                                action = Some(Action::publish(TvMsg::SelectDeselectNode(node_id)));
                                // -----------------------------------------------------------------
                            }
                        }
                        MouseButton::Right => {
                            if let Some(hovered_node) = &st.hovered_node {
                                action = Some(Action::publish(TvMsg::ContextMenuInteractionBegin(
                                    TvContextMenuListing::for_node(
                                        &edges[hovered_node.edge_idx].node_id, tst,
                                    ),
                                )));
                            }
                        }
                        _ => {}
                    },
                    MouseEvent::ButtonReleased(_btn) => {}
                    _ => {}
                }
            }
            Event::Keyboard(KeyboardEvent::ModifiersChanged(modifiers)) => {
                st.modifiers = match *modifiers {
                    Modifiers::SHIFT => Some(Modifiers::SHIFT),
                    _ => None,
                };
            }
            Event::Window(WindowEvent::RedrawRequested(_)) => action = None,
            _ => {}
        }
        // -----------------------------------------------------------------------------------------
        if let Some(crsr_x_rel) = self.crsr_x_rel {
            match tre_sty {
                TreSty::PhyGrm => {
                    st.cursor_tracking_point = Some(Point { x: crsr_x_rel * st.tre_vs.w, y: ZRO })
                }
                TreSty::Fan => {
                    st.cursor_tracking_point = Some(Point {
                        x: st.root_len + crsr_x_rel * (st.tre_vs.radius_min - st.root_len),
                        y: ZRO,
                    })
                }
            }
        }
        // -----------------------------------------------------------------------------------------
        if self.stale_tre_rect {
            action = Some(Action::publish(TvMsg::TreeRectNoLongerStale));
        }
        // -----------------------------------------------------------------------------------------
        if st.is_new {
            st.is_new = false
        }
        action
    }

    fn mouse_interaction(&self, st: &St, _bnds: Rectangle, _crsr: Cursor) -> Interaction {
        if st.hovered_node.is_some() { Interaction::Pointer } else { Interaction::default() }
    }

    fn draw(
        &self, st: &St, rndr: &Renderer, thm: &Theme, bnds: Rectangle, _crsr: Cursor,
    ) -> Vec<Geometry> {
        let tree_state_opt = self.tree_state.as_deref();
        let mut geoms: Vec<Geometry> = Vec::new();
        if let Some(tree_state_opt) = tree_state_opt
            && self.drawing_enabled
        {
            // -------------------------------------------------------------------------------------
            let size = bnds.size();
            let tst: &TreeState = tree_state_opt;
            if self.draw_debug {
                draw_bounds(self, st, rndr, bnds, &mut geoms);
            }
            draw_clade_labels(self, st, tst, rndr, size, &mut geoms);
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            draw_legend(self, st, tst, rndr, size, &mut geoms);
            draw_cursor_line(self, st, rndr, size, &mut geoms);
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            draw_hovered_node(self, st, rndr, size, &mut geoms);
            draw_selected_nodes(st, tst, rndr, size, &mut geoms);
            draw_filtered_nodes(self, st, tst, rndr, size, &mut geoms);
            if self.draw_debug {
                draw_palette(self, st, thm, rndr, size, &mut geoms);
            }
            // -------------------------------------------------------------------------------------
        }
        geoms
    }
}
