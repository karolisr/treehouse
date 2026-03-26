use crate::*;

type RicedTableColHeader<'a, Msg> = Option<Element<'a, Msg>>;

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[allow(missing_debug_implementations)]
pub struct TableCell<'a, Msg: Clone + 'a> {
    pub cell_content: Element<'a, Msg>,
    pub is_selected: bool,
    pub select_msg: Option<Msg>,
}

#[allow(missing_debug_implementations)]
pub struct TableColumnSpecification<'a, Msg: Clone + 'a, RowData: Clone + 'a> {
    pub header_text: String,
    pub sort_order: Option<SortOrder>,
    pub sort_msg: Msg,
    pub width: f32,
    pub fn_cell_data: Box<dyn Fn(RowData) -> TableCell<'a, Msg> + 'a>,
}

#[allow(clippy::too_many_arguments)]
pub fn table<'a, Msg: Clone + 'a, RowData: Clone + 'a>(
    column_specs: Vec<TableColumnSpecification<'a, Msg, RowData>>,
    fn_visible_rows: impl Fn(usize, usize) -> Vec<RowData>,
    fn_total_row_count: impl Fn() -> usize,
    scrollable_id: &'static str,
    scrollable_y_offset: f32,
    w: f32,
    h: f32,
    on_scroll: impl Fn(ScrollableViewport) -> Msg + 'a,
) -> Element<'a, Msg> {
    if column_specs.is_empty() {
        return txt("NO COLUMNS").into();
    }

    let scrollable_content_height =
        scrollable_content_height(fn_total_row_count());

    let width_available_to_columns = width_available_for_columns(
        w,
        h,
        scrollable_content_height,
        column_specs.len(),
    );

    let mut width_allocated_to_columns: f32 = 0e0;
    let calc_width =
        |i: usize, width_allocated: f32, column_width: f32| -> f32 {
            if i == (column_specs.len() - 1) {
                width_available_to_columns - width_allocated
            } else {
                column_width
            }
        };

    let mut header_cells: Row<'a, Msg> = Row::new();
    let mut columns: Vec<RicedTableCol<'a, '_, RowData, Msg>> = Vec::new();
    for (i, col) in column_specs.iter().enumerate() {
        let width = calc_width(i, width_allocated_to_columns, col.width);
        width_allocated_to_columns += width;

        let header_style = match i {
            0 => sty_table_cell_header_left,
            last if last == column_specs.len() - 1 => {
                sty_table_cell_header_right
            }
            _ => sty_table_cell_header,
        };

        header_cells = header_cells.push(header_cell(
            &col.header_text,
            col.sort_order,
            col.sort_msg.clone(),
            width,
            TABLE_ROW_H,
            header_style,
        ));

        let table_cell_func = move |row_data: RowData| {
            let cell_data = (col.fn_cell_data)(row_data);
            let content = cell_data.cell_content;
            let is_selected = cell_data.is_selected;
            let select_msg = cell_data.select_msg;

            cell(content, width, TABLE_ROW_H, is_selected, select_msg)
        };

        columns.push(
            riced_table_col(RicedTableColHeader::None, table_cell_func)
                .width(Length::Fixed(width)),
        );
    }

    let header_row = container(header_cells.padding(ZERO).spacing(TABLE_SEP_W))
        .style(sty_table_row_header)
        .width(Length::Shrink);

    let scrollable_height = scrollable_height(h);

    let row_data = fn_visible_rows(
        first_visible_row_idx(scrollable_y_offset),
        max_visible_row_count(scrollable_height),
    );

    let table_body =
        riced_table(columns, row_data).padding(0e0).separator(TABLE_SEP_W);

    let scrollable_body_prelim = Scrollable::new(
        container(table_body)
            .padding(Padding {
                top: scrollable_y_offset,
                bottom: 0e0,
                left: 0e0,
                right: 0e0,
            })
            .height(scrollable_content_height),
    )
    .direction(ScrollableDirection::Both {
        horizontal: scroll_bar(),
        vertical: scroll_bar(),
    })
    .id(scrollable_id)
    .on_scroll(on_scroll);

    let scrollable_body = scrollable_common(
        scrollable_body_prelim,
        w - BORDER_W * TWO,
        scrollable_height,
    );

    let mut assembly = Column::new();
    assembly = assembly.push(header_row);
    assembly = assembly.push(scrollable_body);
    assembly = assembly.padding(BORDER_W).width(w).height(h);
    container(assembly).style(sty_cont_no_shadow_no_border).clip(true).into()
}

