use crate::{TreeView, TreeViewMsg};
use iced::widget::{button, container};
use iced::window::settings::PlatformSpecific;
use iced::window::{Level, Position, Settings};
use iced::{Element, Length, Size, Task};

#[derive(Debug, Default)]
pub struct MainWin {
    pub tree_view: TreeView,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MainWinMsg {
    TreeViewMsg(TreeViewMsg),
    OpenFile,
    Title(String),
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
            MainWinMsg::Title(title) => {
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

pub fn main_win_settings() -> Settings {
    Settings {
        size: Size {
            width: 900.0,
            height: 600.0,
        },
        min_size: Some(Size {
            width: 500.0,
            height: 300.0,
        }),
        // position: Position::Specific(Point { x: 10.0, y: 30.0 }),
        // position: Position::Default,
        position: Position::Centered,
        resizable: true,
        level: Level::Normal,
        #[cfg(target_os = "macos")]
        platform_specific: PlatformSpecific {
            title_hidden: false,
            titlebar_transparent: false,
            fullsize_content_view: false,
        },
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecific {
            application_id: String::from("APP_ID"),
            override_redirect: true,
        },
        exit_on_close_request: false,
        ..Default::default()
    }
}
