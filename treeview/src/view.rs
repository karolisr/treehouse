use crate::*;

impl TreeView {
    pub fn view(&'_ self) -> Element<'_, TvMsg> {
        let ts: Rc<TreeState>;

        if let Some(sel_ts_opt) = self.sel_tre() {
            ts = sel_ts_opt;
        } else {
            return center(txt("No trees loaded")).into();
        }

        let mut main_row: Row<TvMsg> = Row::new();
        let mut tool_bar_and_content_col: Column<TvMsg> = Column::new();

        main_row = main_row.padding(Padding {
            top: PADDING,
            right: PADDING,
            bottom: PADDING,
            left: PADDING,
        });

        main_row = main_row.spacing(PADDING);

        tool_bar_and_content_col = tool_bar_and_content_col.spacing(PADDING);

        if self.show_tool_bar {
            tool_bar_and_content_col =
                tool_bar_and_content_col.push(toolbar(self, ts.clone()));
        }

        if self.show_search_bar {
            tool_bar_and_content_col =
                tool_bar_and_content_col.push(search_bar(self, ts.clone()));
        }

        tool_bar_and_content_col = tool_bar_and_content_col.push(content(self));

        if self.show_side_bar {
            let sb = side_bar(self, ts);
            match self.sidebar_pos {
                SidebarPosition::Left => {
                    main_row = main_row.push(sb);
                    main_row = main_row.push(tool_bar_and_content_col);
                }
                SidebarPosition::Right => {
                    main_row = main_row.push(tool_bar_and_content_col);
                    main_row = main_row.push(sb);
                }
            }
        } else {
            main_row = main_row.push(tool_bar_and_content_col);
        }

        main_row.into()
    }
}

fn content<'a>(tv: &'a TreeView) -> Element<'a, TvMsg> {
    let ele: Element<'a, TvMsg> = if let Some(pane_grid) = &tv.pane_grid {
        PaneGrid::new(pane_grid, |pane_idx, tv_pane, _is_maximized| {
            PgContent::new(
                center(responsive(move |size| pane_content(tv, tv_pane, size)))
                    .padding(PADDING),
            )
            .style(
                match &pane_idx == pane_grid.panes.last_key_value().unwrap().0 {
                    true => match tv.sidebar_pos {
                        SidebarPosition::Left => sty_pane_body_bottom_right,
                        SidebarPosition::Right => sty_pane_body_bottom_left,
                    },
                    false => sty_pane_body,
                },
            )
        })
        .style(sty_pane_grid)
        .on_resize(ZRO, TvMsg::PaneResized)
        .min_size(SF * 2e2)
        .spacing(PADDING)
        .into()
    } else {
        space_v(ZRO, ZRO).into()
    };
    center(ele).into()
}

fn pane_content<'a>(
    tv: &'a TreeView,
    tv_pane: &TvPane,
    size: Size,
) -> Element<'a, TvMsg> {
    let w = size.width;
    let h = size.height;
    let cnv_w = tv.calc_tre_cnv_w(w);
    let cnv_h = tv.calc_tre_cnv_h(h);
    let scrollable = match tv_pane {
        TvPane::Tree => {
            let cnv = Cnv::new(&tv.tre_cnv).width(cnv_w).height(cnv_h);
            scrollable_cnv_tre(tv.tre_scr_id, cnv, w, h)
        }
        TvPane::LttPlot => {
            let mut cnv_w = cnv_w;
            if tv.tre_cnv.tre_sty == TreSty::Fan {
                cnv_w = w;
            }
            let cnv = Cnv::new(&tv.ltt_cnv).width(cnv_w).height(h);
            scrollable_cnv_ltt(tv.ltt_scr_id, cnv, w, h)
        }
    };
    scrollable.into()
}

