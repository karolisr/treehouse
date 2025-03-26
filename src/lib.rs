// #![allow(clippy::too_many_arguments)]
// #![cfg_attr(
//     debug_assertions,
//     allow(
//         // clippy::collapsible_if,
//         dead_code,
//         // unused_assignments,
//         unused_imports,
//         // unused_mut,
//         unused_variables,
//     )
// )]

mod application;
mod colors;
mod tree;

pub use application::app::{App, AppMsg, read_text_file};
pub use application::menus::{MenuEvent, MenuEventReplyMsg, menu_events, prepare_app_menu};
pub use application::treeview::{TreeView, TreeViewMsg};
pub use application::widgets::Canvas;
pub use application::windows::{AppWin, MainWin, MainWinMsg, main_win_settings};
pub use colors::SimpleColor;
pub use tree::{Node, Tree, node, nodes, nodes_from_string, parse_newick};
