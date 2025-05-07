use super::styles::{sty_btn, sty_pick_lst, sty_rule, sty_scrlbl, sty_slider, sty_toggler};
use crate::{
    NODE_ORD_OPTS, NodeOrd, PlotCnv, TREE_STYLE_OPTS, TreeCnv, TreeState, TreeStateMsg, TreeStyle,
    TreeViewMsg,
};
use iced::{
    Alignment, Element, Font,
    Length::{self, Fill, Shrink},
    Pixels, Theme,
    alignment::{Horizontal, Vertical},
    widget::{
        Button, Canvas, Column, PickList, Row, Rule, Scrollable, Slider, Space, Text, container,
        horizontal_space,
        pick_list::Handle as PickListHandle,
        row,
        scrollable::{Direction as ScrollableDirection, Scrollbar},
        text, vertical_space,
    },
};
use num_traits::cast::FromPrimitive;
use numfmt::Formatter as NumFmt;
use std::fmt::Display;
use widget::toggler::{Roundness as TogglerRoundness, Toggler};

const SF: f32 = 1e0;
const TEXT_SIZE: f32 = 13.0 * SF;
const RADIUS_WIDGET: f32 = 3e0 * SF;

// --------------------------------------------------------------------------------------------

pub(crate) fn btn(lab: &str, msg: Option<TreeViewMsg>) -> Button<TreeViewMsg> {
    let mut txt = Text::new(lab);
    txt = txt.align_x(Horizontal::Center);
    txt = txt.align_y(Vertical::Center);
    let mut btn = Button::new(txt);
    btn = btn.on_press_maybe(msg);
    // btn = btn.style(sty_btn);
    btn
}

pub(crate) fn btn_root(ts: &TreeState) -> Button<TreeViewMsg> {
    btn("Root", {
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

pub(crate) fn btn_unroot(ts: &TreeState) -> Button<TreeViewMsg> {
    btn(
        "Unroot",
        match ts.is_rooted {
            true => Some(TreeViewMsg::TreeStateMsg(TreeStateMsg::Unroot)),
            false => None,
        },
    )
}

// --------------------------------------------------------------------------------------------

pub(crate) fn pick_list_common<'a, T: PartialEq + Display + Clone>(
    pl: PickList<'a, T, &[T], T, TreeViewMsg>,
) -> PickList<'a, T, &'a [T], T, TreeViewMsg> {
    let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TEXT_SIZE)) };
    let mut pl = pl;
    pl = pl.handle(h);
    // pl = pl.style(sty_pick_lst);
    pl
}

pub(crate) fn pick_list_node_ordering<'a>(sel_node_ord_opt: NodeOrd) -> Row<'a, TreeViewMsg> {
    let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TreeViewMsg> =
        PickList::new(&NODE_ORD_OPTS, Some(sel_node_ord_opt), TreeViewMsg::NodeOrdOptChanged);
    pl = pick_list_common(pl);
    row![text!("Node Order").width(Fill), pl].align_y(Vertical::Center)
}

pub(crate) fn pick_list_tree_style<'a>(sel_tree_style_opt: TreeStyle) -> Row<'a, TreeViewMsg> {
    let mut pl: PickList<TreeStyle, &[TreeStyle], TreeStyle, TreeViewMsg> = PickList::new(
        &TREE_STYLE_OPTS,
        Some(sel_tree_style_opt),
        TreeViewMsg::TreeStyleOptionChanged,
    );
    pl = pick_list_common(pl);
    row![text!("Style").width(Fill), pl].align_y(Vertical::Center)
}

// --------------------------------------------------------------------------------------------

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

// --------------------------------------------------------------------------------------------

