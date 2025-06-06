use crate::elements::*;
use crate::iced::*;
use crate::style::*;
use crate::*;

impl TreeView {
    pub fn view(&self) -> Element<TvMsg> {
        let ts: Rc<TreeState>;

        if let Some(sel_ts_opt) = self.sel_tre() {
            ts = sel_ts_opt;
        } else {
            return center(txt("No trees loaded")).into();
        }

        let mut main_col: Column<TvMsg> = Column::new();
        let mut main_row: Row<TvMsg> = Row::new();
        let mut toolbar_content_col: Column<TvMsg> = Column::new();

        main_col = main_col.padding(ZRO);
        main_col = main_col.spacing(ZRO);
        main_row = main_row.padding(Padding { top: ZRO, right: ZRO, bottom: ZRO, left: ZRO });
        main_row = main_row.spacing(ZRO);

        if self.show_toolbar {
            toolbar_content_col = toolbar_content_col.push(toolbar(self, ts.clone()));
        }
        toolbar_content_col = toolbar_content_col.push(find_bar(self, ts.clone()));
        toolbar_content_col = toolbar_content_col.push(content(self));

        if self.show_sidebar {
            let sb = sidebar(self, ts);
            match self.sidebar_pos {
                SidebarPosition::Left => {
                    main_row = main_row.push(sb);
                    main_row = main_row.push(toolbar_content_col);
                }
                SidebarPosition::Right => {
                    main_row = main_row.push(toolbar_content_col);
                    main_row = main_row.push(sb);
                }
            }
        } else {
            main_row = main_row.push(toolbar_content_col);
        }

        main_col = main_col.push(main_row);
        main_col.into()
    }
}

fn find_bar<'a>(tv: &'a TreeView, ts: Rc<TreeState>) -> Container<'a, TvMsg> {
    let mut ttc: Column<TvMsg> = Column::new();
    let mut ttr1: Row<TvMsg> = Row::new();
    let mut ttr2: Row<TvMsg> = Row::new();

    ttr1 = ttr1.push(TextInput::new("Search", &tv.search_string).on_input(TvMsg::Search));

    let add_rem_btn_row = iced_row![
        btn("+", {
            match ts.found_node_ids().is_empty() {
                true => None,
                false => Some(TvMsg::AddFoundToSelection),
            }
        }),
        btn("-", {
            match ts.found_node_ids().is_empty() {
                true => None,
                false => Some(TvMsg::RemFoundFromSelection),
            }
        })
    ];

    let nxt_prv_btn_row = iced_row![
        btn("<", {
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
        }),
        btn(">", {
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
    ];

    ttr1 = ttr1.push(add_rem_btn_row);
    ttr1 = ttr1.push(nxt_prv_btn_row);
    ttr2 = ttr2
        .push(checkbox("Tips Only", tv.tip_only_search).on_toggle(TvMsg::TipOnlySearchSelChanged));

    ttr1 = ttr1.spacing(ZRO).align_y(Vertical::Center);
    ttr2 = ttr2.spacing(ZRO);

    ttc = ttc.push(ttr1);
    ttc = ttc.push(ttr2);

    ttc = ttc.spacing(ZRO);

    container(ttc)
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn content<'a>(tv: &'a TreeView) -> Element<'a, TvMsg> {
    let ele: Element<'a, TvMsg> = if let Some(pane_grid) = &tv.pane_grid {
        PaneGrid::new(pane_grid, |_pane_idx, tv_pane, _is_maximized| {
            PgContent::new(center(responsive(move |size| pane_content(tv, tv_pane, size))))
                .style(sty_pane_body)
        })
        .style(sty_pane_grid)
        .on_resize(ZRO, TvMsg::PaneResized)
        .min_size(150)
        .spacing(TEN)
        .into()
    } else {
        space_v(ZRO, ZRO).into()
    };
    center(ele).into()
}

