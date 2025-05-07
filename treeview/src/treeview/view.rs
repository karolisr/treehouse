use super::elements::{
    btn, btn_root, btn_unroot, pick_list_node_ordering, pick_list_tree_style, scrollable_cnv_ltt,
    scrollable_cnv_tree, scrollable_v, slider, space_v, toggler_cursor_line, toggler_label_branch,
    toggler_label_int, toggler_label_tip, toggler_legend, toggler_ltt, txt, txt_bool,
    txt_bool_option, txt_float, txt_usize,
};
use super::styles::{sty_pane_body, sty_pane_grid, sty_pane_titlebar};
use super::{SidebarLocation, TreeState};
use super::{
    TreeViewPane,
    styles::{sty_cont_main, sty_cont_sidebar, sty_cont_statusbar, sty_cont_toolbar},
};
use crate::TreeStyle;
use crate::{TreeView, TreeViewMsg};
use iced::{
    Element,
    widget::{Column, Row},
};
use iced::{
    Length::{self, Fill, Shrink},
    alignment::{Horizontal, Vertical},
    widget::{
        Canvas, column, container,
        pane_grid::{Content, PaneGrid, State as PaneGridState, TitleBar},
        responsive,
    },
};

impl TreeView {
    pub fn view(&self) -> Element<TreeViewMsg> {
        let ts: &TreeState;
        let pgs: &PaneGridState<TreeViewPane>;

        if let Some(idx) = self.sel_tree_idx {
            ts = &self.trees[idx];
        } else {
            return space_v(Length::Fill, Length::Fill).into();
        }

        if let Some(pane_grid_state) = &self.panes {
            pgs = pane_grid_state
        } else {
            return space_v(Length::Fill, Length::Fill).into();
        }

        let mut mc = Column::new();
        let mut mr = Row::new();

        mc = mc.padding(5e0);
        mc = mc.spacing(5e0);
        mr = mr.spacing(5e0);

        if self.show_sidebar {
            match self.sidebar_position {
                SidebarLocation::Left => {
                    mr = mr.push(self.sidebar(ts));
                    mr = mr.push(self.content(ts, pgs));
                }
                SidebarLocation::Right => {
                    mr = mr.push(self.content(ts, pgs));
                    mr = mr.push(self.sidebar(ts));
                }
            }
        } else {
            mr = mr.push(self.content(ts, pgs));
        }

        if self.show_toolbar {
            mc = mc.push(self.toolbar(ts));
        }

        mc = mc.push(mr);

        if self.show_statusbar {
            mc = mc.push(self.statusbar(ts));
        }

        mc.into()
    }

