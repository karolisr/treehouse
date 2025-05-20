use crate::elements::*;
use crate::*;
use iced::Task;
use iced::widget::canvas::Cache;
use iced::widget::pane_grid::{Axis, Pane, ResizeEvent, State as PaneGridState};
use iced::widget::scrollable::{AbsoluteOffset, Viewport, scroll_to};
use iced::{Element, Padding};
use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub struct TreeView {
    // --------------------------------------------
    pub(super) tree_states: Vec<TreeState>,
    tree_state_idx_sel: Option<usize>,
    // --------------------------------------------
    pub(super) pane_grid: Option<PaneGridState<TvPane>>,
    tree_pane_id: Option<Pane>,
    lttp_pane_id: Option<Pane>,
    // --------------------------------------------
    pub(super) ltt_plot: PlotCnv,
    pub(super) show_lttp: bool,
    // --------------------------------------------
    show_toolbar: bool,
    show_sidebar: bool,
    // --------------------------------------------
    pub(super) sidebar_pos_sel: SidebarPos,
    // --------------------------------------------
    pub(super) root_len_idx_min: u16,
    pub(super) root_len_idx_max: u16,
    pub(super) root_len_idx_sel: u16,
    // --------------------------------------------
    pub(super) tre_cnv_w_idx_min: u16,
    pub(super) tre_cnv_h_idx_min: u16,
    pub(super) tre_cnv_z_idx_min: u16,
    // --------------------------------------------
    pub(super) tre_cnv_w_idx_max: u16,
    pub(super) tre_cnv_h_idx_max: u16,
    pub(super) tre_cnv_z_idx_max: u16,
    // --------------------------------------------
    pub(super) tre_cnv_w_idx_sel: u16,
    pub(super) tre_cnv_h_idx_sel: u16,
    pub(super) tre_cnv_z_idx_sel: u16,
    // --------------------------------------------
    pub(super) lab_size_idx_min: u16,
    pub(super) tip_lab_size_idx_sel: u16,
    pub(super) int_lab_size_idx_sel: u16,
    pub(super) brnch_lab_size_idx_sel: u16,
    pub(super) lab_size_idx_max: u16,
    // --------------------------------------------
    lab_size_min: Float,
    // lab_size_max: Float,
    pub(super) lab_size_tip: Float,
    pub(super) lab_offset_tip: Float,
    pub(super) lab_size_int: Float,
    pub(super) lab_offset_int: Float,
    pub(super) lab_size_brnch: Float,
    pub(super) lab_offset_brnch: Float,
    // --------------------------------------------
    pub(super) opn_angle: Float,
    pub(super) rot_angle: Float,
    pub(super) opn_angle_idx_sel: u16,
    pub(super) rot_angle_idx_sel: u16,
    pub(super) opn_angle_idx_min: u16,
    pub(super) rot_angle_idx_min: u16,
    pub(super) opn_angle_idx_max: u16,
    pub(super) rot_angle_idx_max: u16,
    // --------------------------------------------
    pub(super) tree_style_opt_sel: TreeStyle,
    pub(super) node_ord_opt_sel: NodeOrd,
    pub(super) drawing_enabled: bool,
    pub(super) draw_tip_labs: bool,
    pub(super) draw_int_labs: bool,
    pub(super) draw_brnch_labs: bool,
    tip_labs_vis_max: usize,
    pub(super) labs_allowed: bool,
    // --------------------------------------------
    pub(super) cache_bnds: Cache,
    // --------------------------------------------
    // ltt_cnv_w: Float,
    // ltt_cnv_h: Float,
    ltt_cnv_vis_x0: Float,
    ltt_cnv_vis_y0: Float,
    pub(super) ltt_scr_id: &'static str,
    ltt_cnv_scrolled: bool,
    // --------------------------------------------
    node_size: Float,
    // node_size_min: Float,
    // node_size_max: Float,
    pub(super) tre_cnv_size_step: Float,

    tre_cnv_w: Float,
    pub(super) tre_cnv_h: Float,

    tre_cnv_vis_x0: Float,
    tre_cnv_vis_x0_rel: Float,
    tre_cnv_vis_x_mid: Float,
    tre_cnv_vis_x_mid_rel: Float,
    tre_cnv_vis_x1: Float,
    tre_cnv_vis_x1_rel: Float,

    tre_cnv_vis_y0: Float,
    pub(super) tre_cnv_vis_y0_rel: Float,
    tre_cnv_vis_y_mid: Float,
    tre_cnv_vis_y_mid_rel: Float,
    tre_cnv_vis_y1: Float,
    pub(super) tre_cnv_vis_y1_rel: Float,

    tre_cnv_scrolled: bool,

    tre_scr_h: Float,
    tre_scr_w: Float,

    pub(super) tre_scr_id: &'static str,
    pub(super) is_dirty: bool,
    // --------------------------------------------
}

