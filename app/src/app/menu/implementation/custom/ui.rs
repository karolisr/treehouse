use super::super::super::menu_model::Accelerator;
use super::super::super::menu_model::KeyCode;
use super::super::super::menu_model::Menu;
use super::super::super::menu_model::MenuItem;
use super::super::super::menu_model::MenuItemId;
use super::super::super::menu_model::Modifier;

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
use riced::Radius;
use riced::Renderer;
use riced::SF;
use riced::Shadow;
use riced::Text;
use riced::Theme;
use riced::Vector;
use riced::Vertical;
use riced::WIDGET_RADIUS;
use riced::container;
use riced::horizontal_rule;
use riced::iced_col;
use riced::sty_cont_menu_bar;

use iced_aw::Menu as AwMenu;
use iced_aw::MenuBar as AwMenuBar;
use iced_aw::menu::Item as AwMenuItem;
use iced_aw::style::menu_bar::Style as MenuBarStyle;
use iced_aw::style::status::Status as AwStatus;

pub(crate) fn btn_menu_item<'a, Msg>(
    lab: impl Into<String>,
    msg: Option<Msg>,
) -> Button<'a, Msg> {
    let mut txt = Text::new(lab.into());
    txt = txt.align_x(Horizontal::Left);
    txt = txt.align_y(Vertical::Center);
    let mut btn = Button::new(txt);
    btn = btn.on_press_maybe(msg);
    btn = btn.clip(true);
    btn = btn.width(100.0 * SF);
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