pub(crate) fn scrollable_common(
    scrl: Scrollable<TreeViewMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<TreeViewMsg> {
    let mut s = scrl;
    // s = s.style(sty_scrlbl);
    s = s.width(w.into());
    s = s.height(h.into());
    s
}

pub(crate) fn scrollable_v<'a>(
    content: impl Into<Element<'a, TreeViewMsg>>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<'a, TreeViewMsg> {
    let mut s: Scrollable<TreeViewMsg> = Scrollable::new(content);
    s = s.direction(ScrollableDirection::Vertical(Scrollbar::new()));
    scrollable_common(s, w, h)
}

pub(crate) fn scrollable_cnv_ltt(
    cnv: Canvas<&PlotCnv, TreeViewMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<TreeViewMsg> {
    let mut s: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Horizontal(Scrollbar::new()));
    s = s.on_scroll(TreeViewMsg::LttCnvScrolled);
    s = s.id("ltt");
    scrollable_common(s, w, h)
}

pub(crate) fn scrollable_cnv_tree(
    cnv: Canvas<&TreeCnv, TreeViewMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<TreeViewMsg> {
    let mut s: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
    let sb = Scrollbar::new();
    s = s.direction(ScrollableDirection::Both { horizontal: sb, vertical: sb });
    s = s.on_scroll(TreeViewMsg::TreCnvScrolled);
    s = s.id("tre");
    scrollable_common(s, w, h)
}

// --------------------------------------------------------------------------------------------

pub(crate) fn slider<'a, T>(
    lab: Option<&str>,
    min: T,
    max: T,
    sel: T,
    msg: impl 'a + Fn(T) -> TreeViewMsg,
) -> Element<'a, TreeViewMsg>
where
    f64: From<T>,
    T: 'a + PartialOrd + From<u8> + Copy + FromPrimitive,
{
    let mut slider: Slider<T, TreeViewMsg> = Slider::new(min..=max, sel, msg);
    slider = slider.step(1);
    slider = slider.shift_step(2);
    slider = slider.height(TEXT_SIZE * 1.2);
    // slider = slider.style(sty_slider);

    if let Some(lab) = lab {
        let mut lab = container(text!("{lab}"));
        lab = lab.align_x(Horizontal::Right);
        lab = lab.align_y(Vertical::Center);
        lab = lab.width(Fill);

        let mut c: Column<TreeViewMsg> = Column::new();
        c = c.push(lab);
        c = c.push(slider);
        c = c.align_x(Horizontal::Center);
        c = c.spacing(3e0);
        c.into()
    } else {
        slider.into()
    }
}

// --------------------------------------------------------------------------------------------

pub(crate) fn space_h(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    horizontal_space().width(w).height(h)
}

pub(crate) fn space_v(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    vertical_space().width(w).height(h)
}

// --------------------------------------------------------------------------------------------

pub(crate) fn toggler(label: &str, value: bool) -> Toggler<TreeViewMsg> {
    let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
    tglr = tglr.label(label);
    tglr = tglr.text_size(TEXT_SIZE);
    tglr = tglr.size(TEXT_SIZE * 1.5);
    tglr = tglr.text_alignment(Alignment::End);
    tglr = tglr.width(Fill);
    // tglr = tglr.roundness(TogglerRoundness::Radius(RADIUS_WIDGET));
    // tglr = tglr.style(sty_toggler);
    tglr
}

pub(crate) fn toggler_cursor_line<'a>(
    enabled: bool,
    show_cursor_line: bool,
    sel_tree_style_opt: TreeStyle,
) -> Toggler<'a, TreeViewMsg> {
    let lab = match sel_tree_style_opt {
        TreeStyle::Phylogram => "Cursor Tracking Line",
        TreeStyle::Fan => "Cursor Tracking Circle",
    };

    let mut tglr = toggler(lab, show_cursor_line);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::CursorLineVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_label_branch<'a>(
    enabled: bool,
    draw_brnch_labs: bool,
) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Branch Lengths", enabled && draw_brnch_labs);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::BranchLabelVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_label_int<'a>(
    enabled: bool,
    draw_int_labs: bool,
) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Internal Labels", enabled && draw_int_labs);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::IntLabelVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_label_tip<'a>(
    enabled: bool,
    draw_tip_labs: bool,
) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Tip Labels", enabled && draw_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::TipLabelVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_legend<'a>(enabled: bool, draw_legend: bool) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Legend", enabled && draw_legend);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::LegendVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_ltt<'a>(enabled: bool, show_ltt: bool) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("LTT Plot", show_ltt);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::LttPlotVisibilityChanged);
    }
    tglr
}

// --------------------------------------------------------------------------------------------

pub(crate) fn txt(s: impl Into<String>) -> Text<'static> {
    Text::new(s.into()).align_x(Horizontal::Right).align_y(Vertical::Center).width(Shrink)
}

pub(crate) fn txt_bool(b: bool) -> Text<'static> {
    let s = match b {
        true => "Yes",
        false => "No",
    };
    txt(s)
}

pub(crate) fn txt_bool_option(ob: Option<bool>) -> Text<'static> {
    match ob {
        Some(b) => txt_bool(b),
        None => txt("N/A"),
    }
}

pub(crate) fn txt_float(n: impl Into<f32>) -> Text<'static> {
    let mut num_fmt = NumFmt::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

pub(crate) fn txt_usize(n: impl Into<usize>) -> Text<'static> {
    let mut num_fmt = NumFmt::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}