    pub(crate) fn content<'a>(
        &'a self,
        _ts: &TreeState,
        pgs: &'a PaneGridState<TreeViewPane>,
    ) -> Element<'a, TreeViewMsg> {
        container(
            PaneGrid::new(pgs, |_pane_idx, pane, _is_maximized| {
                Content::new(responsive(move |size| {
                    let w = size.width;
                    let h = size.height;
                    match pane {
                        TreeViewPane::Tree => scrollable_cnv_tree(
                            Canvas::new(&self.tre_cnv)
                                .width(w.max(w * self.sel_tre_cnv_w_idx as f32))
                                .height(h.max(h * self.sel_node_size_idx as f32)),
                            w,
                            h,
                        )
                        .into(),
                        TreeViewPane::LttPlot => {
                            let width = match self.sel_tree_style_opt {
                                TreeStyle::Phylogram => w.max(w * self.sel_tre_cnv_w_idx as f32),
                                TreeStyle::Fan => w,
                            };
                            scrollable_cnv_ltt(
                                Canvas::new(&self.ltt_cnv).width(width).height(h),
                                w,
                                h,
                            )
                            .into()
                        }
                    }
                }))
                .style(sty_pane_body)
                // .title_bar(
                //     TitleBar::new(container(space_v(0, 30)))
                //         .style(sty_pane_titlebar)
                //         .always_show_controls(),
                // )
            })
            .width(Fill)
            .height(Fill)
            .min_size(1e2)
            .spacing(6e0)
            .style(sty_pane_grid)
            // .on_drag(TreeViewMsg::PaneDragged)
            .on_resize(1e1, TreeViewMsg::PaneResized),
        )
        // .style(sty_cont_main)
        .into()
    }

    pub(crate) fn sidebar<'a>(&'a self, ts: &'a TreeState) -> Element<'a, TreeViewMsg> {
        container(
            container(responsive(|size| {
                let mut sc: Column<TreeViewMsg> = Column::new();

                sc = sc.push(self.stats(ts));
                sc = sc.push(pick_list_tree_style(self.sel_tree_style_opt));
                sc = sc.push(pick_list_node_ordering(self.sel_node_ord_opt));

                match self.sel_tree_style_opt {
                    TreeStyle::Phylogram => {
                        sc = sc.push(slider(
                            Some("Tree Width"),
                            self.min_tre_cnv_w_idx,
                            self.max_tre_cnv_w_idx,
                            self.sel_tre_cnv_w_idx,
                            TreeViewMsg::CanvasWidthSelectionChanged,
                        ));
                        if self.min_node_size_idx != self.max_node_size_idx {
                            sc = sc.push(slider(
                                Some("Edge Spacing"),
                                self.min_node_size_idx,
                                self.max_node_size_idx,
                                self.sel_node_size_idx,
                                TreeViewMsg::NodeSizeSelectionChanged,
                            ));
                        }
                    }
                    TreeStyle::Fan => {
                        sc = sc.push(slider(
                            Some("Zoom"),
                            self.min_tre_cnv_w_idx,
                            self.max_tre_cnv_w_idx,
                            self.sel_tre_cnv_w_idx,
                            TreeViewMsg::CanvasWidthSelectionChanged,
                        ));
                        sc = sc.push(slider(
                            Some("Opening Angle"),
                            self.min_opn_angle_idx,
                            self.max_opn_angle_idx,
                            self.sel_opn_angle_idx,
                            TreeViewMsg::OpnAngleSelectionChanged,
                        ));
                        sc = sc.push(slider(
                            Some("Rotation Angle"),
                            self.min_rot_angle_idx,
                            self.max_rot_angle_idx,
                            self.sel_rot_angle_idx,
                            TreeViewMsg::RotAngleSelectionChanged,
                        ));
                    }
                }

                if self.tip_brnch_labs_allowed && ts.has_tip_labs && self.draw_tip_labs {
                    sc = sc.push(
                        column![
                            toggler_label_tip(
                                self.tip_brnch_labs_allowed && ts.has_tip_labs,
                                self.draw_tip_labs,
                            ),
                            slider(
                                None,
                                self.min_lab_size_idx,
                                self.max_lab_size_idx,
                                self.sel_tip_lab_size_idx,
                                TreeViewMsg::TipLabelSizeSelectionChanged,
                            )
                        ]
                        .spacing(3e0),
                    )
                } else {
                    sc = sc.push(toggler_label_tip(
                        self.tip_brnch_labs_allowed && ts.has_tip_labs,
                        self.draw_tip_labs,
                    ))
                }

                if ts.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
                    sc = sc.push(
                        column![
                            toggler_label_branch(
                                ts.has_brlen && self.tip_brnch_labs_allowed,
                                self.draw_brnch_labs
                            ),
                            slider(
                                None,
                                self.min_lab_size_idx,
                                self.max_lab_size_idx,
                                self.sel_brnch_lab_size_idx,
                                TreeViewMsg::BranchLabelSizeSelectionChanged,
                            )
                        ]
                        .spacing(3e0),
                    )
                } else {
                    sc = sc.push(toggler_label_branch(
                        ts.has_brlen && self.tip_brnch_labs_allowed,
                        self.draw_brnch_labs,
                    ))
                }

                if ts.has_int_labs && self.draw_int_labs {
                    sc = sc.push(
                        column![
                            toggler_label_int(ts.has_int_labs, self.draw_int_labs),
                            slider(
                                None,
                                self.min_lab_size_idx,
                                self.max_lab_size_idx,
                                self.sel_int_lab_size_idx,
                                TreeViewMsg::IntLabelSizeSelectionChanged,
                            )
                        ]
                        .spacing(3e0),
                    )
                } else {
                    sc = sc.push(toggler_label_int(ts.has_int_labs, self.draw_int_labs))
                }

                sc = sc.push(toggler_legend(ts.has_brlen, self.draw_legend));
                sc = sc.push(toggler_ltt(true, self.show_ltt));
                sc = sc.push(toggler_cursor_line(
                    true, self.show_cursor_line, self.sel_tree_style_opt,
                ));

                sc = sc.spacing(8e0);
                sc = sc.width(Length::Fill);

                scrollable_v(sc, Length::Fill, size.height).spacing(4e0).into()
            }))
            .clip(true),
        )
        .padding(4e0)
        .width(200)
        .style(sty_cont_sidebar)
        .into()
    }

    pub(crate) fn toolbar<'a>(&self, ts: &'a TreeState) -> Element<'a, TreeViewMsg> {
        let mut tbr: Row<TreeViewMsg> = Row::new();

        tbr = tbr.spacing(2e0);
        tbr = tbr.padding(6e0);

        tbr = tbr.push(btn_unroot(ts));
        tbr = tbr.push(btn_root(ts));

        let btn_sb_pos = match self.sidebar_position {
            SidebarLocation::Left => {
                btn("SBR", Some(TreeViewMsg::SetSidebarLocation(SidebarLocation::Right)))
            }
            SidebarLocation::Right => {
                btn("SBL", Some(TreeViewMsg::SetSidebarLocation(SidebarLocation::Left)))
            }
        };
        tbr = tbr.push(btn_sb_pos);

        container(tbr)
            .width(Fill)
            .height(Shrink)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Center)
            .style(sty_cont_toolbar)
            .into()
    }

    pub(crate) fn statusbar(&self, _ts: &TreeState) -> Element<TreeViewMsg> {
        container(space_v(0, 30)).width(Fill).height(30).style(sty_cont_statusbar).into()
    }

    pub(crate) fn stats(&self, ts: &TreeState) -> Row<TreeViewMsg> {
        let mut sr: Row<TreeViewMsg> = Row::new();

        let lc: Column<TreeViewMsg> = column![
            txt("Tips"),
            txt("Nodes"),
            txt("Height"),
            txt("Rooted"),
            txt("Branch Lengths"),
            txt("Ultrametric")
        ]
        .width(Fill);

        sr = sr.push(lc);

        let rc: Column<TreeViewMsg> = column![
            txt_usize(ts.tip_count),
            txt_usize(ts.node_count),
            match ts.has_brlen {
                true => txt_float(ts.tree_height),
                false => txt_usize(ts.tree_height as usize),
            },
            txt_bool(ts.is_rooted),
            txt_bool(ts.has_brlen),
            txt_bool_option(ts.is_ultrametric),
        ]
        .align_x(Horizontal::Right);
        sr = sr.push(rc);

        sr
    }
}
