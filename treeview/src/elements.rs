use crate::SidebarPos;
use crate::TreeView;
use crate::TvMsg;
use crate::cnv_plot::PlotCnv;
use crate::treestate::TreeState;
use crate::treeview::NODE_ORD_OPTS;
use crate::treeview::NodeOrd;
use crate::treeview::TREE_STYLE_OPTS;
use crate::treeview::TreeStyle;
use crate::treeview::TvPane;

use num_traits::FromPrimitive;
use std::fmt::Display;

pub use iced::widget::Column;
pub use iced::widget::Row;
pub use iced::widget::center;

use iced::Alignment;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Pixels;
use iced::Size;
use iced::Theme;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::widget::Button;
use iced::widget::Canvas;
use iced::widget::PaneGrid;
use iced::widget::PickList;
use iced::widget::Rule;
use iced::widget::Scrollable;
use iced::widget::Slider;
use iced::widget::Space;
use iced::widget::Text;
use iced::widget::column;
use iced::widget::container;
use iced::widget::container::bordered_box;
use iced::widget::horizontal_space;
use iced::widget::pane_grid::Content as PaneGridContent;
use iced::widget::responsive;
use iced::widget::row;
use iced::widget::scrollable::Direction as ScrollableDirection;
use iced::widget::scrollable::Scrollbar;
use iced::widget::vertical_space;
use widget::toggler::Toggler;

pub(super) fn content<'a>(tv: &'a TreeView, _sel_tree: &'a TreeState) -> Element<'a, TvMsg> {
    let ele: Element<'a, TvMsg> = if let Some(pane_grid) = &tv.pane_grid {
        PaneGrid::new(pane_grid, |_pane_idx, tv_pane, _is_maximized| {
            PaneGridContent::new(
                center(responsive(move |size| tv_pane_content(tv, tv_pane, size)))
                    .padding(10)
                    .style(bordered_box),
            )
        })
        .on_resize(1e1, TvMsg::PaneResized)
        .min_size(150)
        .spacing(5)
        .into()
    } else {
        space_v(0, 0).into()
    };
    center(ele).into()
}

fn tv_pane_content<'a>(tv: &'a TreeView, tv_pane: &TvPane, size: Size) -> Element<'a, TvMsg> {
    let w = size.width;
    let h = size.height;
    let scrollable = match tv_pane {
        TvPane::Tree => {
            let cnv = Canvas::new(tv).width(w + tv.tre_cnv_w).height(h + tv.tre_cnv_h);
            scrollable_cnv_tree(cnv, w, h)
        }
        TvPane::LttPlot => {
            let cnv = Canvas::new(&tv.ltt_plot).width(w).height(h);
            scrollable_cnv_ltt(cnv, w, h)
        }
    };
    scrollable.into()
}

