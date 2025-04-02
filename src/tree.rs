mod flatten;
mod newick;
mod node;
mod utils;

pub use flatten::{Edges, flatten_tree};
pub use newick::parse_newick;
pub use node::{NodeType, Tree, node, nodes_from_string};
pub use utils::max_name_len;

pub type TreeFloat = f64;
