pub enum TreeViewMsg {
    // -------------------------------------------
    Search(String),
    NextResult,
    PrevResult,
    AddFoundToSelection,
    RemFoundFromSelection,
    TipOnlySearchSelectionChanged(bool),
    // -------------------------------------------
}
