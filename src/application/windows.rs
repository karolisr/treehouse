use iced::window::settings::PlatformSpecific;
mod main;
use iced::Size;
use iced::window::{Level, Position, Settings};
pub use main::{MainWin, MainWinMsg};
pub enum AppWin {
    MainWin(Box<MainWin>),
}

pub fn window_settings() -> Settings {
    Settings {
        size: Size {
            width: 700.0,
            height: 800.0,
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
