mod play_window;
mod tree_window;

#[cfg(target_os = "linux")]
pub use crate::app::APP_ID;
use iced::{
    Size,
    window::{Id as WinId, Level, Position, Settings, settings::PlatformSpecific},
};
pub use play_window::{PlayWin, PlayWinMsg};
pub use tree_window::{TreeWin, TreeWinMsg};

pub enum AppWin {
    TreeWin(TreeWin),
    PlayWin(PlayWin),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AppWinType {
    TreeWin,
    PlayWin,
}

#[allow(dead_code)]
pub trait Win {
    fn win_id(&self) -> WinId;
    fn win_type(&self) -> &AppWinType;
    fn settings() -> Settings;
}

pub fn window_settings() -> Settings {
    Settings {
        size: Size { width: 700.0, height: 700.0 },
        min_size: Some(Size { width: 600.0, height: 630.0 }),
        position: Position::Centered,
        resizable: true,
        level: Level::Normal,
        #[cfg(target_os = "macos")]
        platform_specific: PlatformSpecific {
            title_hidden: false,
            titlebar_transparent: false,
            fullsize_content_view: false,
        },
        #[cfg(target_os = "windows")]
        platform_specific: PlatformSpecific {
            drag_and_drop: true,
            skip_taskbar: false,
            undecorated_shadow: true,
        },
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecific {
            application_id: String::from(APP_ID),
            override_redirect: true,
        },
        exit_on_close_request: false,
        ..Default::default()
    }
}
