// #![cfg_attr(
//     debug_assertions,
//     allow(
//         dead_code,
//         unused_imports,
//         unused_variables,
//         unused_assignments,
//         unused_mut,
//         clippy::collapsible_if,
//         clippy::collapsible_match,
//         clippy::derivable_impls,
//         clippy::too_many_arguments,
//         clippy::type_complexity,
//     )
// )]

mod application;
mod tree;

pub use application::{
    APP_SCALE_FACTOR, Float, PADDING, PADDING_INNER, S_BAR_W, SF, SPACING, TEXT_SIZE,
    app::{App, AppMsg, read_text_file},
    colors::ColorSimple,
    menus::{MenuEvent, MenuEventReplyMsg, menu_events, prepare_app_menu},
    treeview::{TreeView, TreeViewMsg},
    windows::window_settings,
};
pub use tree::{Edges, Tree, TreeFloat, flatten_tree, max_name_len, parse_newick};
