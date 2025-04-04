use super::TreeFloat;
use slotmap::{DefaultKey as NodeId, SlotMap};
use std::{collections::HashMap as Dict, fmt::Display, sync::Arc};

#[derive(Clone, Debug, Default)]
pub struct Tree {
    nodes: SlotMap<NodeId, Node>,
    first_node_id: Option<NodeId>,
    root_node_id: Option<NodeId>,
    parent_children_map: Dict<NodeId, Vec<NodeId>>,
    child_parent_map: Dict<NodeId, Option<NodeId>>,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Node {
    name: Option<Arc<str>>,
    branch_length: Option<TreeFloat>,
    node_type: Option<NodeType>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub enum NodeType {
    Tip,
    Internal,
    Root,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::new(),
            first_node_id: None,
            root_node_id: None,
            parent_children_map: Dict::new(),
            child_parent_map: Dict::new(),
        }
    }

    pub fn root(&mut self, node_id: NodeId) -> Option<NodeId> {
        self.root_node_id
    }

    pub fn unroot(&mut self) -> Option<NodeId> {
        if let Some(root_node_id) = self.mark_root_node_if_possible() {
            let child_node_ids = self.child_node_ids(root_node_id);

            assert_eq!(child_node_ids.len(), 2);

            let c1 = child_node_ids[0];
            let c2 = child_node_ids[1];

            if self.is_tip(c1) && self.is_tip(c2) {
                return None;
            }

            // At this point we know that at most one of c1 and c2 could still be a tip.
            let node_id_to_drop;
            let node_id_to_retain;
            if self.is_tip(c1) || self.is_tip(c2) {
                if self.is_tip(c1) {
                    // c1 is a tip
                    node_id_to_drop = c2;
                    node_id_to_retain = c1;
                } else {
                    // c2 must be a tip a this point
                    node_id_to_drop = c1;
                    node_id_to_retain = c2
                }
            } else {
                // The only possible state at this point is that neither c1 or c2 are tips.
                // Arbitrarily choose c1 to drop.
                node_id_to_drop = c1;
                node_id_to_retain = c2;
            }

            let node_to_drop_clone = self.nodes[node_id_to_drop].clone();
            let node_to_retain = &mut self.nodes[node_id_to_retain];

            if let Some(brlen_flatten) = node_to_drop_clone.branch_length {
                if let Some(brlen_retain) = node_to_retain.branch_length {
                    node_to_retain.branch_length = Some(brlen_retain + brlen_flatten)
                } else {
                    node_to_retain.branch_length = Some(brlen_flatten)
                }
            }

            let child_node_ids: Vec<NodeId> = self.child_node_ids(node_id_to_drop).to_vec();
            let mut new_child_node_ids: Vec<NodeId> = vec![node_id_to_retain];

            for child_node_id in child_node_ids {
                self.child_parent_map
                    .insert(child_node_id, Some(root_node_id));
                new_child_node_ids.push(child_node_id);
            }

            self.parent_children_map.remove(&node_id_to_drop);
            self.parent_children_map
                .insert(root_node_id, new_child_node_ids);
            self.nodes.remove(node_id_to_drop);
            self.nodes[root_node_id].node_type = None;
            self.first_node_id = Some(root_node_id);
            self.root_node_id = None;

            assert_eq!(self.mark_root_node_if_possible(), None);

            self.first_node_id
        } else {
            None
        }
    }

    pub fn first_node_id(&self) -> Option<NodeId> {
        self.first_node_id
    }

