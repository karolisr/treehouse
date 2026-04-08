use std::{cmp::Ordering, str::FromStr};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributesTableField {
    Selected,
    NodeId,
    Selector,
    Name,
    Value,
}

impl From<AttributesTableField> for String {
    fn from(f: AttributesTableField) -> Self {
        match f {
            AttributesTableField::Selected => "Selected".to_string(),
            AttributesTableField::NodeId => "Node".to_string(),
            AttributesTableField::Selector => "Selector".to_string(),
            AttributesTableField::Name => "Name".to_string(),
            AttributesTableField::Value => "Value".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttributesTableRowData {
    name: String,
    attribute: Attribute,
    selector: AttributeSelector,
    node_id: NodeId,
}

impl From<(&String, &Attribute, AttributeSelector, NodeId)>
    for AttributesTableRowData
{
    fn from(
        attr_tuple: (&String, &Attribute, AttributeSelector, NodeId),
    ) -> Self {
        AttributesTableRowData {
            name: attr_tuple.0.clone(),
            attribute: attr_tuple.1.clone(),
            selector: attr_tuple.2,
            node_id: attr_tuple.3,
        }
    }
}

pub(crate) fn attributes_table<'a>(
    tv: &'a TreeView,
    ts: Rc<TreeState>,
    w: f32,
    h: f32,
) -> Element<'a, TvMsg> {
    let fn_visible_rows = |start_idx: usize, max_to_return: usize| {
        let node_attr_rows = ts
            .sel_node_ids()
            .iter()
            .flat_map(|&node_id| {
                ts.tree()
                    .node_attributes(node_id)
                    .iter()
                    .map(move |(k, v)| (k, v, node_id))
            })
            .map(|(k, v, node_id)| {
                AttributesTableRowData::from((
                    k,
                    v,
                    AttributeSelector::Node,
                    node_id,
                ))
            });

        let branch_attr_rows = ts
            .sel_node_ids()
            .iter()
            .flat_map(|&node_id| {
                ts.tree()
                    .branch_attributes(node_id)
                    .iter()
                    .map(move |(k, v)| (k, v, node_id))
            })
            .map(|(k, v, node_id)| {
                AttributesTableRowData::from((
                    k,
                    v,
                    AttributeSelector::Branch,
                    node_id,
                ))
            });

        let mut visible_rows: Vec<AttributesTableRowData> = Vec::new();
        visible_rows.extend(node_attr_rows);
        visible_rows.extend(branch_attr_rows);

        let sorting_order = |ord: Ordering| match tv.attributes_table_sort_ord {
            SortOrder::Ascending => ord,
            SortOrder::Descending => ord.reverse(),
        };

        match tv.attributes_table_sort_col {
            AttributesTableField::Selected => (),
            AttributesTableField::NodeId => {
                visible_rows
                    .sort_by(|a, b| sorting_order(a.node_id.cmp(&b.node_id)));
            }
            AttributesTableField::Selector => {
                visible_rows
                    .sort_by(|a, b| sorting_order(a.selector.cmp(&b.selector)));
            }
            AttributesTableField::Name => {
                visible_rows.sort_by(|a, b| sorting_order(a.name.cmp(&b.name)));
            }
            AttributesTableField::Value => {
                visible_rows.sort_by(|a, b| {
                    sorting_order(
                        a.attribute
                            .partial_cmp(&b.attribute)
                            .unwrap_or(Ordering::Equal),
                    )
                });
            }
        };

        visible_rows
            .iter()
            .skip(start_idx)
            .take(max_to_return)
            .cloned()
            .collect()
    };

    let fn_total_row_count = || {
        let nac = ts
            .sel_node_ids()
            .iter()
            .flat_map(|&node_id| ts.tree().node_attributes(node_id))
            .count();

        let bac = ts
            .sel_node_ids()
            .iter()
            .flat_map(|&node_id| ts.tree().branch_attributes(node_id))
            .count();

        nac + bac
    };

    table(
        attributes_table_columns_spec(ts.clone(), tv),
        fn_visible_rows,
        fn_total_row_count,
        tv.attributes_table_scrollable_id,
        tv.attributes_table_scroll_y_offset,
        w,
        h,
        TvMsg::AttributesTableScrolledOrResized,
    )
}

fn attributes_table_columns_spec<'a>(
    ts: Rc<TreeState>,
    tv: &TreeView,
) -> Vec<TableColumnSpecification<'a, TvMsg, AttributesTableRowData>> {
    let mut columns: Vec<TableColumnSpecification<'_, _, _>> = vec![];

    let column_order = [
        AttributesTableField::Selected,
        AttributesTableField::Selector,
        AttributesTableField::NodeId,
        AttributesTableField::Name,
        AttributesTableField::Value,
    ];

    column_order.iter().for_each(|&f| {
        let width = match f {
            AttributesTableField::Selected => 3e0 * TABLE_TXT_SIZE,
            AttributesTableField::NodeId => 5e0 * TABLE_TXT_SIZE,
            AttributesTableField::Selector => 3e0 * TABLE_TXT_SIZE,
            AttributesTableField::Name => 1e1 * TABLE_TXT_SIZE,
            AttributesTableField::Value => 8e0 * TABLE_TXT_SIZE,
        };
        let common = |key: String| (false, None);
        let fn_cell_data: Box<
            dyn Fn(AttributesTableRowData) -> TableCell<'a, TvMsg> + 'a,
        > = match f {
            AttributesTableField::Selected => {
                Box::new(move |kv: AttributesTableRowData| {
                    let (is_selected, select_msg) = common(kv.name);
                    TableCell {
                        cell_content: txt_bool(is_selected)
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected,
                        select_msg,
                    }
                })
            }
            AttributesTableField::NodeId => {
                Box::new(move |kv: AttributesTableRowData| {
                    let (is_selected, select_msg) = common(kv.name.clone());
                    TableCell {
                        cell_content: txt(kv.node_id.to_string())
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected,
                        select_msg,
                    }
                })
            }
            AttributesTableField::Selector => {
                Box::new(move |kv: AttributesTableRowData| {
                    let (is_selected, select_msg) = common(kv.name.clone());
                    TableCell {
                        cell_content: txt(kv.selector.to_string())
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected,
                        select_msg,
                    }
                })
            }
            AttributesTableField::Name => {
                Box::new(move |kv: AttributesTableRowData| {
                    let (is_selected, select_msg) = common(kv.name.clone());
                    TableCell {
                        cell_content: txt(kv.name.clone())
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected,
                        select_msg,
                    }
                })
            }
            AttributesTableField::Value => {
                Box::new(move |kv: AttributesTableRowData| {
                    let (is_selected, select_msg) = common(kv.name);
                    TableCell {
                        cell_content: match kv.attribute {
                            Attribute::Integer(i) => {
                                txt_int(i).size(TABLE_TXT_SIZE).into()
                            }
                            Attribute::Decimal(f) => {
                                txt_float(f, 3).size(TABLE_TXT_SIZE).into()
                            }
                            Attribute::Color(c) => {
                                container(txt(&c).size(TABLE_TXT_SIZE))
                                    .style(move |theme| {
                                        let bg = Color::from_str(&c)
                                            .map_or(Clr::TRN, |c| c);
                                        sty_cont_with_bg_color(theme, bg)
                                    })
                                    .into()
                            }
                            Attribute::Text(t) => {
                                txt(t).size(TABLE_TXT_SIZE).into()
                            }
                            Attribute::List(attr_vals) => {
                                let mut row: Row<'_, TvMsg> = Row::new();

                                for attr_val in attr_vals {
                                    match attr_val {
                                        AttributeValue::Integer(i) => {
                                            row = row.push(
                                                txt_int(i).size(TABLE_TXT_SIZE),
                                            );
                                        }
                                        AttributeValue::Decimal(f) => {
                                            row = row.push(
                                                txt_float(f, 3)
                                                    .size(TABLE_TXT_SIZE),
                                            );
                                        }
                                        AttributeValue::Color(c) => {
                                            row = row.push(
                                                txt(c).size(TABLE_TXT_SIZE),
                                            );
                                        }
                                        AttributeValue::Text(t) => {
                                            row = row.push(
                                                txt(t).size(TABLE_TXT_SIZE),
                                            );
                                        }
                                    }
                                }

                                row.align_y(Vertical::Center)
                                    .width(width)
                                    .spacing(PADDING)
                                    .into()
                            }
                        },
                        is_selected,
                        select_msg,
                    }
                })
            }
        };

        let sort_order = if tv.attributes_table_sort_col == f {
            Some(tv.attributes_table_sort_ord)
        } else {
            None
        };

        let col_spec = TableColumnSpecification {
            header_text: f.into(),
            sort_order,
            sort_msg: TvMsg::AttributesTableSortColumnChanged(f),
            width,
            fn_cell_data,
        };

        columns.push(col_spec);
    });

    columns
}
