mod buttons;
mod pick_lists;
mod scrollables;
mod tables;
mod togglers;

use crate::*;

use buttons::*;
use pick_lists::*;
use scrollables::*;
use tables::*;
use togglers::*;

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
            top: 0e0,
            right: PADDING,
            bottom: PADDING,
            left: PADDING,
        });

        main_row = main_row.spacing(PADDING);

        tool_bar_and_content_col = tool_bar_and_content_col.spacing(PADDING);

        if self.cfg.show_tool_bar {
            tool_bar_and_content_col =
                tool_bar_and_content_col.push(toolbar(self, ts.clone()));
        }

        if self.cfg.show_search_bar {
            tool_bar_and_content_col =
                tool_bar_and_content_col.push(search_bar(self, ts.clone()));
        }

        tool_bar_and_content_col = tool_bar_and_content_col.push(content(self));

        if self.cfg.show_side_bar {
            let side_bar_main = side_bar_main(self, ts.clone());
            let side_bar_annotations = side_bar_annotations(self, ts.clone());
            main_row = main_row.push(side_bar_main);
            main_row = main_row.push(tool_bar_and_content_col);
            main_row = main_row.push(side_bar_annotations);
        } else {
            main_row = main_row.push(tool_bar_and_content_col);
        }

        main_row.into()
    }
}

fn content<'a>(tv: &'a TreeView) -> Element<'a, TvMsg> {
    let ele: Element<'a, TvMsg> = if let Some(pane_grid) = &tv.pane_grid {
        PaneGrid::new(pane_grid, |_pane_idx, tv_pane, _is_maximized| {
            PgContent::new(
                {
                    center(responsive({
                        move |size| pane_content(tv, tv_pane, size)
                    }))
                }
                .padding(PADDING),
            )
            .style(match tv_pane {
                TreeViewPane::Tree => sty_pane_body,
                TreeViewPane::Plot => sty_pane_body_plot,
                TreeViewPane::NodesTable => sty_pane_body,
            })
        })
        .style(sty_pane_grid)
        .on_resize(ZRO, TvMsg::PaneResized)
        .min_size(TXT_SIZE * 14.0)
        .spacing(PADDING)
        .into()
    } else {
        space_v(ZRO, ZRO).into()
    };
    center(ele).into()
}

