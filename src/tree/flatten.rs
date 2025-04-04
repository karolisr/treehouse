use slotmap::DefaultKey;

use super::{Tree, TreeFloat};
use std::{collections::BTreeMap, sync::Arc};

pub type Edges = Vec<Edge>;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Edge {
    pub parent: Option<DefaultKey>,
    pub child: DefaultKey,
    pub name: Option<Arc<str>>,
    pub x0: TreeFloat,
    pub x1: TreeFloat,
    pub y_prev: Option<TreeFloat>,
    pub y: TreeFloat,
    pub is_tip: bool,
}

pub fn flatten_tree(tree: &Tree, chunk_count: usize) -> Vec<Edges> {
    let ntip = tree.tip_count_all();
    let tree_height = tree.height();
    let mut tip_id_counter = ntip;
    if let Some(id) = tree.first_node_id() {
        let edges: Edges = flatten(id, None, 0e0, tree, tree_height, ntip, &mut tip_id_counter);
        chunk_edges(calc_verticals(edges), chunk_count)
    } else {
        Vec::new()
    }
}

fn calc_verticals(mut edges: Edges) -> Edges {
    if edges.is_empty() {
        return edges;
    }
    edges.sort_by(|a, b| a.y.total_cmp(&b.y));
    // edges.reverse();
    edges.sort_by_key(|x| x.child);
    edges.reverse();
    edges.sort_by_key(|x| x.parent);
    edges.reverse();

    let mut mem: BTreeMap<DefaultKey, TreeFloat> = BTreeMap::new();
    let mut parent_prev = edges[0].parent;
    let mut y_min = edges[0].y;
    let mut y_max = edges[0].y;
    let mut y_prev = edges[0].y;
    for e in &mut edges[1..] {
        let parent = e.parent;
        let child = e.child;
        let mut y = e.y;
        if parent == parent_prev {
            if y.is_nan() {
                y = match mem.get(&child) {
                    Some(&y) => y,
                    None => 0e0,
                };
                e.y = y;
            }
            if y > y_max {
                y_max = y;
            }
            if y < y_min {
                y_min = y;
            }
            e.y_prev = Some(y_prev);
        } else {
            let y_parent = (y_max - y_min) / 2e0 + y_min;

            if let Some(pp) = parent_prev {
                if child == pp {
                    if let Some(p) = parent {
                        mem.insert(p, y_parent);
                    }
                } else {
                    mem.insert(pp, y_parent);
                }
            }

            if y.is_nan() {
                y = y_parent;
                e.y = y_parent;
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
    node_id: DefaultKey,
    parent_node_id: Option<DefaultKey>,
    height: TreeFloat,
    tree: &Tree,
    tree_height: TreeFloat,
    ntip: usize,
    tip_id_counter: &mut usize,
) -> Edges {
    let mut edges: Edges = Vec::new();
    if ntip == 0 {
        return edges;
    }
    let brlen: TreeFloat = tree.branch_length(node_id) as TreeFloat / tree_height;
    let name: Option<Arc<str>> = tree.name(node_id);
    let child_node_ids: &[DefaultKey] = tree.child_node_ids(node_id);
    let descending_tip_count: usize = tree.tip_count_recursive(node_id);
    let mut is_tip: bool = false;

    let mut y = TreeFloat::NAN;
    if descending_tip_count == 0 {
        *tip_id_counter -= 1;
        let tip_id = ntip - *tip_id_counter;
        y = (tip_id - 1) as TreeFloat / (ntip - 1) as TreeFloat;
        is_tip = true;
    }

    let this_edge: Edge = Edge {
        parent: parent_node_id,
        child: node_id,
        name,
        x0: height,
        x1: height + brlen,
        y_prev: None,
        y,
        is_tip,
    };

    edges.push(this_edge);
    for &child_node_id in child_node_ids {
        edges.append(&mut flatten(
            child_node_id,
            Some(node_id),
            height + brlen,
            tree,
            tree_height,
            ntip,
            tip_id_counter,
        ));
    }
    edges
}

fn chunk_edges(edges: Edges, chunk_count: usize) -> Vec<Edges> {
    let edge_count = edges.len();
    if edge_count == 0 {
        return Vec::new();
    }
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