#[derive(Debug, Clone)]
pub enum TvMsg {
    // --------------------------------------------
    BrnchLabSizeChanged(u16),
    BrnchLabVisChanged(bool),
    CnvHeightSelChanged(u16),
    CnvWidthSelChanged(u16),
    CnvZoomSelChanged(u16),
    IntLabSizeChanged(u16),
    IntLabVisChanged(bool),
    LttpVisChanged(bool),
    NextTree,
    NodeOrdOptChanged(NodeOrd),
    OpnAngleChanged(u16),
    PaneResized(ResizeEvent),
    PrevTree,
    Root(NodeId),
    RotAngleChanged(u16),
    SelectDeselectNode(NodeId),
    SetSidebarPos(SidebarPos),
    TipLabSizeChanged(u16),
    TipLabVisChanged(bool),
    TreesLoaded(Vec<Tree>),
    TreeStyOptChanged(TreeStyle),
    Unroot,
    // --------------------------------------------
    RootLenSelChanged(u16),
    // --------------------------------------------
    TreCnvScrolled(Viewport),
    LttCnvScrolled(Viewport),
    // --------------------------------------------
}

const TRE_CNV_PADDING: Float = 2e1;

impl TreeView {
    pub fn new(sel_sidebar_pos: SidebarPos) -> Self {
        let opn_angle_idx_sel = 359;
        let opn_angle = angle_from_idx(opn_angle_idx_sel);
        let rot_angle_idx_sel = 360;
        let rot_angle = angle_from_idx(rot_angle_idx_sel);

        let lab_size_idx_max = 24;
        let lab_size_min = 1e0;
        // let lab_size_max = lab_size_min * (lab_size_idx_max - 1) as Float;
        let tip_lab_size_idx_sel = 6;
        let int_lab_size_idx_sel = 6;
        let brnch_lab_size_idx_sel = 6;

        let tip_lab_size = lab_size_min * tip_lab_size_idx_sel as Float;
        let int_lab_size = lab_size_min * int_lab_size_idx_sel as Float;
        let brnch_lab_size = lab_size_min * brnch_lab_size_idx_sel as Float;

        Self {
            sidebar_pos_sel: sel_sidebar_pos,
            is_dirty: false,
            show_toolbar: true,
            show_sidebar: true,
            draw_tip_labs: false,
            draw_int_labs: false,
            draw_brnch_labs: false,
            drawing_enabled: false,
            // --------------------------------------------
            tip_labs_vis_max: 200,
            labs_allowed: false,
            // --------------------------------------------
            tree_style_opt_sel: TreeStyle::Phylogram,
            node_ord_opt_sel: NodeOrd::Unordered,
            // --------------------------------------------
            opn_angle_idx_min: 45,
            opn_angle_idx_sel,
            opn_angle,
            opn_angle_idx_max: 359,
            // --------------------------------------------
            rot_angle_idx_min: 360 - 180,
            rot_angle_idx_sel,
            rot_angle,
            rot_angle_idx_max: 360 + 180,
            // --------------------------------------------
            lab_size_idx_min: 1,
            tip_lab_size_idx_sel,
            int_lab_size_idx_sel,
            brnch_lab_size_idx_sel,
            lab_size_idx_max,
            // --------------------------------------------
            root_len_idx_min: 0,
            root_len_idx_sel: 10,
            root_len_idx_max: 100,
            // --------------------------------------------
            lab_size_min,
            // lab_size_max,
            lab_size_tip: tip_lab_size,
            lab_offset_tip: 3e0,
            lab_size_int: int_lab_size,
            lab_offset_int: 3e0,
            lab_size_brnch: brnch_lab_size,
            lab_offset_brnch: 0e0,
            // --------------------------------------------
            tre_cnv_size_step: 1e2,
            // --------------------------------------------
            tre_cnv_w_idx_min: 1,
            tre_cnv_w_idx_sel: 1,
            tre_cnv_w_idx_max: 24,
            tre_cnv_h_idx_min: 1,
            tre_cnv_h_idx_sel: 1,
            tre_cnv_h_idx_max: 24,
            tre_cnv_z_idx_min: 1,
            tre_cnv_z_idx_sel: 1,
            tre_cnv_z_idx_max: 24,
            // --------------------------------------------
            tre_scr_id: "tre",
            ltt_scr_id: "ltt",
            // --------------------------------------------
            ..Default::default()
        }
    }

