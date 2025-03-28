#![allow(clippy::too_many_arguments)]
#![cfg_attr(
    debug_assertions,
    allow(
        clippy::collapsible_if,
        dead_code,
        unused_assignments,
        unused_imports,
        unused_mut,
        unused_variables,
    )
)]

mod application;
mod tree;

pub use application::app::{App, AppMsg, read_text_file};
pub use application::colors::SimpleColor;
pub use application::elements::{TreeView1, TreeView1Msg, TreeView2, TreeView2Msg};
pub use application::menus::{MenuEvent, MenuEventReplyMsg, menu_events, prepare_app_menu};
pub use application::windows::window_settings;
pub use tree::{
    BrLen, Child, Edge, Edges, Node, Parent, Tree, flatten_tree, node, nodes, nodes_from_string,
    parse_newick,
};