    pub fn mark_root_node_if_possible(&mut self) -> Option<NodeId> {
        match self.first_node_id {
            Some(id) => {
                if self.child_node_count(id) == 2 {
                    let root_node = &mut self.nodes[id];
                    root_node.node_type = Some(NodeType::Root);
                    self.root_node_id = Some(id);
                    Some(id)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn add_node(&mut self, parent_id: Option<NodeId>, child_node: Node) -> NodeId {
        let child_id = self.nodes.insert(child_node);
        if self.first_node_id.is_none() {
            self.first_node_id = Some(child_id);
            self.nodes[child_id].branch_length = None;
        }
        if let Some(id) = parent_id {
            if let std::collections::hash_map::Entry::Vacant(e) = self.parent_children_map.entry(id)
            {
                e.insert(Vec::new());
            }
            let chld_vec = self.parent_children_map.get_mut(&id).unwrap();
            chld_vec.push(child_id);
        }
        self.child_parent_map.insert(child_id, parent_id);
        self.parent_children_map.insert(child_id, Vec::new());
        child_id
    }

    pub fn add_nodes(&mut self, parent_id: Option<NodeId>, child_nodes: Vec<Node>) {
        for child_node in child_nodes {
            self.add_node(parent_id, child_node);
        }
    }

    pub fn name(&self, node_id: NodeId) -> Option<Arc<str>> {
        match self.nodes.get(node_id) {
            Some(node) => node.name.clone(),
            None => None,
        }
    }

    pub fn name_empty_if_none(&self, node_id: NodeId) -> Arc<str> {
        match self.name(node_id) {
            Some(name) => name,
            None => "".into(),
        }
    }

    pub fn branch_length(&self, node_id: NodeId) -> TreeFloat {
        match self.nodes.get(node_id) {
            Some(n) => n.branch_length.unwrap_or(0e0),
            None => 0e0,
        }
    }

    pub fn child_node_ids(&self, node_id: NodeId) -> &[NodeId] {
        match self.parent_children_map.get(&node_id) {
            Some(x) => x,
            None => &[],
        }
    }

    pub fn parent_node_id(&self, node_id: NodeId) -> Option<&NodeId> {
        match self.child_parent_map.get(&node_id) {
            Some(opt) => opt.as_ref(),
            None => None,
        }
    }

    pub fn child_node_count(&self, node_id: NodeId) -> usize {
        self.child_node_ids(node_id).len()
    }

    pub fn child_node_count_recursive(&self, node_id: NodeId) -> usize {
        let mut rv = self.child_node_count(node_id);
        for &child_id in self.child_node_ids(node_id) {
            rv += self.child_node_count_recursive(child_id);
        }
        rv
    }

    pub fn node_count_all(&self) -> usize {
        if let Some(id) = self.first_node_id {
            self.child_node_count_recursive(id) + 1
        } else {
            0
        }
    }

    pub fn tip_node_ids(&self, node_id: NodeId) -> Vec<NodeId> {
        let cs: &[NodeId] = self.child_node_ids(node_id);
        let mut rv: Vec<NodeId> = Vec::new();
        for &c in cs {
            if self.is_tip(c) {
                rv.push(c);
            } else {
                rv.append(&mut self.tip_node_ids(c));
            }
        }
        rv
    }

    pub fn tip_node_ids_all(&self) -> Vec<NodeId> {
        if let Some(id) = self.first_node_id {
            self.tip_node_ids(id)
        } else {
            Vec::new()
        }
    }

    pub fn dist(&self, left: NodeId, right: NodeId) -> TreeFloat {
        let mut h: TreeFloat = 0e0;
        if left != right {
            h += self.branch_length(right);
        }
        match self.parent_node_id(right) {
            Some(&p) => {
                if p == left {
                    h
                } else {
                    h + self.dist(left, p)
                }
            }
            None => 0e0,
        }
    }

    pub fn height(&self) -> TreeFloat {
        let mut h = 0e0;
        if let Some(id) = self.first_node_id {
            for right in self.tip_node_ids_all() {
                let curr = self.dist(id, right);
                if curr > h {
                    h = curr
                }
            }
        }
        h
    }

    pub fn is_tip(&self, node_id: NodeId) -> bool {
        self.child_node_ids(node_id).is_empty()
    }

    pub fn tip_count(&self, node_id: NodeId) -> usize {
        let mut rv: usize = 0;
        for &child_node_id in self.child_node_ids(node_id) {
            if self.is_tip(child_node_id) {
                rv += 1
            }
        }
        rv
    }

    pub fn tip_count_recursive(&self, node_id: NodeId) -> usize {
        let mut rv = self.tip_count(node_id);
        for &child_node_id in self.child_node_ids(node_id) {
            rv += self.tip_count_recursive(child_node_id);
        }
        rv
    }

    pub fn tip_count_all(&self) -> usize {
        if let Some(id) = self.first_node_id {
            self.tip_count_recursive(id)
        } else {
            0
        }
    }

    pub fn tip_node_counts_for_children(&self, node_id: NodeId) -> Vec<usize> {
        self.child_node_ids(node_id)
            .iter()
            .map(|&node_id| self.tip_count_recursive(node_id))
            .map(|count| if count == 0 { 1 } else { count })
            .collect()
    }

    pub fn is_rooted(&self) -> bool {
        self.root_node_id.is_some()
    }

    pub fn root_node_id(&self) -> Option<NodeId> {
        self.root_node_id
    }

    pub fn sort(&mut self, reverse: bool) {
        if let Some(id) = self.first_node_id {
            self.sort_nodes(id, reverse);
        }
    }

    fn sort_nodes(&mut self, node_id: NodeId, reverse: bool) {
        let mut sorted_ids: Vec<NodeId> = self.child_node_ids(node_id).into();
        // sorted_ids.sort_by_key(|c| self.child_node_count(*c));
        // sorted_ids.sort_by(|a, b| {
        //     self.dist(self.first_node_id(), *b)
        //         .total_cmp(&self.dist(self.first_node_id(), *a))
        // });
        // sorted_ids.sort_by_key(|s| self.name(*s));
        // sorted_ids.sort_by(|a, b| self.branch_length(*a).total_cmp(&self.branch_length(*b)));
        sorted_ids.sort_by_key(|c| self.child_node_count_recursive(*c));
        if reverse {
            sorted_ids.reverse();
        }
        self.parent_children_map.insert(node_id, sorted_ids.clone());
        for id in sorted_ids {
            self.sort_nodes(id, reverse);
        }
    }
}

fn display(tree: &Tree, node_id: NodeId, mut level: usize) -> String {
    let mut rv: String = String::new();
    let is_root = tree.root_node_id == Some(node_id);
    let name = tree.name_empty_if_none(node_id);
    let brln = tree.branch_length(node_id);

    rv.push_str(&format!(
        "{}- |{node_id:?}| {name} {brln:4.2} {} {} {} {}{}\n",
        " ".repeat(level * 4),
        tree.child_node_count_recursive(node_id),
        tree.child_node_count(node_id),
        tree.tip_count_recursive(node_id),
        tree.tip_count(node_id),
        if is_root { " ROOT" } else { "" }
    ));

    if tree.is_tip(node_id) {
        rv = format!(
            "{}- |{node_id:?}| {name} {brln:4.2}\n",
            " ".repeat(level * 4)
        );
    }

    if level == 0 {
        let rv_prefix = format!(
            " Nodes: {:5}\n  Tips: {:5}\nHeight: {:<6.3}\n\n",
            tree.node_count_all(),
            tree.tip_count_recursive(node_id),
            tree.height()
        );
        rv = rv_prefix + &rv;
    }

    level += 1;

    for &child_node_id in tree.child_node_ids(node_id) {
        rv.push_str(&display(tree, child_node_id, level));
    }

    rv
}

impl Node {
    pub fn new(name: Option<String>, branch_length: Option<TreeFloat>) -> Self {
        Self {
            name: name.map(|name| name.into()),
            branch_length,
            node_type: None,
        }
    }

    pub fn named<'a>(name: impl Into<&'a str>) -> Self {
        let (name, branch_length) = parse_name(name);
        Node::new(name, branch_length)
    }
}

pub fn parse_name<'a>(name: impl Into<&'a str>) -> (Option<String>, Option<TreeFloat>) {
    let name: &str = name.into();
    let (name, brln) = match name.rsplit_once(':') {
        Some((name, brln)) => (
            name,
            match brln.parse::<TreeFloat>() {
                Ok(x) => Some(x),
                Err(_) => Some(1e0),
            },
        ),
        None => (name, Some(1e0)),
    };

    let name = match name.trim_matches(['\'', '"']) {
        "" => None,
        x => Some(x.replace("_", " ").replace("|", " ").to_string()),
    };

    (name, brln)
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::named(value.as_str())
    }
}

impl<'a> From<&'a str> for Node {
    fn from(value: &'a str) -> Self {
        Node::named(value)
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{}",
            if let Some(id) = self.first_node_id {
                display(self, id, 0)
            } else {
                String::new()
            }
        )
    }
}

pub fn node<'a>(name: impl Into<&'a str>) -> Node {
    let (name, branch_length) = parse_name(name);
    Node::new(name, branch_length)
}

pub fn nodes<'a>(names: impl Into<Vec<&'a str>>) -> Vec<Node> {
    names.into().iter().map(|&n| n.into()).collect()
}

pub fn nodes_from_string<'a>(s: impl Into<&'a str>, sep: impl Into<&'a str>) -> Vec<Node> {
    let s: &str = s.into();
    let sep: &str = sep.into();
    let nds: Vec<&str> = s.split(sep).collect();
    nodes(nds)
}
