use crate::Float;
use crate::PI;
use crate::RectVals;
use crate::cnv_plot::PlotCnv;
use crate::elements::*;
use crate::treestate::TreeState;
use dendros::NodeId;
use dendros::Tree;
use iced::Element;
use iced::Padding;
use iced::Rectangle;
use iced::Size;
use iced::Task;
use iced::widget::canvas::Cache;
use iced::widget::pane_grid::Axis;
use iced::widget::pane_grid::Pane;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::pane_grid::State as PaneGridState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Default)]
pub struct TreeView {
    pub(super) tree_vals: RectVals<Float>,
    // --------------------------------------------
    pub(super) tree_states: Vec<TreeState>,
    sel_tree_state_idx: Option<usize>,
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
    pub(super) sel_sidebar_pos: SidebarPos,
    // --------------------------------------------
    pub(super) min_tre_cnv_h_idx: u16,
    pub(super) min_tre_cnv_w_idx: u16,
    pub(super) min_tre_cnv_z_idx: u16,
    pub(super) max_tre_cnv_h_idx: u16,
    pub(super) max_tre_cnv_w_idx: u16,
    pub(super) max_tre_cnv_z_idx: u16,
    pub(super) sel_tre_cnv_h_idx: u16,
    pub(super) sel_tre_cnv_w_idx: u16,
    pub(super) sel_tre_cnv_z_idx: u16,
    // --------------------------------------------
    pub(super) min_lab_size_idx: u16,
    pub(super) sel_tip_lab_size_idx: u16,
    pub(super) sel_int_lab_size_idx: u16,
    pub(super) sel_brnch_lab_size_idx: u16,
    pub(super) max_lab_size_idx: u16,
    // --------------------------------------------
    pub(super) min_lab_size: Float,
    pub(super) tip_lab_size: Float,
    pub(super) tip_lab_offset: Float,
    pub(super) int_lab_size: Float,
    pub(super) int_lab_offset: Float,
    pub(super) brnch_lab_size: Float,
    pub(super) brnch_lab_offset: Float,
    // --------------------------------------------
    pub(super) tre_cnv_w: Float,
    pub(super) tre_cnv_h: Float,
    // --------------------------------------------
    pub(super) opn_angle: Float,
    pub(super) rot_angle: Float,
    pub(super) sel_opn_angle_idx: u16,
    pub(super) sel_rot_angle_idx: u16,
    pub(super) min_opn_angle_idx: u16,
    pub(super) min_rot_angle_idx: u16,
    pub(super) max_opn_angle_idx: u16,
    pub(super) max_rot_angle_idx: u16,
    // --------------------------------------------
    pub(super) sel_tree_style_opt: TreeStyle,
    pub(super) sel_node_ord_opt: NodeOrd,
    pub(super) drawing_enabled: bool,
    pub(super) draw_tip_labs: bool,
    pub(super) draw_int_labs: bool,
    pub(super) draw_brnch_labs: bool,
    pub(super) tip_brnch_labs_allowed: bool,
    // --------------------------------------------
    pub(super) cache_bounds: Cache,
}

#[derive(Debug, Clone)]
pub enum TvMsg {
    // --------------------------------------------
    LttpVisChanged(bool),
    NextTree,
    TreeIdxChanged(Option<usize>),
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
    BrnchLabVisChanged(bool),
    IntLabVisChanged(bool),
    IntLabSizeChanged(u16),
    BrnchLabSizeChanged(u16),
    CnvHeightChanged(u16),
    TreesLoaded(Vec<Tree>),
    TreeStyOptChanged(TreeStyle),
    TreeUpdated,
    CnvWidthChanged(u16),
    CnvZoomChanged(u16),
    Unroot,
    RectValsChanged(RectVals<Float>),
    Refresh,
    CnvDimRecalc,
}

fn angle_from_idx(idx: u16) -> Float {
    idx as Float / 360e0 * 2e0 * PI
}

