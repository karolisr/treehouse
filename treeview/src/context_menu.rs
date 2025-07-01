use crate::TvMsg;

#[derive(Debug, Clone)]
pub enum TreeViewContextMenuAction {
    Root,
    AddCladeLabel,
}

#[derive(Debug, Clone)]
pub struct TreeViewContextMenuItem {
    // pub action: TreeViewContextMenuAction,
    pub tv_msg: TvMsg,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone)]
pub struct TreeViewContextMenuListing {
    items: Vec<TreeViewContextMenuItem>,
}

impl TreeViewContextMenuListing {
    pub fn new() -> Self { Default::default() }

    pub fn push(&mut self, tv_msg: TvMsg) {
        let enabled = true;

        let item: TreeViewContextMenuItem = TreeViewContextMenuItem { tv_msg, enabled };
        self.items.push(item);
    }

    pub fn items(&self) -> &[TreeViewContextMenuItem] { &self.items }
}
