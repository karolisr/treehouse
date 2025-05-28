use crate::elements::*;
use crate::iced::*;
use crate::style::*;
use crate::*;

impl TreeView {
    pub fn view(&self) -> Element<TvMsg> {
        let ts: &TreeState;

        if let Some(sel_ts_opt) = self.sel_tre() {
            ts = sel_ts_opt;
        } else {
            return center(txt("No trees loaded")).into();
        }

        let mut main_col: Column<TvMsg> = Column::new();
        let mut main_row: Row<TvMsg> = Row::new();

        main_col = main_col.padding(0);
        main_col = main_col.spacing(0);
        main_row = main_row.padding(Padding { top: 0e0, right: 5e0, bottom: 5e0, left: 5e0 });
        main_row = main_row.spacing(5);

        if self.show_toolbar {
            main_col = main_col.push(toolbar(self, ts));
        }

        if self.show_sidebar {
            match self.sidebar_pos_sel {
                SidebarPos::Left => {
                    main_row = main_row.push(sidebar(self, ts));
                    main_row = main_row.push(content(self, ts));
                }
                SidebarPos::Right => {
                    main_row = main_row.push(content(self, ts));
                    main_row = main_row.push(sidebar(self, ts));
                }
            }
        } else {
            main_row = main_row.push(content(self, ts));
        }

        main_col = main_col.push(main_row);

        main_col.into()
    }
}

fn content<'a>(tv: &'a TreeView, ts: &'a TreeState) -> Element<'a, TvMsg> {
    let ele: Element<'a, TvMsg> = if let Some(pane_grid) = &tv.pane_grid {
        PaneGrid::new(pane_grid, |_pane_idx, tv_pane, _is_maximized| {
            PgContent::new(center(responsive(move |size| pane_content(tv, ts, tv_pane, size))).padding(10))
                .style(sty_pane_body)
        })
        .style(sty_pane_grid)
        .on_resize(1e1, TvMsg::PaneResized)
        .min_size(tv.tre_padd * 3e0 + 5e1)
        .spacing(5)
        .into()
    } else {
        space_v(0, 0).into()
    };
    center(ele).into()
}

fn pane_content<'a>(tv: &'a TreeView, _ts: &'a TreeState, tv_pane: &TvPane, size: Size) -> Element<'a, TvMsg> {
    let w = size.width;
    let h = size.height;
    let cnv_w = tv.calc_tre_cnv_w(w);
    let cnv_h = tv.calc_tre_cnv_h(h);
    let scrollable = match tv_pane {
        TvPane::Tree => {
            let cnv = Cnv::new(tv).width(cnv_w).height(cnv_h);
            scrollable_cnv_tre(tv.tre_scr_id, cnv, w, h)
        }
        TvPane::LttPlot => {
            let mut cnv_w = cnv_w;
            if tv.tre_style_opt_sel == TreSty::Fan {
                cnv_w = w;
            }
            let cnv = Cnv::new(&tv.ltt_plot).width(cnv_w).height(h);
            scrollable_cnv_ltt(tv.ltt_scr_id, cnv, w, h)
        }
    };
    scrollable.into()
}