fn header_cell<'a, Msg: Clone + 'a>(
    header_text: &str,
    sort_ord: Option<SortOrder>,
    on_click: Msg,
    w: f32,
    h: f32,
    style: impl Fn(&Theme) -> ContainerStyle + 'a,
) -> Element<'a, Msg> {
    let sort_indicator = if let Some(sort_ord) = sort_ord {
        match sort_ord {
            SortOrder::Ascending => "▲",
            SortOrder::Descending => "▼",
        }
    } else {
        ""
    };

    let header_content = container(iced_row!(
        txt(header_text).size(TABLE_TXT_SIZE),
        space_h(Length::Fixed(TABLE_CELL_PADDING), Length::Shrink),
        space_h(Length::Fill, Length::Shrink),
        txt(sort_indicator).size(TABLE_TXT_SIZE),
    ))
    .padding(Padding {
        left: TABLE_CELL_PADDING,
        right: TABLE_CELL_PADDING,
        top: ZERO,
        bottom: ZERO,
    })
    .width(Length::Fixed(w))
    .center_y(Length::Fixed(h))
    .clip(true)
    .style(style);

    mouse_area(header_content).on_press(on_click).into()
}

fn cell<'a, Msg: Clone + 'a>(
    content: impl Into<Element<'a, Msg>>,
    w: f32,
    h: f32,
    is_selected: bool,
    select_msg: Option<Msg>,
) -> Element<'a, Msg> {
    let cell_container = container(content)
        .padding(Padding {
            left: TABLE_CELL_PADDING,
            right: TABLE_CELL_PADDING,
            top: ZERO,
            bottom: ZERO,
        })
        .center_y(Length::Fixed(h))
        .width(Length::Fixed(w))
        .clip(true);

    let styled_cell = if is_selected {
        cell_container.style(sty_table_cell_selected)
    } else {
        cell_container.style(sty_table_cell)
    };

    let ma = mouse_area(styled_cell);
    match select_msg {
        Some(msg) => ma.on_press(msg).into(),
        None => ma.into(),
    }
}

fn scrollable_height(h: f32) -> f32 {
    (h - TABLE_ROW_H - TABLE_SEP_W).max(0e0)
}

fn scrollable_content_height(total_row_count: usize) -> f32 {
    ((TABLE_ROW_H + TABLE_SEP_W) * total_row_count as f32).floor()
        + TABLE_ROW_H / TWO
}

fn first_visible_row_idx(scroll_y_offset: f32) -> usize {
    (scroll_y_offset / (TABLE_ROW_H + TABLE_SEP_W)) as usize
}

fn max_visible_row_count(table_scrollable_height: f32) -> usize {
    (table_scrollable_height / (TABLE_ROW_H + TABLE_SEP_W)).ceil() as usize
}

fn width_available_for_columns(
    w: f32,
    h: f32,
    table_scrollable_content_height: f32,
    column_count: usize,
) -> f32 {
    if column_count > 0 {
        if table_scrollable_content_height + TABLE_ROW_H + TABLE_SEP_W <= h {
            w - TABLE_SEP_W * (column_count - 1) as f32 - BORDER_W * TWO
        } else {
            w - TABLE_SEP_W * (column_count - 1) as f32
                - BORDER_W * TWO
                - SCROLLBAR_W
                - PADDING
        }
    } else {
        0e0
    }
}
