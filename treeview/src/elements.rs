use crate::cnv_plot::AXIS_SCALE_TYPE_OPTS;
use crate::iced::*;
use crate::style::*;
use crate::*;

fn btn(btn: Button<'_, TvMsg>, msg: Option<TvMsg>) -> Button<'_, TvMsg> {
    let mut btn = btn;
    btn = btn.on_press_maybe(msg);
    btn = btn.width(BTN_H);
    btn = btn.height(BTN_H);
    btn = btn.style(sty_btn);
    btn
}

pub(crate) fn btn_txt(lab: &'_ str, msg: Option<TvMsg>) -> Button<'_, TvMsg> {
    let mut txt = Text::new(lab);
    txt = txt.align_x(Horizontal::Center);
    txt = txt.align_y(Vertical::Center);
    btn(Button::new(txt), msg).padding(ZERO)
}

pub(crate) fn btn_svg(handle: impl Into<SvgHandle>, msg: Option<TvMsg>) -> Button<'static, TvMsg> {
    let svg = Svg::new(handle).style(sty_svg);
    btn(Button::new(svg), msg).padding(PADDING / THREE)
}

pub(crate) fn btn_prev_tre(enabled: bool) -> Button<'static, TvMsg> {
    btn_svg(
        Icon::ArrowLeft,
        match enabled {
            true => Some(TvMsg::PrevTre),
            false => None,
        },
    )
}

pub(crate) fn btn_next_tre(enabled: bool) -> Button<'static, TvMsg> {
    btn_svg(
        Icon::ArrowRight,
        match enabled {
            true => Some(TvMsg::NextTre),
            false => None,
        },
    )
}

pub(crate) fn btn_root(sel_tre: Rc<TreeState>) -> Button<'static, TvMsg> {
    btn_txt("Root", {
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
    .width(BTN_H * TWO)
}

pub(crate) fn btn_unroot(sel_tre: Rc<TreeState>) -> Button<'static, TvMsg> {
    btn_txt(
        "Unroot",
        match sel_tre.is_rooted() {
            true => Some(TvMsg::Unroot),
            false => None,
        },
    )
    .width(BTN_H * TWO)
}

pub(crate) fn checkbox(
    lab: &str, is_checked: bool, msg: impl Fn(bool) -> TvMsg + 'static,
) -> Checkbox<'_, TvMsg> {
    Checkbox::new(lab, is_checked)
        .on_toggle(msg)
        .size(CHECKBOX_H)
        .spacing(PADDING)
        .text_line_height(LINE_HEIGHT)
        .style(sty_checkbox)
}

fn pick_list_common<'a, T: PartialEq + Display + Clone>(
    pl: PickList<'a, T, &[T], T, TvMsg>,
) -> PickList<'a, T, &'a [T], T, TvMsg> {
    let mut pl = pl;
    let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TXT_SIZE)) };
    pl = pl.handle(h);
    pl = pl.text_line_height(Pixels(TXT_SIZE));
    pl = pl.text_size(TXT_SIZE);
    pl = pl.padding(PADDING);
    pl = pl.width(Length::FillPortion(10));
    pl = pl.style(sty_pick_lst);
    pl
}

// pub(crate) fn pick_list_ltt_x_axis_scale_type<'a>(
//     axis_scale_type: &AxisScaleType,
// ) -> Row<'a, TvMsg> {
//     let mut pl: PickList<AxisScaleType, &[AxisScaleType], AxisScaleType, TvMsg> = PickList::new(
//         &AXIS_SCALE_TYPE_OPTS,
//         Some(axis_scale_type.clone()),
//         TvMsg::LttXAxisScaleTypeChanged,
//     );
//     pl = pick_list_common(pl);
//     iced_row![txt("X-Axis Scale").width(Length::FillPortion(9)), pl].align_y(Vertical::Center)
// }

pub(crate) fn pick_list_ltt_y_axis_scale_type<'a>(
    axis_scale_type: &AxisScaleType,
) -> Row<'a, TvMsg> {
    let mut pl: PickList<AxisScaleType, &[AxisScaleType], AxisScaleType, TvMsg> = PickList::new(
        &AXIS_SCALE_TYPE_OPTS,
        Some(axis_scale_type.clone()),
        TvMsg::LttYAxisScaleTypeChanged,
    );
    pl = pick_list_common(pl);
    iced_row![txt("Y-Axis Scale").width(Length::FillPortion(9)), pl].align_y(Vertical::Center)
}