fn toolbar<'a>(tv: &'a TreeView, ts: &'a TreeState) -> Element<'a, TvMsg> {
    let mut tb_row: Row<TvMsg> = Row::new();

    tb_row = tb_row.push(
        center(iced_row![btn_unroot(ts), btn_root(ts)].align_y(Vertical::Center).spacing(5))
            .width(Length::Shrink)
            .height(Length::Shrink)
            .padding(5),
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
            .spacing(5),
        )
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(5),
    );

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    tb_row = tb_row.push(
        center(
            iced_row![
                match tv.show_ltt {
                    true => btn("Hide LTT", Some(TvMsg::LttpVisChanged(false))),
                    false => btn("Show LTT", Some(TvMsg::LttpVisChanged(true))),
                },
                match tv.sidebar_pos_sel {
                    SidebarPos::Left => btn("SBR", Some(TvMsg::SetSidebarPos(SidebarPos::Right))),
                    SidebarPos::Right => btn("SBL", Some(TvMsg::SetSidebarPos(SidebarPos::Left))),
                }
            ]
            .align_y(Vertical::Center)
            .spacing(5),
        )
        .width(Length::Fixed(15e1))
        .height(Length::Shrink)
        .padding(5),
    );

    tb_row = tb_row.align_y(Vertical::Center);
    tb_row = tb_row.spacing(10);
    tb_row = tb_row.padding(5);

    container(tb_row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

fn stats(ts: &TreeState) -> Row<TvMsg> {
    let mut stats_row: Row<TvMsg> = Row::new();

    let lc: Column<TvMsg> =
        iced_col![txt("Tips"), txt("Nodes"), txt("Height"), txt("Rooted"), txt("Branch Lengths"), txt("Ultrametric")]
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

fn sidebar<'a>(tv: &'a TreeView, ts: &'a TreeState) -> Element<'a, TvMsg> {
    let mut sb_col: Column<TvMsg> = Column::new();

    sb_col = sb_col.spacing(10);
    sb_col = sb_col.width(Length::Fill);
    sb_col = sb_col.height(Length::Fill);

    sb_col = sb_col.push(stats(ts));
    sb_col = sb_col.push(rule_h(1));
    sb_col = sb_col.push(pick_list_tre_sty(tv.tre_style_opt_sel));
    sb_col = sb_col.push(pick_list_node_ordering(tv.node_ord_opt_sel));
    sb_col = sb_col.push(rule_h(1));

    match tv.tre_style_opt_sel {
        TreSty::PhyGrm => {
            if tv.tre_cnv_h_idx_min != tv.tre_cnv_h_idx_max {
                sb_col = sb_col.push(slider(
                    Some("Edge Spacing"),
                    tv.tre_cnv_h_idx_min,
                    tv.tre_cnv_h_idx_max,
                    tv.tre_cnv_h_idx_sel,
                    1,
                    2,
                    TvMsg::CnvHeightSelChanged,
                ));
            }
            sb_col = sb_col.push(slider(
                Some("Width"),
                tv.tre_cnv_w_idx_min,
                tv.tre_cnv_w_idx_max,
                tv.tre_cnv_w_idx_sel,
                1,
                2,
                TvMsg::CnvWidthSelChanged,
            ));
        }
        TreSty::Fan => {
            sb_col = sb_col.push(slider(
                Some("Zoom"),
                tv.tre_cnv_z_idx_min,
                tv.tre_cnv_z_idx_max,
                tv.tre_cnv_z_idx_sel,
                1,
                2,
                TvMsg::CnvZoomSelChanged,
            ));
            sb_col = sb_col.push(slider(
                Some("Opening Angle"),
                tv.opn_angle_idx_min,
                tv.opn_angle_idx_max,
                tv.opn_angle_idx_sel,
                1,
                15,
                TvMsg::OpnAngleChanged,
            ));
            sb_col = sb_col.push(slider(
                Some("Rotation Angle"),
                tv.rot_angle_idx_min,
                tv.rot_angle_idx_max,
                tv.rot_angle_idx_sel,
                1,
                15,
                TvMsg::RotAngleChanged,
            ));
        }
    }

    sb_col = sb_col.push(rule_h(1));

    if ts.has_tip_labs() && tv.draw_labs_tip && tv.draw_labs_allowed {
        sb_col = sb_col.push(iced_col![
            toggler_label_tip(true, tv.draw_labs_tip,),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.tip_lab_size_idx_sel,
                1,
                2,
                TvMsg::TipLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_tip(ts.has_tip_labs() && tv.draw_labs_allowed, tv.draw_labs_tip))
    }

    if ts.has_int_labs() && tv.draw_labs_int && tv.draw_labs_allowed {
        sb_col = sb_col.push(iced_col![
            toggler_label_int(true, tv.draw_labs_int),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.int_lab_size_idx_sel,
                1,
                2,
                TvMsg::IntLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_int(ts.has_int_labs() && tv.draw_labs_allowed, tv.draw_labs_int))
    }

    if ts.has_brlen() && tv.draw_labs_brnch && tv.draw_labs_allowed {
        sb_col = sb_col.push(iced_col![
            toggler_label_branch(true, tv.draw_labs_brnch),
            slider(
                None,
                tv.lab_size_idx_min,
                tv.lab_size_idx_max,
                tv.brnch_lab_size_idx_sel,
                1,
                2,
                TvMsg::BrnchLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_branch(ts.has_brlen() && tv.draw_labs_allowed, tv.draw_labs_brnch))
    }

    sb_col = sb_col.push(rule_h(1));

    if ts.is_rooted() {
        sb_col = sb_col.push(slider(
            Some("Root Length"),
            tv.root_len_idx_min,
            tv.root_len_idx_max,
            tv.root_len_idx_sel,
            1,
            2,
            TvMsg::RootLenSelChanged,
        ));

        sb_col = sb_col.push(rule_h(1));
    }

    container(container(sb_col).clip(true)).padding(10).width(220).into()
}
