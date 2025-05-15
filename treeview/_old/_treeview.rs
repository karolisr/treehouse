impl TreeView {
    pub fn new() -> Self {
        Self {
            draw_legend: true,
            show_cursor_line: true,
            tip_brnch_labs_allowed: true,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct TreeView {
    pub(crate) show_cursor_line: bool,
    pub(crate) show_statusbar: bool,
    pub(crate) draw_legend: bool,
    pub(crate) ltt_cnv_scrolled: bool,
    pub(crate) tre_cnv_scrolled: bool,
    pub(crate) tre_cnv_x0: Float,
    pub(crate) tre_cnv_y0: Float,
    pub(crate) tre_cnv_y1: Float,
    pub(crate) ltt_cnv_w: Float,
    pub(crate) ltt_cnv_x0: Float,
    pub(crate) ltt_cnv_y0: Float,
}

#[derive(Debug, Clone)]
pub enum TreeViewMsg {
    NodeSizeSelectionChanged(u16),
    CanvasWidthSelectionChanged(u16),
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
    // -------------------------------------------
    TreCnvScrolled(iced::widget::scrollable::Viewport),
    LttCnvScrolled(iced::widget::scrollable::Viewport),
    ScrollTo { x: f32, y: f32 },
    ScrollToX { sender: &'static str, x: f32 },
    // -------------------------------------------
    CursorOnTreCnv { x: Option<f32> },
    CursorOnLttCnv { x: Option<f32> },
    // -------------------------------------------
    Search(String),
    NextResult,
    PrevResult,
    AddFoundToSelection,
    RemFoundFromSelection,
    TipOnlySearchSelectionChanged(bool),
    // -------------------------------------------
}
