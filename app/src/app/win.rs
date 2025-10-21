use riced::{
    PlatformSpecificWindowSettings, SF, Size, WindowLevel, WindowPosition,
    WindowSettings,
};

#[cfg(target_os = "linux")]
use super::consts::APP_ID;

pub fn window_settings() -> WindowSettings {
    WindowSettings {
        size: Size { width: 600.0 * SF, height: 600.0 * SF },
        min_size: Some(Size { width: 600.0, height: 600.0 }),
        position: WindowPosition::Centered,
        resizable: true,
        level: WindowLevel::Normal,
        #[cfg(target_os = "macos")]
        platform_specific: PlatformSpecificWindowSettings {
            title_hidden: false,
            titlebar_transparent: false,
            fullsize_content_view: false,
        },
        #[cfg(target_os = "windows")]
        platform_specific: PlatformSpecificWindowSettings {
            drag_and_drop: true,
            skip_taskbar: false,
            undecorated_shadow: true,
        },
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecificWindowSettings {
            application_id: String::from(APP_ID),
            override_redirect: false,
        },
        exit_on_close_request: false,
        decorations: true,
        ..Default::default()
    }
}
