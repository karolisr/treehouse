use crate::edge_utils::*;
use crate::iced::*;
use crate::*;

pub struct TreeView {
    // -------------------------------------------------------------------
    pub(super) tre_states: Vec<Rc<TreeState>>,
    tre_state_idx: Option<usize>,
    // -------------------------------------------------------------------
    pub(super) pane_grid: Option<PgState<TvPane>>,
    pub(super) tre_pane_id: Option<Pane>,
    pub(super) ltt_pane_id: Option<Pane>,
    // -------------------------------------------------------------------
    pub(super) tre_cnv: TreeCnv,
    pub(super) ltt_cnv: PlotCnv,
    pub(super) show_ltt: bool,
    // -------------------------------------------------------------------
    pub(super) show_tool_bar: bool,
    pub(super) show_side_bar: bool,
    pub(super) show_search_bar: bool,
    // -------------------------------------------------------------------
    pub(super) sidebar_pos: SidebarPosition,
    // -------------------------------------------------------------------
    pub(super) root_len_idx_min: u16,
    pub(super) root_len_idx: u16,
    pub(super) root_len_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) tre_cnv_size_idx_min: u16,
    pub(super) tre_cnv_w_idx: u16,
    pub(super) tre_cnv_h_idx: u16,
    pub(super) tre_cnv_z_idx: u16,
    pub(super) tre_cnv_size_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) lab_size_idx_min: u16,
    pub(super) lab_size_idx_tip: u16,
    pub(super) lab_size_idx_int: u16,
    pub(super) lab_size_idx_brnch: u16,
    pub(super) lab_size_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) opn_angle_idx_min: u16,
    pub(super) opn_angle_idx: u16,
    pub(super) opn_angle_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) rot_angle_idx_min: u16,
    pub(super) rot_angle_idx: u16,
    pub(super) rot_angle_idx_max: u16,
    // -------------------------------------------------------------------
    pub(super) node_ord_opt: NodeOrd,
    // -------------------------------------------------------------------
    pub(super) ltt_scr_id: &'static str,
    pub(super) tre_scr_id: &'static str,
    pub(super) search_text_input_id: &'static str,
    // -------------------------------------------------------------------
    keep_scroll_position_requested: bool,
    // -------------------------------------------------------------------
    ltt_cnv_needs_to_be_scrolled: bool,
    ltt_cnv_scrolled: bool,
    // -------------------------------------------------------------------
    pub(super) tre_scr_h: Float,
    pub(super) tre_scr_w: Float,
    tre_cnv_scrolled: bool,
    // -------------------------------------------------------------------
    is_new: bool,
    // -------------------------------------------------------------------
    pub(super) search_string: String,
    pub(super) tip_only_search: bool,
    // -------------------------------------------------------------------
}

impl TreeView {
    pub fn new(sidebar_position: SidebarPosition) -> Self {
        Self {
            sidebar_pos: sidebar_position,
            // -----------------------------------------------------------
            show_ltt: false,
            show_tool_bar: true,
            show_side_bar: true,
            show_search_bar: false,
            // -----------------------------------------------------------
            node_ord_opt: NodeOrd::Ascending,
            // -----------------------------------------------------------
            opn_angle_idx_min: 45,
            opn_angle_idx: 359,
            opn_angle_idx_max: 359,
            // -----------------------------------------------------------
            rot_angle_idx_min: 360 - 180,
            rot_angle_idx: 360,
            rot_angle_idx_max: 360 + 180,
            // -----------------------------------------------------------
            lab_size_idx_min: 8,
            lab_size_idx_tip: TIP_LAB_SIZE_IDX,
            lab_size_idx_int: INTERNAL_LAB_SIZE_IDX,
            lab_size_idx_brnch: BRANCH_LAB_SIZE_IDX,
            lab_size_idx_max: 22,
            // -----------------------------------------------------------
            root_len_idx_min: 0,
            root_len_idx: 10,
            root_len_idx_max: 100,
            // -----------------------------------------------------------
            tre_cnv_size_idx_min: 1,
            tre_cnv_w_idx: 1,
            tre_cnv_h_idx: 1,
            tre_cnv_z_idx: 1,
            tre_cnv_size_idx_max: 22,
            // -----------------------------------------------------------
            tre_scr_id: "tre",
            ltt_scr_id: "ltt",
            search_text_input_id: "srch",
            // -----------------------------------------------------------
            is_new: true,
            // -----------------------------------------------------------
            tip_only_search: true,
            tre_states: vec![],
            tre_state_idx: None,
            pane_grid: None,
            tre_pane_id: None,
            ltt_pane_id: None,
            ltt_cnv: PlotCnv::default(),
            tre_cnv: TreeCnv::new(),
            keep_scroll_position_requested: false,
            ltt_cnv_needs_to_be_scrolled: false,
            ltt_cnv_scrolled: false,
            tre_scr_h: ZERO,
            tre_scr_w: ZERO,
            tre_cnv_scrolled: false,
            search_string: String::new(),
            // -----------------------------------------------------------
        }
    }