fn toolbar<'a>(tv: &'a TreeView, ts: Rc<TreeState>) -> Container<'a, TvMsg> {
    let mut tb_row: Row<TvMsg> = Row::new();

    tb_row = tb_row.push(
        center(
            iced_row![btn_unroot(ts.clone()), btn_root(ts.clone())].spacing(SF),
        )
        .width(Length::Shrink)
        .height(Length::Shrink),
    );

    tb_row = tb_row.push(btn_clade_label(ts.clone()));

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    if tv.tre_states.len() > 1 {
        tb_row = tb_row.push(tree_switcher(tv, ts));
    }

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    tb_row = tb_row.push(
        center(
            iced_row![
                match tv.show_search_bar {
                    true =>
                        btn_svg(Icon::HideSearch, Some(TvMsg::ToggleSearchBar)),
                    false =>
                        btn_svg(Icon::ShowSearch, Some(TvMsg::ToggleSearchBar)),
                },
                match tv.show_ltt {
                    true => btn_svg(
                        Icon::HidePlot,
                        Some(TvMsg::LttVisChanged(false))
                    ),
                    false => btn_svg(
                        Icon::ShowPlot,
                        Some(TvMsg::LttVisChanged(true))
                    ),
                },
                match tv.sidebar_pos {
                    SidebarPosition::Left => btn_svg(
                        Icon::SidebarRight,
                        Some(TvMsg::SetSidebarPos(SidebarPosition::Right))
                    ),
                    SidebarPosition::Right => btn_svg(
                        Icon::SidebarLeft,
                        Some(TvMsg::SetSidebarPos(SidebarPosition::Left))
                    ),
                }
            ]
            .spacing(SF),
        )
        .width(Length::Shrink)
        .height(Length::Shrink),
    );

    tb_row = tb_row.height(Length::Shrink).spacing(PADDING);

    container(tb_row)
        .padding(PADDING)
        .style(sty_cont_tool_bar)
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn tree_switcher<'a>(
    tv: &'a TreeView,
    ts: Rc<TreeState>,
) -> Container<'a, TvMsg> {
    center(
        iced_row![
            btn_prev_tre(tv.prev_tre_exists()),
            center(
                iced_row![
                    txt_usize(ts.id())
                        .align_x(Alignment::Center)
                        .width(Length::Fixed(TXT_SIZE * 3e0)),
                    rule_v(SF),
                    txt_usize(tv.tre_states.len())
                        .align_x(Alignment::Center)
                        .width(Length::Fixed(TXT_SIZE * 3e0))
                ]
                .spacing(PADDING / TWO)
                .padding(PADDING / TWO)
                .align_y(Vertical::Center),
            )
            .width(Length::Shrink)
            .style(sty_cont_no_shadow),
            btn_next_tre(tv.next_tre_exists())
        ]
        .spacing(SF * 3e0)
        .width(Length::Shrink)
        .align_y(Vertical::Center),
    )
    .width(Length::Shrink)
    .height(Length::Fixed(BTN_H1))
}

fn search_bar<'a>(tv: &'a TreeView, ts: Rc<TreeState>) -> Container<'a, TvMsg> {
    let mut main_col: Column<TvMsg> = Column::new();
    let mut row1: Row<TvMsg> = Row::new();
    let mut row2: Row<TvMsg> = Row::new();

    row1 = row1.push(txt_input(
        "Search",
        &tv.search_string,
        tv.search_text_input_id,
        TvMsg::Search,
    ));

    let nxt_prv_btn_row = iced_row![
        btn_svg(Icon::ArrowLeft, {
            match ts.found_edge_idxs().is_empty() {
                true => None,
                false => {
                    if ts.found_edge_idx() > 0 {
                        Some(TvMsg::PrevResult)
                    } else {
                        None
                    }
                }
            }
        })
        .width(BTN_H2)
        .height(BTN_H2),
        btn_svg(Icon::ArrowRight, {
            match ts.found_edge_idxs().is_empty() {
                true => None,
                false => {
                    if ts.found_edge_idx() < ts.found_edge_idxs().len() - 1 {
                        Some(TvMsg::NextResult)
                    } else {
                        None
                    }
                }
            }
        })
        .width(BTN_H2)
        .height(BTN_H2)
    ]
    .spacing(SF)
    .align_y(Alignment::Center);

    let add_rem_btn_row = iced_row![
        btn_svg(Icon::AddToSelection, {
            match ts.found_node_ids().is_empty() {
                true => None,
                false => Some(TvMsg::AddFoundToSelection),
            }
        })
        .width(BTN_H2)
        .height(BTN_H2),
        btn_svg(Icon::RemoveFromSelection, {
            match ts.found_node_ids().is_empty() {
                true => None,
                false => Some(TvMsg::RemFoundFromSelection),
            }
        })
        .width(BTN_H2)
        .height(BTN_H2)
    ]
    .spacing(SF)
    .align_y(Alignment::Center);

    row1 = row1.push(nxt_prv_btn_row);
    row1 = row1.push(add_rem_btn_row);
    row2 = row2.push(checkbox(
        "Tips Only",
        tv.tip_only_search,
        TvMsg::TipOnlySearchSelChanged,
    ));

    row1 =
        row1.spacing(PADDING).height(Length::Shrink).align_y(Vertical::Center);
    row2 =
        row2.spacing(PADDING).height(Length::Shrink).align_y(Vertical::Center);

    main_col = main_col.push(row1);
    main_col = main_col.push(row2);

    main_col = main_col.spacing(PADDING);
    main_col = main_col.padding(PADDING);

    container(main_col)
        .style(sty_cont_search_bar)
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn stats(ts: Rc<TreeState>) -> Row<'static, TvMsg> {
    let mut stats_row: Row<TvMsg> = Row::new();

    let lc: Column<TvMsg> = iced_col![
        txt("Tips"),
        txt("Nodes"),
        txt("Height"),
        txt("Rooted"),
        txt("Branch Lengths"),
        txt("Ultrametric")
    ]
    .width(Length::Fill);

    stats_row = stats_row.push(lc);

    let rc: Column<TvMsg> = iced_col![
        txt_usize(ts.tip_count()),
        txt_usize(ts.node_count()),
        match ts.has_brlen() {
            true => txt_float(ts.tre_height() as Float),
            false => txt_usize(ts.tre_height() as usize),
        },
        txt_bool(ts.is_rooted()),
        txt_bool(ts.has_brlen()),
        txt_bool_option(ts.is_ultrametric()),
    ]
    .align_x(Horizontal::Right);
    stats_row = stats_row.push(rc);
    stats_row
}