    fn update_node_size(&mut self) {
        let tip_count: usize;
        if let Some(tree) = self.get_sel_tree() {
            tip_count = tree.tip_count();
        } else {
            return;
        }

        self.node_size = (self.tre_cnv_h - TRE_CNV_PADDING) / tip_count as Float;

        match self.tree_style_opt_sel {
            TreeStyle::Phylogram => {
                self.labs_allowed =
                    (self.tre_scr_h / self.node_size) as usize <= self.tip_labs_vis_max;
            }
            TreeStyle::Fan => {
                self.labs_allowed = tip_count <= self.tip_labs_vis_max * 5;
            }
        }
    }

    fn cnv_dim_recalc(&mut self) {
        let tip_count: usize;
        if let Some(tree) = self.get_sel_tree() {
            tip_count = tree.tip_count();
        } else {
            return;
        }

        match self.tree_style_opt_sel {
            TreeStyle::Phylogram => {
                let tre_cnv_size_step = self.tre_cnv_size_step.max(tip_count as Float * 1e0);
                let delta_w = (self.tre_cnv_w_idx_sel - 1) as Float * self.tre_cnv_size_step;
                let delta_h = (self.tre_cnv_h_idx_sel - 1) as Float * tre_cnv_size_step;
                self.tre_cnv_w = delta_w + self.tre_scr_w;
                self.tre_cnv_h = delta_h + self.tre_scr_h;
            }
            TreeStyle::Fan => {
                let delta = (self.tre_cnv_z_idx_sel - 1) as Float * self.tre_cnv_size_step;
                self.tre_cnv_w = delta + self.tre_scr_w;
                self.tre_cnv_h = delta + self.tre_scr_h;
            }
        }
    }

