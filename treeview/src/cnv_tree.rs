mod draw;
mod state;

use draw::*;
use state::St;

use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

#[derive(Debug)]
pub(super) struct TreeCnv {
    pub(super) tre_sty: TreSty,
    // ---------------------------------------------------------------------------------------------
    cache_bnds: Cache,
    cache_legend: Cache,
    cache_hovered_node: Cache,
    cache_cursor_line: Cache,
    cache_palette: Cache,
    // ---------------------------------------------------------------------------------------------
    pub(super) padd_l: Float,
    pub(super) padd_r: Float,
    pub(super) padd_t: Float,
    pub(super) padd_b: Float,
    // ---------------------------------------------------------------------------------------------
    pub(super) crsr_x_rel: Option<Float>,
    // ---------------------------------------------------------------------------------------------
    pub(super) text_w_tip: Option<TextWidth<'static>>,
    // ---------------------------------------------------------------------------------------------
    pub(super) vis_x0: Float,
    pub(super) vis_y0: Float,
    pub(super) vis_x1: Float,
    pub(super) vis_y1: Float,
    pub(super) vis_x_mid: Float,
    pub(super) vis_y_mid: Float,
    pub(super) vis_x_mid_rel: Float,
    pub(super) vis_y_mid_rel: Float,
    // ---------------------------------------------------------------------------------------------
    pub(super) draw_debug: bool,
    pub(super) draw_cursor_line: bool,
    pub(super) draw_labs_allowed: bool,
    pub(super) draw_labs_brnch: bool,
    pub(super) draw_labs_int: bool,
    pub(super) draw_labs_tip: bool,
    pub(super) draw_legend: bool,
    pub(super) drawing_enabled: bool,
    // ---------------------------------------------------------------------------------------------
    pub(super) tip_labs_vis_max: usize,
    pub(super) node_labs_vis_max: usize,
    // ---------------------------------------------------------------------------------------------
    pub(super) lab_size_min: Float,
    pub(super) lab_size_max: Float,
    pub(super) lab_size_tip: Float,
    pub(super) lab_size_int: Float,
    pub(super) lab_size_brnch: Float,
    // ---------------------------------------------------------------------------------------------
    pub(super) lab_offset_tip: Float,
    pub(super) lab_offset_int: Float,
    pub(super) lab_offset_brnch: Float,
    // ---------------------------------------------------------------------------------------------
    pub(super) opn_angle: Float,
    pub(super) rot_angle: Float,
    // ---------------------------------------------------------------------------------------------
    pub(super) tre_vs: RectVals<Float>,
    pub(super) root_len_frac: Float,
    pub(super) stale_tre_rect: bool,
    // ---------------------------------------------------------------------------------------------
    pub(super) tree_state: Option<Rc<TreeState>>,
}

impl TreeCnv {
    pub fn new() -> Self {
        Self {
            tre_sty: TreSty::PhyGrm,
            // -------------------------------------------------------------------------------------
            padd_l: TREE_PADDING,
            padd_r: TREE_PADDING,
            padd_t: TREE_PADDING,
            padd_b: TREE_PADDING,
            // -------------------------------------------------------------------------------------
            draw_debug: false,
            drawing_enabled: false,
            draw_labs_allowed: false,
            draw_labs_tip: false,
            draw_labs_int: false,
            draw_labs_brnch: false,
            draw_legend: true,
            draw_cursor_line: true,
            // -------------------------------------------------------------------------------------
            tip_labs_vis_max: 400,
            node_labs_vis_max: 900,
            // -------------------------------------------------------------------------------------
            opn_angle: ZERO,
            rot_angle: ZERO,
            // -------------------------------------------------------------------------------------
            lab_size_min: SF,
            lab_size_max: SF,
            lab_size_tip: SF,
            lab_size_int: SF,
            lab_size_brnch: SF,
            // -------------------------------------------------------------------------------------
            lab_offset_tip: ZERO,
            lab_offset_int: ZERO,
            lab_offset_brnch: ZERO,
            // -------------------------------------------------------------------------------------
            text_w_tip: Some(text_width(SF * TIP_LAB_SIZE_IDX as Float, FNT_NAME_LAB)),
            // -------------------------------------------------------------------------------------
            cache_bnds: Default::default(),
            cache_legend: Default::default(),
            cache_hovered_node: Default::default(),
            cache_cursor_line: Default::default(),
            cache_palette: Default::default(),
            // -------------------------------------------------------------------------------------
            crsr_x_rel: None,
            // -------------------------------------------------------------------------------------
            vis_x0: ZERO,
            vis_y0: ZERO,
            vis_x1: ZERO,
            vis_y1: ZERO,
            vis_x_mid: ZERO,
            vis_y_mid: ZERO,
            vis_x_mid_rel: ZERO,
            vis_y_mid_rel: ZERO,
            // -------------------------------------------------------------------------------------
            tre_vs: RectVals::default(),
            root_len_frac: ZERO,
            stale_tre_rect: false,
            // -------------------------------------------------------------------------------------
            tree_state: None,
            // -------------------------------------------------------------------------------------
        }
    }