fn side_bar<'a>(tv: &'a TreeView, ts: Rc<TreeState>) -> Element<'a, TvMsg> {
    let mut sb_col: Column<TvMsg> = Column::new();

    sb_col = sb_col.spacing(PADDING + SF * TWO);
    sb_col = sb_col.width(Length::Fill);
    sb_col = sb_col.height(Length::Fill);

    sb_col = sb_col.push(stats(ts.clone()));
    sb_col = sb_col.push(rule_h(SF));
    sb_col = sb_col.push(pick_list_tre_sty(tv.tre_cnv.tre_sty));
    sb_col = sb_col.push(pick_list_node_ordering(tv.node_ord_opt));
    sb_col = sb_col.push(rule_h(SF));

    match tv.tre_cnv.tre_sty {
        TreSty::PhyGrm => {
            if tv.tre_cnv_size_idx_min != tv.tre_cnv_size_idx_max {
                sb_col = sb_col.push(slider(
                    Some("Edge Spacing"),
                    tv.tre_cnv_size_idx_min,
                    tv.tre_cnv_size_idx_max,
                    tv.tre_cnv_h_idx,
                    1,
                    2,
                    TvMsg::CnvHeightSelChanged,
                ));
            }
            sb_col = sb_col.push(slider(
                Some("Width"),
                tv.tre_cnv_size_idx_min,
                tv.tre_cnv_size_idx_max,
                tv.tre_cnv_w_idx,
                1,
                2,
                TvMsg::CnvWidthSelChanged,
            ));
        }
        TreSty::Fan => {
            sb_col = sb_col.push(slider(
                Some("Zoom"),
                tv.tre_cnv_size_idx_min,
                tv.tre_cnv_size_idx_max,
                tv.tre_cnv_z_idx,
                1,
                2,
                TvMsg::CnvZoomSelChanged,
            ));
            sb_col = sb_col.push(slider(
                Some("Opening Angle"),
                tv.opn_angle_idx_min,
                tv.opn_angle_idx_max,
                tv.opn_angle_idx,
                1,
                15,
                TvMsg::OpnAngleChanged,
            ));
            sb_col = sb_col.push(slider(
                Some("Rotation Angle"),
                tv.rot_angle_idx_min,
                tv.rot_angle_idx_max,
                tv.rot_angle_idx,
                1,
                15,
                TvMsg::RotAngleChanged,
            ));
        }
    }

    sb_col = sb_col.push(rule_h(SF));

    if ts.is_rooted() && tv.tre_cnv.draw_root {
        sb_col = sb_col.push(iced_col![
            toggler_root(true, tv.tre_cnv.draw_root),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                tv.root_len_idx_min,
                tv.root_len_idx_max,
                tv.root_len_idx,
                1,
                2,
                TvMsg::RootLenSelChanged,
            )
        ]);
    } else {
        sb_col =
            sb_col.push(toggler_root(ts.is_rooted(), tv.tre_cnv.draw_root));
    }

    if ts.has_tip_labs()
        && tv.tre_cnv.draw_labs_tip
        && tv.tre_cnv.draw_labs_allowed
    {
        sb_col = sb_col.push(iced_col![
            toggler_label_tip(true, tv.tre_cnv.draw_labs_tip,),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.lab_size_idx_tip,
                1,
                2,
                TvMsg::TipLabSizeChanged,
            ),
            space_v(ONE, PADDING / TWO),
            toggler_label_tip_align(true, tv.tre_cnv.align_tip_labs),
            space_v(ONE, PADDING / TWO),
            toggler_label_tip_trim(true, tv.tre_cnv.trim_tip_labs),
            match tv.tre_cnv.trim_tip_labs {
                true => {
                    iced_col![
                        space_v(ONE, PADDING / TWO),
                        slider(
                            None,
                            3,
                            75,
                            tv.tre_cnv.trim_tip_labs_to_nchar,
                            1,
                            10,
                            TvMsg::TipLabTrimValChanged,
                        ),
                    ]
                }
                false => {
                    iced_col![]
                }
            },
        ]);
    } else {
        sb_col = sb_col.push(toggler_label_tip(
            ts.has_tip_labs() && tv.tre_cnv.draw_labs_allowed,
            tv.tre_cnv.draw_labs_tip,
        ));
    }

    if ts.has_int_labs()
        && tv.tre_cnv.draw_labs_int
        && tv.tre_cnv.draw_labs_allowed
    {
        sb_col = sb_col.push(iced_col![
            toggler_label_int(true, tv.tre_cnv.draw_labs_int),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.lab_size_idx_int,
                1,
                2,
                TvMsg::IntLabSizeChanged,
            )
        ]);
    } else {
        sb_col = sb_col.push(toggler_label_int(
            ts.has_int_labs() && tv.tre_cnv.draw_labs_allowed,
            tv.tre_cnv.draw_labs_int,
        ));
    }

    if ts.has_brlen()
        && tv.tre_cnv.draw_labs_brnch
        && tv.tre_cnv.draw_labs_allowed
    {
        sb_col = sb_col.push(iced_col![
            toggler_label_branch(true, tv.tre_cnv.draw_labs_brnch),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.lab_size_idx_brnch,
                1,
                2,
                TvMsg::BrnchLabSizeChanged,
            )
        ]);
    } else {
        sb_col = sb_col.push(toggler_label_branch(
            ts.has_brlen() && tv.tre_cnv.draw_labs_allowed,
            tv.tre_cnv.draw_labs_brnch,
        ));
    }

    sb_col = sb_col.push(iced_col![toggler_legend(
        ts.has_brlen(),
        tv.tre_cnv.draw_legend
    )]);
    sb_col = sb_col.push(iced_col![toggler_cursor_line(
        true, tv.tre_cnv.draw_cursor_line, tv.tre_cnv.tre_sty
    )]);
    sb_col = sb_col.push(rule_h(SF));

    sb_col =
        sb_col.push(toggler_selection_lock(true, tv.tre_cnv.selection_lock));

    if tv.show_ltt {
        sb_col =
            sb_col.push(pick_list_ltt_y_axis_scale_type(&tv.ltt_cnv.scale_y));
    }

    container(sb_col.clip(true))
        .style(match tv.sidebar_pos {
            SidebarPosition::Left => sty_cont_bottom_left,
            SidebarPosition::Right => sty_cont_bottom_right,
        })
        .padding(PADDING)
        .width(SIDE_BAR_W)
        .into()
}