    pub fn update(&mut self, tv_msg: TvMsg) -> Task<TvMsg> {
        self.is_dirty = false;
        let mut task: Option<Task<TvMsg>> = None;
        match tv_msg {
            TvMsg::TreCnvScrolled(vp) => {
                self.tre_scr_w = vp.bounds().width;
                self.tre_scr_h = vp.bounds().height;

                self.tre_cnv_vis_x0 = vp.absolute_offset().x;
                self.tre_cnv_vis_x1 = self.tre_cnv_vis_x0 + self.tre_scr_w;
                self.tre_cnv_vis_x_mid = self.tre_cnv_vis_x0.midpoint(self.tre_cnv_vis_x1);

                self.tre_cnv_vis_y0 = vp.absolute_offset().y;
                self.tre_cnv_vis_y1 = self.tre_cnv_vis_y0 + self.tre_scr_h;
                self.tre_cnv_vis_y_mid = self.tre_cnv_vis_y0.midpoint(self.tre_cnv_vis_y1);

                if self.tree_style_opt_sel == TreeStyle::Phylogram {
                    if self.tre_cnv_scrolled && self.tre_cnv_vis_x0 != self.ltt_cnv_vis_x0 {
                        self.ltt_cnv_scrolled = false;
                        task = self.scroll_cnv_to_x(self.ltt_scr_id, self.tre_cnv_vis_x0);
                    } else {
                        self.tre_cnv_scrolled = true;
                    }
                }

                self.cnv_dim_recalc();
                self.update_node_size();

                self.tre_cnv_vis_x0_rel = self.tre_cnv_vis_x0 / self.tre_cnv_w;
                self.tre_cnv_vis_x_mid_rel = self.tre_cnv_vis_x_mid / self.tre_cnv_w;
                self.tre_cnv_vis_x1_rel = self.tre_cnv_vis_x1 / self.tre_cnv_w;

                self.tre_cnv_vis_y0_rel = self.tre_cnv_vis_y0 / self.tre_cnv_h;
                self.tre_cnv_vis_y_mid_rel = self.tre_cnv_vis_y_mid / self.tre_cnv_h;
                self.tre_cnv_vis_y1_rel = self.tre_cnv_vis_y1 / self.tre_cnv_h;

                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::LttCnvScrolled(vp) => {
                self.ltt_cnv_vis_x0 = vp.absolute_offset().x;
                self.ltt_cnv_vis_y0 = vp.absolute_offset().y;

                if self.tree_style_opt_sel == TreeStyle::Phylogram {
                    if self.ltt_cnv_scrolled && self.tre_cnv_vis_x0 != self.ltt_cnv_vis_x0 {
                        self.tre_cnv_scrolled = false;
                        task = self.scroll_cnv_to_x(self.tre_scr_id, self.ltt_cnv_vis_x0);
                    } else {
                        self.ltt_cnv_scrolled = true;
                    }
                }
            }

            TvMsg::PrevTree => {
                self.prev_tree();
                self.sort();
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::NextTree => {
                self.next_tree();
                self.sort();
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::NodeOrdOptChanged(node_ord_opt) => {
                self.node_ord_opt_sel = node_ord_opt;
                self.sort();
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::Unroot => {
                if let Some(tree) = self.get_sel_tree_mut() {
                    if let Some(_node) = tree.unroot() {
                        self.sort();
                        self.clear_cache_edge();
                        self.clear_cache_lab_tip();
                        self.clear_cache_lab_int();
                        self.clear_cache_lab_brnch();
                    }
                }
            }

            TvMsg::Root(node_id) => {
                if let Some(tree) = self.get_sel_tree_mut() {
                    if let Some(_node_id) = tree.root(&node_id) {
                        self.sort();
                        self.clear_cache_edge();
                        self.clear_cache_lab_tip();
                        self.clear_cache_lab_int();
                        self.clear_cache_lab_brnch();
                    }
                }
            }

            TvMsg::RootLenSelChanged(idx) => {
                self.root_len_idx_sel = idx;
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::TreesLoaded(trees) => {
                self.drawing_enabled = false;
                self.tree_states = Vec::new();

                let mut i: usize = 1;
                for tree in trees {
                    let mut tree_state = TreeState::new(i);
                    tree_state.init(tree);
                    self.tree_states.push(tree_state);
                    i += 1;
                }

                if !self.tree_states.is_empty() {
                    self.tree_state_idx_sel = Some(0);
                } else {
                    self.tree_state_idx_sel = None;
                }

                if let Some(_tree_pane_id) = &self.tree_pane_id {
                } else {
                    let (pane_grid, tree_pane_id) = PaneGridState::new(TvPane::Tree);
                    self.pane_grid = Some(pane_grid);
                    self.tree_pane_id = Some(tree_pane_id)
                }

                self.sort();
                self.cnv_dim_recalc();
                self.update_node_size();
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
                self.drawing_enabled = true;
            }

            TvMsg::CnvWidthSelChanged(idx) => {
                self.tre_cnv_w_idx_sel = idx;
                self.cnv_dim_recalc();
                self.update_node_size();
                if self.tre_cnv_vis_x0 > self.tre_cnv_size_step
                    && self.tre_cnv_vis_x1 < self.tre_cnv_w - self.tre_cnv_size_step
                {
                    task = self.scroll_tre_cnv(
                        self.tre_cnv_w * self.tre_cnv_vis_x_mid_rel,
                        self.tre_cnv_vis_y_mid,
                    );
                }
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::CnvHeightSelChanged(idx) => {
                self.tre_cnv_h_idx_sel = idx;
                self.cnv_dim_recalc();
                self.update_node_size();
                if self.tre_cnv_vis_y0 > self.node_size
                    && self.tre_cnv_vis_y1 < self.tre_cnv_h - self.node_size
                {
                    task = self.scroll_tre_cnv(
                        self.tre_cnv_vis_x_mid,
                        self.tre_cnv_h * self.tre_cnv_vis_y_mid_rel,
                    );
                }
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::CnvZoomSelChanged(idx) => {
                self.tre_cnv_z_idx_sel = idx;
                self.cnv_dim_recalc();
                self.update_node_size();
                task = self.scroll_tre_cnv(
                    self.tre_cnv_w * self.tre_cnv_vis_x_mid_rel,
                    self.tre_cnv_h * self.tre_cnv_vis_y_mid_rel,
                );
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::TreeStyOptChanged(tree_style_opt) => {
                self.tree_style_opt_sel = tree_style_opt;
                self.cnv_dim_recalc();
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::TipLabVisChanged(state) => {
                self.is_dirty = true;
                self.draw_tip_labs = state;
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::TipLabSizeChanged(idx) => {
                self.is_dirty = true;
                self.tip_lab_size_idx_sel = idx;
                self.lab_size_tip = self.lab_size_min * idx as Float;
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::IntLabVisChanged(state) => {
                self.is_dirty = true;
                self.draw_int_labs = state;
                self.clear_cache_lab_int();
            }

            TvMsg::IntLabSizeChanged(idx) => {
                self.is_dirty = true;
                self.int_lab_size_idx_sel = idx;
                self.lab_size_int = self.lab_size_min * idx as Float;
                self.clear_cache_lab_int();
            }

            TvMsg::BrnchLabVisChanged(state) => {
                self.is_dirty = true;
                self.draw_brnch_labs = state;
                self.clear_cache_lab_brnch();
            }

            TvMsg::BrnchLabSizeChanged(idx) => {
                self.is_dirty = true;
                self.brnch_lab_size_idx_sel = idx;
                self.lab_size_brnch = self.lab_size_min * idx as Float;
                self.clear_cache_lab_brnch();
            }

            TvMsg::SelectDeselectNode(node_id) => {
                if let Some(tree) = self.get_sel_tree_mut() {
                    tree.select_deselect_node(&node_id);
                }
            }

            TvMsg::OpnAngleChanged(idx) => {
                self.opn_angle_idx_sel = idx;
                self.opn_angle = angle_from_idx(idx);
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::RotAngleChanged(idx) => {
                self.rot_angle_idx_sel = idx;
                self.rot_angle = angle_from_idx(idx);
                self.clear_cache_edge();
                self.clear_cache_lab_tip();
                self.clear_cache_lab_int();
                self.clear_cache_lab_brnch();
            }

            TvMsg::PaneResized(ResizeEvent { split, ratio }) => {
                if let Some(pane_grid) = &mut self.pane_grid {
                    self.is_dirty = true;
                    pane_grid.resize(split, ratio);
                }
            }

            TvMsg::LttpVisChanged(show_lttp) => {
                self.show_lttp = show_lttp;
                self.show_hide_lttp();
            }

            TvMsg::SetSidebarPos(sidebar_pos) => {
                self.sidebar_pos_sel = sidebar_pos;
            }
        }

        // ----------------------------------------------------------------------------------------
        match task {
            Some(task) => task,
            None => Task::none(),
        }
    }

    pub fn view(&self) -> Element<TvMsg> {
        let sel_tree: &TreeState;

        if let Some(sel_tree_opt) = self.get_sel_tree() {
            sel_tree = sel_tree_opt;
        } else {
            return center(txt("No trees loaded")).into();
        }

        let mut main_col: Column<TvMsg> = Column::new();
        let mut main_row: Row<TvMsg> = Row::new();

        main_col = main_col.padding(0);
        main_col = main_col.spacing(0);
        main_row = main_row.padding(Padding { top: 0e0, right: 5e0, bottom: 5e0, left: 5e0 });
        main_row = main_row.spacing(5);

        if self.show_toolbar {
            main_col = main_col.push(toolbar(self, sel_tree));
        }

        if self.show_sidebar {
            match self.sidebar_pos_sel {
                SidebarPos::Left => {
                    main_row = main_row.push(sidebar(self, sel_tree));
                    main_row = main_row.push(content(self, sel_tree));
                }
                SidebarPos::Right => {
                    main_row = main_row.push(content(self, sel_tree));
                    main_row = main_row.push(sidebar(self, sel_tree));
                }
            }
        } else {
            main_row = main_row.push(content(self, sel_tree));
        }

        main_col = main_col.push(main_row);

        main_col.into()
    }

    pub(super) fn prev_tree_exists(&self) -> bool {
        match self.tree_state_idx_sel {
            Some(idx) => idx > 0,
            None => false,
        }
    }

    pub(super) fn next_tree_exists(&self) -> bool {
        match self.tree_state_idx_sel {
            Some(idx) => idx < self.tree_states.len() - 1,
            None => false,
        }
    }

    pub(super) fn get_sel_tree(&self) -> Option<&TreeState> {
        if let Some(sel_tree_state_idx) = self.tree_state_idx_sel {
            let sel_tree_state = &self.tree_states[sel_tree_state_idx];
            Some(sel_tree_state)
        } else {
            None
        }
    }

    fn get_sel_tree_mut(&mut self) -> Option<&mut TreeState> {
        if let Some(sel_tree_state_idx) = self.tree_state_idx_sel {
            let sel_tree_state = &mut self.tree_states[sel_tree_state_idx];
            Some(sel_tree_state)
        } else {
            None
        }
    }

    fn scroll_tre_cnv(&self, x: Float, y: Float) -> Option<Task<TvMsg>> {
        Some(scroll_to(
            self.tre_scr_id,
            AbsoluteOffset { x: x - self.tre_scr_w / 2e0, y: y - self.tre_scr_h / 2e0 },
        ))
    }

    fn scroll_cnv_to_x(&self, receiver_id: &'static str, x: Float) -> Option<Task<TvMsg>> {
        let y = match receiver_id {
            id if id == self.ltt_scr_id => self.ltt_cnv_vis_y0,
            id if id == self.tre_scr_id => self.tre_cnv_vis_y0,
            _ => 0e0,
        };
        Some(scroll_to(receiver_id, AbsoluteOffset { x, y }))
    }

    fn sort(&mut self) {
        let node_ord_opt = self.node_ord_opt_sel;
        if let Some(tree) = self.get_sel_tree_mut() {
            tree.sort(node_ord_opt);
        }
        self.is_dirty = true;
    }

    fn update_sel_tree_state_idx(&mut self, idx: Option<usize>) -> bool {
        if idx != self.tree_state_idx_sel {
            self.tree_state_idx_sel = idx;
            self.is_dirty = true;
            true
        } else {
            false
        }
    }

    fn prev_tree(&mut self) -> bool {
        let prev_idx = match self.tree_state_idx_sel {
            Some(idx) => {
                if idx > 0 {
                    Some(idx - 1)
                } else {
                    Some(idx)
                }
            }
            None => None,
        };

        self.update_sel_tree_state_idx(prev_idx)
    }

    fn next_tree(&mut self) -> bool {
        let next_idx = match self.tree_state_idx_sel {
            Some(idx) => {
                if idx < self.tree_states.len() - 1 {
                    Some(idx + 1)
                } else {
                    Some(idx)
                }
            }
            None => None,
        };
        self.update_sel_tree_state_idx(next_idx)
    }

    fn clear_cache_edge(&mut self) {
        if let Some(tree) = self.get_sel_tree_mut() {
            tree.clear_cache_edge();
        }
    }

    fn clear_cache_lab_tip(&mut self) {
        if let Some(tree) = self.get_sel_tree_mut() {
            tree.clear_cache_lab_tip();
        }
    }

    fn clear_cache_lab_int(&mut self) {
        if let Some(tree) = self.get_sel_tree_mut() {
            tree.clear_cache_lab_int();
        }
    }

    fn clear_cache_lab_brnch(&mut self) {
        if let Some(tree) = self.get_sel_tree_mut() {
            tree.clear_cache_lab_brnch();
        }
    }

    fn show_hide_lttp(&mut self) {
        if let Some(pane_grid) = &mut self.pane_grid {
            if let Some(lttp_pane_id) = self.lttp_pane_id {
                if !self.show_lttp {
                    pane_grid.close(lttp_pane_id);
                    self.lttp_pane_id = None;
                }
            } else if self.show_lttp {
                if let Some(tree_pane_id) = self.tree_pane_id {
                    if let Some((lttp_pane_id, _split)) =
                        pane_grid.split(Axis::Horizontal, tree_pane_id, TvPane::LttPlot)
                    {
                        self.lttp_pane_id = Some(lttp_pane_id);
                    }
                }
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
pub enum TreeStyle {
    #[default]
    Phylogram,
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
pub(super) const TREE_STYLE_OPTS: [TreeStyle; 2] = [TreeStyle::Phylogram, TreeStyle::Fan];

impl Display for NodeOrd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            NodeOrd::Unordered => "Unordered",
            NodeOrd::Ascending => "Ascending",
            NodeOrd::Descending => "Descending",
        })
    }
}

impl Display for TreeStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            TreeStyle::Phylogram => "Phylogram",
            TreeStyle::Fan => "Fan",
        })
    }
}

#[derive(Debug)]
pub(crate) enum TvPane {
    Tree,
    LttPlot,
}

impl Display for TvPane {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl From<TvPane> for String {
    fn from(value: TvPane) -> Self {
        (&value).into()
    }
}

impl From<&TvPane> for String {
    fn from(value: &TvPane) -> Self {
        match value {
            TvPane::Tree => String::from("Tree"),
            TvPane::LttPlot => String::from("LttPlot"),
        }
    }
}

fn angle_from_idx(idx: u16) -> Float {
    (idx as Float).to_radians()
}
