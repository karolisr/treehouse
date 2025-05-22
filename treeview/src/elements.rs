use crate::iced::*;
use crate::*;

pub(crate) fn btn(lab: &str, msg: Option<TvMsg>) -> Button<TvMsg> {
    let mut txt = Text::new(lab);
    txt = txt.align_x(Horizontal::Center);
    txt = txt.align_y(Vertical::Center);
    let mut btn = Button::new(txt);
    btn = btn.on_press_maybe(msg);
    btn
}

pub(crate) fn btn_prev_tre<'a>(enabled: bool) -> Button<'a, TvMsg> {
    btn(
        "Prev",
        match enabled {
            true => Some(TvMsg::PrevTre),
            false => None,
        },
    )
}

pub(crate) fn btn_next_tre<'a>(enabled: bool) -> Button<'a, TvMsg> {
    btn(
        "Next",
        match enabled {
            true => Some(TvMsg::NextTre),
            false => None,
        },
    )
}

pub(crate) fn btn_root(sel_tre: &TreeState) -> Button<TvMsg> {
    btn("Root", {
        if sel_tre.sel_node_ids().len() == 1 {
            let &node_id = sel_tre.sel_node_ids().iter().last().unwrap();
            match sel_tre.can_root(&node_id) {
                true => Some(TvMsg::Root(node_id)),
                false => None,
            }
        } else {
            None
        }
    })
}

pub(crate) fn btn_unroot(sel_tre: &TreeState) -> Button<TvMsg> {
    btn(
        "Unroot",
        match sel_tre.is_rooted() {
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

pub(crate) fn pick_list_node_ordering<'a>(sel_node_ord_opt: NodeOrd) -> Row<'a, TvMsg> {
    let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TvMsg> =
        PickList::new(&NODE_ORD_OPTS, Some(sel_node_ord_opt), TvMsg::NodeOrdOptChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Node Order").width(Length::Fill), pl].align_y(Vertical::Center)
}

pub(crate) fn pick_list_tre_sty<'a>(sel_tre_style_opt: TreSty) -> Row<'a, TvMsg> {
    let mut pl: PickList<TreSty, &[TreSty], TreSty, TvMsg> =
        PickList::new(&TRE_STY_OPTS, Some(sel_tre_style_opt), TvMsg::TreStyOptChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Style").width(Length::Fill), pl].align_y(Vertical::Center)
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

fn scrollable_common(scrl: Scrollable<TvMsg>, w: impl Into<Length>, h: impl Into<Length>) -> Scrollable<TvMsg> {
    let mut s = scrl;
    s = s.width(w.into());
    s = s.height(h.into());
    s
}

pub(crate) fn scrollable_cnv_ltt<'a>(
    id: &'static str, cnv: Cnv<&'a PlotCnv, TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Horizontal(Scrollbar::new()));
    s = s.id(id);
    s = s.on_scroll(TvMsg::LttCnvScrolled);
    scrollable_common(s, w, h)
}

pub(crate) fn scrollable_cnv_tre<'a>(
    id: &'static str, cnv: Cnv<&'a TreeView, TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    let sb = Scrollbar::new();
    s = s.direction(ScrollableDirection::Both { horizontal: sb, vertical: sb });
    s = s.id(id);
    s = s.on_scroll(TvMsg::TreCnvScrolledOrResized);
    scrollable_common(s, w, h)
}

fn scrollable_v<'a>(
    content: impl Into<Element<'a, TvMsg>>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(content);
    s = s.direction(ScrollableDirection::Vertical(Scrollbar::new()));
    scrollable_common(s, w, h)
}

pub(crate) fn slider<'a, T>(
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

pub(crate) fn space_h(w: impl Into<Length>, h: impl Into<Length>) -> Space { horizontal_space().width(w).height(h) }
pub(crate) fn space_v(w: impl Into<Length>, h: impl Into<Length>) -> Space { vertical_space().width(w).height(h) }

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

pub(crate) fn toggler_label_tip<'a>(enabled: bool, draw_tip_labs: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Tip Labels", enabled && draw_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabVisChanged);
    }
    tglr
}

pub(super) fn txt<'a>(s: impl Into<String>) -> Text<'a> { Text::new(s.into()) }

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

pub(crate) fn txt_float(n: impl Into<Float>) -> Text<'static> {
    let mut num_fmt = numfmt::Formatter::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

pub(crate) fn txt_usize(n: impl Into<usize>) -> Text<'static> {
    let mut num_fmt = numfmt::Formatter::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}