pub(crate) fn btn_prev_tre(enabled: bool) -> Button<'static, TvMsg> {
    btn_svg(
        Icon::ArrowLeft,
        match enabled {
            true => Some(TvMsg::PrevTre),
            false => None,
        },
    )
    .width(BTN_H2)
    .height(BTN_H2)
}

pub(crate) fn btn_next_tre<'a>(enabled: bool) -> Button<'a, TvMsg> {
    btn_svg(
        Icon::ArrowRight,
        match enabled {
            true => Some(TvMsg::NextTre),
            false => None,
        },
    )
    .width(BTN_H2)
    .height(BTN_H2)
}

pub(crate) fn btn_clade_label<'a>(sel_tre: Rc<TreeState>) -> Button<'a, TvMsg> {
    let (lab, msg) = match sel_tre.sel_node_ids().len() == 1 {
        true => {
            let &node_id = sel_tre.sel_node_ids().iter().last().unwrap();
            match sel_tre.clade_has_label(node_id) {
                false => (
                    "Label",
                    Some(TvMsg::AddCladeLabel((node_id, Clr::BLU_25))),
                ),
                true => ("Unlabel", Some(TvMsg::RemoveCladeLabel(node_id))),
            }
        }
        false => ("Label", None),
    };
    btn_txt(lab, msg).width(BTN_H1 * TWO)
}

