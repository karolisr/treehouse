#[cfg(target_os = "linux")]
use super::consts::APP_ID;
use iced::{
    Size,
    window::{Level, Position, Settings, settings::PlatformSpecific},
};

pub fn window_settings() -> Settings {
    Settings {
        size: Size { width: 800.0, height: 600.0 },
        // min_size: Some(Size { width: 100.0, height: 100.0 }),
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
        platform_specific: PlatformSpecific { drag_and_drop: true, skip_taskbar: false, undecorated_shadow: true },
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecific { application_id: String::from(APP_ID), override_redirect: true },
        exit_on_close_request: false,
        ..Default::default()
    }
}
