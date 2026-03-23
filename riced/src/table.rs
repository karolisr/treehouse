use crate::*;

pub type TableColHeader<'a, Msg> = Option<Element<'a, Msg>>;

#[derive(Debug, Clone, Copy)]
pub enum SortOrd {
    Ascending,
    Descending,
}

#[allow(clippy::too_many_arguments)]
pub fn table<
    'a,
    'b,
    Msg: Clone + 'a,
    RowData: Clone + 'a,
    E: Into<Element<'a, Msg>>,
>(
    headers: Vec<(String, Option<SortOrd>, Msg)>,
    columns: Vec<Box<dyn Fn(RowData) -> (E, bool, Option<Msg>)>>,
    column_widths: Option<Vec<f32>>,
    visible_rows: impl IntoIterator<Item = RowData>,
    scrollable_id: &'static str,
    scrollable_y_offset: f32,
    w: f32,
    h: f32,
    table_scrollable_height: f32,
    table_scrollable_content_height: f32,
    on_scroll: impl Fn(ScrollableViewport) -> Msg + 'a,
) -> Element<'a, Msg> {
    let width_available_to_columns = table_width_available_to_columns(
        w,
        h,
        table_scrollable_content_height,
        columns.len(),
    );

    let min_column_width_fraction = 1e0 / 1e1;

    let base_col_width =
        (width_available_to_columns * min_column_width_fraction).max(8e1 * SF);

    let mut width_allocated: f32 = 0e0;
    let calc_width = |i: usize,
                      width_allocated: f32,
                      column_widths: Option<Vec<f32>>|
     -> f32 {
        if i == (columns.len() - 1) {
            width_available_to_columns - width_allocated
        } else {
            if let Some(column_widths) = column_widths {
                column_widths[i]
            } else {
                base_col_width
            }
        }
    };

    let mut header_cells: Row<'a, Msg> = Row::new();

    for (i, (hdr_txt, sort_ord, on_click)) in headers.iter().enumerate() {
        let width = calc_width(i, width_allocated, column_widths.clone());
        width_allocated += width;
        let header_style = match i {
            0 => sty_table_cell_header_left,
            last if last == columns.len() - 1 => sty_table_cell_header_right,
            _ => sty_table_cell_header,
        };

        let header_cell: Element<'a, Msg> = table_header_cell(
            hdr_txt,
            *sort_ord,
            on_click.clone(),
            width,
            TABLE_ROW_H,
            header_style,
        );

        header_cells = header_cells.push(header_cell);
    }

    let header_row = container(header_cells.padding(ZERO).spacing(TABLE_SEP_W))
        .style(sty_table_row_header)
        .width(Length::Shrink);

    width_allocated = 0e0;
    let cols: Vec<RicedTableCol<'_, '_, RowData, Msg>> = columns
        .iter()
        .enumerate()
        .map(|(i, row_data_extractor_func)| {
            let width = calc_width(i, width_allocated, column_widths.clone());
            width_allocated += width;
            let table_cell_func = move |row_data: RowData| {
                let (content_txt, is_selected, on_click) =
                    row_data_extractor_func(row_data);
                table_cell(
                    content_txt, width, TABLE_ROW_H, is_selected, on_click,
                )
            };

            riced_table_col(TableColHeader::None, table_cell_func)
                .width(Length::Fixed(width))
        })
        .collect();

    let mut table_body = riced_table(cols, visible_rows);
    table_body = table_body.padding(0e0);
    table_body = table_body.separator(TABLE_SEP_W);

    let mut scrollable_body = Scrollable::new(
        container(table_body)
            .padding(Padding {
                top: scrollable_y_offset,
                bottom: 0e0,
                left: 0e0,
                right: 0e0,
            })
            .height(table_scrollable_content_height),
    );
    scrollable_body = scrollable_body.direction(ScrollableDirection::Both {
        horizontal: scroll_bar(),
        vertical: scroll_bar(),
    });
    scrollable_body = scrollable_body.id(scrollable_id);
    scrollable_body = scrollable_body.on_scroll(on_scroll);
    scrollable_body = scrollable_common(
        scrollable_body,
        w - BORDER_W * TWO,
        table_scrollable_height,
    );

    let mut assembly = Column::new();
    assembly = assembly.push(header_row);
    assembly = assembly.push(scrollable_body);
    assembly = assembly.padding(BORDER_W);
    assembly = assembly.width(w);
    assembly = assembly.height(h);
    container(assembly).style(sty_cont_no_shadow).clip(true).into()
}

pub fn table_header_cell<'a, Msg: Clone + 'a>(
    header_text: &str,
    sort_ord: Option<SortOrd>,
    on_click: Msg,
    w: f32,
    h: f32,
    style: impl Fn(&Theme) -> ContainerStyle + 'a,
) -> Element<'a, Msg> {
    let sort_indicator = if let Some(sort_ord) = sort_ord {
        match sort_ord {
            SortOrd::Ascending => "▲",
            SortOrd::Descending => "▼",
        }
    } else {
        ""
    };

    let text_size = 1e1 * SF;

    let header_content = container(iced_row!(
        txt(header_text).size(text_size),
        space_h(Length::Fixed(PADDING / TWO), Length::Shrink),
        space_h(Length::Fill, Length::Shrink),
        txt(sort_indicator).size(text_size),
    ))
    .padding(Padding { left: PADDING, right: PADDING, top: ZERO, bottom: ZERO })
    .width(Length::Fixed(w))
    .center_y(Length::Fixed(h))
    .style(style);

    mouse_area(header_content).on_press(on_click).into()
}

pub fn table_cell<'a, Msg: Clone + 'a>(
    content: impl Into<Element<'a, Msg>>,
    w: f32,
    h: f32,
    is_selected: bool,
    on_click: Option<Msg>,
) -> Element<'a, Msg> {
    let cell_container = container(content)
        .padding(Padding {
            left: PADDING,
            right: PADDING,
            top: ZERO,
            bottom: ZERO,
        })
        .center_y(Length::Fixed(h))
        .width(Length::Fixed(w));

    let styled_cell = if is_selected {
        cell_container.style(sty_table_cell_selected)
    } else {
        cell_container.style(sty_table_cell)
    };

    let ma = mouse_area(styled_cell);
    match on_click {
        Some(msg) => ma.on_press(msg).into(),
        None => ma.into(),
    }
}

pub fn table_scrollable_height(h: f32) -> f32 {
    (h - TABLE_ROW_H - TABLE_SEP_W).max(0e0)
}

pub fn table_scrollable_content_height(total_row_count: usize) -> f32 {
    ((TABLE_ROW_H + TABLE_SEP_W) * total_row_count as f32).floor()
        + TABLE_ROW_H / TWO
}

pub fn table_first_visible_row(scroll_y_offset: f32) -> usize {
    (scroll_y_offset / (TABLE_ROW_H + TABLE_SEP_W)) as usize
}

pub fn table_max_visible_rows(table_scrollable_height: f32) -> usize {
    (table_scrollable_height / (TABLE_ROW_H + TABLE_SEP_W)).ceil() as usize
}

pub fn table_width_available_to_columns(
    w: f32,
    h: f32,
    table_scrollable_content_height: f32,
    column_count: usize,
) -> f32 {
    if table_scrollable_content_height + TABLE_ROW_H + TABLE_SEP_W <= h {
        w - TABLE_SEP_W * (column_count - 1) as f32 - BORDER_W * TWO
    } else {
        w - TABLE_SEP_W * (column_count - 1) as f32
            - BORDER_W * TWO
            - SCROLLBAR_W
    }
}