pub(crate) fn btn_root<'a>(sel_tre: Rc<TreeState>) -> Button<'a, TvMsg> {
    btn_txt("Root", {
        if sel_tre.sel_node_ids().len() == 1 {
            let &node_id = sel_tre.sel_node_ids().iter().last().unwrap();
            match sel_tre.can_root(node_id) {
                true => Some(TvMsg::Root(node_id)),
                false => None,
            }
        } else {
            None
        }
    })
    .width(BTN_H1 * TWO)
}

pub(crate) fn btn_unroot<'a>(sel_tre: Rc<TreeState>) -> Button<'a, TvMsg> {
    btn_txt(
        "Unroot",
        match sel_tre.is_rooted() {
            true => Some(TvMsg::Unroot),
            false => None,
        },
    )
    .width(BTN_H1 * TWO)
}

pub(crate) fn pick_list_ltt_y_axis_scale_type<'a>(
    axis_scale_type: &AxisScaleType,
) -> Row<'a, TvMsg> {
    let mut pl: PickList<
        AxisScaleType,
        &[AxisScaleType],
        AxisScaleType,
        TvMsg,
    > = PickList::new(
        &AXIS_SCALE_TYPE_OPTS,
        Some(axis_scale_type.clone()),
        TvMsg::LttYAxisScaleTypeChanged,
    );
    pl = pick_list_common(pl);
    iced_row![txt("Y-Axis Scale").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}

pub(crate) fn pick_list_node_ordering<'a>(node_ord: NodeOrd) -> Row<'a, TvMsg> {
    let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TvMsg> =
        PickList::new(&NODE_ORD_OPTS, Some(node_ord), TvMsg::NodeOrdOptChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Node Order").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}

pub(crate) fn pick_list_tre_sty<'a>(tre_sty: TreSty) -> Row<'a, TvMsg> {
    let mut pl: PickList<TreSty, &[TreSty], TreSty, TvMsg> =
        PickList::new(&TRE_STY_OPTS, Some(tre_sty), TvMsg::TreStyOptChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Style").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}

pub(crate) fn scrollable_cnv_ltt<'a>(
    id: &'static str,
    cnv: Cnv<&'a PlotCnv, TvMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Horizontal(scroll_bar()));
    s = s.id(id);
    s = s.on_scroll(TvMsg::LttCnvScrolledOrResized);
    scrollable_common(s, w, h)
}

pub(crate) fn scrollable_cnv_tre<'a>(
    id: &'static str,
    cnv: Cnv<&'a TreeCnv, TvMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Both {
        horizontal: scroll_bar(),
        vertical: scroll_bar(),
    });
    s = s.id(id);
    s = s.on_scroll(TvMsg::TreCnvScrolledOrResized);
    scrollable_common(s, w, h)
}

pub(crate) fn toggler_cursor_line<'a>(
    enabled: bool,
    draw_cursor_line: bool,
    tre_sty: TreSty,
) -> Toggler<'a, TvMsg> {
    let lab = match tre_sty {
        TreSty::PhyGrm => "Cursor Tracking Line",
        TreSty::Fan => "Cursor Tracking Circle",
    };
    let mut tglr = toggler(lab, draw_cursor_line);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::CursorLineVisChanged);
    }
    tglr
}

pub(crate) fn toggler_label_branch<'a>(
    enabled: bool,
    draw_brnch_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Branch Lengths", draw_brnch_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::BrnchLabVisChanged);
    }
    tglr
}

pub(crate) fn toggler_label_int<'a>(
    enabled: bool,
    draw_int_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Internal Labels", draw_int_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::IntLabVisChanged);
    }
    tglr
}

pub(crate) fn toggler_label_tip<'a>(
    enabled: bool,
    draw_tip_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Tip Labels", draw_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabVisChanged);
    }
    tglr
}

pub(crate) fn toggler_label_tip_align<'a>(
    enabled: bool,
    align_tip_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Align Tip Labels", align_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabAlignOptChanged);
    }
    tglr
}

pub(crate) fn toggler_label_tip_trim<'a>(
    enabled: bool,
    trim_tip_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Trim Tip Labels", trim_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabTrimOptChanged);
    }
    tglr
}

pub(crate) fn toggler_legend<'a>(
    enabled: bool,
    draw_legend: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Legend", draw_legend);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::LegendVisChanged);
    }
    tglr
}

pub(crate) fn toggler_root<'a>(
    enabled: bool,
    draw_root: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Root", draw_root);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::RootVisChanged);
    }
    tglr
}

pub(crate) fn toggler_selection_lock<'a>(
    enabled: bool,
    selection_lock: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Selection Lock", selection_lock);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::SelectionLockChanged);
    }
    tglr
}
