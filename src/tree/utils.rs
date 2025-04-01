use crate::Tree;

pub fn max_name_len(tree: &Tree) -> usize {
    let ml: usize = tree
        .tip_node_ids_all()
        .iter()
        .map(|id| tree.name(*id).map(|name| name.char_indices().count()))
        .map(|l| l.unwrap_or_default())
        .max()
        .unwrap_or_default();
    ml
}
