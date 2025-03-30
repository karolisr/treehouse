use super::TreeFloat;
use std::{collections::HashMap as Dict, fmt::Display};

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

#[derive(Debug, Default, Clone)]
pub struct Node {
    name: Option<String>,
    branch_length: Option<TreeFloat>,
}

#[derive(Debug, Default, Clone)]
pub struct Tree {
    nodes: Vec<Node>,
    parent_children_map: Dict<usize, Vec<usize>>,
    child_parent_map: Dict<usize, usize>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new(Some(String::from("WRAPPER")), Some(0e0))],
            parent_children_map: Dict::new(),
            child_parent_map: Dict::new(),
        }
    }

    pub fn add_child_node(&mut self, parent_id: usize, child_node: Node) -> usize {
        self.nodes.push(child_node);
        let child_id = self.nodes.len() - 1;
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.parent_children_map.entry(parent_id)
        {
            e.insert(Vec::new());
        }
        let chld_vec = self.parent_children_map.get_mut(&parent_id).unwrap();
        chld_vec.push(child_id);
        self.child_parent_map.insert(child_id, parent_id);
        self.parent_children_map.insert(child_id, Vec::new());
        // HACK! ===================================
        if self.child_node_count(0) == 1 {
            let x = self.child_node_ids(0)[0];
            self.nodes[x].branch_length = Some(0.0);
        }
        // =========================================
        child_id
    }

    pub fn add_child_nodes(&mut self, parent_id: usize, child_nodes: Vec<Node>) {
        for child_node in child_nodes {
            self.add_child_node(parent_id, child_node);
        }
    }

    pub fn name(&self, node_id: usize) -> String {
        match self.nodes.get(node_id) {
            Some(n) => match &n.name {
                Some(n) => n.clone(),
                None => String::new(),
            },
            None => String::new(),
        }
    }

    pub fn branch_length(&self, node_id: usize) -> TreeFloat {
        match self.nodes.get(node_id) {
            Some(n) => n.branch_length.unwrap_or(0e0),
            None => 0e0,
        }
    }

    pub fn child_node_ids(&self, node_id: usize) -> &[usize] {
        match self.parent_children_map.get(&node_id) {
            Some(x) => x,
            None => &[],
        }
    }

    pub fn parent_node_id(&self, node_id: usize) -> usize {
        match self.child_parent_map.get(&node_id) {
            Some(&p) => p,
            None => 0,
        }
    }

    pub fn child_node_count(&self, node_id: usize) -> usize {
        self.child_node_ids(node_id).len()
    }

    pub fn child_node_count_recursive(&self, node_id: usize) -> usize {
        let mut rv = self.child_node_count(node_id);
        for &child_id in self.child_node_ids(node_id) {
            rv += self.child_node_count_recursive(child_id);
        }
        rv
    }

    pub fn node_count_all(&self) -> usize {
        self.child_node_count_recursive(0)
    }

    pub fn tip_node_counts_for_children(&self, node_id: usize) -> Vec<usize> {
        self.child_node_ids(node_id)
            .iter()
            .map(|&node_id| self.tip_count_recursive(node_id))
            .map(|count| if count == 0 { 1 } else { count })
            .collect()
    }

    pub fn tip_node_ids(&self, node_id: usize) -> Vec<usize> {
        let cs: &[usize] = self.child_node_ids(node_id);
        let mut rv: Vec<usize> = Vec::new();
        for &c in cs {
            if self.is_tip(c) {
                rv.push(c);
            } else {
                rv.append(&mut self.tip_node_ids(c));
            }
        }
        rv
    }

    pub fn tip_node_ids_all(&self) -> Vec<usize> {
        self.tip_node_ids(self.first_node_id())
    }

    pub fn dist(&self, left: usize, right: usize) -> TreeFloat {
        let mut h: TreeFloat = 0e0;
        if left != right {
            h += match self.nodes.get(right) {
                Some(n) => n.branch_length.unwrap_or(0e0),
                None => 0e0,
            };
        }
        match self.child_parent_map.get(&right) {
            Some(&p) => {
                if p == left {
                    h
                } else {
                    self.dist(left, p) + h
                }
            }
            None => 0e0,
        }
    }

    pub fn height(&self) -> TreeFloat {
        let mut h = 0e0;
        let ids = self.child_node_ids(0);
        if ids.len() == 1 {
            let &left = ids.first().unwrap();
            for right in self.tip_node_ids_all() {
                let curr = self.dist(left, right);
                if curr > h {
                    h = curr
                }
            }
        }
        h
    }

    pub fn is_tip(&self, node_id: usize) -> bool {
        self.child_node_ids(node_id).is_empty()
    }

    pub fn tip_count(&self, node_id: usize) -> usize {
        let mut rv: usize = 0;
        for &child_node_id in self.child_node_ids(node_id) {
            if self.is_tip(child_node_id) {
                rv += 1
            }
        }
        rv
    }

    pub fn tip_count_recursive(&self, node_id: usize) -> usize {
        let mut rv = self.tip_count(node_id);
        for &child_node_id in self.child_node_ids(node_id) {
            rv += self.tip_count_recursive(child_node_id);
        }
        rv
    }

    pub fn tip_count_all(&self) -> usize {
        self.tip_count_recursive(0)
    }

    pub fn first_node_id(&self) -> usize {
        if self.child_node_count(0) == 1 {
            self.child_node_ids(0)[0]
        } else {
            0
        }
    }

    pub fn sort(&mut self, reverse: bool) {
        self.sort_nodes(0, reverse);
    }

    fn sort_nodes(&mut self, node_id: usize, reverse: bool) {
        let mut sorted_ids: Vec<usize> = self.child_node_ids(node_id).into();
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

fn display(tree: &Tree, node_id: usize, mut level: usize) -> String {
    let mut rv: String = String::new();
    if node_id != 0 {
        let name = tree.name(node_id);
        let brln = tree.branch_length(node_id);

        rv.push_str(&format!(
            "{}- {name} {brln:4.2} {} {} {} {}\n",
            " ".repeat(level * 4),
            tree.child_node_count_recursive(node_id),
            tree.child_node_count(node_id),
            tree.tip_count_recursive(node_id),
            tree.tip_count(node_id)
        ));

        if tree.is_tip(node_id) {
            rv = format!("{}- {name} {brln:4.2}\n", " ".repeat(level * 4));
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
    }

    // if level > 3 {
    //     return rv;
    // }

    for &child_node_id in tree.child_node_ids(node_id) {
        rv.push_str(&display(tree, child_node_id, level));
    }

    rv
}

impl Node {
    pub fn new(name: Option<String>, branch_length: Option<TreeFloat>) -> Self {
        Self {
            name,
            branch_length,
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
        write!(f, "\n{}", display(self, self.first_node_id(), 0))
    }
}
