mod cnv_plot;
mod cnv_tree;
mod styles;
mod tree_state;
mod ui;
mod update;
mod view;

use crate::Float;
pub(crate) use cnv_plot::PlotCnv;
pub(crate) use cnv_tree::TreeCnv;
use dendros::Tree;
use iced::widget::pane_grid::{DragEvent, Pane, ResizeEvent, State as PaneGridState};
use std::fmt::{Display, Formatter, Result};
pub(crate) use tree_state::{TreeState, TreeStateMsg};

#[derive(Default)]
pub struct TreeView {
    pub(crate) trees: Vec<TreeState>,
    pub(crate) sel_tree_idx: Option<usize>,

    pub(crate) panes: Option<PaneGridState<TreeViewPane>>,
    pub(crate) tree_pane_id: Option<Pane>,
    pub(crate) lttp_pane_id: Option<Pane>,

    pub(crate) show_cursor_line: bool,
    pub(crate) show_ltt: bool,
    pub(crate) show_sidebar: bool,
    pub(crate) show_toolbar: bool,
    pub(crate) show_statusbar: bool,
    pub(crate) sidebar_position: SidebarLocation,
    pub(crate) tip_brnch_labs_allowed: bool,

    pub(crate) draw_brnch_labs: bool,
    pub(crate) draw_int_labs: bool,
    pub(crate) draw_legend: bool,
    pub(crate) draw_tip_labs: bool,

    pub(crate) max_lab_size_idx: u16,
    pub(crate) max_node_size_idx: u16,
    pub(crate) max_opn_angle_idx: u16,
    pub(crate) max_rot_angle_idx: u16,
    pub(crate) max_tre_cnv_w_idx: u16,

    pub(crate) min_lab_size_idx: u16,
    pub(crate) min_node_size_idx: u16,
    pub(crate) min_opn_angle_idx: u16,
    pub(crate) min_rot_angle_idx: u16,
    pub(crate) min_tre_cnv_w_idx: u16,

    pub(crate) sel_tree_style_opt: TreeStyle,
    pub(crate) sel_node_ord_opt: NodeOrd,
    //
    pub(crate) sel_brnch_lab_size_idx: u16,
    pub(crate) sel_int_lab_size_idx: u16,
    pub(crate) sel_node_size_idx: u16,
    pub(crate) sel_opn_angle_idx: u16,
    pub(crate) sel_rot_angle_idx: u16,
    pub(crate) sel_tip_lab_size_idx: u16,
    pub(crate) sel_tre_cnv_w_idx: u16,
    //
    pub(crate) opn_angle: Float,
    pub(crate) rot_angle: Float,
    //
    pub(crate) tre_cnv: TreeCnv,
    pub(crate) ltt_cnv: PlotCnv,
    pub(crate) ltt_cnv_scrolled: bool,
    pub(crate) tre_cnv_scrolled: bool,
    pub(crate) min_tre_cnv_h: Float,
    pub(crate) min_tre_cnv_w: Float,
    pub(crate) tree_scroll_w: Float,
    pub(crate) tree_scroll_h: Float,
    pub(crate) tre_cnv_w: Float,
    pub(crate) tre_cnv_h: Float,
    pub(crate) tre_cnv_x0: Float,
    pub(crate) tre_cnv_y0: Float,
    pub(crate) tre_cnv_y1: Float,
    pub(crate) ltt_cnv_w: Float,
    pub(crate) ltt_cnv_x0: Float,
    pub(crate) ltt_cnv_y0: Float,
}

