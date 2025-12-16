use riced::BTN_H_MENU;
use riced::Background;
use riced::Border;
use riced::Button;
use riced::ButtonStatus;
use riced::ButtonStyle;
use riced::Clr;
use riced::Element;
use riced::Horizontal;
use riced::SF;
use riced::Text;
use riced::Theme;
use riced::Vertical;
use riced::WIDGET_RADIUS;

pub(crate) fn btn_menu_item_txt<'a, Msg>(
    lab: impl Into<String>,
    msg: Option<Msg>,
) -> Button<'a, Msg> {
    let mut txt = Text::new(lab.into());
    txt = txt.align_x(Horizontal::Left);
    txt = txt.align_y(Vertical::Center);
    let mut btn = Button::new(txt);
    btn = btn.on_press_maybe(msg);
    btn = btn.clip(true);
    btn = btn.width(150.0 * SF);
    btn = btn.height(BTN_H_MENU);
    btn = btn.style(sty_btn_menu_item);
    btn
}

pub(crate) fn btn_menu_item_ele<'a, Msg>(
    ele: impl Into<Element<'a, Msg>>,
    msg: Option<Msg>,
) -> Button<'a, Msg> {
    let mut btn = Button::new(ele.into());
    btn = btn.on_press_maybe(msg);
    btn = btn.clip(true);
    btn = btn.width(150.0 * SF);
    btn = btn.height(BTN_H_MENU);
    btn = btn.style(sty_btn_menu_item);
    btn
}

fn sty_btn_menu_item(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let ep = theme.extended_palette();

    let base = ButtonStyle {
        background: Some(ep.background.weakest.color.into()),
        text_color: ep.secondary.base.text,
        border: Border {
            radius: WIDGET_RADIUS.into(),
            width: 0.0,
            color: Clr::TRN,
        },
        ..ButtonStyle::default()
    };

    match status {
        ButtonStatus::Active | ButtonStatus::Pressed => base,
        ButtonStatus::Hovered => ButtonStyle {
            background: Some(Background::Color(ep.primary.strong.color)),
            text_color: ep.primary.base.text,
            ..base
        },
        ButtonStatus::Disabled => ButtonStyle {
            background: base
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}
