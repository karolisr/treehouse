use super::styles::{
    sty_btn, sty_cont_main, sty_cont_sidebar, sty_cont_statusbar, sty_cont_toolbar, sty_pick_lst,
    sty_rule, sty_scrlbl, sty_slider,
};
use crate::{
    NODE_ORD_OPTS, NodeOrd, PlotCnv, TREE_STYLE_OPTS, TreeCnv, TreeState, TreeStateMsg, TreeStyle,
    TreeView, TreeViewMsg,
};
use iced::{
    Alignment, Element, Font,
    Length::{self, Fill, Fixed, Shrink},
    Pixels, Theme,
    alignment::{Horizontal, Vertical},
    widget::{
        Button, Canvas, Column, PickList, Row, Rule, Scrollable, Slider, Space, Text, column,
        container, horizontal_space,
        pick_list::Handle as PickListHandle,
        responsive, row,
        scrollable::{Direction as ScrollableDirection, Scrollbar},
        text, vertical_space,
    },
};
use numfmt::Formatter as NumFmt;
use std::{clone::Clone, cmp::PartialEq, convert::From, fmt::Display};
use widget::{Roundness as TogglerRoundness, Toggler};

use super::styles::{sty_pane_body, sty_pane_grid, sty_pane_titlebar};
use iced::{
    Size,
    widget::{
        center,
        pane_grid::{self as pg, Content, TitleBar},
    },
};

const SF: f32 = 1e0;
const TEXT_SIZE: f32 = 13.0 * SF;
const RADIUS_WIDGET: f32 = 3e0 * SF;

pub(crate) fn scrollbar() -> Scrollbar {
    Scrollbar::new()
}

pub(crate) fn scrollable_common(scrl: Scrollable<TreeViewMsg>) -> Scrollable<TreeViewMsg> {
    let mut scrl = scrl;
    scrl = scrl.style(sty_scrlbl);
    scrl
}

pub(crate) fn scrollable_cnv_ltt(cnv: Canvas<&PlotCnv, TreeViewMsg>) -> Scrollable<TreeViewMsg> {
    let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
    scrl = scrl.direction(ScrollableDirection::Horizontal(scrollbar()));
    // scrl = scrl.width(self.tree_scroll_w);
    scrl = scrl.height(1e2);
    scrl = scrl.on_scroll(TreeViewMsg::LttCnvScrolled);
    scrl = scrl.id("ltt");
    scrollable_common(scrl)
}

pub(crate) fn scrollable_cnv_tree(cnv: Canvas<&TreeCnv, TreeViewMsg>) -> Scrollable<TreeViewMsg> {
    let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
    scrl = scrl
        .direction(ScrollableDirection::Both { horizontal: scrollbar(), vertical: scrollbar() });
    // scrl = scrl.width(self.tree_scroll_w);
    // scrl = scrl.height(self.tree_scroll_h);
    scrl = scrl.on_scroll(TreeViewMsg::TreCnvScrolled);
    scrl = scrl.id("tre");
    scrollable_common(scrl)
}

