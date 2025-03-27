use crate::{TreeView, TreeViewMsg};
use iced::widget::{button, container};
use iced::{Element, Length, Task};

#[derive(Debug, Default)]
pub struct MainWin {
    pub tree_view: TreeView,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MainWinMsg {
    TreeViewMsg(TreeViewMsg),
    OpenFile,
    SetTitle(String),
}

impl MainWin {
    pub fn title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => String::new(),
        }
    }

    pub fn update(&mut self, main_win_msg: MainWinMsg) -> Task<MainWinMsg> {
        match main_win_msg {
            MainWinMsg::SetTitle(title) => {
                self.title = Some(title);
                Task::none()
            }
            MainWinMsg::TreeViewMsg(tree_view_msg) => self
                .tree_view
                .update(tree_view_msg)
                .map(MainWinMsg::TreeViewMsg),

            MainWinMsg::OpenFile => Task::none(),
        }
    }

    pub fn view(&self) -> Element<MainWinMsg> {
        if self.tree_view.tree().tip_count_all() > 0 {
            self.tree_view.view().map(MainWinMsg::TreeViewMsg)
        } else {
            container(button("Open NEWICK File").on_press(MainWinMsg::OpenFile))
                .center(Length::Fill)
                .into()
        }
    }
}
