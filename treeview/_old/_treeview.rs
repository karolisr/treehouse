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

pub enum TreeViewMsg {
    // -------------------------------------------
    LegendVisibilityChanged(bool),
    CursorLineVisibilityChanged(bool),
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
