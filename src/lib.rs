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
//     )
// )]

mod application;
mod tree;

pub use application::{
    app::{App, AppMsg, read_text_file},
    colors::ColorSimple,
    elements::{TreeView, TreeViewMsg},
    menus::{MenuEvent, MenuEventReplyMsg, menu_events, prepare_app_menu},
    windows::window_settings,
};
pub use tree::{Edges, Tree, TreeFloat, flatten_tree, parse_newick};