fn pane_content<'a>(tv: &'a TreeView, tv_pane: &TvPane, size: Size) -> Element<'a, TvMsg> {
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
            iced_row![btn_unroot(ts.clone()), btn_root(ts.clone())]
                .align_y(Vertical::Center)
                .spacing(0),
        )
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(ZRO),
    );

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    let i = format!("{:>4}", ts.id());
    let s = "/";
    let n = format!("{:<4}", tv.tre_states.len());
    tb_row = tb_row.push(
        center(
            iced_row![
                btn_prev_tre(tv.prev_tre_exists()),
                txt(i).align_x(Alignment::Center).width(Length::Fixed(3e1)),
                txt(s).align_x(Alignment::Center).width(Length::Fixed(1e1)),
                txt(n).align_x(Alignment::Center).width(Length::Fixed(3e1)),
                btn_next_tre(tv.next_tre_exists())
            ]
            .align_y(Vertical::Center)
            .spacing(ZRO),
        )
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(ZRO),
    );

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    tb_row = tb_row.push(
        center(
            iced_row![
                match tv.show_ltt {
                    true => btn("LTTH", Some(TvMsg::LttVisChanged(false))),
                    false => btn("LTTV", Some(TvMsg::LttVisChanged(true))),
                },
                match tv.sidebar_pos {
                    SidebarPosition::Left =>
                        btn("SBR", Some(TvMsg::SetSidebarPos(SidebarPosition::Right))),
                    SidebarPosition::Right =>
                        btn("SBL", Some(TvMsg::SetSidebarPos(SidebarPosition::Left))),
                }
            ]
            .align_y(Vertical::Center)
            .spacing(ZRO),
        )
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(ZRO),
    );

    tb_row = tb_row.align_y(Vertical::Center);
    tb_row = tb_row.spacing(ZRO);
    tb_row = tb_row.padding(ZRO);

    container(tb_row)
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

fn sidebar<'a>(tv: &'a TreeView, ts: Rc<TreeState>) -> Element<'a, TvMsg> {
    let mut sb_col: Column<TvMsg> = Column::new();

    sb_col = sb_col.spacing(ZRO);
    sb_col = sb_col.width(Length::Fill);
    sb_col = sb_col.height(Length::Fill);

    sb_col = sb_col.push(stats(ts.clone()));
    sb_col = sb_col.push(rule_h(ONE));
    sb_col = sb_col.push(pick_list_tre_sty(tv.tre_cnv.tre_sty));
    sb_col = sb_col.push(pick_list_node_ordering(tv.node_ord_opt));
    sb_col = sb_col.push(rule_h(ONE));

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

    sb_col = sb_col.push(rule_h(ONE));

    if ts.has_tip_labs() && tv.tre_cnv.draw_labs_tip && tv.tre_cnv.draw_labs_allowed {
        sb_col = sb_col.push(iced_col![
            toggler_label_tip(true, tv.tre_cnv.draw_labs_tip,),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.lab_size_idx_tip,
                1,
                2,
                TvMsg::TipLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_tip(
            ts.has_tip_labs() && tv.tre_cnv.draw_labs_allowed,
            tv.tre_cnv.draw_labs_tip,
        ))
    }

    if ts.has_int_labs() && tv.tre_cnv.draw_labs_int && tv.tre_cnv.draw_labs_allowed {
        sb_col = sb_col.push(iced_col![
            toggler_label_int(true, tv.tre_cnv.draw_labs_int),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.lab_size_idx_int,
                1,
                2,
                TvMsg::IntLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_int(
            ts.has_int_labs() && tv.tre_cnv.draw_labs_allowed,
            tv.tre_cnv.draw_labs_int,
        ))
    }

    if ts.has_brlen() && tv.tre_cnv.draw_labs_brnch && tv.tre_cnv.draw_labs_allowed {
        sb_col = sb_col.push(iced_col![
            toggler_label_branch(true, tv.tre_cnv.draw_labs_brnch),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.lab_size_idx_brnch,
                1,
                2,
                TvMsg::BrnchLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_branch(
            ts.has_brlen() && tv.tre_cnv.draw_labs_allowed,
            tv.tre_cnv.draw_labs_brnch,
        ))
    }

    sb_col = sb_col.push(iced_col![toggler_legend(ts.has_brlen(), tv.tre_cnv.draw_legend)]);
    sb_col = sb_col.push(iced_col![toggler_cursor_line(
        true, tv.tre_cnv.draw_cursor_line, tv.tre_cnv.tre_sty
    )]);
    sb_col = sb_col.push(rule_h(ONE));

    if ts.is_rooted() {
        sb_col = sb_col.push(slider(
            Some("Root Length"),
            tv.root_len_idx_min,
            tv.root_len_idx_max,
            tv.root_len_idx,
            1,
            2,
            TvMsg::RootLenSelChanged,
        ));
        sb_col = sb_col.push(rule_h(ONE));
    }

    container(container(sb_col).clip(true)).padding(ZRO).width(SIDEBAR_W).into()
}
