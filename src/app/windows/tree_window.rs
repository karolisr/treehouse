use std::path::PathBuf;

use super::super::{TreeView, TreeViewMsg};
use crate::Tree;
use dendros::{parse_newick, write_newick};
use iced::{Element, Task, window::Id as WinId};

#[derive(Default, Debug)]
pub struct TreeWin {
    pub title: Option<String>,
    pub tv: TreeView,
}

#[derive(Debug, Clone)]
pub enum TreeWinMsg {
    SetTitle(String),
    TreeUpdated(WinId, Tree),
    TreeViewMsg(WinId, TreeViewMsg),
    SaveNewick(WinId, PathBuf),
    SaveNewickAck(WinId, String, PathBuf),
}

impl TreeWin {
    pub fn update(&mut self, main_win_msg: TreeWinMsg) -> Task<TreeWinMsg> {
        match main_win_msg {
            TreeWinMsg::SaveNewick(id, path_buf) => {
                let newick_string = write_newick(&self.tv.tree);
                Task::done(TreeWinMsg::SaveNewickAck(id, newick_string, path_buf))
            }
            TreeWinMsg::SaveNewickAck(id, newick_string, path_buf) => Task::done(
                TreeWinMsg::TreeUpdated(id, parse_newick(newick_string).unwrap()),
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
            TreeWinMsg::TreeUpdated(id, tree) => {
                Task::done(TreeWinMsg::TreeViewMsg(id, TreeViewMsg::TreeUpdated(tree)))
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
        Self { tv: TreeView::new(), ..Default::default() }
    }
}
