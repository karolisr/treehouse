#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #![cfg_attr(
//     debug_assertions,
//     allow(
//         dead_code,
//         unused_assignments,
//         unused_imports,
//         unused_mut,
//         unused_variables,
//         clippy::collapsible_if,
//         clippy::collapsible_match,
//         clippy::derivable_impls,
//         clippy::too_many_arguments,
//         clippy::type_complexity,
//     )
// )]

mod app;
mod colors;
mod text_width;
mod utils;

use app::{ANTIALIASING, App};
pub use colors::ColorSimple;
pub use dendros::{Edge, Edges, NodeId, NodeType, Tree, TreeFloat, flatten_tree, parse_newick};
pub use text_width::text_width;
pub use utils::lerp;

pub type Float = f32;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .title(App::title)
        .subscription(App::subscription)
        .antialiasing(ANTIALIASING)
        .scale_factor(App::scale_factor)
        .run()
}
