mod win1;
mod win2;

use iced::Size;
use iced::window::settings::PlatformSpecific;
use iced::window::{Level, Position, Settings};

pub use win1::{Win1, Win1Msg};
pub use win2::{Win2, Win2Msg};

pub enum AppWin {
    Win1(Box<Win1>),
    Win2(Box<Win2>),
}

pub fn window_settings() -> Settings {
    Settings {
        size: Size {
            width: 700.0,
            height: 800.0,
        },
        min_size: Some(Size {
            width: 100.0,
            height: 100.0,
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
