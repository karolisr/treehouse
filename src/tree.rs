mod flatten;
mod newick;
mod node;

pub use flatten::{Edges, flatten_tree};
pub use newick::parse_newick;
pub use node::{Tree, node, nodes_from_string};

pub type TreeFloat = f64;
