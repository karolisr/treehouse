use super::super::super::menu_model::Menu;
use super::super::super::menu_model::MenuItem;
use super::super::super::menu_model::MenuItemId;

use super::SUBMENU_W;
use super::ui::btn_menu_item;
use super::ui::btn_menu_item_txt;

use riced::BORDER_W;
use riced::Border;
use riced::Clr;
use riced::Element;
use riced::Length;
use riced::PADDING;
use riced::Padding;
use riced::Radius;
use riced::Renderer;
use riced::SF;
use riced::Shadow;
use riced::Theme;
use riced::Vector;
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

impl<'a, Msg: 'a + Clone + From<MenuItemId>> From<Menu>
    for AwMenuBar<'a, Msg, Theme, Renderer>
{
    fn from(menu: Menu) -> Self {
        AwMenuBar::new({
            menu.items()
                .iter()
                .map(|menu_item| menu_item.clone().into())
                .collect()
        })
        .close_on_background_click(true)
        .close_on_background_click_global(true)
        .close_on_item_click(true)
        .close_on_item_click_global(true)
        .safe_bounds_margin(PADDING)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(PADDING)
        .spacing(PADDING)
        .style(sty_menu_bar)
    }
}

impl<'a, Msg: 'a + Clone + From<MenuItemId>> From<Menu>
    for AwMenu<'a, Msg, Theme, Renderer>
{
    fn from(menu: Menu) -> Self {
        AwMenu::new({
            menu.items()
                .iter()
                .map(|menu_item| menu_item.clone().into())
                .collect()
        })
        .width(SUBMENU_W)
        .padding(PADDING)
        .spacing(0)
        .offset(PADDING * 2.0)
        .close_on_background_click(true)
        .close_on_item_click(true)
    }
}

impl<'a, Msg: 'a + Clone + From<MenuItemId>> From<MenuItem>
    for AwMenuItem<'a, Msg, Theme, Renderer>
{
    fn from(itm: MenuItem) -> Self {
        match itm {
            MenuItem::Item { label, enabled, id, accelerator } => {
                AwMenuItem::new(btn_menu_item(
                    label,
                    accelerator,
                    if enabled { Some(id.into()) } else { None },
                ))
            }
            MenuItem::Submenu { label, enabled, id, menu } => {
                AwMenuItem::with_menu(
                    btn_menu_item_txt(label, {
                        if enabled { Some(id.into()) } else { None }
                    }),
                    menu.into(),
                )
            }
            MenuItem::Separator => {
                AwMenuItem::new(container(horizontal_rule(SF)).padding(PADDING))
            }
        }
        .close_on_click(true)
    }
}

pub fn menu_bar<'a, Msg: 'a + Clone + From<MenuItemId>>(
    menu: Menu,
    base: Element<'a, Msg>,
) -> Element<'a, Msg, Theme, Renderer> {
    let mb: AwMenuBar<'a, Msg, Theme, Renderer> = menu.into();
    iced_col![
        container(container(mb).style(sty_cont_menu_bar)).padding(Padding {
            top: 0.0,
            right: PADDING,
            bottom: 0.0,
            left: PADDING
        }),
        base
    ]
    .into()
}

fn sty_menu_bar(theme: &Theme, _status: AwStatus) -> MenuBarStyle {
    let ep = theme.extended_palette();
    let base_background = ep.background.weakest.color.into();

    let bar_border = Border {
        color: Clr::TRN,
        width: BORDER_W,
        radius: WIDGET_RADIUS.into(),
    };

    let menu_border = Border {
        color: ep.background.strong.color,
        width: BORDER_W,
        radius: Radius {
            top_left: WIDGET_RADIUS,
            top_right: WIDGET_RADIUS,
            bottom_right: WIDGET_RADIUS,
            bottom_left: WIDGET_RADIUS,
        },
    };

    let path_border = Border {
        color: Clr::RED,
        width: BORDER_W,
        radius: Radius {
            top_left: WIDGET_RADIUS,
            top_right: WIDGET_RADIUS,
            bottom_right: WIDGET_RADIUS,
            bottom_left: WIDGET_RADIUS,
        },
    };

    let bar_shadow = Shadow {
        color: Clr::TRN,
        offset: Vector { x: 0.0, y: 0.0 },
        blur_radius: 0.0,
    };

    let menu_shadow = Shadow {
        color: ep.background.strong.color.scale_alpha(0.77),
        offset: Vector { x: 0.0, y: 0.0 },
        blur_radius: PADDING - PADDING / 3.0,
    };

    MenuBarStyle {
        bar_background: Clr::TRN.into(),
        bar_border,
        bar_shadow,

        menu_background: base_background,
        menu_border,
        menu_shadow,

        path: base_background,
        path_border,
    }
}