impl TreeView {
    pub fn new(sel_sidebar_pos: SidebarPos) -> Self {
        let sel_opn_angle_idx = 359;
        let opn_angle = angle_from_idx(sel_opn_angle_idx);
        let sel_rot_angle_idx = 360;
        let rot_angle = angle_from_idx(sel_rot_angle_idx);

        let min_lab_size = 1e0;
        let sel_tip_lab_size_idx = 10;
        let sel_int_lab_size_idx = 10;
        let sel_brnch_lab_size_idx = 10;

        let tip_lab_size = min_lab_size * sel_tip_lab_size_idx as Float;
        let int_lab_size = min_lab_size * sel_int_lab_size_idx as Float;
        let brnch_lab_size = min_lab_size * sel_brnch_lab_size_idx as Float;

        Self {
            sel_sidebar_pos,
            show_toolbar: true,
            show_sidebar: true,
            draw_tip_labs: true,
            draw_int_labs: true,
            draw_brnch_labs: true,
            drawing_enabled: true,
            // --------------------------------------------
            tip_brnch_labs_allowed: true,
            // --------------------------------------------
            sel_tree_style_opt: TreeStyle::Phylogram,
            sel_node_ord_opt: NodeOrd::Ascending,
            // --------------------------------------------
            min_opn_angle_idx: 45,
            sel_opn_angle_idx,
            opn_angle,
            max_opn_angle_idx: 359,
            // --------------------------------------------
            min_rot_angle_idx: 360 - 180,
            sel_rot_angle_idx,
            rot_angle,
            max_rot_angle_idx: 360 + 180,
            // --------------------------------------------
            min_lab_size_idx: 1,
            sel_tip_lab_size_idx,
            sel_int_lab_size_idx,
            sel_brnch_lab_size_idx,
            max_lab_size_idx: 24,
            // --------------------------------------------
            min_lab_size,
            tip_lab_size,
            tip_lab_offset: 0e0,
            int_lab_size,
            int_lab_offset: 0e0,
            brnch_lab_size,
            brnch_lab_offset: 0e0,
            // --------------------------------------------
            min_tre_cnv_h_idx: 1,
            sel_tre_cnv_h_idx: 1,
            max_tre_cnv_h_idx: 24,
            min_tre_cnv_w_idx: 1,
            sel_tre_cnv_w_idx: 1,
            max_tre_cnv_w_idx: 24,
            min_tre_cnv_z_idx: 1,
            sel_tre_cnv_z_idx: 1,
            max_tre_cnv_z_idx: 24,

            ..Default::default()
        }
    }

