use super::Tree;
use std::collections::BTreeMap;
type Float = f64;
type Parent = usize;
type Child = usize;
type PHeight = Float;
type Height = Float;
type Name = String;
type Y = Float;
type Yprev = Float;
pub type Edge = (Parent, Child, Name, PHeight, Height, Y, Yprev);
pub type Edges = Vec<Edge>;

pub fn flatten_tree(tree: &Tree, chunk_count: usize) -> Vec<Edges> {
    let ntip = tree.tip_count_all();
    let tree_height = tree.height();
    let mut tip_id_counter = ntip;
    let edges: Edges = flatten(
        tree.first_node_id(),
        0,
        0e0,
        tree,
        tree_height,
        ntip,
        &mut tip_id_counter,
    );
    chunk_edges(calc_verticals(edges), chunk_count)
}

fn chunk_edges(edges: Edges, chunk_count: usize) -> Vec<Edges> {
    let edge_count = edges.len();
    let mut chunk_count = chunk_count;
    if chunk_count == 0 {
        chunk_count = 1;
    }
    let n_edge_per_thread = edge_count / chunk_count;
    let remainder = edge_count % chunk_count;
    let mut chunks: Vec<Vec<Edge>> = Vec::new();
    for t in 0..chunk_count {
        let i1 = n_edge_per_thread * t;
        let i2 = n_edge_per_thread * (t + 1);
        let edges = &edges[i1..i2];
        chunks.push(edges.to_vec());
    }
    if remainder > 0 {
        let edges = &edges[n_edge_per_thread * chunk_count..];
        chunks.push(edges.to_vec());
    }
    chunks
}

fn calc_verticals(mut edges: Edges) -> Edges {
    if edges.is_empty() {
        return edges;
    }
    edges.sort_by(|a, b| a.5.total_cmp(&b.5));
    edges.sort_by_key(|x| x.1);
    edges.sort_by_key(|x| -(x.0 as i32));
    let mut mem: BTreeMap<usize, Float> = BTreeMap::new();
    let mut parent_prev = edges[0].0;
    let mut y_min = edges[0].5;
    let mut y_max = edges[0].5;
    let mut y_prev = edges[0].5;
    for e in &mut edges[1..] {
        let parent = e.0;
        let child = e.1;
        let mut y = e.5;
        if parent == parent_prev {
            if y.is_nan() {
                y = match mem.get(&child) {
                    Some(&y) => y,
                    None => 0e0,
                };
                e.5 = y;
            }
            if y > y_max {
                y_max = y;
            }
            if y < y_min {
                y_min = y;
            }
            e.6 = y_prev;
        } else {
            let y_parent = (y_max - y_min) / 2e0 + y_min;
            if child == parent_prev {
                mem.insert(parent, y_parent);
            } else {
                mem.insert(parent_prev, y_parent);
            }
            if y.is_nan() {
                y = y_parent;
                e.5 = y_parent;
            }
            y_min = y;
            y_max = y;
            parent_prev = parent;
        }
        y_prev = y;
    }
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
    let mut y = Float::NAN;
    if descending_tip_count == 0 {
        *tip_id_counter -= 1;
        let tip_id = ntip - *tip_id_counter;
        y = (tip_id - 1) as Float / (ntip - 1) as Float;
    }
    let mut all_edges: Edges = Vec::new();
    let this_edge: Edge = (
        parent_node_id,
        node_id,
        name,
        height,
        height + brlen,
        y,
        Float::NAN,
    );
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
