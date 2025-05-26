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
    pub(super) ltt_plot: PlotCnv,
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
    pub(super) tre_style_opt_sel: TreSty,
    pub(super) node_ord_opt_sel: NodeOrd,
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
    // -------------------------------------------------------------------
    pub(super) ltt_scr_id: &'static str,
    pub(super) tre_scr_id: &'static str,
    pub(super) tre_padd: Float,
    // -------------------------------------------------------------------
    pub(super) stale_tre_dims: bool,
    pub(super) stale_vis_rect: bool,
    // -------------------------------------------------------------------
    keep_scrl_pos_req: bool,
    // -------------------------------------------------------------------
    // ltt_cnv_w: Float,
    // ltt_cnv_h: Float,
    ltt_cnv_needs_scrl: bool,
    ltt_cnv_scrolled: bool,
    ltt_cnv_vis_x0: Float,
    ltt_cnv_vis_y0: Float,
    // -------------------------------------------------------------------
    tre_scr_h: Float,
    tre_scr_w: Float,
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
}

#[derive(Debug, Clone)]
pub enum TvMsg {
    CnvWidthSelChanged(u16),
    CnvHeightSelChanged(u16),
    CnvZoomSelChanged(u16),
    TipLabVisChanged(bool),
    IntLabVisChanged(bool),
    BrnchLabVisChanged(bool),
    TipLabSizeChanged(u16),
    IntLabSizeChanged(u16),
    BrnchLabSizeChanged(u16),
    LttpVisChanged(bool),
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
    TreCnvScrolled(Viewport),
    LttCnvScrolled(Viewport),
    RefreshedVisRect(RectVals<Float>),
    RefreshedTreeDims(RectVals<Float>),
}

impl TreeView {
    pub fn new(sel_sidebar_pos: SidebarPos) -> Self {
        let opn_angle_idx_sel = 359;
        let rot_angle_idx_sel = 360;

        let opn_angle = angle_from_idx(opn_angle_idx_sel);
        let rot_angle = angle_from_idx(rot_angle_idx_sel);

        let lab_size_idx_min = 1;
        let tip_lab_size_idx_sel = TIP_LAB_SIZE;
        let int_lab_size_idx_sel = INT_LAB_SIZE;
        let brnch_lab_size_idx_sel = BRNCH_LAB_SIZE;
        let lab_size_idx_max = 24;

        let lab_size_min = lab_size_idx_min as Float;
        let tip_lab_size = lab_size_min * tip_lab_size_idx_sel as Float;
        let int_lab_size = lab_size_min * int_lab_size_idx_sel as Float;
        let brnch_lab_size = lab_size_min * brnch_lab_size_idx_sel as Float;
        let lab_size_max = lab_size_min * lab_size_idx_max as Float;

        Self {
            tre_padd: 1e1,
            sidebar_pos_sel: sel_sidebar_pos,
            show_toolbar: true,
            show_sidebar: true,
            draw_labs_tip: false,
            draw_labs_int: false,
            draw_labs_brnch: false,
            drawing_enabled: true,
            // -----------------------------------------------------------
            tip_labs_vis_max: 200,
            node_labs_vis_max: 400,
            draw_labs_allowed: true,
            // -----------------------------------------------------------
            tre_style_opt_sel: TreSty::Fan,
            node_ord_opt_sel: NodeOrd::Ascending,
            // -----------------------------------------------------------
            opn_angle_idx_min: 0,
            opn_angle_idx_max: 360,
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
            root_len_idx_sel: 10,
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
            tre_cnv_w_idx_max: 24,
            tre_cnv_h_idx_min: 1,
            tre_cnv_h_idx_sel: 1,
            tre_cnv_h_idx_max: 24,
            tre_cnv_z_idx_min: 1,
            tre_cnv_z_idx_sel: 1,
            tre_cnv_z_idx_max: 24,
            // -----------------------------------------------------------
            tre_scr_id: "tre",
            ltt_scr_id: "ltt",
            // -----------------------------------------------------------
            stale_tre_dims: true,
            stale_vis_rect: true,
            tre_cnv_vis_x_mid_rel: 0.5,
            tre_cnv_vis_y_mid_rel: 0.5,
            // -----------------------------------------------------------
            ..Default::default()
        }
    }