fn pane_content<'a>(
    tv: &'a TreeView,
    tv_pane: &TreeViewPane,
    size: Size,
) -> Element<'a, TvMsg> {
    let w = size.width;
    let h = size.height;
    let cnv_w = tv.calc_tre_cnv_w(w);
    let cnv_h = tv.calc_tre_cnv_h(h);
    let content: Element<'a, TvMsg> = match tv_pane {
        TreeViewPane::Tree => {
            let cnv = Cnv::new(&tv.tre_cnv).width(cnv_w).height(cnv_h);
            scrollable_cnv_tre(tv.tre_scrollable_id, cnv, w, h).into()
        }
        TreeViewPane::Plot => {
            let mut cnv_w = cnv_w;
            if tv.cfg.tre_sty == TreSty::Fan {
                cnv_w = w;
            }
            let cnv =
                Cnv::new(&tv.plot_cnv).width(cnv_w - SIDE_BAR_W).height(h);
            let scrl = scrollable_cnv_plot(
                tv.plot_scrollable_id,
                cnv,
                w - SIDE_BAR_W,
                h,
            );
            let mut pane_row: Row<TvMsg> = Row::new();
            let mut psc: Column<TvMsg> = Column::new();

            psc = psc.push(toggler_gts(
                tv.cfg.tre_unit == TreUnit::MillionYears,
                tv.plot_cnv.cfg.draw_gts,
            ));

            psc = psc.push(toggler_ltt(
                tv.cfg.tre_unit != TreUnit::Unitless,
                tv.plot_cnv.cfg.draw_ltt,
            ));

            if tv.plot_cnv.cfg.draw_ltt {
                psc = psc.push(pick_list_plot_y_axis_scale_type(
                    tv.plot_cnv.y_axis_scale_type,
                ));
            }

            psc = psc.spacing(PADDING + SF * TWO).padding(PADDING);

            pane_row = pane_row.push(scrl);
            pane_row = pane_row.push(psc.clip(true));

            pane_row.into()
        }
        // TreeViewPane::NodesTable => table_nodes(tv, w, h),
        // TreeViewPane::NodesTable => table_attributes(tv, w, h),
        TreeViewPane::NodesTable => table_node_data(tv, w, h),
    };
    content
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

    tb_row = tb_row.push(
        center(
            iced_row![
                btn_subtree_parent_node(ts.clone()),
                btn_clear_subtree_view(ts.clone())
            ]
            .spacing(SF),
        )
        .width(Length::Shrink)
        .height(Length::Shrink),
    );

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    if tv.tre_states.len() > 1 {
        tb_row = tb_row.push(tree_switcher(tv, ts.clone()));
    }

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    tb_row = tb_row.push(
        center(
            iced_row![
                btn_svg_stateful(
                    Icon::Search,
                    Icon::Search,
                    Some(TvMsg::ToggleSearchBar),
                    tv.cfg.show_search_bar,
                ),
                btn_svg_stateful(
                    Icon::Plot,
                    Icon::Plot,
                    match ts.has_brlen() && tv.cfg.tre_unit != TreUnit::Unitless
                    {
                        true => Some(TvMsg::TogglePlot(!tv.cfg.show_plot)),
                        false => None,
                    },
                    tv.cfg.show_plot && tv.cfg.tre_unit != TreUnit::Unitless,
                ),
                btn_svg_stateful(
                    Icon::DataTable,
                    Icon::DataTable,
                    Some(TvMsg::ToggleNodesTable),
                    tv.cfg.show_nodes_table,
                ),
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
        tv.cfg.tip_only_search,
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

    let rc: Column<TvMsg> =
        iced_col![
            iced_row![
                match ts.is_subtree_view_active() {
                    true => {
                        iced_row![
                            txt_usize(ts.tip_count_for_subtree_view().unwrap()),
                            txt("/")
                        ]
                    }
                    false => {
                        iced_row![]
                    }
                },
                txt_usize(ts.tip_count_tree())
            ],
            iced_row![
                match ts.is_subtree_view_active() {
                    true => {
                        iced_row![
                            txt_usize(
                                ts.node_count_for_subtree_view().unwrap()
                            ),
                            txt("/")
                        ]
                    }
                    false => {
                        iced_row![]
                    }
                },
                txt_usize(ts.node_count_tree())
            ],
            iced_row![
                match ts.is_subtree_view_active() {
                    true => {
                        iced_row![
                    match ts.has_brlen() {
                        true => txt_float(
                            ts.max_first_node_to_tip_distance_for_subtree_view()
                                .unwrap() as Float, 2
                        ),
                        false => txt_usize(
                            ts.max_first_node_to_tip_distance_for_subtree_view()
                                .unwrap() as usize
                        ),
                    },
                    txt("/")
                ]
                    }
                    false => {
                        iced_row![]
                    }
                },
                match ts.has_brlen() {
                    true => txt_float(
                        ts.max_first_node_to_tip_distance_tree() as Float,
                        2
                    ),
                    false => txt_usize(
                        ts.max_first_node_to_tip_distance_tree() as usize
                    ),
                }
            ],
            txt_bool(ts.is_rooted_tree()),
            txt_bool(ts.has_brlen()),
            txt_bool_option(ts.is_ultrametric()),
        ]
        .align_x(Horizontal::Right);
    stats_row = stats_row.push(rc);
    stats_row
}

fn side_bar_main<'a>(
    tv: &'a TreeView,
    ts: Rc<TreeState>,
) -> Element<'a, TvMsg> {
    let mut sb: Column<TvMsg> = Column::new();

    sb = sb.spacing(PADDING + SF * TWO);
    sb = sb.width(Length::Fill);
    sb = sb.height(Length::Fill);

    sb = sb.push(stats(ts.clone()));
    sb = sb.push(rule_h(SF));
    if ts.has_brlen() {
        sb = sb.push(pick_list_tree_unit(tv.cfg.tre_unit));
        sb = sb.push(rule_h(SF));
    }
    sb = sb.push(pick_list_tre_sty(tv.cfg.tre_sty));
    sb = sb.push(pick_list_node_ordering(tv.cfg.node_ord_opt));
    sb = sb.push(rule_h(SF));

    match tv.cfg.tre_sty {
        TreSty::PhyGrm => {
            if TRE_CNV_SIZE_IDX_MIN != TRE_CNV_SIZE_IDX_MAX {
                sb = sb.push(slider(
                    Some("Edge Spacing"),
                    TRE_CNV_SIZE_IDX_MIN,
                    TRE_CNV_SIZE_IDX_MAX,
                    tv.tre_cnv_h_idx,
                    1,
                    2,
                    TvMsg::CnvHeightSelChanged,
                ));
            }
            sb = sb.push(slider(
                Some("Width"),
                TRE_CNV_SIZE_IDX_MIN,
                TRE_CNV_SIZE_IDX_MAX,
                tv.tre_cnv_w_idx,
                1,
                2,
                TvMsg::CnvWidthSelChanged,
            ));
        }
        TreSty::Fan => {
            sb = sb.push(slider(
                Some("Zoom"),
                TRE_CNV_SIZE_IDX_MIN,
                TRE_CNV_SIZE_IDX_MAX,
                tv.tre_cnv_z_idx,
                1,
                2,
                TvMsg::CnvZoomSelChanged,
            ));
            sb = sb.push(slider(
                Some("Opening Angle"),
                OPN_ANGLE_IDX_MIN,
                OPN_ANGLE_IDX_MAX,
                tv.cfg
                    .opn_angle_idx
                    .clamp(OPN_ANGLE_IDX_MIN, OPN_ANGLE_IDX_MAX),
                1,
                15,
                TvMsg::OpnAngleChanged,
            ));
            sb = sb.push(slider(
                Some("Rotation Angle"),
                ROT_ANGLE_IDX_MIN,
                ROT_ANGLE_IDX_MAX,
                tv.cfg
                    .rot_angle_idx
                    .clamp(ROT_ANGLE_IDX_MIN, ROT_ANGLE_IDX_MAX),
                1,
                15,
                TvMsg::RotAngleChanged,
            ));
        }
    }

    sb = sb.push(rule_h(SF));

    if ts.is_rooted() && tv.cfg.draw_root && !ts.is_subtree_view_active() {
        sb = sb.push(iced_col![
            toggler_root(true, tv.cfg.draw_root),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                ROOT_LEN_IDX_MIN,
                ROOT_LEN_IDX_MAX,
                tv.cfg.root_len_idx.clamp(ROOT_LEN_IDX_MIN, ROOT_LEN_IDX_MAX),
                1,
                2,
                TvMsg::RootLenSelChanged,
            )
        ]);
    } else {
        sb = sb.push(toggler_root(
            ts.is_rooted() && !ts.is_subtree_view_active(),
            tv.cfg.draw_root,
        ));
    }

    sb = sb.push(toggler_selection_lock(true, tv.cfg.selection_lock));

    sb = sb.push(rule_h(SF));

    sb = sb.push(iced_col![toggler_cursor_line(
        true, tv.cfg.draw_cursor_line, tv.cfg.tre_sty
    )]);

    sb = sb.push(iced_col![toggler_scale_bar(
        ts.has_brlen(),
        tv.cfg.show_scale_bar
    )]);

    if tv.cfg.show_scale_bar {
        sb = sb.push(iced_col![toggler_full_width_scale_bar(
            ts.has_brlen(),
            tv.cfg.full_width_scale_bar
        )]);
    }

    container(sb.clip(true))
        .style(sty_cont_bottom_left)
        .padding(PADDING)
        .width(SIDE_BAR_W)
        .into()
}

