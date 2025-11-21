use super::St;
use super::draw::*;
use crate::*;

impl Program<TvMsg> for TreeCnv {
    type State = St;
    fn update(
        &self,
        st: &mut St,
        ev: &Event,
        bnds: Rectangle,
        crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        // ---------------------------------------------------------------------
        let mut action: Option<Action<TvMsg>> = None;
        // ---------------------------------------------------------------------

        let tree_state_opt = self.tree_state.as_deref();

        if !self.drawing_enabled || tree_state_opt.is_none() {
            return None;
        }

        let tst: &TreeState = tree_state_opt?;

        let edges = tst.edges()?;
        let tip_edge_idxs = tst.tip_edge_idxs();
        let tip_count = tst.tip_count();
        let sel_edge_idxs = tst.sel_edge_idxs();
        let found_edge_idxs = tst.found_edge_idxs();
        let is_rooted = tst.is_rooted();

        st.tre_sty = self.tre_sty;
        st.opn_angle = self.opn_angle;
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
        } // -------------------------------------------------------------------
        if bnds != st.bnds
            || st.stale_vis_rect
            || st.is_new
            || self.stale_tre_rect
        {
            st.cnv_vs = RectVals::cnv(bnds);
            st.cnv_rect = st.cnv_vs.clone().into();
            (st.tre_vs, st.root_len) = self.calc_tre_vs(
                &st.cnv_vs,
                tst.edges_tip_tallest(),
                is_rooted,
                tst.has_clade_labels(),
                st.text_w_tip.as_mut()?,
            );
            st.tre_rect = st.tre_vs.clone().into();

            match st.tre_sty {
                TreSty::PhyGrm => {
                    st.tip_lab_w_ring = None;
                    let tip_lab_w_rect: Rectangle = st
                        .tre_vs
                        .padded(st.tre_vs.w + SF, -PADDING - SF, ZRO, ZRO)
                        .into();
                    st.tip_lab_w_rect = Some(tip_lab_w_rect);
                }
                TreSty::Fan => {
                    st.tip_lab_w_rect = None;
                    st.tip_lab_w_ring = Some(st.tre_vs.radius_min + SF);
                }
            }

            st.bnds = bnds;
            // -----------------------------------------------------------------
            self.clear_cache_cnv_bnds();
            tst.clear_cache_cnv_lab_tip();
            tst.clear_cache_cnv_lab_int();
            tst.clear_cache_cnv_lab_brnch();
        }
        // ---------------------------------------------------------------------
        let align_tips_at: Float;
        match st.tre_sty {
            TreSty::PhyGrm => {
                align_tips_at = st.tre_vs.w;
                st.rotation = ZRO;
                st.translation =
                    Vector { x: st.tre_vs.trans.x, y: st.tre_vs.trans.y };
            }
            TreSty::Fan => {
                align_tips_at = st.tre_vs.radius_min;
                st.rotation = self.rot_angle;
                st.translation = st.tre_vs.cntr;
            }
        }
        // ---------------------------------------------------------------------
        if st.stale_vis_rect || st.is_new || self.stale_tre_rect {
            match st.tre_sty {
                TreSty::PhyGrm => {
                    let node_size = st.tre_vs.h / tip_count as Float;
                    st.update_vis_edge_idxs_phygrm(node_size, tip_edge_idxs);
                }
                TreSty::Fan => {
                    st.update_vis_edge_idxs_fan(edges);
                }
            }
        }
        // ---------------------------------------------------------------------
        st.update_vis_nodes(edges);
        st.update_filtered_nodes(edges, found_edge_idxs);
        st.update_selected_nodes(edges, sel_edge_idxs);
        // ---------------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // ---------------------------------------------------------------------
        if tst.has_tip_labels() && self.draw_labs_tip && self.draw_labs_allowed
        {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_tip,
                true,
                false,
                match self.align_tip_labs {
                    true => Some(align_tips_at),
                    false => None,
                },
                match self.trim_tip_labs {
                    true => Some(self.trim_tip_labs_to_nchar as usize),
                    false => None,
                },
                st.text_w_tip.as_mut()?,
                &mut st.labs_tip,
            );
        } // -------------------------------------------------------------------
        if tst.has_int_labs() && self.draw_labs_int && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_int,
                false,
                false,
                None,
                None,
                st.text_w_int.as_mut()?,
                &mut st.labs_int,
            );
        } // -------------------------------------------------------------------
        if tst.has_brlen() && self.draw_labs_brnch && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_brnch,
                false,
                true,
                None,
                None,
                st.text_w_brnch.as_mut()?,
                &mut st.labs_brnch,
            );
        } // -------------------------------------------------------------------
        match ev {
            Event::Mouse(mouse_ev) => {
                self.clear_cache_cnv_hovered_node();
                self.clear_cache_cnv_cursor_line();
                match mouse_ev {
                    MouseEvent::CursorEntered => {
                        st.mouse_angle = None;
                        st.mouse_zone = None;
                        st.mouse = st.mouse_point(crsr);
                        st.hovered_node = None;
                        st.cursor_tracking_point = None;
                        st.mouse_is_over_tip_w_resize_area = false;
                    }
                    MouseEvent::CursorMoved { position: _ } => {
                        st.mouse = st.mouse_point(crsr);
                        st.mouse_angle = st.mouse_angle(crsr);
                        st.mouse_zone = st.mouse_angle_to_zone();
                        st.hovered_node = st.hovered_node(edges);
                        st.cursor_tracking_point = st.cursor_tracking_point();
                        st.mouse_is_over_tip_w_resize_area = self.draw_labs_tip
                            && self.draw_labs_allowed
                            && st.is_mouse_over_tip_w_resize_area();

                        if st.tip_lab_w_is_being_resized {
                            st.cursor_tracking_point = None;
                            action = Some(Action::publish(
                                TvMsg::TipLabWidthSetByUser(Some(
                                    st.calc_tip_lab_w(),
                                )),
                            ));
                        } else if let Some(pt) = st.cursor_tracking_point {
                            let crsr_x_rel = match st.tre_sty {
                                TreSty::PhyGrm => pt.x / st.tre_vs.w,
                                TreSty::Fan => {
                                    (pt.distance(ORIGIN) - st.root_len)
                                        / (st.tre_vs.radius_min - st.root_len)
                                }
                            };

                            if (ZRO - EPSILON..=ONE + EPSILON)
                                .contains(&crsr_x_rel)
                            {
                                action = Some(Action::publish(
                                    TvMsg::CursorOnTreCnv {
                                        x: Some(crsr_x_rel),
                                    },
                                ));
                            } else {
                                st.cursor_tracking_point = None;
                                action = Some(Action::publish(
                                    TvMsg::CursorOnTreCnv { x: None },
                                ));
                            }
                        }
                    }
                    MouseEvent::CursorLeft => {
                        st.mouse_angle = None;
                        st.mouse_zone = None;
                        st.mouse = None;
                        st.hovered_node = None;
                        st.cursor_tracking_point = None;
                        st.mouse_is_over_tip_w_resize_area = false;
                    }
                    MouseEvent::ButtonPressed(btn) => {
                        match btn {
                            MouseButton::Left => {
                                if st.mouse_is_over_tip_w_resize_area {
                                    st.tip_lab_w_is_being_resized = true;
                                } else if let Some((node_id, _)) =
                                    &st.hovered_node
                                {
                                    match self.selection_lock {
                                        true => action = Some(Action::publish(
                                        TvMsg::SelectDeselectNode(*node_id),
                                    )),
                                        false => action = Some(Action::publish(
                                        TvMsg::SelectDeselectNodeExclusive(*node_id),
                                    )),
                                }
                                }
                            }
                            MouseButton::Right => {
                                if st.mouse_is_over_tip_w_resize_area
                                    && self.tip_w_set_by_user.is_some()
                                {
                                    let listing = TvContextMenuListing::for_tip_lab_w_resize_area();
                                    action = Some(Action::publish(
                                        TvMsg::ContextMenuInteractionBegin(
                                            listing,
                                        ),
                                    ));
                                } else if let Some((node_id, _)) =
                                    &st.hovered_node
                                {
                                    action = Some(Action::publish(
                                        TvMsg::ContextMenuInteractionBegin(
                                            TvContextMenuListing::for_node(
                                                *node_id, tst,
                                            ),
                                        ),
                                    ));
                                }
                            }
                            _ => {}
                        }
                    }
                    MouseEvent::ButtonReleased(_btn) => {
                        st.tip_lab_w_is_being_resized = false;
                    }
                    MouseEvent::WheelScrolled { .. } => {}
                }
            }
            Event::Keyboard(e) => match e {
                KeyboardEvent::ModifiersChanged(modifs) => {
                    let shift = Modifiers::SHIFT;
                    if modifs.contains(shift) && !st.modifs.contains(shift) {
                        action = Some(Action::publish(
                            TvMsg::SelectionLockChanged(!self.selection_lock),
                        ));
                        st.modifs.insert(shift);
                    } else if !modifs.contains(shift)
                        && st.modifs.contains(shift)
                    {
                        action = Some(Action::publish(
                            TvMsg::SelectionLockChanged(!self.selection_lock),
                        ));
                        st.modifs.remove(shift);
                    }
                }
                KeyboardEvent::KeyPressed {
                    key,
                    modified_key: _,
                    physical_key: _,
                    location: _,
                    modifiers: _,
                    text: _,
                } => {
                    if let Some((node_id, _)) = &st.hovered_node
                        && let Key::Character(k) = key
                    {
                        let k: &str = k.as_str();
                        match k {
                            "1" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeLabel((
                                        *node_id,
                                        Clr::BLU_25,
                                    )),
                                ));
                            }
                            "2" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeLabel((
                                        *node_id,
                                        Clr::CYA_25,
                                    )),
                                ));
                            }
                            "3" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeLabel((
                                        *node_id,
                                        Clr::GRN_25,
                                    )),
                                ));
                            }
                            "4" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeLabel((
                                        *node_id,
                                        Clr::MAG_25,
                                    )),
                                ));
                            }
                            "5" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeLabel((
                                        *node_id,
                                        Clr::YEL_25,
                                    )),
                                ));
                            }
                            "6" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeLabel((
                                        *node_id,
                                        Clr::RED_25,
                                    )),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                KeyboardEvent::KeyReleased {
                    key: _,
                    modified_key: _,
                    physical_key: _,
                    location: _,
                    modifiers: _,
                } => {}
            },

            Event::Window(WindowEvent::RedrawRequested(_)) => action = None,
            _ => {}
        }
        // ---------------------------------------------------------------------
        if let Some(crsr_x_rel) = self.crsr_x_rel {
            match st.tre_sty {
                TreSty::PhyGrm => {
                    st.cursor_tracking_point =
                        Some(Point { x: crsr_x_rel * st.tre_vs.w, y: ZRO });
                }
                TreSty::Fan => {
                    st.cursor_tracking_point = Some(Point {
                        x: st.root_len
                            + crsr_x_rel * (st.tre_vs.radius_min - st.root_len),
                        y: ZRO,
                    });
                }
            }
        }
        // ---------------------------------------------------------------------
        if self.stale_tre_rect {
            action = Some(Action::publish(TvMsg::TreeRectNoLongerStale));
        }
        // ---------------------------------------------------------------------
        if st.is_new {
            st.is_new = false;
        }
        action
    }

    fn mouse_interaction(
        &self,
        st: &St,
        _bnds: Rectangle,
        _crsr: Cursor,
    ) -> MouseInteraction {
        if st.mouse_is_over_tip_w_resize_area || st.tip_lab_w_is_being_resized {
            if st.tip_lab_w_rect.is_some() {
                MouseInteraction::ResizingHorizontally
            } else if st.tip_lab_w_ring.is_some() {
                match &st.mouse_zone {
                    Some(zone) => match zone {
                        Zone::TopLeft => {
                            MouseInteraction::ResizingDiagonallyDown
                        }
                        Zone::TopRight => {
                            MouseInteraction::ResizingDiagonallyUp
                        }
                        Zone::BottomLeft => {
                            MouseInteraction::ResizingDiagonallyUp
                        }
                        Zone::BottomRight => {
                            MouseInteraction::ResizingDiagonallyDown
                        }
                        Zone::Top => MouseInteraction::ResizingVertically,
                        Zone::Left => MouseInteraction::ResizingHorizontally,
                        Zone::Right => MouseInteraction::ResizingHorizontally,
                        Zone::Bottom => MouseInteraction::ResizingVertically,
                    },
                    None => MouseInteraction::default(),
                }
            } else {
                MouseInteraction::default()
            }
        } else if st.hovered_node.is_some() {
            MouseInteraction::Pointer
        } else {
            MouseInteraction::default()
        }
    }

    fn draw(
        &self,
        st: &St,
        rndr: &Renderer,
        thm: &Theme,
        bnds: Rectangle,
        _crsr: Cursor,
    ) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();
        if let Some(tst) = self.tree_state.as_deref()
            && self.drawing_enabled
        {
            let size = bnds.size();
            if self.draw_debug {
                draw_bounds(self, st, rndr, bnds, &mut geoms);
            }
            draw_clade_labels(st, tst, rndr, size, &mut geoms);
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            if st.mouse_is_over_tip_w_resize_area
                || st.tip_lab_w_is_being_resized
            {
                draw_tip_lab_w_resize_area(self, st, rndr, bnds, &mut geoms);
            }
            draw_hovered_node(self, st, tst, rndr, size, &mut geoms);
            draw_legend(self, st, tst, rndr, size, &mut geoms);
            draw_cursor_line(self, st, rndr, size, &mut geoms);
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            draw_selected_nodes(st, tst, rndr, size, &mut geoms);
            draw_filtered_nodes(self, st, tst, rndr, size, &mut geoms);
            if self.draw_debug {
                draw_palette(self, st, thm, rndr, size, &mut geoms);
            }
        }
        geoms
    }
}