    pub fn update(&mut self, tv_msg: TvMsg) -> Task<TvMsg> {
        let mut task: Option<Task<TvMsg>> = None;
        // match tv_msg {
        //     TvMsg::TreesLoaded(_) => println!("TreesLoaded"),
        //     TvMsg::TreCnvScrolled(vp) => println!(
        //         "TreCnvScrolled: {:7.2} {:7.2}",
        //         vp.content_bounds().center_x(),
        //         vp.content_bounds().center_y()
        //     ),
        //     TvMsg::RefreshedTreeDims(_) => {}
        //     TvMsg::RefreshedVisRect(_) => {}
        //     _ => println!("{tv_msg:?}"),
        // }
        match tv_msg {
            TvMsg::LttpVisChanged(show_lttp) => {
                self.show_ltt = show_lttp;
                self.show_hide_lttp();
                self.tre_cnv_scrolled = true;
                self.ltt_cnv_scrolled = false;
                self.keep_scrl_pos_req = true;
                self.ltt_cnv_needs_scrl = true;
            }

            TvMsg::TreCnvScrolled(vp) => {
                self.tre_scr_w = vp.bounds().width;
                self.tre_scr_h = vp.bounds().height;

                if self.keep_scrl_pos_req {
                    task = self.keep_scroll_pos_wh();
                    self.tre_cnv_scrolled = true;
                    self.keep_scrl_pos_req = false;
                    self.ltt_cnv_needs_scrl = false;
                }

                let x0 = vp.absolute_offset().x;
                let y0 = vp.absolute_offset().y;
                let x1 = x0 + self.tre_scr_w;
                let y1 = y0 + self.tre_scr_h;

                if x0 != self.tre_cnv_vis_x0
                    || y0 != self.tre_cnv_vis_y0
                    || x1 != self.tre_cnv_vis_x1
                    || y1 != self.tre_cnv_vis_y1
                {
                    // println!("TreCnvScrolled:    ({x0:7.2}, {y0:7.2}), ({x1:7.2}, {y1:7.2})",);
                    self.stale_vis_rect = true;
                    self.clear_cache_bnds();

                    self.tre_cnv_vis_x0 = x0;
                    self.tre_cnv_vis_y0 = y0;
                    self.tre_cnv_vis_x1 = x1;
                    self.tre_cnv_vis_y1 = y1;
                    self.tre_cnv_vis_x_mid = x0.midpoint(x1);
                    self.tre_cnv_vis_y_mid = y0.midpoint(y1);

                    self.update_rel_scrl_pos();
                }

                if task.is_none() && self.tre_style_opt_sel == TreSty::PhyGrm {
                    if self.tre_cnv_scrolled && self.tre_cnv_vis_x0 != self.ltt_cnv_vis_x0 {
                        self.ltt_cnv_scrolled = false;
                        task = self.scroll_cnv_to_x(self.ltt_scr_id, self.tre_cnv_vis_x0);
                    } else {
                        self.tre_cnv_scrolled = true;
                    }
                }
            }

            TvMsg::RefreshedVisRect(_vis_vs) => {
                // println!("RefreshedVisRect:  {vis_vs}");
                self.stale_vis_rect = false;
            }

            TvMsg::RefreshedTreeDims(_cnv_vs) => {
                // println!("RefreshedTreeDims: {cnv_vs}");
                self.stale_tre_dims = false;
            }

            TvMsg::LttCnvScrolled(vp) => {
                self.ltt_cnv_vis_x0 = vp.absolute_offset().x;
                self.ltt_cnv_vis_y0 = vp.absolute_offset().y;
                if self.tre_style_opt_sel == TreSty::PhyGrm {
                    if self.ltt_cnv_scrolled && self.tre_cnv_vis_x0 != self.ltt_cnv_vis_x0 {
                        self.tre_cnv_scrolled = false;
                        task = self.scroll_cnv_to_x(self.tre_scr_id, self.ltt_cnv_vis_x0);
                    } else {
                        self.ltt_cnv_scrolled = true;
                    }
                }
            }

            TvMsg::PrevTre => {
                self.prev_tre();
                self.sort();
            }

            TvMsg::NextTre => {
                self.next_tre();
                self.sort();
            }

            TvMsg::NodeOrdOptChanged(node_ord_opt) => {
                self.node_ord_opt_sel = node_ord_opt;
                self.sort();
            }

            TvMsg::Unroot => {
                if let Some(ts) = self.sel_tre_mut()
                    && let Some(_node) = ts.unroot()
                {
                    self.sort();
                }
            }

            TvMsg::Root(node_id) => {
                if let Some(ts) = self.sel_tre_mut()
                    && let Some(_node_id) = ts.root(&node_id)
                {
                    self.sort();
                }
            }

            TvMsg::RootLenSelChanged(idx) => {
                self.root_len_idx_sel = idx;
                self.clear_caches_all();
            }

            TvMsg::TreesLoaded(tres) => {
                self.tre_states = Vec::new();

                let mut i: usize = 1;
                for tre in tres {
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
                self.clear_caches_all();
            }

            TvMsg::CnvWidthSelChanged(idx) => {
                self.tre_cnv_w_idx_sel = idx;
                task = self.keep_scroll_pos_w();
            }

            TvMsg::CnvHeightSelChanged(idx) => {
                self.tre_cnv_h_idx_sel = idx;
                task = self.keep_scroll_pos_h();
            }

            TvMsg::CnvZoomSelChanged(idx) => {
                self.tre_cnv_z_idx_sel = idx;
                task = self.keep_scroll_pos_z();
            }

            TvMsg::TreStyOptChanged(tre_sty_opt) => {
                self.tre_style_opt_sel = tre_sty_opt;
                self.update_rel_scrl_pos();
                self.clear_caches_all();
            }

            TvMsg::TipLabVisChanged(state) => {
                self.draw_labs_tip = state;
                self.stale_tre_dims = true;
                self.clear_caches_all();
            }

            TvMsg::TipLabSizeChanged(idx) => {
                self.tip_lab_size_idx_sel = idx;
                self.lab_size_tip = self.lab_size_min * idx as Float;
                self.stale_tre_dims = true;
                self.clear_caches_all();
            }

            TvMsg::IntLabVisChanged(state) => {
                self.draw_labs_int = state;
                self.clear_caches_all();
            }

            TvMsg::IntLabSizeChanged(idx) => {
                self.int_lab_size_idx_sel = idx;
                self.lab_size_int = self.lab_size_min * idx as Float;
                self.clear_caches_all();
            }

            TvMsg::BrnchLabVisChanged(state) => {
                self.draw_labs_brnch = state;
                self.clear_caches_all();
            }

            TvMsg::BrnchLabSizeChanged(idx) => {
                self.brnch_lab_size_idx_sel = idx;
                self.lab_size_brnch = self.lab_size_min * idx as Float;
                self.clear_caches_all();
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
            }

            TvMsg::RotAngleChanged(idx) => {
                self.rot_angle_idx_sel = idx;
                self.rot_angle = angle_from_idx(idx);
                self.clear_caches_all();
            }

            TvMsg::PaneResized(ResizeEvent { split, ratio }) => {
                if let Some(pane_grid) = &mut self.pane_grid {
                    pane_grid.resize(split, ratio);
                    self.stale_tre_dims = true;
                }
            }

            TvMsg::SetSidebarPos(sidebar_pos) => {
                self.sidebar_pos_sel = sidebar_pos;
            }
        }

        match task {
            Some(task) => task,
            None => Task::none(),
        }
    }

    fn tre_cnv_w(&self) -> Float { self.calc_tre_cnv_w(self.tre_scr_w) }
    fn tre_cnv_h(&self) -> Float { self.calc_tre_cnv_h(self.tre_scr_h) }

    fn update_rel_scrl_pos(&mut self) {
        self.tre_cnv_vis_x_mid_rel = self.tre_cnv_vis_x_mid / self.tre_cnv_w();
        self.tre_cnv_vis_y_mid_rel = self.tre_cnv_vis_y_mid / self.tre_cnv_h();

        match self.tre_style_opt_sel {
            TreSty::PhyGrm => {
                if self.tre_cnv_vis_x1 <= self.tre_scr_w {
                    self.tre_cnv_vis_x_mid_rel = 1.0;
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

    fn keep_scroll_pos_wh(&self) -> Option<Task<TvMsg>> {
        self.scroll_tre_cnv(self.tre_cnv_vis_x_mid, self.tre_cnv_vis_y_mid)
    }

    fn keep_scroll_pos_w(&self) -> Option<Task<TvMsg>> {
        let w = self.tre_cnv_w();
        let mut x = w * self.tre_cnv_vis_x_mid_rel;
        if self.tre_cnv_vis_x1 == w {
            x += self.tre_scr_w / 2e0
        } else if self.tre_cnv_vis_x0 == 0e0 {
            x -= self.tre_scr_w / 2e0
        }
        self.scroll_tre_cnv(x, self.tre_cnv_vis_y_mid)
    }

    fn keep_scroll_pos_h(&self) -> Option<Task<TvMsg>> {
        let h = self.tre_cnv_h();
        let mut y = h * self.tre_cnv_vis_y_mid_rel;
        if self.tre_cnv_vis_y1 == h {
            y += self.tre_scr_h / 2e0
        } else if self.tre_cnv_vis_y0 == 0e0 {
            y -= self.tre_scr_h / 2e0
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
        let x = (x - self.tre_scr_w / 2e0).max(0e0);
        let y = (y - self.tre_scr_h / 2e0).max(0e0);
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
            _ => 0e0,
        };
        Some(scroll_to(receiver_id, AbsoluteOffset { x, y }))
    }

    pub(super) fn clear_cache_bnds(&self) { self.cache_bnds.clear() }

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

    pub(super) fn clear_caches_all(&self) {
        self.clear_cache_bnds();
        self.clear_cache_edge();
        self.clear_caches_lab(true, true, true)
    }

    pub(super) fn calc_tre_cnv_w(&self, w: Float) -> Float {
        match self.tre_style_opt_sel {
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
        match self.tre_style_opt_sel {
            TreSty::PhyGrm => {
                if self.tre_cnv_h_idx_sel == 1 {
                    h
                } else {
                    let tip_count = self.tip_count() as Float;
                    if h / tip_count > 1e0 {
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

    fn tip_count(&self) -> usize { if let Some(ts) = self.sel_tre() { ts.tip_count() } else { 1 } }

    fn show_hide_lttp(&mut self) {
        if let Some(pane_grid) = &mut self.pane_grid {
            if let Some(lttp_pane_id) = self.ltt_pane_id {
                if !self.show_ltt {
                    pane_grid.close(lttp_pane_id);
                    self.ltt_pane_id = None;
                }
            } else if self.show_ltt
                && let Some(tre_pane_id) = self.tre_pane_id
                && let Some((lttp_pane_id, _split)) = pane_grid.split(Axis::Horizontal, tre_pane_id, TvPane::LttPlot)
            {
                self.ltt_pane_id = Some(lttp_pane_id);
            }
        }
    }
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

pub(super) const NODE_ORD_OPTS: [NodeOrd; 3] = [NodeOrd::Unordered, NodeOrd::Ascending, NodeOrd::Descending];
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
