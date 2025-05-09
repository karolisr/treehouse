use super::{AppWinType, Win, window_settings};
use iced::{
    Element, Point, Task,
    window::{Id as WinId, Position, Settings},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PlayWin {
    pub title: Option<String>,
    win_id: WinId,
    win_type: AppWinType,
}

#[derive(Debug, Clone)]
pub enum PlayWinMsg {
    #[allow(dead_code)]
    SetTitle(String),
}

impl Win for PlayWin {
    fn win_id(&self) -> WinId {
        self.win_id
    }

    fn win_type(&self) -> &AppWinType {
        &self.win_type
    }

    fn settings() -> Settings {
        let tmp = window_settings();
        Settings {
            position: Position::Specific(Point { x: tmp.size.width + 5e1, y: 0e0 }),
            ..tmp
        }
    }
}

impl PlayWin {
    pub fn update(&mut self, play_win_msg: PlayWinMsg) -> Task<PlayWinMsg> {
        match play_win_msg {
            PlayWinMsg::SetTitle(title) => {
                self.title = Some(title);
                Task::none()
            }
        }
    }

    pub fn view(&self, _: WinId) -> Element<PlayWinMsg> {
        iced::widget::text!("PlayWin").into()
    }

    pub fn title(&self) -> String {
        match &self.title {
            Some(title) => title.clone(),
            None => String::new(),
        }
    }

    pub fn new(win_id: WinId, win_type: &AppWinType) -> Self {
        Self { title: None, win_id, win_type: *win_type }
    }
}
