use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

#[derive(Default)]
pub struct TreeView {
    // -------------------------------------------------------------------
    pub(super) tre_states: Vec<TreeState>,
    pub(super) tre_state_idx_sel: Option<usize>,
    // -------------------------------------------------------------------
    pub(super) pane_grid: Option<PgState<TvPane>>,
    pub(super) tre_pane_id: Option<Pane>,
    pub(super) ltt_pane_id: Option<Pane>,
    // -------------------------------------------------------------------
    pub(super) ltt_cnv: PlotCnv,
    pub(super) show_ltt: bool,
    // -------------------------------------------------------------------
    pub(super) show_toolbar: bool,
    pub(super) show_sidebar: bool,
    // -------------------------------------------------------------------
    pub(super) sidebar_pos_sel: SidebarPos,
    // -------------------------------------------------------------------
    pub(super) root_len_idx_min: u16,
    pub(super) root_len_idx_max: u16,
    pub(super) root_len_idx_sel: u16,
    // -------------------------------------------------------------------
    pub(super) tre_cnv_w_idx_min: u16,
    pub(super) tre_cnv_h_idx_min: u16,
    pub(super) tre_cnv_z_idx_min: u16,
    // -------------------------------------------------------------------
    pub(super) tre_cnv_w_idx_max: u16,
    pub(super) tre_cnv_h_idx_max: u16,
    pub(super) tre_cnv_z_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) tre_cnv_w_idx_sel: u16,
    pub(super) tre_cnv_h_idx_sel: u16,
    pub(super) tre_cnv_z_idx_sel: u16,
    // -------------------------------------------------------------------
    pub(super) lab_size_idx_min: u16,
    pub(super) tip_lab_size_idx_sel: u16,
    pub(super) int_lab_size_idx_sel: u16,
    pub(super) brnch_lab_size_idx_sel: u16,
    pub(super) lab_size_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) lab_size_min: Float,
    pub(super) lab_size_max: Float,
    pub(super) lab_size_tip: Float,
    pub(super) lab_offset_tip: Float,
    pub(super) lab_size_int: Float,
    pub(super) lab_offset_int: Float,
    pub(super) lab_size_brnch: Float,
    pub(super) lab_offset_brnch: Float,
    // -------------------------------------------------------------------
    pub(super) opn_angle: Float,
    pub(super) rot_angle: Float,
    pub(super) opn_angle_idx_sel: u16,
    pub(super) rot_angle_idx_sel: u16,
    pub(super) opn_angle_idx_min: u16,
    pub(super) rot_angle_idx_min: u16,
    pub(super) opn_angle_idx_max: u16,
    pub(super) rot_angle_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) tre_sty_opt_sel: TreSty,
    pub(super) node_ord_opt_sel: NodeOrd,
    // -------------------------------------------------------------------
    pub(super) drawing_enabled: bool,
    pub(super) draw_labs_tip: bool,
    pub(super) draw_labs_int: bool,
    pub(super) draw_labs_brnch: bool,
    pub(super) draw_labs_allowed: bool,
    // -------------------------------------------------------------------
    pub(super) tip_labs_vis_max: usize,
    pub(super) node_labs_vis_max: usize,
    // -------------------------------------------------------------------
    pub(super) cache_bnds: Cache,
    pub(super) cache_legend: Cache,
    pub(super) cache_hovered_node: Cache,
    pub(super) cache_cursor_line: Cache,
    // -------------------------------------------------------------------
    pub(super) ltt_scr_id: &'static str,
    pub(super) tre_scr_id: &'static str,
    // -------------------------------------------------------------------
    pub(super) tre_padd_l: Float,
    pub(super) tre_padd_r: Float,
    pub(super) tre_padd_t: Float,
    pub(super) tre_padd_b: Float,
    // -------------------------------------------------------------------
    pub(super) stale_vis_rect: bool,
    pub(super) stale_edge_cache: bool,
    // -------------------------------------------------------------------
    keep_scrl_pos_req: bool,
    // -------------------------------------------------------------------
    ltt_cnv_needs_scrl: bool,
    ltt_cnv_scrolled: bool,
    ltt_cnv_vis_x0: Float,
    ltt_cnv_vis_y0: Float,
    // -------------------------------------------------------------------
    pub(super) crsr_x_rel: Option<Float>,
    // -------------------------------------------------------------------
    pub(super) tre_scr_h: Float,
    pub(super) tre_scr_w: Float,
    tre_cnv_scrolled: bool,
    pub(super) tre_cnv_vis_x0: Float,
    pub(super) tre_cnv_vis_y0: Float,
    pub(super) tre_cnv_vis_x1: Float,
    pub(super) tre_cnv_vis_y1: Float,
    tre_cnv_vis_x_mid: Float,
    tre_cnv_vis_y_mid: Float,
    tre_cnv_vis_x_mid_rel: Float,
    tre_cnv_vis_y_mid_rel: Float,
    // -------------------------------------------------------------------
    pub(super) draw_legend: bool,
    pub(super) draw_cursor_line: bool,
    // -------------------------------------------------------------------
    is_new: bool,
    // -------------------------------------------------------------------
    pub(super) search_string: String,
    pub(super) tip_only_search: bool,
    // -------------------------------------------------------------------
    tre_vs: RectVals<Float>,
    pub(super) root_len_frac: Float,
    // -------------------------------------------------------------------
    text_w_tip: Option<TextWidth<'static>>,
    // -------------------------------------------------------------------
}

