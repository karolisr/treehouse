use super::{Tree, node, nodes_from_string};

pub fn parse_newick(s: String) -> Option<Tree> {
    let mut tree: Tree = Tree::new();
    let sc = clean_newick_str(&s);
    tree = parse(sc, 0, tree);
    if tree.tip_count_all() > 0 {
        Some(tree)
    } else {
        None
    }
}

fn parse(s: String, parent_id: usize, mut tree: Tree) -> Tree {
    let mut i: usize = 0;
    let mut i0: usize = 0;
    let mut n_open: i32 = 0;
    let mut is_open: bool = false;
    let mut was_open: bool = false;
    let s_len = s.char_indices().count();
    let mut s_iter = s.char_indices();
    while i < s_len {
        let character: char;
        if let Some((c_idx, c)) = s_iter.next() {
            character = c;
            if i > c_idx {
                continue;
            } else {
                i = c_idx;
            }
        } else {
            i += 1;
            continue;
        }
        match character {
            '(' => {
                n_open += 1;
                if !is_open {
                    is_open = true;
                    was_open = true;
                    i0 = i + 1;
                }
            }
            ')' => {
                n_open -= 1;
                if is_open && n_open == 0 {
                    is_open = false;
                    let label = match s[i + 1..].find([';', ',', '(']) {
                        Some(x) => &s[i + 1..i + 1 + x],
                        None => &s[i + 1..],
                    };
                    let child_id = tree.add_child_node(parent_id, node(label));
                    tree = parse(s[i0..i].into(), child_id, tree);
                    i += label.len();
                    i0 = i;
                }
            }
            ',' => {
                // ---------------------------------------------------------------------------------
                // This whole section is here to account for one thing only: nodes not surrounded by
                // parentheses that occur next to nodes that are and share a parent node.
                // (((One:0.2,Two:0.3):0.3,(Three:0.5,Four:0.3):0.2):0.3,Five:0.7):0.0;
                //                                                      |||||||||
                if !is_open && was_open {
                    let no_parens = match s[i + 1..].find(['(']) {
                        Some(x) => {
                            let mut rv = &s[i + 1..i + 1 + x];
                            // Make sure additional (empty) node is not created when the ",("
                            // pattern is encountered; e.g. "...node1,node2,(..."
                            if rv.ends_with(",") && &s[i + 1 + x..i + 2 + x] == "(" {
                                rv = &rv[0..rv.len() - 1];
                            }
                            i += x;
                            rv
                        }
                        None => {
                            let mut rv = &s[i + 1..];
                            rv = rv.trim_end_matches(';');
                            i = s_len;
                            rv
                        }
                    };

                    if !no_parens.is_empty() {
                        tree.add_child_nodes(parent_id, nodes_from_string(no_parens, ","));
                    }
                }
                // --------------------------------------------------------------------------------

                // --------------------------------------------------------------------------------
                // ((One:0.1,Two:0.2,(Three:0.3,Four:0.4)Five:0.5)Six:0.6,Seven:0.7);
                //   ||||||||||||||||
                else if !is_open && !was_open && s.len() >= i + 2 && &s[i + 1..i + 2] == "(" {
                    tree.add_child_nodes(parent_id, nodes_from_string(&s[0..i], ","));
                }
                // --------------------------------------------------------------------------------
            }
            // ------------------------------------------------------------------------------------
            _ => (),
        }
        // i += 1;
    }
    if !was_open {
        tree.add_child_nodes(parent_id, nodes_from_string(s.as_str(), ","));
    }
    tree
}

fn clean_newick_str(s: &str) -> String {
    let rv: String = s
        .split(char::is_whitespace)
        .filter_map(|c| match c.trim() {
            "" => None,
            x => Some(format!("{x} ")),
        })
        .collect();
    let rv = clean_sep(rv.as_str(), ",");
    let rv = clean_sep(rv.as_str(), "(");
    let rv = clean_sep(rv.as_str(), ")");
    rv
}

fn clean_sep<'a>(s: impl Into<&'a str>, sep: impl Into<&'a str>) -> String {
    let sep: &str = sep.into();
    let ss: String = s
        .into()
        .split(sep)
        .map(|c| match c.trim() {
            "" => sep.into(),
            x => format!("{}{}", x, sep),
        })
        .collect();
    ss.trim_end_matches(sep).into()
}
