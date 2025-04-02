use crate::{Tree, TreeView, TreeViewMsg};
use iced::{Element, Task, window::Id as WinId};

#[derive(Default, Debug)]
pub struct TreeWin {
    pub title: Option<String>,
    pub tv: TreeView,
}

#[derive(Debug, Clone)]
pub enum TreeWinMsg {
    OpenFile(WinId),
    SetTitle(WinId, String),
    TreeUpdated(WinId, Tree),
    TreeViewMsg(WinId, TreeViewMsg),
}

impl TreeWin {
    pub fn update(&mut self, main_win_msg: TreeWinMsg) -> Task<TreeWinMsg> {
        match main_win_msg {
            TreeWinMsg::OpenFile(_) => Task::none(),
            TreeWinMsg::SetTitle(_, title) => {
                self.title = Some(title);
                Task::none()
            }
            TreeWinMsg::TreeUpdated(id, tree) => {
                Task::done(TreeWinMsg::TreeViewMsg(id, TreeViewMsg::SetWinId(id))).chain(
                    Task::done(TreeWinMsg::TreeViewMsg(id, TreeViewMsg::TreeUpdated(tree))),
                )
            }
            TreeWinMsg::TreeViewMsg(id, msg) => self
                .tv
                .update(msg)
                .map(move |msg| TreeWinMsg::TreeViewMsg(id, msg)),
        }
    }

    pub fn view(&self, id: WinId) -> Element<TreeWinMsg> {
        self.tv
            .view()
            .map(move |msg| TreeWinMsg::TreeViewMsg(id, msg))
    }

    pub fn title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => String::new(),
        }
    }

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
