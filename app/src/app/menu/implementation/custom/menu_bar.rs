use super::super::super::menu_model::Menu;
use super::super::super::menu_model::MenuItem;
use super::super::super::menu_model::MenuItemId;

use super::ui::btn_menu_item_ele;
use super::ui::btn_menu_item_txt;

use riced::Border;
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
use riced::iced_row;
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
        .padding(0.0)
        .spacing(PADDING / 2.0)
        .width(Length::Fill)
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
        .offset(PADDING * 2.0)
        .padding(PADDING)
        .spacing(PADDING / 2.0)
        .width(Length::Shrink)
    }
}

impl<'a, Msg: 'a + Clone + From<MenuItemId>> From<MenuItem>
    for AwMenuItem<'a, Msg, Theme, Renderer>
{
    fn from(itm: MenuItem) -> Self {
        match itm {
            MenuItem::Item { label, enabled, id, accelerator } => {
                let label_txt = Text::new(label)
                    .align_x(Horizontal::Left)
                    .align_y(Vertical::Center)
                    .width(90.0 * SF);

                let accelerator_ele = if let Some(accelerator) = accelerator {
                    Text::new(accelerator.to_string())
                        .align_x(Horizontal::Left)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                } else {
                    Text::new(String::new())
                };

                let content = iced_row![label_txt, accelerator_ele];
                AwMenuItem::new(btn_menu_item_ele(content, {
                    if enabled { Some(id.into()) } else { None }
                }))
            }
            MenuItem::Submenu { label, enabled, id, menu } => {
                let aw_menu = menu.into();
                AwMenuItem::with_menu(
                    btn_menu_item_txt(label, {
                        if enabled { Some(id.into()) } else { None }
                    }),
                    aw_menu,
                )
            }
            MenuItem::Separator => AwMenuItem::new(horizontal_rule(SF)),
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
        container(mb)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(PADDING)
            .style(sty_cont_menu_bar),
        base
    ]
    .width(Length::Fill)
    .into()
}

fn sty_menu_bar(theme: &Theme, _status: AwStatus) -> MenuBarStyle {
    let ep = theme.extended_palette();
    let base_background = ep.background.weakest.color.into();

    let bar_border = Border {
        color: Clr::TRN,
        width: 0.0,
        radius: Radius {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        },
    };

    let menu_border = Border {
        color: Clr::TRN,
        width: 0.0,
        radius: Radius {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: WIDGET_RADIUS,
            bottom_left: WIDGET_RADIUS,
        },
    };

    let path_border = Border {
        color: Clr::RED,
        width: SF,
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
        blur_radius: PADDING,
    };

    MenuBarStyle {
        bar_background: base_background,
        bar_border,
        bar_shadow,

        menu_background: base_background,
        menu_border,
        menu_shadow,

        path: base_background,
        path_border,
    }
}