    pub fn update(&mut self, tv_msg: TvMsg) -> Task<TvMsg> {
        match tv_msg {
            // --- TvMsg::TreeIdxChanged ----------------------------------------------------------
            TvMsg::PrevTree => {
                let new_idx = match self.sel_tree_state_idx {
                    Some(idx) => {
                        if idx > 0 {
                            Some(idx - 1)
                        } else {
                            Some(idx)
                        }
                    }
                    None => None,
                };
                Task::done(TvMsg::TreeIdxChanged(new_idx))
            }

            TvMsg::NextTree => {
                let new_idx = match self.sel_tree_state_idx {
                    Some(idx) => {
                        if idx < self.tree_states.len() - 1 {
                            Some(idx + 1)
                        } else {
                            Some(idx)
                        }
                    }
                    None => None,
                };
                Task::done(TvMsg::TreeIdxChanged(new_idx))
            }

            // --- TvMsg::TreeUpdated -------------------------------------------------------------
            TvMsg::TreeIdxChanged(idx) => {
                if idx != self.sel_tree_state_idx {
                    self.sel_tree_state_idx = idx;
                    Task::done(TvMsg::TreeUpdated)
                } else {
                    Task::none()
                }
            }

            TvMsg::NodeOrdOptChanged(node_ord_opt) => {
                self.sel_node_ord_opt = node_ord_opt;
                Task::done(TvMsg::TreeUpdated)
            }

            TvMsg::Unroot => {
                if let Some(tree) = self.get_sel_tree_mut() {
                    if let Some(_node) = tree.unroot() {
                        Task::done(TvMsg::TreeUpdated)
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }

            TvMsg::Root(node_id) => {
                if let Some(tree) = self.get_sel_tree_mut() {
                    if let Some(_node_id) = tree.root(&node_id) {
                        Task::done(TvMsg::TreeUpdated)
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }

            TvMsg::TreesLoaded(trees) => {
                self.tree_states = Vec::new();

                let mut i: usize = 1;
                for tree in trees {
                    let mut tree_state = TreeState::new(i);
                    tree_state.init(tree);
                    self.tree_states.push(tree_state);
                    i += 1;
                }

                if !self.tree_states.is_empty() {
                    self.sel_tree_state_idx = Some(0);
                } else {
                    self.sel_tree_state_idx = None;
                }

                if let Some(_tree_pane_id) = &self.tree_pane_id {
                } else {
                    let (pane_grid, tree_pane_id) = PaneGridState::new(TvPane::Tree);
                    self.pane_grid = Some(pane_grid);
                    self.tree_pane_id = Some(tree_pane_id)
                }

                self.update_lttp_visibility();

                Task::done(TvMsg::TreeUpdated)
            }
            // --- TvMsg::CnvDimRecalc ------------------------------------------------------------
            TvMsg::TreeStyOptChanged(tree_style_opt) => {
                self.sel_tree_style_opt = tree_style_opt;
                Task::done(TvMsg::CnvDimRecalc)
            }

            TvMsg::CnvWidthChanged(idx) => {
                self.sel_tre_cnv_w_idx = idx;
                Task::done(TvMsg::CnvDimRecalc)
            }

            TvMsg::CnvHeightChanged(idx) => {
                self.sel_tre_cnv_h_idx = idx;
                Task::done(TvMsg::CnvDimRecalc)
            }

            TvMsg::CnvZoomChanged(idx) => {
                self.sel_tre_cnv_z_idx = idx;
                Task::done(TvMsg::CnvDimRecalc)
            }

            // --- TvMsg::Refresh -----------------------------------------------------------------
            TvMsg::TreeUpdated => {
                self.sort();
                Task::done(TvMsg::Refresh)
            }

            TvMsg::CnvDimRecalc => {
                let size_delta = 1e2;
                match self.sel_tree_style_opt {
                    TreeStyle::Phylogram => {
                        self.tre_cnv_w = (self.sel_tre_cnv_w_idx - 1) as Float * size_delta;
                        self.tre_cnv_h = (self.sel_tre_cnv_h_idx - 1) as Float * size_delta;
                    }
                    TreeStyle::Fan => {
                        self.tre_cnv_w = (self.sel_tre_cnv_z_idx - 1) as Float * size_delta;
                        self.tre_cnv_h = (self.sel_tre_cnv_z_idx - 1) as Float * size_delta;
                    }
                }
                // Task::done(TvMsg::Refresh)
                Task::none()
            }

            TvMsg::TipLabVisChanged(state) => {
                self.draw_tip_labs = state;
                Task::done(TvMsg::Refresh)
            }

            TvMsg::TipLabSizeChanged(idx) => {
                self.sel_tip_lab_size_idx = idx;
                self.tip_lab_size = self.min_lab_size * idx as Float;
                Task::done(TvMsg::Refresh)
            }

            TvMsg::IntLabVisChanged(state) => {
                self.draw_int_labs = state;
                Task::done(TvMsg::Refresh)
            }

            TvMsg::IntLabSizeChanged(idx) => {
                self.sel_int_lab_size_idx = idx;
                self.int_lab_size = self.min_lab_size * idx as Float;
                Task::done(TvMsg::Refresh)
            }

            TvMsg::BrnchLabVisChanged(state) => {
                self.draw_brnch_labs = state;
                Task::done(TvMsg::Refresh)
            }

            TvMsg::BrnchLabSizeChanged(idx) => {
                self.sel_brnch_lab_size_idx = idx;
                self.brnch_lab_size = self.min_lab_size * idx as Float;
                Task::done(TvMsg::Refresh)
            }

            TvMsg::SelectDeselectNode(node_id) => {
                if let Some(tree) = self.get_sel_tree_mut() {
                    tree.select_deselect_node(&node_id);
                    Task::done(TvMsg::Refresh)
                } else {
                    Task::none()
                }
            }

            TvMsg::OpnAngleChanged(idx) => {
                self.sel_opn_angle_idx = idx;
                self.opn_angle = angle_from_idx(idx);
                Task::done(TvMsg::Refresh)
            }

            TvMsg::RotAngleChanged(idx) => {
                self.sel_rot_angle_idx = idx;
                self.rot_angle = angle_from_idx(idx);
                Task::done(TvMsg::Refresh)
            }

            // --- None ---------------------------------------------------------------------------
            TvMsg::Refresh => {
                self.recalc_all_nodepoints();
                if let Some(tree) = self.get_sel_tree_mut() {
                    tree.clear_cache_edge();
                    tree.clear_cache_lab_tip();
                    tree.clear_cache_lab_int();
                    tree.clear_cache_lab_brnch();
                }
                self.cache_bounds.clear();
                Task::none()
            }

            TvMsg::RectValsChanged(tree_vals) => {
                self.tree_vals = tree_vals;
                Task::none()
            }

            TvMsg::PaneResized(ResizeEvent { split, ratio }) => {
                if let Some(pane_grid) = &mut self.pane_grid {
                    pane_grid.resize(split, ratio);
                }
                Task::none()
            }

            TvMsg::LttpVisChanged(show_lttp) => {
                self.show_lttp = show_lttp;
                self.update_lttp_visibility();
                // Task::done(TvMsg::ScrollToX { sender: "tre", x: self.tre_cnv_x0 })
                Task::none()
            }

            TvMsg::SetSidebarPos(sidebar_pos) => {
                self.sel_sidebar_pos = sidebar_pos;
                Task::none()
            }
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
            match self.sel_sidebar_pos {
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
        match self.sel_tree_state_idx {
            Some(idx) => idx > 0,
            None => false,
        }
    }

    pub(super) fn next_tree_exists(&self) -> bool {
        match self.sel_tree_state_idx {
            Some(idx) => idx < self.tree_states.len() - 1,
            None => false,
        }
    }

    pub(super) fn get_sel_tree(&self) -> Option<&TreeState> {
        if let Some(sel_tree_state_idx) = self.sel_tree_state_idx {
            let sel_tree_state = &self.tree_states[sel_tree_state_idx];
            Some(sel_tree_state)
        } else {
            None
        }
    }

    fn get_sel_tree_mut(&mut self) -> Option<&mut TreeState> {
        if let Some(sel_tree_state_idx) = self.sel_tree_state_idx {
            let sel_tree_state = &mut self.tree_states[sel_tree_state_idx];
            Some(sel_tree_state)
        } else {
            None
        }
    }

    fn sort(&mut self) {
        let node_ord_opt = self.sel_node_ord_opt;
        if let Some(tree) = self.get_sel_tree_mut() {
            tree.sort(node_ord_opt);
        }
    }

    fn recalc_all_nodepoints(&mut self) {
        let tree_vals = self.tree_vals;
        let rot_angle = self.rot_angle;
        let opn_angle = self.opn_angle;
        let sel_tree_style_opt = self.sel_tree_style_opt;
        if let Some(tree) = self.get_sel_tree_mut() {
            tree.all_nodepoints_calc(tree_vals, rot_angle, opn_angle, sel_tree_style_opt);
        }
    }

    fn update_lttp_visibility(&mut self) {
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