pub(crate) fn pick_list_node_ordering<'a>(node_ord: NodeOrd) -> Row<'a, TvMsg> {
    let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TvMsg> =
        PickList::new(&NODE_ORD_OPTS, Some(node_ord), TvMsg::NodeOrdOptChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Node Order").width(Length::FillPortion(9)), pl].align_y(Vertical::Center)
}

pub(crate) fn pick_list_tre_sty<'a>(tre_sty: TreSty) -> Row<'a, TvMsg> {
    let mut pl: PickList<TreSty, &[TreSty], TreSty, TvMsg> =
        PickList::new(&TRE_STY_OPTS, Some(tre_sty), TvMsg::TreStyOptChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Style").width(Length::FillPortion(9)), pl].align_y(Vertical::Center)
}

pub(crate) fn rule_common(rule: Rule<Theme>) -> Rule<Theme> { rule.style(sty_rule) }

pub(crate) fn rule_h<'a>(height: impl Into<Pixels>) -> Rule<'a, Theme> {
    let mut r: Rule<'_, Theme> = Rule::horizontal(height);
    r = rule_common(r);
    r
}

// pub(crate) fn rule_v<'a>(width: impl Into<Pixels>) -> Rule<'a, Theme> {
//     let mut r: Rule<'a, Theme> = Rule::vertical(width);
//     r = rule_common(r);
//     r
// }

fn scrollable_common(
    scrl: Scrollable<TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<TvMsg> {
    let mut s = scrl;
    s = s.width(w.into());
    s = s.height(h.into());
    s = s.style(sty_scrlbl);
    s
}

fn scroll_bar() -> Scrollbar {
    let mut sb = Scrollbar::new();
    sb = sb.width(SCROLL_BAR_W);
    sb = sb.scroller_width(SCROLL_BAR_W);
    sb
}

pub(crate) fn scrollable_cnv_ltt<'a>(
    id: &'static str, cnv: Cnv<&'a PlotCnv, TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Horizontal(scroll_bar()));
    s = s.id(id);
    s = s.on_scroll(TvMsg::LttCnvScrolledOrResized);
    scrollable_common(s, w, h)
}

pub(crate) fn scrollable_cnv_tre<'a>(
    id: &'static str, cnv: Cnv<&'a TreeCnv, TvMsg>, w: impl Into<Length>, h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Both { horizontal: scroll_bar(), vertical: scroll_bar() });
    s = s.id(id);
    s = s.on_scroll(TvMsg::TreCnvScrolledOrResized);
    scrollable_common(s, w, h)
}

// pub(crate) fn scrollable_v<'a>(
//     content: impl Into<Element<'a, TvMsg>>, w: impl Into<Length>, h: impl Into<Length>,
// ) -> Scrollable<'a, TvMsg> {
//     let mut s: Scrollable<TvMsg> = Scrollable::new(content);
//     s = s.direction(ScrollableDirection::Vertical(scroll_bar()));
//     scrollable_common(s, w, h)
// }

pub(crate) fn slider<'a, T>(
    lab: Option<&str>, min: T, max: T, sel: T, step: T, shift_step: T,
    msg: impl 'a + Fn(T) -> TvMsg,
) -> Element<'a, TvMsg>
where
    f64: From<T>,
    T: 'a + PartialOrd + From<u8> + Copy + FromPrimitive,
{
    let mut slider: Slider<T, TvMsg> = Slider::new(min..=max, sel, msg);

    slider = slider.height(SLIDER_H);
    slider = slider.step(step);
    slider = slider.shift_step(shift_step);
    slider = slider.style(sty_slider);

    if let Some(lab) = lab {
        let mut lab = container(txt(lab));
        lab = lab.align_x(Horizontal::Right);
        lab = lab.align_y(Vertical::Center);
        lab = lab.width(Length::Fill);

        let mut c: Column<TvMsg> = Column::new();
        c = c.push(lab);
        c = c.push(slider);
        c = c.align_x(Horizontal::Center);
        c = c.spacing(ZERO);
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

fn toggler(label: &'_ str, value: bool) -> Toggler<'_, TvMsg> {
    let mut tglr: Toggler<TvMsg> = Toggler::new(value);
    tglr = tglr.size(TOGGLER_H);
    tglr = tglr.label(label);
    tglr = tglr.text_size(TXT_SIZE);
    tglr = tglr.text_line_height(LINE_HEIGHT);
    tglr = tglr.text_alignment(TextAlignment::Left);
    tglr = tglr.width(Length::Fill);
    tglr = tglr.spacing(PADDING);
    tglr = tglr.style(sty_toggler);
    tglr
}

pub(crate) fn toggler_cursor_line<'a>(
    enabled: bool, draw_cursor_line: bool, tre_sty: TreSty,
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

pub(crate) fn toggler_label_branch<'a>(enabled: bool, draw_brnch_labs: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Branch Lengths", draw_brnch_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::BrnchLabVisChanged);
    }
    tglr
}

pub(crate) fn toggler_label_int<'a>(enabled: bool, draw_int_labs: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Internal Labels", draw_int_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::IntLabVisChanged);
    }
    tglr
}

pub(crate) fn toggler_label_tip<'a>(enabled: bool, draw_tip_labs: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Tip Labels", draw_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabVisChanged);
    }
    tglr
}

pub(crate) fn toggler_legend<'a>(enabled: bool, draw_legend: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Legend", draw_legend);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::LegendVisChanged);
    }
    tglr
}

pub(crate) fn toggler_root<'a>(enabled: bool, draw_root: bool) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Root", draw_root);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::RootVisChanged);
    }
    tglr
}

pub(crate) fn text_input(
    placeholder: &str, value: &str, id: &'static str, msg: impl Fn(String) -> TvMsg + 'static,
) -> TextInput<'static, TvMsg> {
    TextInput::new(placeholder, value)
        .style(sty_text_input)
        .id(id)
        .on_input(msg)
        .line_height(Pixels(TEXT_INPUT_H))
        .padding(PADDING)
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
