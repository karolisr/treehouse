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
pub use dendros::{
    Edge, Edges, LttPoint, NodeId, NodeType, Tree, TreeFloat, chunk_edges, flatten_tree, ltt,
    parse_newick, write_newick,
};
pub use text_width::text_width;
pub use utils::lerp;

pub type Float = f32;
pub const PI: Float = std::f32::consts::PI;

fn main() -> iced::Result {
    #[cfg(debug_assertions)]
    if app::DEBUG {
        tracing_subscriber::fmt::init();
    }
    iced::daemon(App::new, App::update, App::view)
        .title(App::title)
        .subscription(App::subscription)
        .antialiasing(ANTIALIASING)
        .scale_factor(App::scale_factor)
        .theme(App::theme)
        .run()
}
