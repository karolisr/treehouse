mod cnv_plot;
mod cnv_tree;
mod pane_grid_main;
mod tree_state;
mod ui;
mod update;
mod view;

use pane_grid_main::{PaneGridMain, PaneGridMainMsg};
use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub struct TreeView {
    pub(crate) pane_grid_main: PaneGridMain,
    pub(crate) show_cursor_line: bool,
    pub(crate) show_ltt: bool,
    pub(crate) show_sidebar: bool,
    pub(crate) show_toolbar: bool,
    pub(crate) show_statusbar: bool,
    pub(crate) sidebar_position: SideBarPosition,
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

    pub(crate) sel_brnch_lab_size_idx: u16,
    pub(crate) sel_int_lab_size_idx: u16,
    pub(crate) sel_node_ord_opt: NodeOrdering,
    pub(crate) sel_node_size_idx: u16,
    pub(crate) sel_opn_angle_idx: u16,
    pub(crate) sel_rot_angle_idx: u16,
    pub(crate) sel_tip_lab_size_idx: u16,
    pub(crate) sel_tre_cnv_w_idx: u16,
    pub(crate) sel_tree_style_opt: TreeStyle,
    //
    pub(crate) ltt_cnv_scrolled: bool,
    pub(crate) tre_cnv_scrolled: bool,
    pub(crate) opn_angle: Float,
    pub(crate) rot_angle: Float,
    pub(crate) min_tre_cnv_h: Float,
    pub(crate) min_tre_cnv_w: Float,
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
            pane_grid_main: PaneGridMain::new(),

            show_sidebar: true,
            show_toolbar: true,
            show_statusbar: true,

            draw_brnch_labs: false,
            draw_int_labs: false,
            draw_legend: false,
            draw_tip_labs: false,
            show_cursor_line: false,
            show_ltt: false,
            tip_brnch_labs_allowed: false,

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
            sel_node_ord_opt: NodeOrdering::Unordered,
            sel_node_size_idx: 1,
            sel_opn_angle_idx: 359,
            sel_rot_angle_idx: 360,
            sel_tip_lab_size_idx: 4,
            sel_tre_cnv_w_idx: 1,
            sel_tree_style_opt: TreeStyle::Phylogram,

            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum TreeViewMsg {
    BranchLabelSizeSelectionChanged(u16),
    BranchLabelVisibilityChanged(bool),
    CanvasWidthSelectionChanged(u16),
    CursorLineVisibilityChanged(bool),
    IntLabelSizeSelectionChanged(u16),
    IntLabelVisibilityChanged(bool),
    LegendVisibilityChanged(bool),
    LttVisibilityChanged(bool),
    NodeOrderingOptionChanged(NodeOrdering),
    NodeSizeSelectionChanged(u16),
    OpnAngleSelectionChanged(u16),
    RotAngleSelectionChanged(u16),
    TipLabelSizeSelectionChanged(u16),
    TipLabelVisibilityChanged(bool),
    TreeStyleOptionChanged(TreeStyle),
    TreeWinPaneGridMsg(PaneGridMainMsg),
    //
    Unroot,
    Root(NodeId),
    AddFoundToSelection,
    CursorOnLttCnv { x: Option<f32> },
    CursorOnTreCnv { x: Option<f32> },
    DeselectNode(NodeId),
    EnableDrawing,
    Init,
    LttCnvScrolled(ScrollableViewport),
    NextResult,
    OpenFile,
    PrevResult,
    Refresh,
    RemFoundFromSelection,
    ScrollTo { x: f32, y: f32 },
    ScrollToX { sender: &'static str, x: f32 },
    Search(String),
    SelectDeselectNode(NodeId),
    SelectNode(NodeId),
    TipOnlySearchSelectionChanged(bool),
    TreCnvScrolled(ScrollableViewport),
    TreeUpdated(Tree),
    WindowResized(f32, f32),
}

#[derive(Default)]
pub(crate) enum SideBarPosition {
    Left,
    #[default]
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TreeStyle {
    #[default]
    Phylogram,
    Fan,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeOrdering {
    #[default]
    Unordered,
    Ascending,
    Descending,
}

pub(crate) const NODE_ORDERING_OPTIONS: [NodeOrdering; 3] =
    [NodeOrdering::Unordered, NodeOrdering::Ascending, NodeOrdering::Descending];

pub(crate) const TREE_STYLE_OPTIONS: [TreeStyle; 2] = [TreeStyle::Phylogram, TreeStyle::Fan];

impl Display for NodeOrdering {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            NodeOrdering::Unordered => "Unordered",
            NodeOrdering::Ascending => "Ascending",
            NodeOrdering::Descending => "Descending",
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