pub(super) fn toolbar<'a>(tv: &'a TreeView, sel_tree: &'a TreeState) -> Element<'a, TvMsg> {
    let mut tb_row: Row<TvMsg> = Row::new();

    tb_row = tb_row.push(
        center(row![btn_unroot(sel_tree), btn_root(sel_tree)].align_y(Vertical::Center).spacing(5))
            .width(Length::Shrink)
            .height(Length::Shrink)
            .padding(5)
            .style(bordered_box),
    );

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    let i = format!("{:>4}", sel_tree.id());
    let s = "/";
    let n = format!("{:<4}", tv.tree_states.len());
    tb_row = tb_row.push(
        center(
            row![
                btn_prev_tree(tv.prev_tree_exists()),
                txt(i).align_x(Alignment::Center).width(Length::Fixed(3e1)),
                txt(s).align_x(Alignment::Center).width(Length::Fixed(1e1)),
                txt(n).align_x(Alignment::Center).width(Length::Fixed(3e1)),
                btn_next_tree(tv.next_tree_exists())
            ]
            .align_y(Vertical::Center)
            .spacing(5),
        )
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(5)
        .style(bordered_box),
    );

    tb_row = tb_row.push(space_h(Length::Fill, Length::Shrink));

    tb_row = tb_row.push(
        center(
            row![
                match tv.show_lttp {
                    true => btn("Hide LTTP", Some(TvMsg::LttpVisChanged(false))),
                    false => btn("Show LTTP", Some(TvMsg::LttpVisChanged(true))),
                },
                match tv.sel_sidebar_pos {
                    SidebarPos::Left => btn("SBR", Some(TvMsg::SetSidebarPos(SidebarPos::Right))),
                    SidebarPos::Right => btn("SBL", Some(TvMsg::SetSidebarPos(SidebarPos::Left))),
                }
            ]
            .align_y(Vertical::Center)
            .spacing(5),
        )
        .width(Length::Fixed(15e1))
        .height(Length::Shrink)
        .padding(5)
        .style(bordered_box),
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

fn stats(sel_tree: &TreeState) -> Row<TvMsg> {
    let mut stats_row: Row<TvMsg> = Row::new();

    let lc: Column<TvMsg> = iced::widget::column![
        txt("Tips"),
        txt("Nodes"),
        txt("Height"),
        txt("Rooted"),
        txt("Branch Lengths"),
        txt("Ultrametric")
    ]
    .width(Length::Fill);

    stats_row = stats_row.push(lc);

    let rc: Column<TvMsg> = iced::widget::column![
        txt_usize(sel_tree.tip_count()),
        txt_usize(sel_tree.node_count()),
        match sel_tree.has_brlen() {
            true => txt_float(sel_tree.tree_height() as f32),
            false => txt_usize(sel_tree.tree_height() as usize),
        },
        txt_bool(sel_tree.is_rooted()),
        txt_bool(sel_tree.has_brlen()),
        txt_bool_option(sel_tree.is_ultrametric()),
    ]
    .align_x(Horizontal::Right);
    stats_row = stats_row.push(rc);

    stats_row
}

pub(super) fn sidebar<'a>(tv: &'a TreeView, sel_tree: &'a TreeState) -> Element<'a, TvMsg> {
    let mut sb_col: Column<TvMsg> = Column::new();

    sb_col = sb_col.spacing(10);
    sb_col = sb_col.width(Length::Fill);
    sb_col = sb_col.height(Length::Fill);

    sb_col = sb_col.push(stats(sel_tree));
    sb_col = sb_col.push(rule_h(1));
    sb_col = sb_col.push(pick_list_tree_style(tv.sel_tree_style_opt));
    sb_col = sb_col.push(pick_list_node_ordering(tv.sel_node_ord_opt));
    sb_col = sb_col.push(rule_h(1));

    match tv.sel_tree_style_opt {
        TreeStyle::Phylogram => {
            sb_col = sb_col.push(slider(
                Some("Tree Width"),
                tv.min_tre_cnv_w_idx,
                tv.max_tre_cnv_w_idx,
                tv.sel_tre_cnv_w_idx,
                TvMsg::CnvWidthChanged,
            ));
            if tv.min_tre_cnv_h_idx != tv.max_tre_cnv_h_idx {
                sb_col = sb_col.push(slider(
                    Some("Edge Spacing"),
                    tv.min_tre_cnv_h_idx,
                    tv.max_tre_cnv_h_idx,
                    tv.sel_tre_cnv_h_idx,
                    TvMsg::CnvHeightChanged,
                ));
            }
        }
        TreeStyle::Fan => {
            sb_col = sb_col.push(slider(
                Some("Zoom"),
                tv.min_tre_cnv_z_idx,
                tv.max_tre_cnv_z_idx,
                tv.sel_tre_cnv_z_idx,
                TvMsg::CnvZoomChanged,
            ));
            sb_col = sb_col.push(slider(
                Some("Opening Angle"),
                tv.min_opn_angle_idx,
                tv.max_opn_angle_idx,
                tv.sel_opn_angle_idx,
                TvMsg::OpnAngleChanged,
            ));
            sb_col = sb_col.push(slider(
                Some("Rotation Angle"),
                tv.min_rot_angle_idx,
                tv.max_rot_angle_idx,
                tv.sel_rot_angle_idx,
                TvMsg::RotAngleChanged,
            ));
        }
    }

    sb_col = sb_col.push(rule_h(1));

    if tv.tip_brnch_labs_allowed && sel_tree.has_tip_labs() && tv.draw_tip_labs {
        sb_col = sb_col.push(column![
            toggler_label_tip(true, tv.draw_tip_labs,),
            slider(
                None,
                tv.min_lab_size_idx,
                tv.max_lab_size_idx,
                tv.sel_tip_lab_size_idx,
                TvMsg::TipLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_tip(
            tv.tip_brnch_labs_allowed && sel_tree.has_tip_labs(),
            tv.draw_tip_labs,
        ))
    }

    if sel_tree.has_brlen() && tv.tip_brnch_labs_allowed && tv.draw_brnch_labs {
        sb_col = sb_col.push(column![
            toggler_label_branch(true, tv.draw_brnch_labs),
            slider(
                None,
                tv.min_lab_size_idx,
                tv.max_lab_size_idx,
                tv.sel_brnch_lab_size_idx,
                TvMsg::BrnchLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_branch(
            sel_tree.has_brlen() && tv.tip_brnch_labs_allowed,
            tv.draw_brnch_labs,
        ))
    }

    if sel_tree.has_int_labs() && tv.draw_int_labs {
        sb_col = sb_col.push(column![
            toggler_label_int(true, tv.draw_int_labs),
            slider(
                None,
                tv.min_lab_size_idx,
                tv.max_lab_size_idx,
                tv.sel_int_lab_size_idx,
                TvMsg::IntLabSizeChanged,
            )
        ])
    } else {
        sb_col = sb_col.push(toggler_label_int(sel_tree.has_int_labs(), tv.draw_int_labs))
    }

    container(container(sb_col).clip(true)).padding(10).width(220).style(bordered_box).into()
}

fn btn(lab: &str, msg: Option<TvMsg>) -> Button<TvMsg> {
    let mut txt = Text::new(lab);
    txt = txt.align_x(Horizontal::Center);
    txt = txt.align_y(Vertical::Center);
    let mut btn = Button::new(txt);
    btn = btn.on_press_maybe(msg);
    btn
}

fn btn_prev_tree<'a>(enabled: bool) -> Button<'a, TvMsg> {
    btn(
        "Prev",
        match enabled {
            true => Some(TvMsg::PrevTree),
            false => None,
        },
    )
}

fn btn_next_tree<'a>(enabled: bool) -> Button<'a, TvMsg> {
    btn(
        "Next",
        match enabled {
            true => Some(TvMsg::NextTree),
            false => None,
        },
    )
}

fn btn_root(sel_tree: &TreeState) -> Button<TvMsg> {
    btn("Root", {
        if sel_tree.selected_node_ids().len() == 1 {
            let &node_id = sel_tree.selected_node_ids().iter().last().unwrap();
            match sel_tree.can_root(&node_id) {
                true => Some(TvMsg::Root(node_id)),
                false => None,
            }
        } else {
            None
        }
    })
}

fn btn_unroot(sel_tree: &TreeState) -> Button<TvMsg> {
    btn(
        "Unroot",
        match sel_tree.is_rooted() {
            true => Some(TvMsg::Unroot),
            false => None,
        },
    )
}

fn pick_list_common<'a, T: PartialEq + Display + Clone>(
    pl: PickList<'a, T, &[T], T, TvMsg>,
) -> PickList<'a, T, &'a [T], T, TvMsg> {
    // let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TEXT_SIZE)) };
    // let mut pl = pl;
    // pl = pl.handle(h);
    // pl = pl.style(sty_pick_lst);
    pl
}

fn pick_list_node_ordering<'a>(sel_node_ord_opt: NodeOrd) -> Row<'a, TvMsg> {
    let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TvMsg> =
        PickList::new(&NODE_ORD_OPTS, Some(sel_node_ord_opt), TvMsg::NodeOrdOptChanged);
    pl = pick_list_common(pl);
    row![txt("Node Order").width(Length::Fill), pl].align_y(Vertical::Center)
}

fn pick_list_tree_style<'a>(sel_tree_style_opt: TreeStyle) -> Row<'a, TvMsg> {
    let mut pl: PickList<TreeStyle, &[TreeStyle], TreeStyle, TvMsg> =
        PickList::new(&TREE_STYLE_OPTS, Some(sel_tree_style_opt), TvMsg::TreeStyOptChanged);
    pl = pick_list_common(pl);
    row![txt("Style").width(Length::Fill), pl].align_y(Vertical::Center)
}

pub(crate) fn rule_common(rule: Rule<Theme>) -> Rule<Theme> {
    // let rule = rule.style(sty_rule);
    rule
}

pub(crate) fn rule_h<'a>(height: impl Into<Pixels>) -> Rule<'a, Theme> {
    let mut r: Rule<'_, Theme> = Rule::horizontal(height);
    r = rule_common(r);
    r
}

pub(crate) fn rule_v<'a>(width: impl Into<Pixels>) -> Rule<'a, Theme> {
    let mut r: Rule<'_, Theme> = Rule::vertical(width);
    r = rule_common(r);
    r
}

fn scrollable_common(
    scrl: Scrollable<TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<TvMsg> {
    let mut s = scrl;
    s = s.width(w.into());
    s = s.height(h.into());
    s
}

fn scrollable_cnv_ltt(
    cnv: Canvas<&PlotCnv, TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Horizontal(Scrollbar::new()));
    s = s.id("ltt");
    scrollable_common(s, w, h)
}

fn scrollable_cnv_tree(
    cnv: Canvas<&TreeView, TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    let sb = Scrollbar::new();
    s = s.direction(ScrollableDirection::Both { horizontal: sb, vertical: sb });
    s = s.id("tre");
    scrollable_common(s, w, h)
}

fn scrollable_v<'a>(
    content: impl Into<Element<'a, TvMsg>>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(content);
    s = s.direction(ScrollableDirection::Vertical(Scrollbar::new()));
    scrollable_common(s, w, h)
}

fn slider<'a, T>(
    lab: Option<&str>, min: T, max: T, sel: T, msg: impl 'a + Fn(T) -> TvMsg,
) -> Element<'a, TvMsg>
where
    f64: From<T>,
    T: 'a + PartialOrd + From<u8> + Copy + FromPrimitive,
{
    let mut slider: Slider<T, TvMsg> = Slider::new(min..=max, sel, msg);
    slider = slider.step(1);
    slider = slider.shift_step(2);
    // slider = slider.height(TEXT_SIZE * 1.2);
    // slider = slider.style(sty_slider);

    if let Some(lab) = lab {
        let mut lab = container(txt(lab));
        lab = lab.align_x(Horizontal::Right);
        lab = lab.align_y(Vertical::Center);
        lab = lab.width(Length::Fill);

        let mut c: Column<TvMsg> = Column::new();
        c = c.push(lab);
        c = c.push(slider);
        c = c.align_x(Horizontal::Center);
        // c = c.spacing(3e0);
        c.into()
    } else {
        slider.into()
    }
}

pub(crate) fn space_h(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    horizontal_space().width(w).height(h)
}

pub(crate) fn space_v(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    vertical_space().width(w).height(h)
}

fn toggler(label: &str, value: bool) -> Toggler<TvMsg> {
    let mut tglr: Toggler<TvMsg> = Toggler::new(value);
    tglr = tglr.label(label);
    // tglr = tglr.text_size(TEXT_SIZE);
    // tglr = tglr.size(TEXT_SIZE * 1.5);
    tglr = tglr.text_alignment(Alignment::End);
    tglr = tglr.width(Length::Fill);
    // tglr = tglr.roundness(TogglerRoundness::Radius(RADIUS_WIDGET));
    // tglr = tglr.style(sty_toggler);
    tglr
}

// fn toggler_lttp<'a>(enabled: bool, show_lttp: bool) -> Toggler<'a, TvMsg> {
//     let mut tglr = toggler("LTT Plot", show_lttp);
//     if enabled {
//         tglr = tglr.on_toggle(TvMsg::LttpVisChanged);
//     }
//     tglr
// }

pub(crate) fn toggler_label_branch<'a>(enabled: bool, draw_brnch_labs: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Branch Lengths", enabled && draw_brnch_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::BrnchLabVisChanged);
    }
    tglr
}

pub(crate) fn toggler_label_int<'a>(enabled: bool, draw_int_labs: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Internal Labels", enabled && draw_int_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::IntLabVisChanged);
    }
    tglr
}

fn toggler_label_tip<'a>(enabled: bool, draw_tip_labs: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Tip Labels", enabled && draw_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabVisChanged);
    }
    tglr
}

pub(super) fn txt<'a>(s: impl Into<String>) -> Text<'a> {
    Text::new(s.into())
}

fn txt_bool(b: bool) -> Text<'static> {
    let s = match b {
        true => "Yes",
        false => "No",
    };
    txt(s)
}

fn txt_bool_option(ob: Option<bool>) -> Text<'static> {
    match ob {
        Some(b) => txt_bool(b),
        None => txt("N/A"),
    }
}

fn txt_float(n: impl Into<f32>) -> Text<'static> {
    let mut num_fmt = numfmt::Formatter::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

fn txt_usize(n: impl Into<usize>) -> Text<'static> {
    let mut num_fmt = numfmt::Formatter::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}
