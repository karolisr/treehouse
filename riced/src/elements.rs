use crate::style::*;
use crate::*;

fn btn_common<Msg>(btn: Button<'_, Msg>, msg: Option<Msg>) -> Button<'_, Msg> {
    let mut btn = btn;
    btn = btn.on_press_maybe(msg);
    btn = btn.width(BTN_H1);
    btn = btn.height(BTN_H1);
    btn.style(sty_btn)
}

pub fn btn_svg<'a, Msg>(
    handle: impl Into<SvgHandle>,
    msg: Option<Msg>,
) -> Button<'a, Msg> {
    let svg = Svg::new(handle).style(sty_svg);
    btn_common(Button::new(svg), msg).padding(PADDING / THREE)
}

pub fn btn_txt<Msg>(lab: &'_ str, msg: Option<Msg>) -> Button<'_, Msg> {
    let mut txt = Text::new(lab);
    txt = txt.align_x(Horizontal::Center);
    txt = txt.align_y(Vertical::Center);
    btn_common(Button::new(txt), msg).padding(ZERO)
}

pub fn checkbox<'a, Msg>(
    lab: &str,
    is_checked: bool,
    msg: impl Fn(bool) -> Msg + 'a,
) -> Checkbox<'a, Msg> {
    Checkbox::new(lab, is_checked)
        .on_toggle(msg)
        .size(CHECKBOX_H)
        .spacing(PADDING)
        .text_line_height(LINE_H_PIX)
        .style(sty_checkbox)
}

pub fn pick_list_common<'a, T: PartialEq + Display + Clone, Msg: Clone + 'a>(
    pl: PickList<'a, T, &[T], T, Msg>,
) -> PickList<'a, T, &'a [T], T, Msg> {
    let mut pl = pl;
    pl = pl.handle(PickListHandle::Arrow { size: Some(LINE_H_PIX) });
    pl = pl.text_line_height(LINE_H_PIX);
    pl = pl.text_size(TXT_SIZE);
    pl = pl.padding(PADDING);
    pl = pl.width(Length::FillPortion(10));
    pl = pl.menu_style(sty_menu);
    pl.style(sty_pick_lst)
}

fn rule_common(rule: Rule<Theme>) -> Rule<Theme> {
    rule.style(sty_rule)
}

pub fn rule_h<'a>(height: impl Into<Pixels>) -> Rule<'a, Theme> {
    let r: Rule<'_, Theme> = horizontal_rule(height);
    rule_common(r)
}

pub fn rule_v<'a>(width: impl Into<Pixels>) -> Rule<'a, Theme> {
    let r: Rule<'a, Theme> = vertical_rule(width);
    rule_common(r)
}

pub fn scrollable_common<Msg>(
    scrl: Scrollable<Msg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<Msg> {
    let mut s = scrl;
    s = s.width(w.into());
    s = s.height(h.into());
    s.style(sty_scrlbl)
}

pub fn scroll_bar() -> Scrollbar {
    let mut sb = Scrollbar::new();
    sb = sb.scroller_width(SCROLLBAR_W);
    sb.width(SCROLLBAR_W)
}

pub fn scrollable_v<'a, Msg: Clone + 'a>(
    content: impl Into<Element<'a, Msg>>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<'a, Msg> {
    let mut s: Scrollable<Msg> = Scrollable::new(content);
    s = s.direction(ScrollableDirection::Vertical(scroll_bar()));
    scrollable_common(s, w, h)
}

pub fn slider<'a, T, Msg: Clone + 'a>(
    lab: Option<&str>,
    min: T,
    max: T,
    sel: T,
    step: T,
    shift_step: T,
    msg: impl 'a + Fn(T) -> Msg,
) -> Element<'a, Msg>
where
    f64: From<T>,
    T: 'a + PartialOrd + From<u8> + Copy + FromPrimitive,
{
    let mut slider: Slider<T, Msg> = Slider::new(min..=max, sel, msg);

    slider = slider.height(SLIDER_H);
    slider = slider.step(step);
    slider = slider.shift_step(shift_step);
    slider = slider.style(sty_slider);

    if let Some(lab) = lab {
        let mut lab = container(txt(lab));
        lab = lab.align_x(Horizontal::Right);
        lab = lab.align_y(Vertical::Center);
        lab = lab.width(Length::Fill);

        let mut c: Column<Msg> = Column::new();
        c = c.push(lab);
        c = c.push(slider);
        c = c.align_x(Horizontal::Center);
        c = c.spacing(ZERO);
        c.into()
    } else {
        slider.into()
    }
}

pub fn space_h(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    horizontal_space().width(w).height(h)
}

pub fn space_v(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    vertical_space().width(w).height(h)
}

pub fn toggler<'a, Msg>(label: &'a str, value: bool) -> Toggler<'a, Msg> {
    let mut tglr: Toggler<Msg> = Toggler::new(value);
    tglr = tglr.size(TOGGLER_H);
    tglr = tglr.label(label);
    tglr = tglr.text_size(TXT_SIZE);
    tglr = tglr.text_line_height(LINE_H_PIX);
    tglr = tglr.text_alignment(TextAlignment::Left);
    tglr = tglr.width(Length::Fill);
    tglr = tglr.spacing(PADDING);
    tglr.style(sty_toggler)
}

pub fn txt<'a>(s: impl Into<String>) -> Text<'a> {
    Text::new(s.into()).line_height(LINE_H_PIX).align_y(Vertical::Center)
}

pub fn txt_bool(b: bool) -> Text<'static> {
    let s = match b {
        true => "Yes",
        false => "No",
    };
    txt(s)
}

pub fn txt_bool_option(ob: Option<bool>) -> Text<'static> {
    match ob {
        Some(b) => txt_bool(b),
        None => txt("N/A"),
    }
}

pub fn txt_float(n: impl Into<f32>) -> Text<'static> {
    let mut num_fmt = numfmt::Formatter::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

pub fn txt_usize(n: impl Into<usize>) -> Text<'static> {
    let mut num_fmt = numfmt::Formatter::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

pub fn txt_i64(n: impl Into<i64>) -> Text<'static> {
    let mut num_fmt = numfmt::Formatter::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

pub fn txt_input<'a, Msg: Clone + 'a>(
    placeholder: &str,
    value: &str,
    id: &'static str,
    msg: impl Fn(String) -> Msg + 'a,
) -> TextInput<'a, Msg> {
    TextInput::new(placeholder, value)
        .style(sty_text_input)
        .id(id)
        .on_input(msg)
        .line_height(Pixels(TEXT_INPUT_H - PADDING * TWO))
        .padding(PADDING)
}
