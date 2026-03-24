use crate::*;

pub(crate) fn nodes_table_headers(
    ts: Rc<TreeState>,
    tv: &TreeView,
) -> Vec<(String, Option<SortOrd>, TvMsg)> {
    let fields = [
        EdgeSortField::NodeId,
        EdgeSortField::NodeType,
        EdgeSortField::Selected,
        EdgeSortField::BranchLength,
        EdgeSortField::NodeLabel,
    ];

    let mut headers = Vec::new();

    fields.iter().for_each(|&f| {
        let mut include_field = true;
        if !ts.has_brlen() && f == EdgeSortField::BranchLength {
            include_field = false;
        }
        if include_field {
            let mut sort_ord: Option<SortOrd> = None;
            if tv.nodes_table_sort_col == f {
                sort_ord = Some(tv.nodes_table_sort_ord);
            }
            let sort_msg = TvMsg::NodesTableSortColumnChanged(f);
            headers.push((String::from(f), sort_ord, sort_msg));
        }
    });

    headers
}

pub(crate) fn nodes_table_columns<'a>(
    ts: Rc<TreeState>,
) -> Vec<Box<dyn Fn(Edge) -> (Element<'a, TvMsg>, bool, Option<TvMsg>)>> {
    let mut cols: Vec<Box<dyn Fn(_) -> _>> = Vec::new();

    let common = |node_id: NodeId, ts: Rc<TreeState>| {
        (
            ts.sel_node_ids().contains(&node_id),
            Some(TvMsg::SelectDeselectNode(node_id)),
        )
    };

    let tmp = ts.clone();
    cols.push(Box::new(move |e: Edge| {
        let (is_selected, on_click) = common(e.node_id, tmp.clone());
        (txt(e.node_id).size(TABLE_TXT_SIZE).into(), is_selected, on_click)
    }));

    let tmp = ts.clone();
    cols.push(Box::new(move |e: Edge| {
        let node_opt = tmp.tree().node(Some(e.node_id));
        let node_type = if let Some(node) = node_opt {
            node.node_type().to_string()
        } else {
            "-".to_string()
        };
        let (is_selected, on_click) = common(e.node_id, tmp.clone());
        (txt(node_type).size(TABLE_TXT_SIZE).into(), is_selected, on_click)
    }));

    let tmp = ts.clone();
    cols.push(Box::new(move |e: Edge| {
        let (is_selected, on_click) = common(e.node_id, tmp.clone());
        (
            txt_bool(is_selected).size(TABLE_TXT_SIZE).into(),
            is_selected,
            on_click,
        )
    }));

    if ts.has_brlen() {
        let tmp = ts.clone();
        cols.push(Box::new(move |e: Edge| {
            let (is_selected, on_click) = common(e.node_id, tmp.clone());
            (
                txt_float(e.branch_length, 3).size(TABLE_TXT_SIZE).into(),
                is_selected,
                on_click,
            )
        }));
    }

    let tmp = ts.clone();
    cols.push(Box::new(move |e: Edge| {
        let (is_selected, on_click) = common(e.node_id, tmp.clone());
        (
            txt(e.label.unwrap_or("-".into()).to_string())
                .size(TABLE_TXT_SIZE)
                .into(),
            is_selected,
            on_click,
        )
    }));

    cols
}

pub(crate) fn nodes_table_column_widths(ts: Rc<TreeState>) -> Vec<f32> {
    let mut col_ws = vec![8e1 * SF, 5e1 * SF, 6e1 * SF];

    if ts.has_brlen() {
        col_ws.push(8e1 * SF);
    }

    col_ws.push(2.5e2 * SF);

    col_ws
}

pub(crate) fn nodes_table<'a>(
    tv: &'a TreeView,
    ts: Rc<TreeState>,
    scroll_y_offset: f32,
    scrollable_id: &'static str,
    w: f32,
    h: f32,
) -> Element<'a, TvMsg> {
    let start_idx = table_first_visible_row(scroll_y_offset);
    let table_scrollable_height = table_scrollable_height(h);
    let max_to_return = table_max_visible_rows(table_scrollable_height);
    if let Some(edges) = ts.edges_in_range(start_idx, max_to_return) {
        let table_scrollable_content_height =
            table_scrollable_content_height(ts.edge_count());

        table(
            nodes_table_headers(ts.clone(), tv),
            nodes_table_columns(ts.clone()),
            Some(nodes_table_column_widths(ts)),
            edges,
            scrollable_id,
            scroll_y_offset,
            w,
            h,
            table_scrollable_height,
            table_scrollable_content_height,
            TvMsg::NodesTableScrolledOrResized,
        )
    } else {
        txt("No edges available").into()
    }
}
