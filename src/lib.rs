// #![allow(clippy::too_many_arguments)]
// #![cfg_attr(
//     debug_assertions,
//     allow(
//         dead_code,
//         unused_imports,
//         unused_variables,
//         unused_assignments,
//         unused_mut,
//         clippy::collapsible_if,
//     )
// )]

mod application;
mod tree;

pub use application::app::{App, AppMsg, read_text_file};
pub use application::colors::CLR;
pub use application::elements::{TreeView, TreeView1, TreeView1Msg, TreeViewMsg};
pub use application::menus::{MenuEvent, MenuEventReplyMsg, menu_events, prepare_app_menu};
pub use application::windows::window_settings;
pub use tree::{Edges, Tree, TreeFloat, flatten_tree, parse_newick};
