use super::{
    super::{TreeView, TreeViewMsg},
    AppWinType, Win, window_settings,
};
use crate::{Tree, parse_newick, write_newick};
#[allow(unused_imports)]
use iced::{
    Element, Point, Task,
    window::{Id as WinId, Position, Settings},
};
use std::path::PathBuf;

pub struct TreeWin {
    pub title: Option<String>,
    pub tv: TreeView,
    win_id: WinId,
    win_type: AppWinType,
}

#[derive(Debug, Clone)]
pub enum TreeWinMsg {
    SetTitle(String),
    TreeUpdated(Tree),
    TreeViewMsg(TreeViewMsg),
    SaveNewick(PathBuf),
    SaveNewickAck(String, PathBuf),
}

impl Win for TreeWin {
    fn win_id(&self) -> WinId {
        self.win_id
    }

    fn win_type(&self) -> &AppWinType {
        &self.win_type
    }

    fn settings() -> Settings {
        let tmp = window_settings();
        Settings {
            // position: Position::Specific(Point { x: 0e0, y: 0e0 }),
            ..tmp
        }
    }
}

impl TreeWin {
    pub fn update(&mut self, tree_win_msg: TreeWinMsg) -> Task<TreeWinMsg> {
        match tree_win_msg {
            TreeWinMsg::SaveNewick(path_buf) => {
                let newick_string = write_newick(&self.tv.tree);
                Task::done(TreeWinMsg::SaveNewickAck(newick_string, path_buf))
            }
            TreeWinMsg::SaveNewickAck(newick_string, path_buf) => Task::done(
                TreeWinMsg::TreeUpdated(parse_newick(newick_string).unwrap()),
            )
            .chain(Task::done(TreeWinMsg::SetTitle(String::from(
                path_buf
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            )))),
            TreeWinMsg::SetTitle(title) => {
                self.title = Some(title);
                Task::none()
            }
            TreeWinMsg::TreeUpdated(tree) => {
                Task::done(TreeWinMsg::TreeViewMsg(TreeViewMsg::TreeUpdated(tree)))
            }
            TreeWinMsg::TreeViewMsg(msg) => self.tv.update(msg).map(TreeWinMsg::TreeViewMsg),
        }
    }

    pub fn view(&self, _: WinId) -> Element<TreeWinMsg> {
        self.tv
            .view()
            // .explain(crate::ColorSimple::CYA)
            .map(TreeWinMsg::TreeViewMsg)
    }

    pub fn title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => String::new(),
        }
    }

    pub fn new(win_id: WinId, win_type: &AppWinType) -> Self {
        Self {
            tv: TreeView::new(),
            title: None,
            win_id,
            win_type: *win_type,
        }
    }
}
