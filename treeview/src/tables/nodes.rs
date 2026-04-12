use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodesTableField {
    Selected,
    NodeId,
    NodeType,
    BranchLength,
    NodeLabel,
}

impl From<NodesTableField> for String {
    fn from(f: NodesTableField) -> Self {
        match f {
            NodesTableField::Selected => "Selected".to_string(),
            NodesTableField::NodeId => "ID".to_string(),
            NodesTableField::NodeType => "Type".to_string(),
            NodesTableField::BranchLength => "Branch Length".to_string(),
            NodesTableField::NodeLabel => "Label".to_string(),
        }
    }
}

pub(crate) fn nodes_table<'a>(
    tv: &'a TreeView,
    ts: Rc<TreeState>,
    w: f32,
    h: f32,
) -> Element<'a, TvMsg> {
    let fn_visible_rows = |start_idx: usize, max_to_return: usize| {
        ts.edges_in_range(start_idx, max_to_return).unwrap_or_default()
    };

    let fn_total_row_count = || ts.edge_count();

    table(
        nodes_table_columns_spec(ts.clone(), tv),
        fn_visible_rows,
        fn_total_row_count,
        tv.nodes_table_scrollable_id,
        tv.nodes_table_scroll_y_offset,
        w,
        h,
        TvMsg::NodesTableScrolledOrResized,
    )
}

fn nodes_table_columns_spec<'a>(
    ts: Rc<TreeState>,
    tv: &'a TreeView,
) -> Vec<TableColumnSpecification<'a, TvMsg, Edge>> {
    let mut columns: Vec<TableColumnSpecification<'a, TvMsg, Edge>> = vec![];

    let column_order = [
        NodesTableField::Selected,
        NodesTableField::NodeId,
        NodesTableField::NodeType,
        NodesTableField::BranchLength,
        NodesTableField::NodeLabel,
    ];

    column_order.iter().for_each(|&f| {
        let include_field = match f {
            NodesTableField::BranchLength => ts.has_brlen(),
            _ => true,
        };

        if include_field {
            let fn_is_selected = |node_id: NodeId, ts: Rc<TreeState>| {
                ts.sel_node_ids().contains(&node_id)
            };

            let fn_select_msg = |node_id: NodeId| {
                Some(match tv.cfg.selection_lock {
                    true => TvMsg::SelectDeselectNode(node_id),
                    false => TvMsg::SelectDeselectNodeExclusive(node_id),
                })
            };

            let fn_cell_data: Box<dyn Fn(Edge) -> TableCell<'a, TvMsg> + 'a>;
            let mut width: f32 = 8e0 * TABLE_TXT_SIZE;
            let ts = ts.clone();
            match f {
                NodesTableField::Selected => {
                    width = 3e0 * TABLE_TXT_SIZE;
                    fn_cell_data = Box::new(move |e: Edge| {
                        let is_selected = fn_is_selected(e.node_id, ts.clone());
                        TableCell {
                            cell_content: txt_bool(is_selected)
                                .size(TABLE_TXT_SIZE)
                                .into(),
                            is_selected,
                            select_msg: fn_select_msg(e.node_id),
                        }
                    });
                }

                NodesTableField::NodeId => {
                    width = 6e0 * TABLE_TXT_SIZE;
                    fn_cell_data = Box::new(move |e: Edge| TableCell {
                        cell_content: txt(e.node_id)
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected: fn_is_selected(e.node_id, ts.clone()),
                        select_msg: fn_select_msg(e.node_id),
                    });
                }

                NodesTableField::NodeType => {
                    width = 5e0 * TABLE_TXT_SIZE;
                    fn_cell_data = Box::new(move |e: Edge| {
                        let node_opt = ts.tree().node(Some(e.node_id));
                        let node_type = if let Some(node) = node_opt {
                            node.node_type().to_string()
                        } else {
                            "-".to_string()
                        };

                        TableCell {
                            cell_content: txt(node_type)
                                .size(TABLE_TXT_SIZE)
                                .into(),
                            is_selected: fn_is_selected(e.node_id, ts.clone()),
                            select_msg: fn_select_msg(e.node_id),
                        }
                    });
                }

                NodesTableField::BranchLength => {
                    width = 7e0 * TABLE_TXT_SIZE;
                    fn_cell_data = Box::new(move |e: Edge| TableCell {
                        cell_content: txt_float(e.branch_length, 3)
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected: fn_is_selected(e.node_id, ts.clone()),
                        select_msg: fn_select_msg(e.node_id),
                    });
                }

                NodesTableField::NodeLabel => {
                    fn_cell_data = Box::new(move |e: Edge| TableCell {
                        cell_content: txt(e
                            .label
                            .unwrap_or("-".into())
                            .to_string())
                        .size(TABLE_TXT_SIZE)
                        .into(),
                        is_selected: fn_is_selected(e.node_id, ts.clone()),
                        select_msg: fn_select_msg(e.node_id),
                    });
                }
            };

            let sort_order = if tv.nodes_table_sort_col == f {
                Some(tv.nodes_table_sort_ord)
            } else {
                None
            };

            let col_spec = TableColumnSpecification {
                header_text: f.into(),
                sort_order,
                sort_msg: TvMsg::NodesTableSortColumnChanged(f),
                width,
                fn_cell_data,
            };

            columns.push(col_spec);
        }
    });

    columns
}
