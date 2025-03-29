mod newick;
mod node;
mod utils;

pub use newick::parse_newick;
pub use node::{Tree, node, nodes_from_string};
pub use utils::{Edges, flatten_tree};