impl TreeView {
    pub fn new(sel_sidebar_pos: SidebarPos) -> Self {
        let opn_angle_idx_sel = 359;
        let rot_angle_idx_sel = 360;

        let opn_angle = angle_from_idx(opn_angle_idx_sel);
        let rot_angle = angle_from_idx(rot_angle_idx_sel);

        let lab_size_idx_min = 8;
        let tip_lab_size_idx_sel = TIP_LAB_SIZE;
        let int_lab_size_idx_sel = INT_LAB_SIZE;
        let brnch_lab_size_idx_sel = BRNCH_LAB_SIZE;
        let lab_size_idx_max = 22;

        let lab_size_min = ONE;
        let tip_lab_size = lab_size_min * tip_lab_size_idx_sel as Float;
        let int_lab_size = lab_size_min * int_lab_size_idx_sel as Float;
        let brnch_lab_size = lab_size_min * brnch_lab_size_idx_sel as Float;
        let lab_size_max = lab_size_min * lab_size_idx_max as Float;

        Self {
            tre_padd_l: TRE_PADD,
            tre_padd_r: TRE_PADD,
            tre_padd_t: TRE_PADD,
            tre_padd_b: TRE_PADD,
            // -----------------------------------------------------------
            sidebar_pos_sel: sel_sidebar_pos,
            show_ltt: false,
            show_toolbar: true,
            show_sidebar: true,
            draw_labs_tip: false,
            draw_labs_int: false,
            draw_labs_brnch: false,
            draw_legend: true,
            draw_cursor_line: false,
            drawing_enabled: false,
            // -----------------------------------------------------------
            tip_labs_vis_max: 400,
            node_labs_vis_max: 900,
            draw_labs_allowed: false,
            // -----------------------------------------------------------
            tre_sty_opt_sel: TreSty::Fan,
            node_ord_opt_sel: NodeOrd::Unordered,
            // -----------------------------------------------------------
            opn_angle_idx_min: 45,
            opn_angle_idx_max: 359,
            opn_angle_idx_sel,
            opn_angle,
            // -----------------------------------------------------------
            rot_angle_idx_min: 360 - 180,
            rot_angle_idx_max: 360 + 180,
            rot_angle_idx_sel,
            rot_angle,
            // -----------------------------------------------------------
            lab_size_idx_min,
            tip_lab_size_idx_sel,
            int_lab_size_idx_sel,
            brnch_lab_size_idx_sel,
            lab_size_idx_max,
            // -----------------------------------------------------------
            root_len_idx_min: 0,
            root_len_idx_sel: 0,
            root_len_idx_max: 100,
            // -----------------------------------------------------------
            lab_size_min,
            lab_size_max,
            lab_size_tip: tip_lab_size,
            lab_offset_tip: 3e0,
            lab_size_int: int_lab_size,
            lab_offset_int: 3e0,
            lab_size_brnch: brnch_lab_size,
            lab_offset_brnch: -3e0,
            // -----------------------------------------------------------
            tre_cnv_w_idx_min: 1,
            tre_cnv_w_idx_sel: 1,
            tre_cnv_w_idx_max: 22,
            tre_cnv_h_idx_min: 1,
            tre_cnv_h_idx_sel: 1,
            tre_cnv_h_idx_max: 22,
            tre_cnv_z_idx_min: 1,
            tre_cnv_z_idx_sel: 3,
            tre_cnv_z_idx_max: 22,
            // -----------------------------------------------------------
            tre_scr_id: "tre",
            ltt_scr_id: "ltt",
            // -----------------------------------------------------------
            is_new: true,
            stale_vis_rect: true,
            stale_edge_cache: true,
            // -----------------------------------------------------------
            tip_only_search: true,
            // -----------------------------------------------------------
            text_w_tip: Some(text_width(TIP_LAB_SIZE as Float, FNT_NAME_LAB)),
            // -----------------------------------------------------------
            ..Default::default()
        }
    }

    fn update_tre_padd(&mut self) {
        if self.tre_cnv_w() <= self.tre_scr_w {
            self.tre_padd_b = TRE_PADD;
        } else {
            self.tre_padd_b = TRE_PADD + SCRLBAR_W;
        }

        if self.tre_cnv_h() <= self.tre_scr_h {
            self.tre_padd_r = TRE_PADD;
        } else {
            self.tre_padd_r = TRE_PADD + SCRLBAR_W;
        }

        if self.tre_sty_opt_sel == TreSty::PhyGrm {
            self.ltt_cnv.plt_padd_b = self.tre_padd_b;
            self.ltt_cnv.plt_padd_r = self.tre_padd_r;
        } else {
            self.ltt_cnv.plt_padd_b = TRE_PADD;
            self.ltt_cnv.plt_padd_r = TRE_PADD;
        }
    }

