#![allow(clippy::too_many_arguments)]
#![cfg_attr(
    debug_assertions,
    allow(
        dead_code,
        unused_assignments,
        unused_imports,
        unused_mut,
        unused_variables,
    )
)]

use treehouse::Edges;
use treehouse::Tree;
use treehouse::flatten_tree;
use treehouse::parse_newick;

fn main() {
    let data = "(((×Five:0.5,Four:0.4,(Two:0.2,One:0.1)Three:0.3)Six:0.6,Seven:0.7)×Eight×:0.8,×Nine×:0.9)Ten×:1.0;";
    let data = String::from(data);
    // println!("{data}");
    let mut tree = match parse_newick(data) {
        Some(t) => t,
        None => Tree::new(),
    };
    // tree.sort(true);
    // println!("{}", &tree);
    let edges: Edges = flatten_tree(&tree);
    for e in edges {
        // println!(
        //     // Prnt Child BrLen  PHeight Height  NChld NTip   Name   Tip     Y
        //     "{:>5} {:>5} {:>3.5} {:>3.5} {:>3.5} {:>5} {:>5} {:>10} {:>5} {:>3.5}",
        //     e.0, e.1, e.2, e.3, e.4, e.5, e.6, e.7, e.8, e.9
        // );
        println!(
            // Prnt Child Name  PHeight  Height    Y
            "{:>5} {:>5} {:>10} {:>3.5} {:>3.5} {:>3.5}",
            e.0, e.1, e.2, e.3, e.4, e.5,
        );
    }
}