fn side_bar_annotations<'a>(
    tv: &'a TreeView,
    ts: Rc<TreeState>,
) -> Element<'a, TvMsg> {
    let mut sb: Column<TvMsg> = Column::new();

    sb = sb.spacing(PADDING + SF * TWO);
    sb = sb.width(Length::Fill);
    sb = sb.height(Length::Fill);

    if ts.has_tip_labels()
        && tv.cfg.draw_labs_tip
        && tv.tre_cnv.draw_labs_allowed
    {
        sb = sb.push(iced_col![
            toggler_label_tip(true, tv.cfg.draw_labs_tip,),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                LAB_SIZE_IDX_MIN,
                LAB_SIZE_IDX_MAX,
                tv.cfg
                    .lab_size_idx_tip
                    .clamp(LAB_SIZE_IDX_MIN, LAB_SIZE_IDX_MAX),
                1,
                2,
                TvMsg::TipLabSizeChanged,
            ),
            space_v(ONE, PADDING / TWO),
            toggler_label_tip_align(true, tv.cfg.align_tip_labs),
            space_v(ONE, PADDING / TWO),
            toggler_label_tip_trim(true, tv.cfg.trim_tip_labs),
            match tv.cfg.trim_tip_labs {
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
        sb = sb.push(toggler_label_tip(
            ts.has_tip_labels() && tv.tre_cnv.draw_labs_allowed,
            tv.cfg.draw_labs_tip,
        ));
    }

    if ts.has_int_labels()
        && tv.cfg.draw_labs_int
        && tv.tre_cnv.draw_labs_allowed
    {
        sb = sb.push(iced_col![
            toggler_label_int(true, tv.cfg.draw_labs_int),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                LAB_SIZE_IDX_MIN,
                LAB_SIZE_IDX_MAX,
                tv.cfg
                    .lab_size_idx_int
                    .clamp(LAB_SIZE_IDX_MIN, LAB_SIZE_IDX_MAX),
                1,
                2,
                TvMsg::IntLabSizeChanged,
            )
        ]);
    } else {
        sb = sb.push(toggler_label_int(
            ts.has_int_labels() && tv.tre_cnv.draw_labs_allowed,
            tv.cfg.draw_labs_int,
        ));
    }

    if ts.has_brlen() && tv.cfg.draw_labs_brnch && tv.tre_cnv.draw_labs_allowed
    {
        sb = sb.push(iced_col![
            toggler_label_branch(true, tv.cfg.draw_labs_brnch),
            space_v(ONE, PADDING / TWO),
            slider(
                None,
                LAB_SIZE_IDX_MIN,
                LAB_SIZE_IDX_MAX,
                tv.cfg
                    .lab_size_idx_brnch
                    .clamp(LAB_SIZE_IDX_MIN, LAB_SIZE_IDX_MAX),
                1,
                2,
                TvMsg::BrnchLabSizeChanged,
            )
        ]);
    } else {
        sb = sb.push(toggler_label_branch(
            ts.has_brlen() && tv.tre_cnv.draw_labs_allowed,
            tv.cfg.draw_labs_brnch,
        ));
    }

    container(sb.clip(true))
        .style(sty_cont_bottom_right)
        .padding(PADDING)
        .width(SIDE_BAR_W)
        .into()
}
