use dendros::Attribute;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributesTableField {
    Selected,
    Key,
    Value,
}

impl From<AttributesTableField> for String {
    fn from(esf: AttributesTableField) -> Self {
        match esf {
            AttributesTableField::Key => "Key".to_string(),
            AttributesTableField::Selected => "Selected".to_string(),
            AttributesTableField::Value => "Value".to_string(),
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
        ts.sel_node_ids()
            .iter()
            .flat_map(|&node_id| ts.tree().node_attributes(node_id))
            .map(|(k, v)| (k.clone(), v.clone()))
            .skip(start_idx)
            .take(max_to_return)
            .collect()
    };

    let fn_total_row_count = || {
        ts.sel_node_ids()
            .iter()
            .flat_map(|&node_id| ts.tree().node_attributes(node_id))
            .count()
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
) -> Vec<TableColumnSpecification<'a, TvMsg, (String, Attribute)>> {
    let mut columns: Vec<TableColumnSpecification<'_, _, _>> = vec![];

    let column_order = [
        AttributesTableField::Selected,
        AttributesTableField::Key,
        AttributesTableField::Value,
    ];

    column_order.iter().for_each(|&f| {
        let common = |key: String| (false, None);
        let fn_cell_data: Box<
            dyn Fn((String, Attribute)) -> TableCell<'a, TvMsg> + 'a,
        > = match f {
            AttributesTableField::Key => {
                Box::new(move |kv: (String, Attribute)| {
                    let (is_selected, select_msg) = common(kv.0.clone());
                    TableCell {
                        cell_content: txt(kv.0.to_string())
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected,
                        select_msg,
                    }
                })
            }
            AttributesTableField::Selected => {
                Box::new(move |kv: (String, Attribute)| {
                    let (is_selected, select_msg) = common(kv.0);
                    TableCell {
                        cell_content: txt_bool(is_selected)
                            .size(TABLE_TXT_SIZE)
                            .into(),
                        is_selected,
                        select_msg,
                    }
                })
            }
            AttributesTableField::Value => {
                Box::new(move |kv: (String, Attribute)| {
                    let (is_selected, select_msg) = common(kv.0);
                    TableCell {
                        cell_content: txt(format!("{}", kv.1))
                            .size(TABLE_TXT_SIZE)
                            .into(),
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
            width: 8e0 * TABLE_TXT_SIZE,
            fn_cell_data,
        };

        columns.push(col_spec);
    });

    columns
}