    pub(super) fn clade_has_label(&self, node_id: &NodeId) -> bool {
        if let Some(tree) = self.sel_tre() { tree.clade_has_label(node_id) } else { false }
    }

    pub(super) fn tree_has_clade_labels(&self) -> bool {
        if let Some(tree) = self.sel_tre() { !tree.labeled_clades().is_empty() } else { false }
    }

    pub fn update(&mut self, tv_msg: TvMsg) -> Task<TvMsg> {
        let mut task: Option<Task<TvMsg>> = None;
        match tv_msg {
            TvMsg::LabelClade(node_id) => {
                self.with_exclusive_sel_tre_mut(&mut |tre| {
                    tre.add_clade_label(node_id, Clr::GRN_50, node_id, CladeLabelType::Outside)
                });
                self.tre_cnv.has_clade_labels = self.tree_has_clade_labels();
                self.tre_cnv.stale_tre_rect = true;
                self.clear_caches_all();
            }

            TvMsg::RemoveCladeLabel(node_id) => {
                self.with_exclusive_sel_tre_mut(&mut |tre| tre.remove_clade_label(&node_id));
                self.tre_cnv.has_clade_labels = self.tree_has_clade_labels();
                self.tre_cnv.stale_tre_rect = true;
                self.clear_caches_all();
            }

            TvMsg::TreeRectNoLongerStale => {
                self.tre_cnv.stale_tre_rect = false;
            }

            TvMsg::TreCnvScrolledOrResized(vp) => {
                // tree canvas scrollable was resized ----------------------------------------------
                if vp.bounds().width != self.tre_scr_w || vp.bounds().height != self.tre_scr_h {
                    self.tre_scr_w = vp.bounds().width;
                    self.tre_scr_h = vp.bounds().height;
                } // -------------------------------------------------------------------------------

                if self.keep_scroll_position_requested {
                    task = self.scroll_tre_cnv(self.tre_cnv.vis_x_mid, self.tre_cnv.vis_y_mid);
                    self.tre_cnv_scrolled = true;
                    self.keep_scroll_position_requested = false;
                }

                self.tre_cnv.vis_x_mid = vp.absolute_offset().x + vp.bounds().width / TWO;
                self.tre_cnv.vis_y_mid = vp.absolute_offset().y + vp.bounds().height / TWO;
                self.update_rel_scrl_pos();
                self.update_vis_x();
                self.update_vis_y();

                if task.is_none() && self.tre_cnv.tre_sty == TreSty::PhyGrm {
                    if self.tre_cnv_scrolled && self.tre_cnv.vis_x0 != self.ltt_cnv.vis_x0 {
                        self.ltt_cnv_scrolled = false;
                        task = self.scroll_cnv_to_x(self.ltt_scr_id, self.tre_cnv.vis_x0);
                    } else {
                        self.tre_cnv_scrolled = true;
                    }
                }

                self.update_draw_labs_allowed();
                self.clear_cache_sel_nodes();
                self.clear_cache_filtered_nodes();
            }

            TvMsg::LttCnvScrolledOrResized(vp) => {
                if self.ltt_cnv.vis_x0 != vp.absolute_offset().x {
                    self.ltt_cnv.vis_x0 = vp.absolute_offset().x;
                    self.ltt_cnv.vis_y0 = vp.absolute_offset().y;
                    if self.tre_cnv.tre_sty == TreSty::PhyGrm {
                        if self.ltt_cnv_scrolled && self.tre_cnv.vis_x0 != self.ltt_cnv.vis_x0 {
                            self.tre_cnv_scrolled = false;
                            task = self.scroll_cnv_to_x(self.tre_scr_id, self.ltt_cnv.vis_x0);
                        } else {
                            self.ltt_cnv_scrolled = true;
                        }
                    }
                }
            }

            TvMsg::LttVisChanged(show_ltt) => {
                self.show_ltt = show_ltt;
                self.show_hide_ltt();
                let x = (self.tre_cnv.vis_x_mid - self.tre_scr_w / TWO).max(ZERO);
                task =
                    Some(scroll_to(self.ltt_scr_id, AbsoluteOffset { x, y: self.ltt_cnv.vis_y0 }));
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
                if node_ord_opt != self.node_ord_opt {
                    self.node_ord_opt = node_ord_opt;
                    self.sort();
                    task = self.scroll_to_current_found_edge();
                }
            }

            TvMsg::Unroot => {
                let mut yanked_node: Option<Node> = None;
                self.with_exclusive_sel_tre_mut(&mut |tre| yanked_node = tre.unroot());
                if yanked_node.is_some() {
                    self.sort()
                }
                self.set_ltt_plot_data();
                self.update_draw_labs_allowed();
                task = self.scroll_to_current_found_edge();
                self.tre_cnv.clear_cache_legend();
            }

            TvMsg::Root(node_id) => {
                let mut node_id_new_root: Option<NodeId> = None;
                self.with_exclusive_sel_tre_mut(&mut |tre| node_id_new_root = tre.root(&node_id));
                if node_id_new_root.is_some() {
                    self.sort()
                }
                self.set_ltt_plot_data();
                self.update_draw_labs_allowed();
                task = self.scroll_to_current_found_edge();
                self.tre_cnv.clear_cache_legend();
            }

            TvMsg::TreesLoaded(trees) => {
                self.tre_cnv.drawing_enabled = false;
                self.tre_states = Vec::new();

                let mut i: usize = 1;
                for tre in trees {
                    let mut ts = TreeState::new(i);
                    ts.init(tre);
                    self.tre_states.push(Rc::new(ts));
                    i += 1;
                }

                if !self.tre_states.is_empty() {
                    self.tre_state_idx = Some(0);
                } else {
                    self.tre_state_idx = None;
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
                    self.tre_cnv.opn_angle = angle_from_idx(self.opn_angle_idx);
                    self.tre_cnv.rot_angle = angle_from_idx(self.rot_angle_idx);

                    let lab_size_min = self.tre_cnv.lab_size_min;
                    self.tre_cnv.lab_size_tip = lab_size_min * self.lab_size_idx_tip as Float;
                    self.tre_cnv.lab_size_int = lab_size_min * self.lab_size_idx_int as Float;
                    self.tre_cnv.lab_size_brnch = lab_size_min * self.lab_size_idx_brnch as Float;
                    self.tre_cnv.lab_size_max = lab_size_min * self.lab_size_idx_max as Float;

                    self.tre_cnv.lab_offset_tip = SF * 8e0;
                    self.tre_cnv.lab_offset_int = SF * 8e0;
                    self.tre_cnv.lab_offset_brnch = -self.tre_cnv.lab_size_brnch / THREE;

                    self.tre_cnv.clade_labs_w = SF * TEN;
                    self.tre_cnv.has_clade_labels = self.tree_has_clade_labels();

                    self.tre_cnv.root_len_frac = self.calc_root_len_frac();

                    self.show_hide_ltt();
                    self.ltt_cnv.draw_cursor_line = self.tre_cnv.draw_cursor_line;
                    self.update_tree_rect_padding();
                    self.is_new = false;

                    self.ltt_cnv.scale_x = AxisScaleType::Linear;
                    self.ltt_cnv.scale_y = AxisScaleType::LogTwo;
                }

                self.update_draw_labs_allowed();
                self.clear_caches_all();
                self.tre_cnv.drawing_enabled = true;
            }

            TvMsg::LttXAxisScaleTypeChanged(axis_scale_type) => {
                self.ltt_cnv.scale_x = axis_scale_type;
                self.ltt_cnv.clear_caches_all();
            }

            TvMsg::LttYAxisScaleTypeChanged(axis_scale_type) => {
                self.ltt_cnv.scale_y = axis_scale_type;
                self.ltt_cnv.clear_caches_all();
            }

            TvMsg::CnvWidthSelChanged(idx) => {
                if idx != self.tre_cnv_w_idx {
                    self.update_rel_scrl_pos();
                    self.tre_cnv_w_idx = idx;
                    self.ltt_cnv_needs_to_be_scrolled = true;
                    self.update_tree_rect_padding();
                    self.update_vis_x();
                    self.update_draw_labs_allowed();
                    task = self.scroll_to_current_found_edge();
                    if task.is_none() {
                        task = self.keep_scroll_pos_w();
                    }
                    self.tre_cnv.stale_tre_rect = true;
                    self.clear_cache_edge();
                }
            }

            TvMsg::CnvHeightSelChanged(idx) => {
                if idx != self.tre_cnv_h_idx {
                    self.update_rel_scrl_pos();
                    self.tre_cnv_h_idx = idx;
                    self.update_tree_rect_padding();
                    self.update_vis_y();
                    self.update_draw_labs_allowed();
                    task = self.scroll_to_current_found_edge();
                    if task.is_none() {
                        task = self.keep_scroll_pos_h();
                    }
                    self.tre_cnv.stale_tre_rect = true;
                    self.clear_cache_edge();
                }
            }

            TvMsg::CnvZoomSelChanged(idx) => {
                if idx != self.tre_cnv_z_idx {
                    self.update_rel_scrl_pos();
                    self.tre_cnv_z_idx = idx;
                    self.update_tree_rect_padding();
                    self.update_vis_x();
                    self.update_vis_y();
                    self.update_draw_labs_allowed();
                    task = self.scroll_to_current_found_edge();
                    if task.is_none() {
                        task = self.keep_scroll_pos_z();
                    }
                    self.tre_cnv.stale_tre_rect = true;
                    self.clear_cache_edge();
                }
            }

            TvMsg::TreStyOptChanged(tre_sty_opt) => {
                if tre_sty_opt != self.tre_cnv.tre_sty {
                    self.tre_cnv.tre_sty = tre_sty_opt;
                    self.update_rel_scrl_pos();
                    self.update_tree_rect_padding();
                    self.update_draw_labs_allowed();
                    task = self.scroll_to_current_found_edge();
                    self.tre_cnv.stale_tre_rect = true;
                    self.clear_caches_all();
                }
            }

            TvMsg::TipLabVisChanged(state) => {
                self.tre_cnv.draw_labs_tip = state;
                self.tre_cnv.stale_tre_rect = true;
                self.clear_caches_all();
            }

            TvMsg::TipLabSizeChanged(idx) => {
                self.lab_size_idx_tip = idx;
                self.tre_cnv.lab_size_tip = self.tre_cnv.lab_size_min * idx as Float;
                // self.tre_cnv.lab_offset_tip = self.tre_cnv.lab_size_tip / THREE;
                self.tre_cnv.lab_offset_tip = SF * 8e0;
                // -------------------------------------------------------------
                if let Some(text_w_tip) = &mut self.tre_cnv.text_w_tip
                    && text_w_tip.font_size() != self.tre_cnv.lab_size_tip
                {
                    text_w_tip.set_font_size(self.tre_cnv.lab_size_tip);
                };
                // -------------------------------------------------------------
                self.tre_cnv.stale_tre_rect = true;
                self.clear_caches_all();
            }

            TvMsg::IntLabVisChanged(state) => {
                self.tre_cnv.draw_labs_int = state;
                self.clear_cache_lab_int();
            }

            TvMsg::IntLabSizeChanged(idx) => {
                self.lab_size_idx_int = idx;
                self.tre_cnv.lab_size_int = self.tre_cnv.lab_size_min * idx as Float;
                // self.tre_cnv.lab_offset_int = self.tre_cnv.lab_size_int / THREE;
                self.tre_cnv.lab_offset_int = SF * 8e0;
                self.clear_cache_lab_int();
            }

            TvMsg::BrnchLabVisChanged(state) => {
                self.tre_cnv.draw_labs_brnch = state;
                self.tre_cnv.stale_tre_rect = true;
                self.clear_caches_all();
            }

            TvMsg::BrnchLabSizeChanged(idx) => {
                self.lab_size_idx_brnch = idx;
                self.tre_cnv.lab_size_brnch = self.tre_cnv.lab_size_min * idx as Float;
                self.tre_cnv.lab_offset_brnch = -self.tre_cnv.lab_size_brnch / THREE;
                self.tre_cnv.stale_tre_rect = true;
                self.clear_caches_all();
            }

            TvMsg::LegendVisChanged(state) => {
                self.tre_cnv.draw_legend = state;
            }

            TvMsg::SelectDeselectNode(node_id) => {
                self.with_exclusive_sel_tre_mut(&mut |tre| tre.select_deselect_node(&node_id));
            }

            TvMsg::OpnAngleChanged(idx) => {
                self.opn_angle_idx = idx;
                self.tre_cnv.opn_angle = angle_from_idx(idx);
                task = self.scroll_to_current_found_edge();
                self.clear_caches_all();
            }

            TvMsg::RotAngleChanged(idx) => {
                self.rot_angle_idx = idx;
                self.tre_cnv.rot_angle = angle_from_idx(idx);
                task = self.scroll_to_current_found_edge();
                self.clear_caches_all();
            }

            TvMsg::RootVisChanged(state) => {
                self.tre_cnv.draw_root = state;
                self.tre_cnv.root_len_frac = self.calc_root_len_frac();
                task = self.scroll_to_current_found_edge();
                self.clear_caches_all();
            }

            TvMsg::RootLenSelChanged(idx) => {
                self.root_len_idx = idx;
                self.tre_cnv.root_len_frac = self.calc_root_len_frac();
                task = self.scroll_to_current_found_edge();
                self.clear_caches_all();
            }

            TvMsg::PaneResized(ResizeEvent { split, ratio }) => {
                if let Some(pane_grid) = &mut self.pane_grid {
                    pane_grid.resize(split, ratio);
                    self.update_vis_y();
                    self.update_draw_labs_allowed();
                    self.clear_cache_sel_nodes();
                    self.clear_cache_filtered_nodes();
                }
            }

            TvMsg::SetSidebarPos(sidebar_pos) => {
                self.sidebar_pos = sidebar_pos;
                self.keep_scroll_position_requested = true;
            }

            TvMsg::CursorLineVisChanged(state) => {
                self.tre_cnv.crsr_x_rel = None;
                self.ltt_cnv.crsr_x_rel = None;
                self.tre_cnv.draw_cursor_line = state;
                self.ltt_cnv.draw_cursor_line = state;
                self.tre_cnv.clear_cache_cursor_line();
                self.ltt_cnv.clear_cache_cursor_line();
            }

            TvMsg::CursorOnTreCnv { x } => {
                self.tre_cnv.crsr_x_rel = None;
                self.ltt_cnv.crsr_x_rel = x;
            }

            TvMsg::CursorOnLttCnv { x } => {
                self.tre_cnv.crsr_x_rel = x;
                self.ltt_cnv.crsr_x_rel = None;
            }

            TvMsg::ToggleSearchBar => {
                self.show_search_bar = !self.show_search_bar;
                if self.show_search_bar {
                    task = Some(
                        Task::done(TvMsg::Search(self.search_string.clone()))
                            .chain(focus_text_input(self.search_text_input_id)),
                    );
                } else {
                    self.with_exclusive_sel_tre_mut(&mut |tre| {
                        tre.clear_filter_results();
                    });
                    self.keep_scroll_position_requested = true;
                }
            }

            TvMsg::TipOnlySearchSelChanged(state) => {
                self.tip_only_search = state;
                task = Some(Task::done(TvMsg::Search(self.search_string.clone())));
            }

            TvMsg::Search(s) => {
                self.search_string = s.clone();
                let tips_only = self.tip_only_search;
                self.with_exclusive_sel_tre_mut(&mut |tre| {
                    tre.filter_nodes(&s, tips_only);
                });
                task = self.scroll_to_current_found_edge();
            }

            TvMsg::PrevResult => {
                self.with_exclusive_sel_tre_mut(&mut TreeState::prev_result);
                task = self.scroll_to_current_found_edge();
            }

            TvMsg::NextResult => {
                self.with_exclusive_sel_tre_mut(&mut TreeState::next_result);
                task = self.scroll_to_current_found_edge();
            }

            TvMsg::AddFoundToSelection => {
                self.with_exclusive_sel_tre_mut(&mut TreeState::add_found_to_sel);
            }

            TvMsg::RemFoundFromSelection => {
                self.with_exclusive_sel_tre_mut(&mut TreeState::rem_found_from_sel);
            }
        }

        match task {
            Some(task) => task,
            None => Task::none(),
        }
    }