impl TreeView {
    pub fn new() -> Self {
        Self {
            show_toolbar: true,
            show_sidebar: true,
            show_statusbar: false,

            draw_brnch_labs: true,
            draw_int_labs: true,
            draw_legend: true,
            draw_tip_labs: true,
            show_cursor_line: true,
            show_ltt: true,

            sel_tree_style_opt: TreeStyle::Phylogram,
            sel_node_ord_opt: NodeOrd::Unordered,

            tip_brnch_labs_allowed: true,

            max_lab_size_idx: 24,
            max_node_size_idx: 24,
            max_opn_angle_idx: 359,
            max_rot_angle_idx: 360 + 180,
            max_tre_cnv_w_idx: 24,
            min_lab_size_idx: 1,
            min_node_size_idx: 1,
            min_opn_angle_idx: 45,
            min_rot_angle_idx: 360 - 180,
            min_tre_cnv_w_idx: 1,
            sel_brnch_lab_size_idx: 4,
            sel_int_lab_size_idx: 4,
            sel_node_size_idx: 1,
            sel_opn_angle_idx: 359,
            sel_rot_angle_idx: 360,
            sel_tip_lab_size_idx: 4,
            sel_tre_cnv_w_idx: 1,

            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum TreeViewMsg {
    PaneDragged(DragEvent),
    PaneResized(ResizeEvent),
    // -------------------------------------------
    // PaneGridMsg(PaneGridMsg),
    TreeStateMsg(TreeStateMsg),
    // -------------------------------------------
    TreeLoaded(Tree),
    TreeUpdated,
    // -------------------------------------------
    SetSidebarLocation(SidebarLocation),
    // -------------------------------------------
    TreeStyleOptionChanged(TreeStyle),
    NodeOrdOptChanged(NodeOrd),
    // -------------------------------------------
    NodeSizeSelectionChanged(u16),
    CanvasWidthSelectionChanged(u16),
    OpnAngleSelectionChanged(u16),
    RotAngleSelectionChanged(u16),
    // -------------------------------------------
    TipLabelVisibilityChanged(bool),
    IntLabelVisibilityChanged(bool),
    BranchLabelVisibilityChanged(bool),
    // -------------------------------------------
    TipLabelSizeSelectionChanged(u16),
    IntLabelSizeSelectionChanged(u16),
    BranchLabelSizeSelectionChanged(u16),
    // -------------------------------------------
    LegendVisibilityChanged(bool),
    CursorLineVisibilityChanged(bool),
    LttPlotVisibilityChanged(bool),
    // -------------------------------------------
    // SelectDeselectNode(NodeId),
    // SelectNode(NodeId),
    // DeselectNode(NodeId),
    // -------------------------------------------
    // Search(String),
    // NextResult,
    // PrevResult,
    // AddFoundToSelection,
    // RemFoundFromSelection,
    // TipOnlySearchSelectionChanged(bool),
    // -------------------------------------------
    TreCnvScrolled(iced::widget::scrollable::Viewport),
    LttCnvScrolled(iced::widget::scrollable::Viewport),
    ScrollTo { x: f32, y: f32 },
    ScrollToX { sender: &'static str, x: f32 },
    // -------------------------------------------
    // CursorOnTreCnv { x: Option<f32> },
    // CursorOnLttCnv { x: Option<f32> },
    // -------------------------------------------
}

#[derive(Debug)]
pub(crate) enum TreeViewPane {
    Tree,
    LttPlot,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SidebarLocation {
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

pub(crate) const NODE_ORD_OPTS: [NodeOrd; 3] =
    [NodeOrd::Unordered, NodeOrd::Ascending, NodeOrd::Descending];

pub(crate) const TREE_STYLE_OPTS: [TreeStyle; 2] = [TreeStyle::Phylogram, TreeStyle::Fan];

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

// impl Default for TreeView {
//     fn default() -> Self {
//         let pane_cfg_empty = PaneGridCfg::Pane(TreeViewPane::Empty);
//         let pane_grid_state = PaneGridState::with_configuration(pane_cfg_empty);

//         Self {
//             pane_grid_state,
//             trees: Default::default(),
//             sel_tree_idx: Default::default(),
//             show_cursor_line: Default::default(),
//             show_ltt: Default::default(),
//             show_sidebar: Default::default(),
//             show_toolbar: Default::default(),
//             show_statusbar: Default::default(),
//             sidebar_position: Default::default(),
//             tip_brnch_labs_allowed: Default::default(),
//             draw_brnch_labs: Default::default(),
//             draw_int_labs: Default::default(),
//             draw_legend: Default::default(),
//             draw_tip_labs: Default::default(),
//             max_lab_size_idx: Default::default(),
//             max_node_size_idx: Default::default(),
//             max_opn_angle_idx: Default::default(),
//             max_rot_angle_idx: Default::default(),
//             max_tre_cnv_w_idx: Default::default(),
//             min_lab_size_idx: Default::default(),
//             min_node_size_idx: Default::default(),
//             min_opn_angle_idx: Default::default(),
//             min_rot_angle_idx: Default::default(),
//             min_tre_cnv_w_idx: Default::default(),
//             sel_tree_style_opt: Default::default(),
//             sel_node_ord_opt: Default::default(),
//             sel_brnch_lab_size_idx: Default::default(),
//             sel_int_lab_size_idx: Default::default(),
//             sel_node_size_idx: Default::default(),
//             sel_opn_angle_idx: Default::default(),
//             sel_rot_angle_idx: Default::default(),
//             sel_tip_lab_size_idx: Default::default(),
//             sel_tre_cnv_w_idx: Default::default(),
//             opn_angle: Default::default(),
//             rot_angle: Default::default(),
//             tre_cnv: Default::default(),
//             ltt_cnv: Default::default(),
//             ltt_cnv_scrolled: Default::default(),
//             tre_cnv_scrolled: Default::default(),
//             min_tre_cnv_h: Default::default(),
//             min_tre_cnv_w: Default::default(),
//             tree_scroll_w: Default::default(),
//             tree_scroll_h: Default::default(),
//             tre_cnv_w: Default::default(),
//             tre_cnv_h: Default::default(),
//             tre_cnv_x0: Default::default(),
//             tre_cnv_y0: Default::default(),
//             tre_cnv_y1: Default::default(),
//             ltt_cnv_w: Default::default(),
//             ltt_cnv_x0: Default::default(),
//             ltt_cnv_y0: Default::default(),
//         }
//     }
// }
