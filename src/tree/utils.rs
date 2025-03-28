use std::collections::BTreeMap;

use crate::Tree;

type Float = f64;

pub type Parent = usize;
pub type Child = usize;
pub type BrLen = Float;
pub type PHeight = Float;
pub type Height = Float;
pub type Name = String;
pub type Y = Float;

pub type Edge = (Parent, Child, Name, PHeight, Height, Y);
pub type Edges = Vec<Edge>;

pub fn flatten_tree(tree: &Tree) -> Edges {
    let ntip = tree.tip_count_all();
    let tree_height = tree.height();
    let mut tip_id_counter = ntip;

    let mut edges: Edges = flatten(
        tree.first_node_id(),
        0,
        0e0,
        tree,
        tree_height,
        ntip,
        &mut tip_id_counter,
    );

    edges.sort_by(|a, b| a.5.total_cmp(&b.5));
    edges.sort_by_key(|x| x.1);
    edges.sort_by_key(|x| -(x.0 as i32));

    // --------------------------------------------------------------------------------------------
    if !edges.is_empty() {
        let mut p_prev = edges[0].0;
        let mut min_y = edges[0].5;
        let mut max_y = edges[0].5;

        let mut mem: BTreeMap<usize, Float> = BTreeMap::new();

        for e in &mut edges[1..] {
            let p = e.0;
            let c = e.1;
            let mut y = e.5;

            if p == p_prev {
                if y.is_nan() {
                    y = match mem.get(&c) {
                        Some(&y) => y,
                        None => 0e0,
                    };
                    e.5 = y;
                }

                if y > max_y {
                    max_y = y;
                }

                if y < min_y {
                    min_y = y;
                }
            } else {
                let y_p = (max_y - min_y) / 2e0 + min_y;
                if c == p_prev {
                    e.5 = y_p;
                    if y.is_nan() {
                        y = y_p;
                        mem.insert(p, y_p);
                    }
                } else {
                    mem.insert(p_prev, y_p);
                }
                if y.is_nan() {
                    y = y_p;
                    e.5 = y_p;
                }
                min_y = y;
                max_y = y;
                p_prev = p;
            }
        }
    }
    // --------------------------------------------------------------------------------------------
    edges
}

fn flatten(
    node_id: usize,
    parent_node_id: usize,
    height: Float,
    tree: &Tree,
    tree_height: Float,
    ntip: usize,
    tip_id_counter: &mut usize,
) -> Edges {
    let brlen: Float = tree.branch_length(node_id) as Float / tree_height;
    let name: String = tree.name(node_id);
    let child_node_ids: &[usize] = tree.child_node_ids(node_id);
    let descending_tip_count: usize = tree.tip_count_recursive(node_id);

    let mut y = f64::NAN;
    if descending_tip_count == 0 {
        *tip_id_counter -= 1;
        let tip_id = ntip - *tip_id_counter;
        y = (tip_id - 1) as Float / (ntip - 1) as Float;
    }

    let mut all_edges: Edges = Vec::new();

    let this_edge: Edge = (parent_node_id, node_id, name, height, height + brlen, y);

    all_edges.push(this_edge);

    for &child_node_id in child_node_ids {
        all_edges.append(&mut flatten(
            child_node_id,
            node_id,
            height + brlen,
            tree,
            tree_height,
            ntip,
            tip_id_counter,
        ));
    }

    all_edges
}