#[derive(Debug)]
pub(crate) enum Pane {
    Tree { cnv_tree: &'static TreeCnv },
    LttPlot { cnv_lttp: &'static PlotCnv },
    Empty,
}

impl Pane {
    pub(crate) fn content(
        &self,
        pane_idx: pg::Pane,
        pane_count: usize,
        size: Size,
        is_maximized: bool,
    ) -> Element<TreeViewMsg> {
        let w = size.width;
        let h = size.height;

        match self {
            Pane::Tree { cnv_tree } => scrollable_cnv_tree(Canvas::new(cnv_tree))
                .width(Length::Fixed(w))
                .height(Length::Fixed(h))
                .into(),

            Pane::LttPlot { cnv_lttp } => scrollable_cnv_ltt(Canvas::new(cnv_lttp))
                .width(Length::Fixed(w))
                .height(Length::Fixed(h))
                .into(),

            Pane::Empty => {
                let mut content =
                    text!("{w} x {h} | {self:?} | {pane_idx:?} | {pane_count} | {is_maximized}");
                content = content.align_x(Alignment::Center);
                content = content.align_y(Alignment::Center);
                center(content).into()
            }
        }
    }
}

impl TreeView {
    pub(crate) fn content(&self, _ts: &TreeState) -> Element<TreeViewMsg> {
        // responsive(move |size| {
        //     container(self.pane_grid_main.view())
        //         .width(Fixed(size.width))
        //         .height(Fixed(size.height))
        //         .align_x(Horizontal::Center)
        //         .align_y(Vertical::Center)
        //         .style(sty_cont_main)
        //         .into()
        // })
        // .into()

        let pane_grid = pg::PaneGrid::new(&self.pane_grid_state, |pane_idx, pane, is_maximized| {
            Content::new(responsive(move |size| {
                pane.content(pane_idx, self.pane_grid_state.len(), size, is_maximized)
            }))
            .style(sty_pane_body)
            .title_bar(
                TitleBar::new(container(iced::widget::vertical_space().height(30)))
                    .style(sty_pane_titlebar)
                    .always_show_controls(),
            )
        })
        .width(Fill)
        .height(Fill)
        .min_size(1e2)
        .style(sty_pane_grid)
        .on_drag(TreeViewMsg::PaneDragged)
        .on_resize(1e1, TreeViewMsg::PaneResized);
        container(pane_grid).into()
    }

    pub(crate) fn sidebar(&self, ts: &TreeState) -> Element<TreeViewMsg> {
        let mut sc: Column<TreeViewMsg> = Column::new();

        sc = sc.push(self.stats(ts));
        sc = sc.push(self.pick_list_tree_style());
        sc = sc.push(self.pick_list_node_ordering());

        match self.sel_tree_style_opt {
            TreeStyle::Phylogram => {
                sc = sc.push(self.slider_width_canvas());
                if self.min_node_size_idx != self.max_node_size_idx {
                    sc = sc.push(self.slider_size_node());
                }
            }
            TreeStyle::Fan => {
                sc = sc.push(self.slider_width_canvas());
                sc = sc.push(self.slider_angle_opn());
                sc = sc.push(self.slider_angle_rot());
            }
        }

        sc = sc.push(self.toggler_label_tip(self.tip_brnch_labs_allowed && ts.has_tip_labs));
        if self.tip_brnch_labs_allowed && ts.has_tip_labs && self.draw_tip_labs {
            sc = sc.push(self.slider_size_label_tip());
        }

        sc = sc.push(self.toggler_label_branch(ts.has_brlen && self.tip_brnch_labs_allowed));
        if ts.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            sc = sc.push(self.slider_size_label_branch());
        }

        sc = sc.push(self.toggler_label_int(ts.has_int_labs));
        if ts.has_int_labs && self.draw_int_labs {
            sc = sc.push(self.slider_size_label_int());
        }

        sc = sc.push(self.toggler_legend(ts.has_brlen));
        sc = sc.push(self.toggler_ltt(true));
        sc = sc.push(self.toggler_cursor_line(true));

        sc = sc.padding(6e0);
        sc = sc.spacing(6e0);

        container(sc).width(260).style(sty_cont_sidebar).into()
    }

    pub(crate) fn toolbar(&self, ts: &TreeState) -> Element<TreeViewMsg> {
        let mut tbr: Row<TreeViewMsg> = Row::new();

        tbr = tbr.spacing(2e0);
        tbr = tbr.padding(6e0);

        tbr = tbr.push(self.btn_unroot(ts));
        tbr = tbr.push(self.btn_root(ts));

        // let btn_sb_pos: Button<'_, TreeViewMsg> = match self.sidebar_position {
        //     SidebarLocation::Left => {
        //         self.btn("SBR", Some(TreeViewMsg::SetSidebarLocation(SidebarLocation::Right)))
        //     }
        //     SidebarLocation::Right => {
        //         self.btn("SBL", Some(TreeViewMsg::SetSidebarLocation(SidebarLocation::Left)))
        //     }
        // };
        // tbr = tbr.push(btn_sb_pos);

        container(tbr)
            .width(Fill)
            .height(Shrink)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Center)
            .style(sty_cont_toolbar)
            .into()
    }