    fn set_ltt_plot_data(&mut self) {
        if let Some(ts) = self.sel_tre() {
            let plot_data = &ltt(ts.tre_height(), ts.edges_srtd_y(), 503);
            self.ltt_cnv.set_plot_data(plot_data);
        }
    }

    pub(super) fn calc_root_len_frac(&self) -> Float {
        if self.tre_cnv.draw_root { self.root_len_idx as Float / 2e2 } else { ZERO }
    }

    pub(super) fn prev_tre_exists(&self) -> bool {
        match self.tre_state_idx {
            Some(idx) => idx > 0,
            None => false,
        }
    }

    pub(super) fn next_tre_exists(&self) -> bool {
        match self.tre_state_idx {
            Some(idx) => idx < self.tre_states.len() - 1,
            None => false,
        }
    }

    pub(super) fn sel_tre(&self) -> Option<Rc<TreeState>> {
        if let Some(sel_tre_state_idx) = self.tre_state_idx {
            let sel_tre_state = &self.tre_states[sel_tre_state_idx];
            Some(sel_tre_state.clone())
        } else {
            None
        }
    }

    fn sel_tre_mut(&mut self) -> Option<&mut TreeState> {
        if let Some(sel_tre_state_idx) = self.tre_state_idx {
            let sel_tre_state = &mut self.tre_states[sel_tre_state_idx];
            Rc::get_mut(sel_tre_state)
        } else {
            None
        }
    }