    pub(super) fn clear_cache_bnds(&self) { self.cache_bnds.clear() }
    pub(super) fn clear_cache_cache_palette(&self) { self.cache_palette.clear() }
    pub(super) fn clear_cache_cursor_line(&self) { self.cache_cursor_line.clear() }
    pub(super) fn clear_cache_hovered_node(&self) { self.cache_hovered_node.clear() }
    pub(super) fn clear_cache_legend(&self) { self.cache_legend.clear() }

    pub(super) fn clear_caches_all(&self) {
        self.clear_cache_bnds();
        self.clear_cache_cache_palette();
        self.clear_cache_cursor_line();
        self.clear_cache_hovered_node();
        self.clear_cache_legend();
    }

    pub(super) fn calc_tre_vs(&self, tip_w: Float, tre_vs: RectVals<Float>) -> RectVals<Float> {
        match self.tre_sty {
            TreSty::PhyGrm => {
                let mut offset_due_to_brnch_lab = ZERO;
                let mut offset_due_to_tip_lab = ZERO;

                if self.draw_labs_allowed && self.draw_labs_tip {
                    offset_due_to_tip_lab = self.lab_size_tip / TWO;
                }

                if self.draw_labs_allowed && self.draw_labs_brnch {
                    offset_due_to_brnch_lab = self.lab_size_brnch + self.lab_offset_brnch.abs();
                }

                let top = (offset_due_to_tip_lab).max(offset_due_to_brnch_lab);
                let bottom = offset_due_to_tip_lab;

                tre_vs.padded(ZERO, tip_w, top, bottom)
            }
            TreSty::Fan => tre_vs.padded(tip_w, tip_w, tip_w, tip_w),
        }
    }
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
        let edges = tst.edges_srtd_y();
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
            st.vis_rect = st.vis_vs.into();
            st.stale_vis_rect = true;
        } // ---------------------------------------------------------------------------------------
        if bnds != st.bnds || st.stale_vis_rect || st.is_new || self.stale_tre_rect {
            st.cnv_vs = RectVals::cnv(bnds);
            st.cnv_rect = st.cnv_vs.into();
            st.tre_vs = st.cnv_vs.padded(self.padd_l, self.padd_r, self.padd_t, self.padd_b);

            let mut tip_w: Float = ZERO;
            if self.draw_labs_tip && self.draw_labs_allowed {
                tip_w = calc_tip_w(
                    tre_sty,
                    st.tre_vs,
                    tst.edges_tip_tallest(),
                    self.lab_offset_tip,
                    st.text_w_tip.as_mut()?,
                );
            }

            st.tre_vs = self.calc_tre_vs(tip_w, st.tre_vs);
            st.tre_rect = st.tre_vs.into();
            st.bnds = bnds;
            // -------------------------------------------------------------------------------------
            self.clear_cache_bnds();
            tst.clear_cache_lab_tip();
            tst.clear_cache_lab_int();
            tst.clear_cache_lab_brnch();
        } // ---------------------------------------------------------------------------------------
        st.root_len = ZERO;
        match tre_sty {
            TreSty::PhyGrm => {
                if tst.is_rooted() {
                    st.root_len = st.tre_vs.w * self.root_len_frac;
                }
                st.rotation = ZERO;
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
        } // ---------------------------------------------------------------------------------------
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
        st.vis_node_idxs
            .par_iter()
            .map(|&idx| match tre_sty {
                TreSty::PhyGrm => {
                    node_data_cart(st.tre_vs.w - st.root_len, st.tre_vs.h, &edges[idx]).into()
                }
                TreSty::Fan => {
                    node_data_rad(opn_angle, ZERO, st.tre_vs.radius_min, st.root_len, &edges[idx])
                        .into()
                }
            })
            .collect_into_vec(&mut st.vis_nodes);
        // -----------------------------------------------------------------------------------------
        tst.found_edge_idxs()
            .par_iter()
            .map(|&idx| match tre_sty {
                TreSty::PhyGrm => {
                    node_data_cart(st.tre_vs.w - st.root_len, st.tre_vs.h, &edges[idx]).into()
                }
                TreSty::Fan => {
                    node_data_rad(opn_angle, ZERO, st.tre_vs.radius_min, st.root_len, &edges[idx])
                        .into()
                }
            })
            .collect_into_vec(&mut st.filtered_nodes);
        // -----------------------------------------------------------------------------------------
        tst.sel_edge_idxs()
            .par_iter()
            .map(|&idx| match tre_sty {
                TreSty::PhyGrm => {
                    node_data_cart(st.tre_vs.w - st.root_len, st.tre_vs.h, &edges[idx]).into()
                }
                TreSty::Fan => {
                    node_data_rad(opn_angle, ZERO, st.tre_vs.radius_min, st.root_len, &edges[idx])
                        .into()
                }
            })
            .collect_into_vec(&mut st.selected_nodes);
        // -----------------------------------------------------------------------------------------
        st.labs_tip.clear();
        st.labs_int.clear();
        st.labs_brnch.clear();
        // -----------------------------------------------------------------------------------------
        if self.draw_labs_tip && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_tip,
                true,
                false,
                st.text_w_tip.as_mut()?,
                &mut st.labs_tip,
            );
        } // ---------------------------------------------------------------------------------------
        if self.draw_labs_int && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_int,
                false,
                false,
                st.text_w_int.as_mut()?,
                &mut st.labs_int,
            );
        } // ---------------------------------------------------------------------------------------
        if self.draw_labs_brnch && self.draw_labs_allowed {
            node_labs(
                &st.vis_nodes,
                edges,
                self.lab_size_brnch,
                false,
                true,
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
                                TreSty::PhyGrm => pt.x / (st.tre_vs.w - st.root_len),
                                TreSty::Fan => {
                                    (pt.distance(ORIGIN) - st.root_len)
                                        / (st.tre_vs.radius_min - st.root_len)
                                }
                            };

                            if (ZERO - EPSILON..=ONE + EPSILON).contains(&crsr_x_rel) {
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
                }
            }
            Event::Keyboard(_e) => {}
            Event::Window(WindowEvent::RedrawRequested(_)) => action = None,
            _ => {}
        }
        // -----------------------------------------------------------------------------------------
        if let Some(crsr_x_rel) = self.crsr_x_rel {
            match tre_sty {
                TreSty::PhyGrm => {
                    st.cursor_tracking_point =
                        Some(Point { x: (crsr_x_rel * (st.tre_vs.w - st.root_len)), y: ZERO })
                }
                TreSty::Fan => {
                    st.cursor_tracking_point = Some(Point {
                        x: st.root_len + crsr_x_rel * (st.tre_vs.radius_min - st.root_len),
                        y: ZERO,
                    })
                }
            }
        }
        // -----------------------------------------------------------------------------------------
        if self.stale_tre_rect {
            action = Some(Action::publish(TvMsg::TreeRectNoLongerStale));
        }
        // -----------------------------------------------------------------------------------------
        st.is_new = false;
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

pub(super) fn calc_tip_w(
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
    let mut max_w: Float = ZERO;
    let mut max_offset: Float = ZERO;
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
