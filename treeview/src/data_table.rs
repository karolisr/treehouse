use crate::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeDataTableSortColumn {
    NodeId,
    NodeLabel,
    Selected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTableSortDirection {
    Ascending,
    Descending,
}

fn data_table_header_cell<'a>(
    tv: &'a TreeView,
    header_text: &str,
    column: NodeDataTableSortColumn,
    width: Float,
    height: Float,
    style: impl Fn(&Theme) -> ContainerStyle + 'a,
) -> Element<'a, TvMsg> {
    let sort_indicator = if tv.node_data_table_sort_col == column {
        match tv.node_data_table_sort_dir {
            DataTableSortDirection::Ascending => "▲",
            DataTableSortDirection::Descending => "▼",
        }
    } else {
        ""
    };

    let header_content = container(iced_row!(
        txt(header_text),
        space_h(Length::Fixed(PADDING / TWO), Length::Fill),
        space_h(Length::Fill, Length::Fill),
        txt(sort_indicator),
    ))
    .padding(PADDING / TWO)
    .width(Length::Fixed(width))
    .center_y(Length::Fixed(height))
    .style(style);

    mouse_area(header_content)
        .on_press(TvMsg::NodeDataTableSortColumnChanged(column))
        .into()
}

fn node_data_table_cell<'a>(
    content: Element<'a, TvMsg>,
    node_id: NodeId,
    is_selected: bool,
    width: Float,
    height: Float,
) -> Element<'a, TvMsg> {
    let cell_container = container(content)
        .padding(PADDING / TWO)
        .width(Length::Fixed(width))
        .height(Length::Fixed(height));

    let styled_cell = if is_selected {
        cell_container.style(sty_table_cell_selected)
    } else {
        cell_container.style(sty_table_cell)
    };

    mouse_area(styled_cell).on_press(TvMsg::SelectDeselectNode(node_id)).into()
}

pub(crate) fn node_data_table<'a>(
    tv: &'a TreeView,
    id: &'static str,
    w: Float,
    h: Float,
    sel_node_ids: HashSet<NodeId>,
    cached_edges: &[Edge],
) -> Element<'a, TvMsg> {
    // ToDo: need to determine column count dynamically.
    let column_count: usize = 3;

    let row_height = TXT_SIZE + PADDING;
    // Scrollable height allowing for a header row.
    let scrollable_height = (h - row_height).max(ZRO);
    // This is the total height of the table. Parent Scrollable needs to know
    // table_body_height to draw the scroll bar correctly.
    let table_body_height = row_height * (cached_edges.len() + 1) as f32;

    // The width of lines drawn between columns or rows.
    let separator_width = BORDER_W;

    let min_column_width_fraction = ONE / TEN;
    let width_available_to_columns = if table_body_height + row_height <= h {
        w - separator_width * (column_count - 1) as f32
    } else {
        w - separator_width * (column_count - 1) as f32 - SCROLLBAR_W - PADDING
    };

    let node_id_column_width =
        (width_available_to_columns * min_column_width_fraction).max(7e1 * SF);

    let node_selected_column_width =
        (width_available_to_columns * min_column_width_fraction).max(7e1 * SF);

    let node_label_column_width = width_available_to_columns
        - node_selected_column_width
        - node_id_column_width;

    // ToDo: should be refectored into a generic "header_row" function.
    // The header row is created manually, because we want it to stay on top of
    // the table when the table body is scrolled.
    let header_row = container(
        iced_row![
            data_table_header_cell(
                tv,
                "Selected",
                NodeDataTableSortColumn::Selected,
                node_selected_column_width,
                row_height,
                sty_table_cell_header_left
            ),
            data_table_header_cell(
                tv,
                "Node ID",
                NodeDataTableSortColumn::NodeId,
                node_id_column_width,
                row_height,
                sty_table_cell_header
            ),
            data_table_header_cell(
                tv,
                "Node Label",
                NodeDataTableSortColumn::NodeLabel,
                node_label_column_width,
                row_height,
                sty_table_cell_header_right
            ),
        ]
        .padding(ZRO)
        .spacing(separator_width),
    )
    .style(sty_table_row_header);

    // ToDo: should be refectored into a generic "columns" function.
    let columns = vec![
        table_col(
            space_h(Length::Shrink, Length::Shrink),
            |edge: Edge| -> Element<'a, TvMsg> {
                let is_selected = sel_node_ids.contains(&edge.node_id);
                node_data_table_cell(
                    txt_bool(is_selected).into(),
                    edge.node_id,
                    is_selected,
                    node_selected_column_width,
                    row_height,
                )
            },
        )
        .width(Length::Fixed(node_selected_column_width)),
        table_col(
            space_h(Length::Shrink, Length::Shrink),
            |edge: Edge| -> Element<'a, TvMsg> {
                let is_selected = sel_node_ids.contains(&edge.node_id);
                node_data_table_cell(
                    txt(edge.node_id).into(),
                    edge.node_id,
                    is_selected,
                    node_id_column_width,
                    row_height,
                )
            },
        )
        .width(Length::Fixed(node_id_column_width)),
        table_col(
            space_h(Length::Shrink, Length::Shrink),
            |edge: Edge| -> Element<'a, TvMsg> {
                let is_selected = sel_node_ids.contains(&edge.node_id);
                node_data_table_cell(
                    txt(edge.label.as_deref().unwrap_or("-")).into(),
                    edge.node_id,
                    is_selected,
                    node_label_column_width,
                    row_height,
                )
            },
        )
        .width(Length::Fixed(node_label_column_width)),
    ];

    let max_visible_rows = (scrollable_height / row_height) as usize;
    let scroll_y = tv.node_data_table_scroll_y;
    let first_visible_row = (scroll_y / row_height) as usize;
    let last_visible_row = first_visible_row + max_visible_rows;
    let start_idx = first_visible_row;
    let end_idx = last_visible_row.min(cached_edges.len());

    let visible_edges: Vec<Edge> = if start_idx < end_idx {
        cached_edges[start_idx..end_idx].to_vec()
    } else {
        cached_edges.iter().take(max_visible_rows).cloned().collect()
    };

    let table_body =
        table(columns, visible_edges).padding(ZRO).separator(separator_width);

    let mut scrollable_body = Scrollable::new(
        container(table_body)
            .padding(Padding {
                top: scroll_y,
                bottom: ZRO,
                left: ZRO,
                right: ZRO,
            })
            .height(table_body_height),
    );
    scrollable_body = scrollable_body.direction(ScrollableDirection::Both {
        horizontal: scroll_bar(),
        vertical: scroll_bar(),
    });
    scrollable_body = scrollable_body.id(id);
    scrollable_body =
        scrollable_body.on_scroll(TvMsg::NodeDataTableScrolledOrResized);
    scrollable_body = scrollable_common(scrollable_body, w, scrollable_height);

    let mut final_table = Column::new();
    final_table = final_table.push(header_row);
    final_table = final_table.push(scrollable_body);
    final_table = final_table.padding(ZRO);
    final_table = final_table.width(w);
    final_table = final_table.height(h);
    final_table.into()
}
