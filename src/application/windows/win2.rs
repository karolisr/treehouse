use crate::Tree;
use iced::{Element, Task, widget::horizontal_space};

#[derive(Default)]
pub struct Win2 {
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Win2Msg {
    OpenFile,
    SetTitle(String),
    TreeUpdated(Tree),
}

impl Win2 {
    pub fn title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => String::new(),
        }
    }

    pub fn update(&mut self, main_win_msg: Win2Msg) -> Task<Win2Msg> {
        match main_win_msg {
            Win2Msg::OpenFile => Task::none(),
            Win2Msg::SetTitle(title) => {
                self.title = Some(title);
                Task::none()
            }
            Win2Msg::TreeUpdated(tree) => {
                println!("{}", tree.tip_count_all());
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Win2Msg> {
        horizontal_space().into()
    }

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