    fn sort(&mut self) {
        let node_ord_opt = self.node_ord_opt;
        self.with_exclusive_sel_tre_mut(&mut |tre| tre.sort(node_ord_opt));
        self.tre_cnv.stale_tre_rect = true; // important! to make sure TreeCnv updates vis_node_idxs
    }

    fn update_sel_tre_st_idx(&mut self, idx: Option<usize>) -> bool {
        if idx != self.tre_state_idx {
            self.tre_state_idx = idx;
            true
        } else {
            false
        }
    }

    fn prev_tre(&mut self) -> bool {
        let prev_idx = match self.tre_state_idx {
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
        let next_idx = match self.tre_state_idx {
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

    fn is_rooted(&self) -> bool {
        if let Some(ts) = self.sel_tre() { ts.is_rooted() } else { false }
    }

    fn current_found_edge(&self) -> Option<Edge> {
        if let Some(tre) = self.sel_tre() { tre.current_found_edge() } else { None }
    }

    fn scroll_to_current_found_edge(&mut self) -> Option<Task<TvMsg>> {
        if let Some(edge) = self.current_found_edge() {
            let edge = &edge.clone();
            self.scroll_to_edge(edge)
        } else {
            None
        }
    }

    pub(super) fn calc_tre_cnv_w(&self, w: Float) -> Float {
        match self.tre_cnv.tre_sty {
            TreSty::PhyGrm => {
                if self.tre_cnv_w_idx <= self.tre_cnv_size_idx_min {
                    w
                } else {
                    let tmp = TREE_CNV_SIZE_DELTA * self.tre_cnv_w_idx as Float;
                    if tmp < w { w } else { tmp }
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_z_idx <= self.tre_cnv_size_idx_min {
                    w
                } else {
                    let tmp = TREE_CNV_SIZE_DELTA * self.tre_cnv_z_idx as Float;
                    if tmp < w { w } else { tmp }
                }
            }
        }
    }

    pub(super) fn calc_tre_cnv_h(&self, h: Float) -> Float {
        match self.tre_cnv.tre_sty {
            TreSty::PhyGrm => {
                if self.tre_cnv_h_idx <= self.tre_cnv_size_idx_min {
                    h
                } else {
                    let tip_count = self.tip_count() as Float;
                    let tmp = if h / tip_count > ONE {
                        TREE_CNV_SIZE_DELTA * self.tre_cnv_h_idx as Float
                    } else {
                        tip_count * self.tre_cnv_h_idx as Float
                    };
                    if tmp < h { h } else { tmp }
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_z_idx <= self.tre_cnv_size_idx_min {
                    h
                } else {
                    let tmp = TREE_CNV_SIZE_DELTA * self.tre_cnv_z_idx as Float;
                    if tmp < h { h } else { tmp }
                }
            }
        }
    }

    fn update_tree_rect_padding(&mut self) {
        if self.calc_tre_cnv_w(self.tre_scr_w) <= self.tre_scr_w {
            self.tre_cnv.padd_b = TREE_PADDING;
        } else {
            self.tre_cnv.padd_b = TREE_PADDING + SCROLL_BAR_W;
        }

        if self.calc_tre_cnv_h(self.tre_scr_h) <= self.tre_scr_h {
            self.tre_cnv.padd_r = TREE_PADDING;
        } else {
            self.tre_cnv.padd_r = TREE_PADDING + SCROLL_BAR_W;
        }

        if self.tre_cnv.tre_sty == TreSty::PhyGrm {
            self.ltt_cnv.padd_b = self.tre_cnv.padd_b;
            self.ltt_cnv.padd_r = self.tre_cnv.padd_r;
        } else {
            self.ltt_cnv.padd_b = TREE_PADDING;
            self.ltt_cnv.padd_r = TREE_PADDING;
        }

        self.ltt_cnv.padd_t = self.tre_cnv.padd_t;
        self.ltt_cnv.padd_l = self.tre_cnv.padd_l;
    }

    fn update_rel_scrl_pos(&mut self) {
        let vis_x_mid = self.tre_cnv.vis_x_mid;
        let vis_y_mid = self.tre_cnv.vis_y_mid;

        self.tre_cnv.vis_x_mid_rel = vis_x_mid / self.calc_tre_cnv_w(self.tre_scr_w);
        self.tre_cnv.vis_y_mid_rel = vis_y_mid / self.calc_tre_cnv_h(self.tre_scr_h);

        match self.tre_cnv.tre_sty {
            TreSty::PhyGrm => {
                if self.tre_cnv_w_idx == self.tre_cnv_size_idx_min {
                    self.tre_cnv.vis_x_mid_rel = ZERO;
                }
                if self.tre_cnv_h_idx == self.tre_cnv_size_idx_min {
                    self.tre_cnv.vis_y_mid_rel = ZERO;
                }
            }
            TreSty::Fan => {
                if self.tre_cnv_z_idx == self.tre_cnv_size_idx_min {
                    self.tre_cnv.vis_x_mid_rel = ONE / TWO;
                    self.tre_cnv.vis_y_mid_rel = ONE / TWO;
                }
            }
        }
    }

    fn keep_scroll_pos_w(&self) -> Option<Task<TvMsg>> {
        let w = self.calc_tre_cnv_w(self.tre_scr_w);
        let mut x = w * self.tre_cnv.vis_x_mid_rel;
        if self.tre_cnv.vis_x1 == w {
            x += self.tre_scr_w / TWO
        } else if self.tre_cnv.vis_x0 == ZERO {
            x -= self.tre_scr_w / TWO
        }
        self.scroll_tre_cnv(x, self.tre_cnv.vis_y_mid)
    }

    fn keep_scroll_pos_h(&self) -> Option<Task<TvMsg>> {
        let h = self.calc_tre_cnv_h(self.tre_scr_h);
        let mut y = h * self.tre_cnv.vis_y_mid_rel;
        if self.tre_cnv.vis_y1 == h {
            y += self.tre_scr_h / TWO
        } else if self.tre_cnv.vis_y0 == ZERO {
            y -= self.tre_scr_h / TWO
        }
        self.scroll_tre_cnv(self.tre_cnv.vis_x_mid, y)
    }

    fn keep_scroll_pos_z(&self) -> Option<Task<TvMsg>> {
        self.scroll_tre_cnv(
            self.calc_tre_cnv_w(self.tre_scr_w) * self.tre_cnv.vis_x_mid_rel,
            self.calc_tre_cnv_h(self.tre_scr_h) * self.tre_cnv.vis_y_mid_rel,
        )
    }

    fn scroll_tre_cnv(&self, x: Float, y: Float) -> Option<Task<TvMsg>> {
        let x = (x - self.tre_scr_w / TWO).max(ZERO);
        let y = (y - self.tre_scr_h / TWO).max(ZERO);
        let task1 = scroll_to(self.tre_scr_id, AbsoluteOffset { x, y });
        if !self.ltt_cnv_needs_to_be_scrolled {
            Some(task1)
        } else {
            let task2 = scroll_to(self.ltt_scr_id, AbsoluteOffset { x, y: self.ltt_cnv.vis_y0 });
            Some(Task::batch([task1, task2]))
        }
    }

    fn scroll_cnv_to_x(&self, receiver_id: &'static str, x: Float) -> Option<Task<TvMsg>> {
        let y = match receiver_id {
            id if id == self.ltt_scr_id => self.ltt_cnv.vis_y0,
            id if id == self.tre_scr_id => self.tre_cnv.vis_y0,
            _ => ZERO,
        };
        Some(scroll_to(receiver_id, AbsoluteOffset { x, y }))
    }

    fn update_tre_vs(&mut self) {
        let tre_vs =
            RectVals::wh(self.calc_tre_cnv_w(self.tre_scr_w), self.calc_tre_cnv_h(self.tre_scr_h))
                .padded(
                    self.tre_cnv.padd_l, self.tre_cnv.padd_r, self.tre_cnv.padd_t,
                    self.tre_cnv.padd_b,
                );

        let mut tip_w: Float = ZERO;

        if let Some(sel_tre) = self.sel_tre()
            && self.tre_cnv.draw_labs_tip
            && self.tre_cnv.draw_labs_allowed
        {
            tip_w = cnv_tree::calc_tip_w(
                self.tre_cnv.tre_sty,
                tre_vs,
                sel_tre.edges_tip_tallest(),
                self.tre_cnv.lab_offset_tip,
                self.tre_cnv.text_w_tip.as_mut().unwrap(),
            );
        }

        self.tre_cnv.tre_vs = self.tre_cnv.calc_tre_vs(tip_w, tre_vs);
    }

    fn update_vis_x(&mut self) {
        let scr_w = self.tre_scr_w;
        let cnv_w = self.calc_tre_cnv_w(scr_w);
        let vis_x_mid = cnv_w * self.tre_cnv.vis_x_mid_rel;
        self.tre_cnv.vis_x_mid = vis_x_mid;
        self.tre_cnv.vis_x0 = (vis_x_mid - scr_w / TWO).min(cnv_w - scr_w).max(ZERO);
        self.tre_cnv.vis_x1 = (vis_x_mid + scr_w / TWO).max(scr_w).min(cnv_w);
    }

    fn update_vis_y(&mut self) {
        let scr_h = self.tre_scr_h;
        let cnv_h = self.calc_tre_cnv_h(scr_h);
        let vis_y_mid = cnv_h * self.tre_cnv.vis_y_mid_rel;
        self.tre_cnv.vis_y_mid = vis_y_mid;
        self.tre_cnv.vis_y0 = (vis_y_mid - scr_h / TWO).min(cnv_h - scr_h).max(ZERO);
        self.tre_cnv.vis_y1 = (vis_y_mid + scr_h / TWO).max(scr_h).min(cnv_h);
    }

    fn update_vis_xy_around_point(&mut self, pt: &Point) {
        let scr_w = self.tre_scr_w;
        let scr_h = self.tre_scr_h;
        let cnv_w = self.calc_tre_cnv_w(scr_w);
        let cnv_h = self.calc_tre_cnv_h(scr_h);
        self.tre_cnv.vis_x0 = (pt.x - scr_w / TWO).min(cnv_w - scr_w).max(ZERO);
        self.tre_cnv.vis_x1 = (pt.x + scr_w / TWO).max(scr_w).min(cnv_w);
        self.tre_cnv.vis_y0 = (pt.y - scr_h / TWO).min(cnv_h - scr_h).max(ZERO);
        self.tre_cnv.vis_y1 = (pt.y + scr_h / TWO).max(scr_h).min(cnv_h);
    }

    fn scroll_to_point(&self, pt: &Point) -> Option<Task<TvMsg>> { self.scroll_tre_cnv(pt.x, pt.y) }

    fn scroll_to_edge(&mut self, edge: &Edge) -> Option<Task<TvMsg>> {
        self.update_tre_vs(); // this should not be done every time "scroll_to_edge" is called.
        let mut root_len: Float = ZERO;
        let pt: Point = match self.tre_cnv.tre_sty {
            TreSty::PhyGrm => {
                if self.is_rooted() {
                    root_len = self.tre_cnv.tre_vs.w * self.tre_cnv.root_len_frac;
                }
                node_data_cart(self.tre_cnv.tre_vs.w - root_len, self.tre_cnv.tre_vs.h, edge)
                    .points
                    .p1
                    + Vector {
                        x: self.tre_cnv.tre_vs.trans.x + root_len,
                        y: self.tre_cnv.tre_vs.trans.y,
                    }
            }
            TreSty::Fan => {
                if self.is_rooted() {
                    root_len = self.tre_cnv.tre_vs.radius_min * self.tre_cnv.root_len_frac;
                }
                node_data_rad(
                    self.tre_cnv.opn_angle, self.tre_cnv.rot_angle, self.tre_cnv.tre_vs.radius_min,
                    root_len, edge,
                )
                .points
                .p1 + self.tre_cnv.tre_vs.cntr
            }
        };
        self.update_vis_xy_around_point(&pt);
        self.scroll_to_point(&pt)
    }

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

    // pub(super) fn clear_cache_lab_tip(&self) {
    //     if let Some(ts) = self.sel_tre() {
    //         ts.clear_cache_lab_tip()
    //     }
    // }

    pub(super) fn clear_cache_lab_int(&self) {
        if let Some(ts) = self.sel_tre() {
            ts.clear_cache_lab_int()
        }
    }

    // pub(super) fn clear_cache_lab_brnch(&self) {
    //     if let Some(ts) = self.sel_tre() {
    //         ts.clear_cache_lab_brnch()
    //     }
    // }

    pub(super) fn clear_cache_all_tre(&self) {
        if let Some(ts) = self.sel_tre() {
            ts.clear_caches_all()
        }
    }

    pub(super) fn clear_caches_all(&self) {
        self.tre_cnv.clear_caches_all();
        self.ltt_cnv.clear_caches_all();
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
                && let Some((ltt_pane_id, split)) =
                    pane_grid.split(Axis::Horizontal, tre_pane_id, TvPane::LttPlot)
            {
                pane_grid.resize(split, ONE);
                self.ltt_pane_id = Some(ltt_pane_id);
            }
        }
    }

    fn update_draw_labs_allowed(&mut self) {
        self.tre_cnv.draw_labs_allowed = match self.tre_cnv.tre_sty {
            TreSty::PhyGrm => {
                let node_size = self.calc_tre_cnv_h(self.tre_scr_h) / self.tip_count() as Float;
                let tip_labs_vis = (self.tre_scr_h / node_size).floor() as usize;
                tip_labs_vis <= self.tre_cnv.tip_labs_vis_max
            }
            TreSty::Fan => self.tip_count() <= self.tre_cnv.tip_labs_vis_max * 2,
        };
    }

    fn with_exclusive_sel_tre_mut(&mut self, f: &mut dyn FnMut(&mut TreeState)) {
        self.tre_cnv.tree_state = None;
        if let Some(tre) = self.sel_tre_mut() {
            f(tre);
        }
        self.tre_cnv.tree_state = self.sel_tre();
    }

    pub fn are_any_trees_loaded(&self) -> bool { !self.tre_states.is_empty() }

    pub fn newick_string(&self) -> String {
        let trees: Vec<Tree> = self.tre_states.iter().map(|ts| ts.tree().clone()).collect();
        write_newick(&trees)
    }

    pub fn toggle_draw_debug(&mut self) {
        self.tre_cnv.draw_debug = !self.tre_cnv.draw_debug;
        self.ltt_cnv.draw_debug = !self.ltt_cnv.draw_debug;
    }
}

fn angle_from_idx(idx: u16) -> Float { (idx as Float).to_radians() }

#[derive(Debug, Clone)]
pub enum TvMsg {
    TreeRectNoLongerStale,
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
    LttXAxisScaleTypeChanged(AxisScaleType),
    LttYAxisScaleTypeChanged(AxisScaleType),
    OpnAngleChanged(u16),
    PaneResized(ResizeEvent),
    Root(NodeId),
    Unroot,
    RotAngleChanged(u16),
    SelectDeselectNode(NodeId),
    SetSidebarPos(SidebarPosition),
    TreesLoaded(Vec<Tree>),
    TreStyOptChanged(TreSty),
    RootVisChanged(bool),
    RootLenSelChanged(u16),
    LttVisChanged(bool),
    // -------------------------------------------
    LabelClade(NodeId),
    RemoveCladeLabel(NodeId),
    // -------------------------------------------
    TreCnvScrolledOrResized(Viewport),
    LttCnvScrolledOrResized(Viewport),
    // -------------------------------------------
    CursorOnTreCnv { x: Option<Float> },
    CursorOnLttCnv { x: Option<Float> },
    // -------------------------------------------
    ToggleSearchBar,
    Search(String),
    NextResult,
    PrevResult,
    AddFoundToSelection,
    RemFoundFromSelection,
    TipOnlySearchSelChanged(bool),
    // -------------------------------------------
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarPosition {
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