    fn update_rel_scrl_pos(&mut self) {
        self.tre_cnv_vis_x_mid_rel = self.tre_cnv_vis_x_mid / self.tre_cnv_w();
        self.tre_cnv_vis_y_mid_rel = self.tre_cnv_vis_y_mid / self.tre_cnv_h();
        match self.tre_sty_opt_sel {
            TreSty::PhyGrm => {
                if self.tre_cnv_vis_x1 <= self.tre_scr_w {
                    self.tre_cnv_vis_x_mid_rel = 0.0;
                }
                if self.tre_cnv_vis_y1 <= self.tre_scr_h {
                    self.tre_cnv_vis_y_mid_rel = 0.0;
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_vis_x1 <= self.tre_scr_w {
                    self.tre_cnv_vis_x_mid_rel = 0.5;
                }
                if self.tre_cnv_vis_y1 <= self.tre_scr_h {
                    self.tre_cnv_vis_y_mid_rel = 0.5;
                }
            }
        }
    }

    fn keep_scroll_pos_w(&mut self) -> Option<Task<TvMsg>> {
        let w = self.tre_cnv_w();
        let mut x = w * self.tre_cnv_vis_x_mid_rel;
        if self.tre_cnv_vis_x1 == w {
            x += self.tre_scr_w / TWO
        } else if self.tre_cnv_vis_x0 == ZRO {
            x -= self.tre_scr_w / TWO
        }
        self.scroll_tre_cnv(x, self.tre_cnv_vis_y_mid)
    }

    fn keep_scroll_pos_h(&self) -> Option<Task<TvMsg>> {
        let h = self.tre_cnv_h();
        let mut y = h * self.tre_cnv_vis_y_mid_rel;
        if self.tre_cnv_vis_y1 == h {
            y += self.tre_scr_h / TWO
        } else if self.tre_cnv_vis_y0 == ZRO {
            y -= self.tre_scr_h / TWO
        }
        self.scroll_tre_cnv(self.tre_cnv_vis_x_mid, y)
    }

    fn keep_scroll_pos_z(&self) -> Option<Task<TvMsg>> {
        self.scroll_tre_cnv(
            self.tre_cnv_w() * self.tre_cnv_vis_x_mid_rel,
            self.tre_cnv_h() * self.tre_cnv_vis_y_mid_rel,
        )
    }

    fn scroll_tre_cnv(&self, x: Float, y: Float) -> Option<Task<TvMsg>> {
        let x = (x - self.tre_scr_w / TWO).max(ZRO);
        let y = (y - self.tre_scr_h / TWO).max(ZRO);
        let task1 = scroll_to(self.tre_scr_id, AbsoluteOffset { x, y });
        if !self.ltt_cnv_needs_scrl {
            Some(task1)
        } else {
            let task2 = scroll_to(self.ltt_scr_id, AbsoluteOffset { x, y: self.ltt_cnv_vis_y0 });
            Some(Task::batch([task1, task2]))
        }
    }

    fn scroll_cnv_to_x(&self, receiver_id: &'static str, x: Float) -> Option<Task<TvMsg>> {
        let y = match receiver_id {
            id if id == self.ltt_scr_id => self.ltt_cnv_vis_y0,
            id if id == self.tre_scr_id => self.tre_cnv_vis_y0,
            _ => ZRO,
        };
        Some(scroll_to(receiver_id, AbsoluteOffset { x, y }))
    }

    fn scroll_to_edge(&mut self, edge: &Edge) -> Option<Task<TvMsg>> {
        self.update_tre_vs(); // This should not be done every time "scroll_to_edge" is called.
        let mut root_len: Float = 0e0;
        let node_pt: Point = match self.tre_sty_opt_sel {
            TreSty::PhyGrm => {
                if self.is_rooted() {
                    root_len = self.tre_vs.w * self.root_len_frac;
                }
                node_data_cart(self.tre_vs.w - root_len, self.tre_vs.h, edge).points.p1
                    + Vector { x: self.tre_vs.trans.x + root_len, y: self.tre_vs.trans.y }
            }
            TreSty::Fan => {
                if self.is_rooted() {
                    root_len = self.tre_vs.radius_min * self.root_len_frac;
                }
                node_data_rad(self.opn_angle, self.tre_vs.radius_min, root_len, edge).points.p1
                    + self.tre_vs.cntr
            }
        };

        // -----------------------------------------------------------------------------------------
        self.tre_cnv_vis_x0 =
            (node_pt.x - self.tre_scr_w / TWO).min(self.tre_cnv_w() - self.tre_scr_w).max(ZRO);
        self.tre_cnv_vis_x1 =
            (node_pt.x + self.tre_scr_w / TWO).max(self.tre_scr_w).min(self.tre_cnv_w());

        self.tre_cnv_vis_y0 =
            (node_pt.y - self.tre_scr_h / TWO).min(self.tre_cnv_h() - self.tre_scr_h).max(ZRO);
        self.tre_cnv_vis_y1 =
            (node_pt.y + self.tre_scr_h / TWO).max(self.tre_scr_h).min(self.tre_cnv_h());

        self.stale_vis_rect = true;
        // -----------------------------------------------------------------------------------------

        self.scroll_tre_cnv(node_pt.x, node_pt.y)
    }

    fn update_tre_vs(&mut self) {
        self.tre_vs = RectVals::wh(self.tre_cnv_w(), self.tre_cnv_h())
            .padded(self.tre_padd_l, self.tre_padd_r, self.tre_padd_t, self.tre_padd_b);

        let mut tip_w: Float = ZRO;
        let mut edges_tip_tallest: Vec<Edge> = vec![];

        if let Some(ett) = self.edges_tip_tallest() {
            edges_tip_tallest = ett.to_vec()
        }

        if self.draw_labs_tip && self.has_tip_labs() && self.draw_labs_allowed {
            tip_w = cnv_tree::calc_tip_w(
                self.tre_sty_opt_sel,
                self.tre_vs,
                &edges_tip_tallest,
                self.lab_offset_tip,
                self.text_w_tip.as_mut().unwrap(),
            );
        }

        self.tre_vs = cnv_tree::calc_tre_vs(
            tip_w, self.tre_vs, self.tre_sty_opt_sel, self.lab_offset_brnch, self.lab_size_max,
        );
    }

    pub(super) fn clear_cache_hovered_node(&self) { self.cache_hovered_node.clear() }
    pub(super) fn clear_cache_cursor_line(&self) { self.cache_cursor_line.clear() }
    pub(super) fn clear_cache_bnds(&self) { self.cache_bnds.clear() }
    pub(super) fn clear_cache_legend(&self) { self.cache_legend.clear() }

    pub(super) fn clear_cache_sel_nodes(&self) {
        if let Some(ts) = self.sel_tre() {
            ts.clear_cache_sel_nodes();
        }
    }

    pub(super) fn clear_cache_filtered_nodes(&self) {
        if let Some(ts) = self.sel_tre() {
            ts.clear_cache_filtered_nodes();
        }
    }

    pub(super) fn clear_cache_edge(&self) {
        if let Some(ts) = self.sel_tre() {
            ts.clear_cache_edge()
        }
    }

    pub(super) fn clear_caches_lab(&self, tip: bool, int: bool, brnch: bool) {
        if let Some(ts) = self.sel_tre() {
            if tip {
                ts.clear_cache_lab_tip()
            };
            if int {
                ts.clear_cache_lab_int()
            };
            if brnch {
                ts.clear_cache_lab_brnch()
            };
        }
    }

    pub(super) fn clear_cache_all_tre(&self) {
        if let Some(ts) = self.sel_tre() {
            ts.clear_caches_all()
        }
    }

    pub(super) fn clear_caches_all(&self) {
        self.clear_cache_bnds();
        self.clear_cache_legend();
        self.clear_cache_hovered_node();
        self.clear_cache_all_tre();
    }

    fn show_hide_ltt(&mut self) {
        if let Some(pane_grid) = &mut self.pane_grid {
            if let Some(ltt_pane_id) = self.ltt_pane_id {
                if !self.show_ltt {
                    pane_grid.close(ltt_pane_id);
                    self.ltt_pane_id = None;
                }
            } else if self.show_ltt
                && let Some(tre_pane_id) = self.tre_pane_id
                && let Some((ltt_pane_id, _split)) =
                    pane_grid.split(Axis::Horizontal, tre_pane_id, TvPane::LttPlot)
            {
                self.ltt_pane_id = Some(ltt_pane_id);
            }
        }
    }

    fn update_draw_labs_allowed(&mut self) {
        self.draw_labs_allowed = match self.tre_sty_opt_sel {
            TreSty::PhyGrm => {
                let node_size = self.tre_cnv_h() / self.tip_count() as Float;
                let tip_labs_vis = (self.tre_scr_h / node_size).floor() as usize;
                tip_labs_vis <= self.tip_labs_vis_max
            }
            TreSty::Fan => self.tip_count() <= self.tip_labs_vis_max * 2,
        };
    }

    pub fn update(&mut self, tv_msg: TvMsg) -> Task<TvMsg> {
        let mut task: Option<Task<TvMsg>> = None;
        self.stale_vis_rect = false;

        match tv_msg {
            TvMsg::TreCnvScrolledOrResized(vp) => {
                if self.stale_edge_cache {
                    self.stale_edge_cache = false;
                    self.clear_cache_edge();
                }

                let tre_scr_w = vp.bounds().width;
                let tre_scr_h = vp.bounds().height;

                // RESIZED -----------------------------------------------------
                // let mut resized = false;
                if tre_scr_w != self.tre_scr_w || tre_scr_h != self.tre_scr_h {
                    self.tre_scr_w = tre_scr_w;
                    self.tre_scr_h = tre_scr_h;
                    // resized = true;
                } // -----------------------------------------------------------

                if self.keep_scrl_pos_req {
                    task = self.scroll_tre_cnv(self.tre_cnv_vis_x_mid, self.tre_cnv_vis_y_mid);
                    self.tre_cnv_scrolled = true;
                    self.keep_scrl_pos_req = false;
                }

                self.tre_cnv_vis_x0 = vp.absolute_offset().x;
                self.tre_cnv_vis_y0 = vp.absolute_offset().y;
                self.tre_cnv_vis_x1 = self.tre_cnv_vis_x0 + tre_scr_w;
                self.tre_cnv_vis_y1 = self.tre_cnv_vis_y0 + tre_scr_h;
                self.tre_cnv_vis_x_mid = self.tre_cnv_vis_x0.midpoint(self.tre_cnv_vis_x1);
                self.tre_cnv_vis_y_mid = self.tre_cnv_vis_y0.midpoint(self.tre_cnv_vis_y1);

                self.update_rel_scrl_pos();

                if task.is_none() && self.tre_sty_opt_sel == TreSty::PhyGrm {
                    if self.tre_cnv_scrolled && self.tre_cnv_vis_x0 != self.ltt_cnv_vis_x0 {
                        self.ltt_cnv_scrolled = false;
                        task = self.scroll_cnv_to_x(self.ltt_scr_id, self.tre_cnv_vis_x0);
                    } else {
                        self.tre_cnv_scrolled = true;
                    }
                }

                // if resized {
                // RESIZED...
                // } else {
                self.update_draw_labs_allowed();
                self.stale_vis_rect = true;
                self.clear_caches_lab(true, true, true);
                self.clear_cache_sel_nodes();
                self.clear_cache_filtered_nodes();
                // }
            }

            TvMsg::LttCnvScrolledOrResized(vp) => {
                if self.ltt_cnv_vis_x0 != vp.absolute_offset().x {
                    self.ltt_cnv_vis_x0 = vp.absolute_offset().x;
                    self.ltt_cnv_vis_y0 = vp.absolute_offset().y;
                    if self.tre_sty_opt_sel == TreSty::PhyGrm {
                        if self.ltt_cnv_scrolled && self.tre_cnv_vis_x0 != self.ltt_cnv_vis_x0 {
                            self.tre_cnv_scrolled = false;
                            task = self.scroll_cnv_to_x(self.tre_scr_id, self.ltt_cnv_vis_x0);
                        } else {
                            self.ltt_cnv_scrolled = true;
                        }
                    }
                }
                self.stale_vis_rect = true;
            }

            TvMsg::LttVisChanged(show_ltt) => {
                self.show_ltt = show_ltt;
                self.show_hide_ltt();
                let x = (self.tre_cnv_vis_x_mid - self.tre_scr_w / TWO).max(ZRO);
                task =
                    Some(scroll_to(self.ltt_scr_id, AbsoluteOffset { x, y: self.ltt_cnv_vis_y0 }));
                self.stale_vis_rect = true;
            }

            TvMsg::PrevTre => {
                self.prev_tre();
                self.sort();
                self.set_ltt_plot_data();
                self.update_draw_labs_allowed();
            }

            TvMsg::NextTre => {
                self.next_tre();
                self.sort();
                self.set_ltt_plot_data();
                self.update_draw_labs_allowed();
            }

            TvMsg::NodeOrdOptChanged(node_ord_opt) => {
                self.node_ord_opt_sel = node_ord_opt;
                self.sort();
                self.stale_vis_rect = true;

                if let Some(edge) = self.current_found_edge() {
                    let edge = &edge.clone();
                    task = self.scroll_to_edge(edge);
                }
            }

            TvMsg::Unroot => {
                if let Some(ts) = self.sel_tre_mut()
                    && let Some(_yanked_node) = ts.unroot()
                {
                    self.sort();
                }
                self.set_ltt_plot_data();
                self.stale_vis_rect = true;
                self.clear_cache_legend();
                self.update_draw_labs_allowed();

                if let Some(edge) = self.current_found_edge() {
                    let edge = &edge.clone();
                    task = self.scroll_to_edge(edge);
                }
            }

            TvMsg::Root(node_id) => {
                if let Some(ts) = self.sel_tre_mut()
                    && let Some(_node_id_new_root) = ts.root(&node_id)
                {
                    self.sort();
                }
                self.set_ltt_plot_data();
                self.stale_vis_rect = true;
                self.clear_cache_legend();
                self.update_draw_labs_allowed();

                if let Some(edge) = self.current_found_edge() {
                    let edge = &edge.clone();
                    task = self.scroll_to_edge(edge);
                }
            }

            TvMsg::TreesLoaded(trees) => {
                self.drawing_enabled = false;
                self.tre_states = Vec::new();

                let mut i: usize = 1;
                for tre in trees {
                    let mut ts = TreeState::new(i);
                    ts.init(tre);
                    self.tre_states.push(ts);
                    i += 1;
                }

                if !self.tre_states.is_empty() {
                    self.tre_state_idx_sel = Some(0);
                } else {
                    self.tre_state_idx_sel = None;
                }

                if let Some(_tre_pane_id) = &self.tre_pane_id {
                } else {
                    let (pane_grid, tre_pane_id) = PgState::new(TvPane::Tree);
                    self.pane_grid = Some(pane_grid);
                    self.tre_pane_id = Some(tre_pane_id)
                }
                self.sort();
                self.set_ltt_plot_data();

                if self.is_new {
                    self.show_hide_ltt();
                    self.ltt_cnv.draw_cursor_line = self.draw_cursor_line;
                    self.update_tre_padd();
                    self.is_new = false;
                }

                self.drawing_enabled = true;
                self.stale_vis_rect = true;
                self.update_draw_labs_allowed();
                self.clear_caches_all();
            }

            TvMsg::CnvWidthSelChanged(idx) => {
                self.tre_cnv_w_idx_sel = idx;
                self.ltt_cnv_needs_scrl = true;
                self.update_tre_padd();
                task = self.keep_scroll_pos_w();
                self.stale_vis_rect = true;
                self.stale_edge_cache = true;
                // ---------------------------------------------------------------------------------
                self.tre_cnv_vis_x_mid = self.tre_cnv_w() * self.tre_cnv_vis_x_mid_rel;
                self.tre_cnv_vis_x0 = (self.tre_cnv_vis_x_mid - self.tre_scr_w)
                    .min(self.tre_cnv_w() - self.tre_scr_w)
                    .max(ZRO);
                self.tre_cnv_vis_x1 = (self.tre_cnv_vis_x_mid + self.tre_scr_w)
                    .max(self.tre_scr_w)
                    .min(self.tre_cnv_w());
                // ---------------------------------------------------------------------------------
            }

            TvMsg::CnvHeightSelChanged(idx) => {
                self.tre_cnv_h_idx_sel = idx;
                self.update_tre_padd();
                task = self.keep_scroll_pos_h();
                self.stale_vis_rect = true;
                self.stale_edge_cache = true;
                // ---------------------------------------------------------------------------------
                self.tre_cnv_vis_y_mid = self.tre_cnv_h() * self.tre_cnv_vis_y_mid_rel;
                self.tre_cnv_vis_y0 = (self.tre_cnv_vis_y_mid - self.tre_scr_h)
                    .min(self.tre_cnv_h() - self.tre_scr_h)
                    .max(ZRO);
                self.tre_cnv_vis_y1 = (self.tre_cnv_vis_y_mid + self.tre_scr_h)
                    .max(self.tre_scr_h)
                    .min(self.tre_cnv_h());
                // ---------------------------------------------------------------------------------
            }

            TvMsg::CnvZoomSelChanged(idx) => {
                self.tre_cnv_z_idx_sel = idx;
                self.update_tre_padd();
                task = self.keep_scroll_pos_z();
                self.stale_vis_rect = true;
                self.stale_edge_cache = true;
                // ---------------------------------------------------------------------------------
                self.tre_cnv_vis_x_mid = self.tre_cnv_w() * self.tre_cnv_vis_x_mid_rel;
                self.tre_cnv_vis_x0 = (self.tre_cnv_vis_x_mid - self.tre_scr_w)
                    .min(self.tre_cnv_w() - self.tre_scr_w)
                    .max(ZRO);
                self.tre_cnv_vis_x1 = (self.tre_cnv_vis_x_mid + self.tre_scr_w)
                    .max(self.tre_scr_w)
                    .min(self.tre_cnv_w());
                // ---------------------------------------------------------------------------------
                self.tre_cnv_vis_y_mid = self.tre_cnv_h() * self.tre_cnv_vis_y_mid_rel;
                self.tre_cnv_vis_y0 = (self.tre_cnv_vis_y_mid - self.tre_scr_h)
                    .min(self.tre_cnv_h() - self.tre_scr_h)
                    .max(ZRO);
                self.tre_cnv_vis_y1 = (self.tre_cnv_vis_y_mid + self.tre_scr_h)
                    .max(self.tre_scr_h)
                    .min(self.tre_cnv_h());
                // ---------------------------------------------------------------------------------
            }

            TvMsg::TreStyOptChanged(tre_sty_opt) => {
                self.tre_sty_opt_sel = tre_sty_opt;
                self.update_rel_scrl_pos();
                self.update_tre_padd();
                self.stale_vis_rect = true;
                self.clear_caches_all();
                self.update_draw_labs_allowed();
            }

            TvMsg::TipLabVisChanged(state) => {
                self.draw_labs_tip = state;
                self.stale_vis_rect = true;
                self.clear_caches_all();
            }

            TvMsg::TipLabSizeChanged(idx) => {
                self.tip_lab_size_idx_sel = idx;
                self.lab_size_tip = self.lab_size_min * idx as Float;
                // -------------------------------------------------------------
                if let Some(text_w_tip) = &mut self.text_w_tip
                    && text_w_tip.font_size() != self.lab_size_tip
                {
                    text_w_tip.set_font_size(self.lab_size_tip);
                };
                // -------------------------------------------------------------
                self.stale_vis_rect = true;
                self.clear_caches_all();
            }

            TvMsg::IntLabVisChanged(state) => {
                self.draw_labs_int = state;
                self.clear_caches_lab(false, true, false);
            }

            TvMsg::IntLabSizeChanged(idx) => {
                self.int_lab_size_idx_sel = idx;
                self.lab_size_int = self.lab_size_min * idx as Float;
                self.clear_caches_lab(false, true, false);
            }

            TvMsg::BrnchLabVisChanged(state) => {
                self.draw_labs_brnch = state;
                self.clear_caches_lab(false, false, true);
            }

            TvMsg::BrnchLabSizeChanged(idx) => {
                self.brnch_lab_size_idx_sel = idx;
                self.lab_size_brnch = self.lab_size_min * idx as Float;
                self.clear_caches_lab(false, false, true);
                self.clear_cache_legend();
            }

            TvMsg::LegendVisChanged(state) => {
                self.draw_legend = state;
            }

            TvMsg::SelectDeselectNode(node_id) => {
                if let Some(tre) = self.sel_tre_mut() {
                    tre.select_deselect_node(&node_id);
                }
            }

            TvMsg::OpnAngleChanged(idx) => {
                self.opn_angle_idx_sel = idx;
                self.opn_angle = angle_from_idx(idx);
                self.clear_caches_all();
                self.stale_vis_rect = true;
            }

            TvMsg::RotAngleChanged(idx) => {
                self.rot_angle_idx_sel = idx;
                self.rot_angle = angle_from_idx(idx);
                self.clear_caches_all();
                self.stale_vis_rect = true;
            }

            TvMsg::RootLenSelChanged(idx) => {
                self.root_len_idx_sel = idx;
                self.root_len_frac = self.root_len_idx_sel as Float / 2e2;
                self.clear_caches_all();
                self.stale_vis_rect = true;
            }

            TvMsg::PaneResized(ResizeEvent { split, ratio }) => {
                if let Some(pane_grid) = &mut self.pane_grid {
                    pane_grid.resize(split, ratio);
                    // -----------------------------------------------------------------------------
                    self.tre_cnv_vis_y_mid = self.tre_cnv_h() * self.tre_cnv_vis_y_mid_rel;
                    self.tre_cnv_vis_y0 = (self.tre_cnv_vis_y_mid - self.tre_scr_h)
                        .min(self.tre_cnv_h() - self.tre_scr_h)
                        .max(ZRO);
                    self.tre_cnv_vis_y1 = (self.tre_cnv_vis_y_mid + self.tre_scr_h)
                        .max(self.tre_scr_h)
                        .min(self.tre_cnv_h());
                    // -----------------------------------------------------------------------------
                    self.stale_vis_rect = true;
                    self.update_draw_labs_allowed();
                    self.clear_caches_lab(true, true, true);
                    self.clear_cache_sel_nodes();
                    self.clear_cache_filtered_nodes();
                }
            }

            TvMsg::SetSidebarPos(sidebar_pos) => {
                self.sidebar_pos_sel = sidebar_pos;
                self.keep_scrl_pos_req = true;
            }

            TvMsg::CursorLineVisChanged(state) => {
                self.draw_cursor_line = state;
                self.ltt_cnv.draw_cursor_line = state;
                self.crsr_x_rel = None;
                self.clear_cache_cursor_line();
                self.ltt_cnv.crsr_x_rel = None;
                self.ltt_cnv.clear_cache_cursor_line();
            }

            TvMsg::CursorOnTreCnv { x } => {
                self.crsr_x_rel = None;
                self.ltt_cnv.crsr_x_rel = x;
            }

            TvMsg::CursorOnLttCnv { x } => {
                self.crsr_x_rel = x;
                self.ltt_cnv.crsr_x_rel = None;
            }

            TvMsg::TipOnlySearchSelChanged(state) => {
                self.tip_only_search = state;
                task = Some(Task::done(TvMsg::Search(self.search_string.clone())));
            }

            TvMsg::Search(s) => {
                self.search_string = s.clone();
                let tips_only = self.tip_only_search;
                if let Some(tre) = self.sel_tre_mut() {
                    tre.filter_nodes(s, tips_only);
                }
                if let Some(edge) = self.current_found_edge() {
                    let edge = &edge.clone();
                    task = self.scroll_to_edge(edge);
                }
            }

            TvMsg::PrevResult => {
                let mut edge: Option<&Edge> = None;
                if let Some(tre) = self.sel_tre_mut() {
                    edge = tre.prev_result();
                }
                if let Some(edge) = edge {
                    let edge = &edge.clone();
                    task = self.scroll_to_edge(edge);
                }
            }

            TvMsg::NextResult => {
                let mut edge: Option<&Edge> = None;
                if let Some(tre) = self.sel_tre_mut() {
                    edge = tre.next_result();
                }
                if let Some(edge) = edge {
                    let edge = &edge.clone();
                    task = self.scroll_to_edge(edge);
                }
            }

            TvMsg::AddFoundToSelection => {
                if let Some(tre) = self.sel_tre_mut() {
                    tre.add_found_to_sel();
                }
            }

            TvMsg::RemFoundFromSelection => {
                if let Some(tre) = self.sel_tre_mut() {
                    tre.rem_found_from_sel();
                }
            }
        }

        match task {
            Some(task) => task,
            None => Task::none(),
        }
    }

    fn set_ltt_plot_data(&mut self) {
        if let Some(ts) = self.sel_tre() {
            let plot_data: Vec<PlotData> =
                ltt(ts.edges_srtd_y(), 100).iter().map(|lttp| lttp.into()).collect();
            self.ltt_cnv.set_plot_data(&plot_data);
        }
    }

    fn tre_cnv_w(&self) -> Float { self.calc_tre_cnv_w(self.tre_scr_w) }
    fn tre_cnv_h(&self) -> Float { self.calc_tre_cnv_h(self.tre_scr_h) }

    pub(super) fn calc_tre_cnv_w(&self, w: Float) -> Float {
        match self.tre_sty_opt_sel {
            TreSty::PhyGrm => {
                if self.tre_cnv_w_idx_sel == 1 {
                    w
                } else {
                    7e2 * self.tre_cnv_w_idx_sel as Float
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_z_idx_sel == 1 {
                    w
                } else {
                    5e2 * self.tre_cnv_z_idx_sel as Float
                }
            }
        }
    }

    pub(super) fn calc_tre_cnv_h(&self, h: Float) -> Float {
        match self.tre_sty_opt_sel {
            TreSty::PhyGrm => {
                if self.tre_cnv_h_idx_sel == 1 {
                    h
                } else {
                    let tip_count = self.tip_count() as Float;
                    if h / tip_count > ONE {
                        5e2 * self.tre_cnv_h_idx_sel as Float
                    } else {
                        tip_count * self.tre_cnv_h_idx_sel as Float
                    }
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_z_idx_sel == 1 {
                    h
                } else {
                    5e2 * self.tre_cnv_z_idx_sel as Float
                }
            }
        }
    }

    pub(super) fn prev_tre_exists(&self) -> bool {
        match self.tre_state_idx_sel {
            Some(idx) => idx > 0,
            None => false,
        }
    }

    pub(super) fn next_tre_exists(&self) -> bool {
        match self.tre_state_idx_sel {
            Some(idx) => idx < self.tre_states.len() - 1,
            None => false,
        }
    }

    pub(super) fn sel_tre(&self) -> Option<&TreeState> {
        if let Some(sel_tre_state_idx) = self.tre_state_idx_sel {
            let sel_tre_state = &self.tre_states[sel_tre_state_idx];
            Some(sel_tre_state)
        } else {
            None
        }
    }

    fn sel_tre_mut(&mut self) -> Option<&mut TreeState> {
        if let Some(sel_tre_state_idx) = self.tre_state_idx_sel {
            let sel_tre_state = &mut self.tre_states[sel_tre_state_idx];
            Some(sel_tre_state)
        } else {
            None
        }
    }

    fn sort(&mut self) {
        let node_ord_opt = self.node_ord_opt_sel;
        if let Some(ts) = self.sel_tre_mut() {
            ts.sort(node_ord_opt);
        }
    }

    fn update_sel_tre_st_idx(&mut self, idx: Option<usize>) -> bool {
        if idx != self.tre_state_idx_sel {
            self.tre_state_idx_sel = idx;
            true
        } else {
            false
        }
    }

    fn prev_tre(&mut self) -> bool {
        let prev_idx = match self.tre_state_idx_sel {
            Some(idx) => {
                if idx > 0 {
                    Some(idx - 1)
                } else {
                    Some(idx)
                }
            }
            None => None,
        };
        self.update_sel_tre_st_idx(prev_idx)
    }

    fn next_tre(&mut self) -> bool {
        let next_idx = match self.tre_state_idx_sel {
            Some(idx) => {
                if idx < self.tre_states.len() - 1 {
                    Some(idx + 1)
                } else {
                    Some(idx)
                }
            }
            None => None,
        };
        self.update_sel_tre_st_idx(next_idx)
    }

    fn edges_tip_tallest(&self) -> Option<&Vec<Edge>> {
        if let Some(ts) = self.sel_tre() { Some(ts.edges_tip_tallest()) } else { None }
    }

    fn tip_count(&self) -> usize { if let Some(ts) = self.sel_tre() { ts.tip_count() } else { 1 } }

    fn is_rooted(&self) -> bool {
        if let Some(ts) = self.sel_tre() { ts.is_rooted() } else { false }
    }

    fn has_tip_labs(&self) -> bool {
        if let Some(ts) = self.sel_tre() { ts.has_tip_labs() } else { false }
    }

    fn current_found_edge(&self) -> Option<&Edge> {
        if let Some(tre) = self.sel_tre() { tre.current_found_edge() } else { None }
    }
}

#[derive(Debug, Clone)]
pub enum TvMsg {
    CursorLineVisChanged(bool),
    CnvWidthSelChanged(u16),
    CnvHeightSelChanged(u16),
    CnvZoomSelChanged(u16),
    TipLabVisChanged(bool),
    IntLabVisChanged(bool),
    BrnchLabVisChanged(bool),
    LegendVisChanged(bool),
    TipLabSizeChanged(u16),
    IntLabSizeChanged(u16),
    BrnchLabSizeChanged(u16),
    PrevTre,
    NextTre,
    NodeOrdOptChanged(NodeOrd),
    OpnAngleChanged(u16),
    PaneResized(ResizeEvent),
    Root(NodeId),
    Unroot,
    RotAngleChanged(u16),
    SelectDeselectNode(NodeId),
    SetSidebarPos(SidebarPos),
    TreesLoaded(Vec<Tree>),
    TreStyOptChanged(TreSty),
    RootLenSelChanged(u16),
    LttVisChanged(bool),
    // -------------------------------------------
    TreCnvScrolledOrResized(Viewport),
    LttCnvScrolledOrResized(Viewport),
    // -------------------------------------------
    CursorOnTreCnv { x: Option<Float> },
    CursorOnLttCnv { x: Option<Float> },
    // -------------------------------------------
    Search(String),
    NextResult,
    PrevResult,
    AddFoundToSelection,
    RemFoundFromSelection,
    TipOnlySearchSelChanged(bool),
    // -------------------------------------------
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarPos {
    Left,
    #[default]
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TreSty {
    #[default]
    PhyGrm,
    Fan,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeOrd {
    #[default]
    Unordered,
    Ascending,
    Descending,
}

pub(super) const NODE_ORD_OPTS: [NodeOrd; 3] =
    [NodeOrd::Unordered, NodeOrd::Ascending, NodeOrd::Descending];
pub(super) const TRE_STY_OPTS: [TreSty; 2] = [TreSty::PhyGrm, TreSty::Fan];

impl Display for NodeOrd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            NodeOrd::Unordered => "Unordered",
            NodeOrd::Ascending => "Ascending",
            NodeOrd::Descending => "Descending",
        })
    }
}

impl Display for TreSty {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            TreSty::PhyGrm => "Phylogram",
            TreSty::Fan => "Fan",
        })
    }
}

#[derive(Debug)]
pub(crate) enum TvPane {
    Tree,
    LttPlot,
}

impl Display for TvPane {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{self:?}") }
}

impl From<TvPane> for String {
    fn from(value: TvPane) -> Self { (&value).into() }
}

impl From<&TvPane> for String {
    fn from(value: &TvPane) -> Self {
        match value {
            TvPane::Tree => String::from("Tree"),
            TvPane::LttPlot => String::from("LttPlot"),
        }
    }
}

fn angle_from_idx(idx: u16) -> Float { (idx as Float).to_radians() }
