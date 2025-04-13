mod tree_window;

#[cfg(target_os = "linux")]
pub use crate::app::APP_ID;
use iced::{
    Size,
    window::{Level, Position, Settings, settings::PlatformSpecific},
};
pub use tree_window::{TreeWin, TreeWinMsg};

#[derive(Debug)]
pub enum AppWin {
    TreeWin(Box<TreeWin>),
}

#[derive(Debug)]
pub enum AppWinType {
    TreeWin,
}

pub fn window_settings() -> Settings {
    Settings {
        size: Size { width: 800.0, height: 700.0 },
        min_size: Some(Size { width: 500.0, height: 500.0 }),
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
            application_id: String::from(APP_ID),
            override_redirect: true,
        },
        exit_on_close_request: false,
        ..Default::default()
    }
}
