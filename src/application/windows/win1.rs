use crate::{Tree, TreeView1, TreeView1Msg};
use iced::widget::{button, container};
use iced::{Element, Length, Task};

#[derive(Default)]
pub struct Win1 {
    pub tree_view: TreeView1,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Win1Msg {
    OpenFile,
    SetTitle(String),
    TreeUpdated(Tree),
    TreeViewMsg(TreeView1Msg),
}

impl Win1 {
    pub fn title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => String::new(),
        }
    }

    pub fn update(&mut self, main_win_msg: Win1Msg) -> Task<Win1Msg> {
        match main_win_msg {
            Win1Msg::OpenFile => Task::none(),
            Win1Msg::SetTitle(title) => {
                self.title = Some(title);
                Task::none()
            }
            Win1Msg::TreeUpdated(tree) => {
                Task::done(Win1Msg::TreeViewMsg(TreeView1Msg::TreeUpdated(tree)))
            }
            Win1Msg::TreeViewMsg(tree_view_msg) => self
                .tree_view
                .update(tree_view_msg)
                .map(Win1Msg::TreeViewMsg),
        }
    }

    pub fn view(&self) -> Element<Win1Msg> {
        if self.tree_view.tree().tip_count_all() > 0 {
            self.tree_view.view().map(Win1Msg::TreeViewMsg)
        } else {
            container(button("Open NEWICK File").on_press(Win1Msg::OpenFile))
                .center(Length::Fill)
                .into()
        }
    }

    pub fn new() -> Self {
        Self {
            tree_view: TreeView1::new(),
            ..Default::default()
        }
    }
}