    pub(crate) fn statusbar(&self, _ts: &TreeState) -> Element<TreeViewMsg> {
        container(iced::widget::vertical_space())
            .width(Fill)
            .height(30)
            .style(sty_cont_statusbar)
            .into()
    }

    pub(crate) fn stats(&self, ts: &TreeState) -> Row<TreeViewMsg> {
        let mut sr: Row<TreeViewMsg> = Row::new();

        let lc: Column<TreeViewMsg> = column![
            self.txt("Tips"),
            self.txt("Nodes"),
            self.txt("Height"),
            self.txt("Rooted"),
            self.txt("Branch Lengths"),
            self.txt("Ultrametric")
        ]
        .width(Fill);

        sr = sr.push(lc);

        let rc: Column<TreeViewMsg> = column![
            self.txt_usize(ts.tip_count),
            self.txt_usize(ts.node_count),
            match ts.has_brlen {
                true => self.txt_float(ts.tree_height),
                false => self.txt_usize(ts.tree_height as usize),
            },
            self.txt_bool(ts.is_rooted),
            self.txt_bool(ts.has_brlen),
            self.txt_bool_option(ts.is_ultrametric),
        ]
        .align_x(Horizontal::Right);
        sr = sr.push(rc);

        sr
    }

    pub(crate) fn btn<'a>(
        &'a self,
        lab: &'a str,
        msg: Option<TreeViewMsg>,
    ) -> Button<'a, TreeViewMsg> {
        let mut txt = Text::new(lab);
        txt = txt.align_x(Horizontal::Center);
        txt = txt.align_y(Vertical::Center);
        let mut btn = Button::new(txt);
        btn = btn.on_press_maybe(msg);
        btn = btn.style(sty_btn);
        btn
    }

    pub(crate) fn btn_root(&self, ts: &TreeState) -> Button<TreeViewMsg> {
        self.btn("Root", {
            if ts.sel_node_ids.len() == 1 {
                let node_id = *ts.sel_node_ids.iter().last().unwrap();
                match ts.can_root(node_id) {
                    true => Some(TreeViewMsg::TreeStateMsg(TreeStateMsg::Root(node_id))),
                    false => None,
                }
            } else {
                None
            }
        })
    }

    pub(crate) fn btn_unroot(&self, ts: &TreeState) -> Button<TreeViewMsg> {
        self.btn(
            "Unroot",
            match ts.is_rooted {
                true => Some(TreeViewMsg::TreeStateMsg(TreeStateMsg::Unroot)),
                false => None,
            },
        )
    }

    // --------------------------------------------------------------------------------------------

    // pub(crate) fn canvas_ltt(&self) -> Canvas<&PlotCnv, TreeViewMsg> {
    //     Canvas::new(&self.ltt_cnv).width(Fixed(self.ltt_cnv_w)).height(Fixed(1e2))
    // }

    // pub(crate) fn canvas_tree(&self) -> Canvas<&TreeCnv, TreeViewMsg> {
    //     Canvas::new(&self.tre_cnv).width(Fixed(self.tre_cnv_w)).height(Fixed(self.tre_cnv_h))
    // }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn pick_list_common<'a, T: PartialEq + Display + Clone>(
        &'a self,
        pl: PickList<'a, T, &[T], T, TreeViewMsg>,
    ) -> PickList<'a, T, &'a [T], T, TreeViewMsg> {
        let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TEXT_SIZE)) };
        let mut pl = pl;
        pl = pl.handle(h);
        pl = pl.style(sty_pick_lst);
        pl
    }

    pub(crate) fn pick_list_node_ordering(&self) -> Row<TreeViewMsg> {
        let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TreeViewMsg> = PickList::new(
            &NODE_ORD_OPTS,
            Some(self.sel_node_ord_opt),
            TreeViewMsg::NodeOrdOptChanged,
        );
        pl = self.pick_list_common(pl);
        row![text!("Node Order").width(Fill), pl].align_y(Vertical::Center)
    }

    pub(crate) fn pick_list_tree_style(&self) -> Row<TreeViewMsg> {
        let mut pl: PickList<TreeStyle, &[TreeStyle], TreeStyle, TreeViewMsg> = PickList::new(
            &TREE_STYLE_OPTS,
            Some(self.sel_tree_style_opt),
            TreeViewMsg::TreeStyleOptionChanged,
        );
        pl = self.pick_list_common(pl);
        row![text!("Style").width(Fill), pl].align_y(Vertical::Center)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn rule_common<'a>(&'a self, rule: Rule<'a, Theme>) -> Rule<'a, Theme> {
        let rule = rule.style(sty_rule);
        rule
    }

    pub(crate) fn rule_h(&self, height: impl Into<Pixels>) -> Rule<'_, Theme> {
        let mut rule: Rule<'_, Theme> = Rule::horizontal(height);
        rule = self.rule_common(rule);
        rule
    }

    pub(crate) fn rule_v(&self, width: impl Into<Pixels>) -> Rule<'_, Theme> {
        let mut rule: Rule<'_, Theme> = Rule::vertical(width);
        rule = self.rule_common(rule);
        rule
    }

    // --------------------------------------------------------------------------------------------

    // pub(crate) fn scrollable_common<'a>(
    //     &'a self,
    //     scrl: Scrollable<'a, TreeViewMsg>,
    // ) -> Scrollable<'a, TreeViewMsg> {
    //     let mut scrl = scrl;
    //     scrl = scrl.style(sty_scrlbl);
    //     scrl
    // }

    // pub(crate) fn scrollbar(&self) -> Scrollbar {
    //     Scrollbar::new()
    // }

    // pub(crate) fn scrollable_cnv_ltt<'a>(
    //     &'a self,
    //     cnv: Canvas<&'a PlotCnv, TreeViewMsg>,
    // ) -> Scrollable<'a, TreeViewMsg> {
    //     let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
    //     scrl = scrl.direction(ScrollableDirection::Horizontal(self.scrollbar()));
    //     scrl = scrl.width(self.tree_scroll_w);
    //     scrl = scrl.height(1e2);
    //     scrl = scrl.on_scroll(TreeViewMsg::LttCnvScrolled);
    //     scrl = scrl.id("ltt");
    //     self.scrollable_common(scrl)
    // }

    // pub(crate) fn scrollable_cnv_tree<'a>(
    //     &'a self,
    //     cnv: Canvas<&'a TreeCnv, TreeViewMsg>,
    // ) -> Scrollable<'a, TreeViewMsg> {
    //     let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);

    //     scrl = scrl.direction(ScrollableDirection::Both {
    //         horizontal: self.scrollbar(),
    //         vertical: self.scrollbar(),
    //     });

    //     scrl = scrl.width(self.tree_scroll_w);
    //     scrl = scrl.height(self.tree_scroll_h);
    //     scrl = scrl.on_scroll(TreeViewMsg::TreCnvScrolled);
    //     scrl = scrl.id("tre");

    //     self.scrollable_common(scrl)
    // }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn slider_common<'a>(
        &'a self,
        lab: &str,
        slider: Slider<'a, u16, TreeViewMsg>,
    ) -> Row<'a, TreeViewMsg> {
        let mut slider = slider;
        slider = slider.height(TEXT_SIZE * 1.4);
        slider = slider.style(sty_slider);
        row![
            slider,
            container(text!("{lab}"))
                .align_x(Horizontal::Right)
                .align_y(Vertical::Center)
                .width(Fill),
        ]
        .align_y(Vertical::Center)
    }

    pub(crate) fn slider_angle_opn(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_opn_angle_idx..=self.max_opn_angle_idx,
            self.sel_opn_angle_idx,
            TreeViewMsg::OpnAngleSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider_common("Opening Angle", sldr)
    }

    pub(crate) fn slider_angle_rot(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_rot_angle_idx..=self.max_rot_angle_idx,
            self.sel_rot_angle_idx,
            TreeViewMsg::RotAngleSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider_common("Rotation Angle", sldr)
    }

    pub(crate) fn slider_size_label_branch(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_brnch_lab_size_idx,
            TreeViewMsg::BranchLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider_common("Branch Label Size", sldr)
    }

    pub(crate) fn slider_size_label_int(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_int_lab_size_idx,
            TreeViewMsg::IntLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider_common("Internal Label Size", sldr)
    }

    pub(crate) fn slider_size_label_tip(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_tip_lab_size_idx,
            TreeViewMsg::TipLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider_common("Tip Label Size", sldr)
    }

    pub(crate) fn slider_size_node(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_node_size_idx..=self.max_node_size_idx,
            self.sel_node_size_idx,
            TreeViewMsg::NodeSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider_common("Edge Spacing", sldr)
    }

    pub(crate) fn slider_width_canvas(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_tre_cnv_w_idx..=self.max_tre_cnv_w_idx,
            self.sel_tre_cnv_w_idx,
            TreeViewMsg::CanvasWidthSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider_common("Zoom", sldr)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn space_h(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        horizontal_space().width(width).height(height)
    }

    pub(crate) fn space_v(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        vertical_space().width(width).height(height)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn toggler<'a>(&self, label: &'a str, value: bool) -> Toggler<'a, TreeViewMsg> {
        let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
        tglr = tglr.label(label);
        tglr = tglr.text_size(TEXT_SIZE);
        tglr = tglr.size(TEXT_SIZE * 1.5);
        tglr = tglr.text_alignment(Alignment::End);
        tglr = tglr.width(Fill);
        tglr = tglr.roundness(TogglerRoundness::Radius(RADIUS_WIDGET));
        tglr
    }

    pub(crate) fn toggler_cursor_line(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let lab = match self.sel_tree_style_opt {
            TreeStyle::Phylogram => "Cursor Tracking Line",
            TreeStyle::Fan => "Cursor Tracking Circle",
        };

        let mut tglr = self.toggler(lab, self.show_cursor_line);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::CursorLineVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_label_branch(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Branch Lengths", enabled && self.draw_brnch_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::BranchLabelVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_label_int(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Internal Labels", enabled && self.draw_int_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::IntLabelVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_label_tip(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Tip Labels", enabled && self.draw_tip_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::TipLabelVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_legend(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Legend", enabled && self.draw_legend);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::LegendVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_ltt(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("LTT Plot", self.show_ltt);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::LttPlotVisibilityChanged);
        }
        tglr
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn txt(&self, s: impl Into<String>) -> Text {
        Text::new(s.into()).align_x(Horizontal::Right).align_y(Vertical::Center).width(Shrink)
    }

    pub(crate) fn txt_bool(&self, b: bool) -> Text {
        let s = match b {
            true => "Yes",
            false => "No",
        };
        self.txt(s)
    }

    pub(crate) fn txt_bool_option(&self, ob: Option<bool>) -> Text {
        match ob {
            Some(b) => self.txt_bool(b),
            None => self.txt("N/A"),
        }
    }

    pub(crate) fn txt_float(&self, n: impl Into<f32>) -> Text {
        let mut num_fmt = NumFmt::new();
        num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
        num_fmt = num_fmt.separator(',').unwrap();
        let s = num_fmt.fmt2(n.into());
        self.txt(s)
    }

    pub(crate) fn txt_usize(&self, n: impl Into<usize>) -> Text {
        let mut num_fmt = NumFmt::new();
        num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
        num_fmt = num_fmt.separator(',').unwrap();
        let s = num_fmt.fmt2(n.into());
        self.txt(s)
    }
}
