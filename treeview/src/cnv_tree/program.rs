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
        let vis_x1 = self.vis_x1 - self.padd_r + PLOT_PADDING;
        let vis_y1 = self.vis_y1 - self.padd_b + PLOT_PADDING;
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
                tst.has_clade_highlights(),
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
            st.update_vis_edge_idxs(edges);
        }
        // ---------------------------------------------------------------------
        st.update_vis_nodes(edges);
        let sel_edge_idxs = tst.sel_edge_idxs();
        if st.vis_edge_idxs.len() < 1000 || sel_edge_idxs.len() < 1000 {
            let sel_edge_idxs_hs: HashSet<&usize> =
                HashSet::from_iter(sel_edge_idxs);
            let vis_edge_idxs_hs: HashSet<&usize> =
                HashSet::from_iter(&st.vis_edge_idxs);
            let vis_sel_edge_idxs_hs: Vec<usize> = vis_edge_idxs_hs
                .intersection(&sel_edge_idxs_hs)
                .copied()
                .copied()
                .collect();
            st.update_selected_nodes(edges, &vis_sel_edge_idxs_hs);
        } else {
            st.selected_nodes.clear();
        }
        // ---------------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // ---------------------------------------------------------------------

        let highlighted_node_ids =
            match tst.is_search_active() && self.search_is_active {
                true => Some(tst.found_node_ids()),
                false => None,
            };

        if tst.has_tip_labels() && self.draw_labs_tip && self.draw_labs_allowed
        {
            node_labs(
                highlighted_node_ids,
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
                highlighted_node_ids,
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
                highlighted_node_ids,
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

                        let hovered_node_prev = st.hovered_node.clone();
                        st.hovered_node = st.hovered_node(edges);

                        if st.hovered_node != hovered_node_prev {
                            action = Some(Action::request_redraw());
                        }

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
                        } else if self.draw_cursor_line
                            && let Some(pt) = st.cursor_tracking_point
                        {
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
                                // ---------------------------------------------
                                if let Some(position) = crsr.position() {
                                    let new_click = MouseClick::new(
                                        position,
                                        MouseButton::Left,
                                        st.previous_click,
                                    );

                                    st.previous_click = Some(new_click);

                                    if new_click.kind()
                                        == MouseClickKind::Double
                                    {
                                        if let Some((node_id, _)) =
                                            st.hovered_node
                                            && tst.is_valid_potential_subtree_view_node(node_id)
                                        {
                                            action = Some(Action::publish(
                                                TvMsg::SetSubtreeView(node_id),
                                            ));
                                        }
                                    }
                                }
                                // ---------------------------------------------
                                if action.is_none() {
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
                                // ---------------------------------------------
                            }
                            MouseButton::Right => {
                                if st.mouse_is_over_tip_w_resize_area
                                    && self.tip_w_set_by_user.is_some()
                                    && let Some(position) = crsr.position()
                                {
                                    let specification = TvContextMenuSpecification::for_tip_lab_w_resize_area(position);
                                    action = Some(Action::publish(
                                        TvMsg::ContextMenuInteractionBegin(
                                            specification,
                                        ),
                                    ));
                                } else if let Some((node_id, _)) =
                                    &st.hovered_node
                                    && let Some(position) = crsr.position()
                                {
                                    action = Some(Action::publish(
                                        TvMsg::ContextMenuInteractionBegin(
                                            TvContextMenuSpecification::for_node(
                                                *node_id, tst, position,
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
                    repeat: _,
                } => {
                    if let Some((node_id, _)) = &st.hovered_node
                        && let Key::Character(k) = key
                    {
                        let k: &str = k.as_str();
                        match k {
                            "1" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeHighlight((
                                        *node_id,
                                        Clr::BLU_25,
                                    )),
                                ));
                            }
                            "2" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeHighlight((
                                        *node_id,
                                        Clr::CYA_25,
                                    )),
                                ));
                            }
                            "3" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeHighlight((
                                        *node_id,
                                        Clr::GRN_25,
                                    )),
                                ));
                            }
                            "4" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeHighlight((
                                        *node_id,
                                        Clr::MAG_25,
                                    )),
                                ));
                            }
                            "5" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeHighlight((
                                        *node_id,
                                        Clr::YEL_25,
                                    )),
                                ));
                            }
                            "6" => {
                                action = Some(Action::publish(
                                    TvMsg::AddCladeHighlight((
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
                // let timer = timer("bounds");
                draw_bounds(self, st, rndr, bnds, &mut geoms);
                // timer.finish();
            }

            let t = timer("clade_highlights");
            draw_clade_highlights(st, tst, rndr, size, &mut geoms);
            t.finish();

            let t = timer("edges");
            draw_edges(self, st, tst, rndr, size, &mut geoms);
            t.finish();

            if st.mouse_is_over_tip_w_resize_area
                || st.tip_lab_w_is_being_resized
            {
                // let t = timer("tip_lab_w_resize_area");
                draw_tip_lab_w_resize_area(self, st, rndr, bnds, &mut geoms);
                // t.finish();
            }

            // let t = timer("hovered_node");
            draw_hovered_node(self, st, tst, rndr, size, &mut geoms);
            // t.finish();

            // let t = timer("scale_bar");
            draw_scale_bar(self, st, tst, rndr, size, &mut geoms);
            // t.finish();

            // let t = timer("cursor_line");
            draw_cursor_line(self, st, rndr, size, &mut geoms);
            // t.finish();

            let t = timer("labs_tip");
            draw_labs_tip(self, st, tst, rndr, size, &mut geoms);
            t.finish();

            let t = timer("labs_int");
            draw_labs_int(self, st, tst, rndr, size, &mut geoms);
            t.finish();

            let t = timer("labs_brnch");
            draw_labs_brnch(self, st, tst, rndr, size, &mut geoms);
            t.finish();

            let t = timer("selected_nodes");
            draw_selected_nodes(st, tst, rndr, size, &mut geoms);
            t.finish();

            let t = timer("filtered_nodes");
            draw_filtered_nodes(self, st, tst, rndr, size, &mut geoms);
            t.finish();

            if self.draw_debug {
                // let t = timer("palette");
                draw_palette(self, st, thm, rndr, size, &mut geoms);
                // t.finish();
            }
        }
        geoms
    }
}
