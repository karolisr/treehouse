// #![cfg_attr(
//     debug_assertions,
//     allow(
//         clippy::collapsible_if,
//         clippy::collapsible_match,
//         clippy::derivable_impls,
//         clippy::too_many_arguments,
//         clippy::type_complexity,
//         dead_code,
//         unused_assignments,
//         unused_imports,
//         unused_mut,
//         unused_variables,
//     )
// )]

mod application;

#[cfg(target_os = "macos")]
pub use application::menus::prepare_app_menu;

pub use application::{
    APP_SCALE_FACTOR, Float, LINE_H, PADDING, PADDING_INNER, SCROLL_BAR_W, SF, SPACING, TEXT_SIZE,
    TREE_LAB_FONT_NAME,
    app::{App, AppMsg, read_text_file},
    colors::ColorSimple,
    menus::{MenuEvent, MenuEventReplyMsg, menu_events},
    treeview::{TreeView, TreeViewMsg},
    windows::window_settings,
};

pub use dendros::{Edges, NodeType, Tree, TreeFloat, flatten_tree, parse_newick};

pub fn lerp(a: impl Into<Float>, b: impl Into<Float>, t: impl Into<Float>) -> Float {
    let a = a.into();
    let b = b.into();
    let t = t.into();
    a + (b - a) * t
}

use iced::advanced::graphics::text::cosmic_text::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping,
};

pub fn text_width(s: &str, font_size: f32, line_height: f32) -> f32 {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(font_size, line_height);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    let mut attrs = Attrs::new();
    attrs.family = Family::Name(TREE_LAB_FONT_NAME);
    buffer.set_text(s, &attrs, Shaping::Basic);
    buffer.line_layout(0).unwrap().first().unwrap().w
}

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .title(App::title)
        .subscription(App::subscription)
        .antialiasing(false)
        .scale_factor(App::scale_factor)
        .run()
}
