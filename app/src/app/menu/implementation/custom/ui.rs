use riced::BTN_H_MENU;
use riced::Background;
use riced::Border;
use riced::Button;
use riced::ButtonStatus;
use riced::ButtonStyle;
use riced::Clr;
use riced::Element;
use riced::Horizontal;
use riced::Length;
use riced::PADDING;
use riced::Padding;
use riced::Text;
use riced::Theme;
use riced::Vertical;
use riced::WIDGET_RADIUS;
use riced::iced_row;

use super::SUBMENU_W;

use crate::app::menu::menu_model::Accelerator;

pub(crate) fn btn_menu_item_txt<'a, Msg>(
    lab: impl Into<String>,
    msg: Option<Msg>,
) -> Button<'a, Msg> {
    let mut txt = Text::new(lab.into());
    txt = txt.align_x(Horizontal::Left);
    txt = txt.align_y(Vertical::Center);
    txt = txt.height(Length::Fill);
    btn_menu_item_ele(txt, msg)
}

pub(crate) fn btn_menu_item<'a, Msg: 'a>(
    lab: impl Into<String>,
    accel: Option<Accelerator>,
    msg: Option<Msg>,
) -> Button<'a, Msg> {
    let mut label_txt = Text::new(lab.into());
    let mut accel_ele = if let Some(accel) = accel {
        Text::new(accel.to_string())
    } else {
        Text::new(String::new())
    };

    let lab_frac = 2e0 / 3e0;
    let lab_w = SUBMENU_W * lab_frac;
    let accel_w = SUBMENU_W - lab_w;

    label_txt = label_txt.align_x(Horizontal::Left);
    label_txt = label_txt.align_y(Vertical::Center);
    label_txt = label_txt.width(lab_w);
    label_txt = label_txt.height(Length::Fill);

    accel_ele = accel_ele.align_x(Horizontal::Left);
    accel_ele = accel_ele.align_y(Vertical::Center);
    accel_ele = accel_ele.width(accel_w);
    accel_ele = accel_ele.height(Length::Fill);

    let content = iced_row![label_txt, accel_ele]
        .padding(0)
        .spacing(0)
        .height(Length::Fill);

    btn_menu_item_ele(content, msg)
}

fn btn_menu_item_ele<'a, Msg>(
    ele: impl Into<Element<'a, Msg>>,
    msg: Option<Msg>,
) -> Button<'a, Msg> {
    let mut btn = Button::new(ele.into());
    btn = btn.on_press_maybe(msg);
    btn = btn.clip(true);
    btn = btn.height(BTN_H_MENU);
    btn = btn.padding(Padding {
        top: 0.0,
        right: PADDING,
        bottom: 0.0,
        left: PADDING,
    });
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
        ButtonStatus::Active => base,
        ButtonStatus::Hovered | ButtonStatus::Pressed => ButtonStyle {
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
