mod context_menu;

use std::collections::HashMap;

// use iced_aw::Menu as AwMenu;
// use iced_aw::MenuBar as AwMenuBar;
// use iced_aw::menu::Item as AwMenuItem;

// use riced::Alignment;
use riced::BTN_H1;
// use riced::Border;
// use riced::Button;
// use riced::ButtonStatus;
// use riced::ButtonStyle;
// use riced::Color;
use riced::Element;
use riced::Length;
// use riced::Radius;
use riced::SF;
// use riced::TooltipPosition;
// use riced::Vertical;
// use riced::btn_txt;
// use riced::button;
// use riced::center;
use riced::container;
use riced::iced_col;
// use riced::iced_row;
use riced::sty_cont_tool_bar;
// use riced::tooltip;
use riced::txt;

use crate::AppMsg;

use super::MenuItem;
use super::MenuItemId;

pub use context_menu::ContextMenu;
pub use context_menu::show_context_menu;

#[derive(Default, Clone, Debug)]
pub struct AppMenu {
    items: Vec<MenuItem>,
    item_indexes: HashMap<MenuItemId, usize>,
}

impl AppMenu {
    pub fn new() -> Option<Self> {
        Some(Self::default())
    }

    pub fn with_items(items: Vec<MenuItem>) -> Self {
        let mut item_indexes: HashMap<MenuItemId, usize> = HashMap::new();
        for (i, item) in items.iter().enumerate() {
            match item {
                MenuItem::Item { id, .. } => {
                    _ = item_indexes.insert(id.clone(), i);
                }
            }
        }
        Self { items, item_indexes }
    }

    pub fn enable(&mut self, id: &MenuItemId) {
        self.set_enabled(id, true);
    }

    pub fn disable(&mut self, id: &MenuItemId) {
        self.set_enabled(id, false);
    }

    fn set_enabled(&mut self, id: &MenuItemId, state: bool) {
        if let Some(&item_index) = self.item_indexes.get(id) {
            if let Some(menu_item) = self.items.get_mut(item_index) {
                match menu_item {
                    MenuItem::Item { enabled, .. } => {
                        *enabled = state;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, id: &MenuItemId) {
        println!("AppMenu::update({id})");
    }

    pub fn menu_bar<'a>(
        &'a self,
        base: Element<'a, AppMsg>,
    ) -> Element<'a, AppMsg> {
        // let mb_items = vec![AwMenuItem::new(btn_txt("Nested Menus", None))];
        // let mb = AwMenuBar::new(mb_items);

        iced_col![
            container(
                // mb.width(Length::Fill).height(Length::Fixed(BTN_H1 * 2.0))
                txt("MENU_BAR").height(BTN_H1)
            )
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding([SF, SF])
            .style(sty_cont_tool_bar),
            base
        ]
        .into()
    }
}

// -----------------------------------------------------------------------------

// use iced_aw::menu_bar;
// use iced_aw::menu_items;

// let menu_tpl_1 = |items| AwMenu::new(items).width(180.0).offset(15.0).spacing(5.0);
// let menu_tpl_2 = |items| AwMenu::new(items).width(180.0).offset(0.0).spacing(5.0);
// let hold_item = |widget| AwMenuItem::new(widget).close_on_click(false);
// let hold_item_wm = |widget, menu| AwMenuItem::with_menu(widget, menu).close_on_click(false);

// let mb = menu_bar!((debug_button_s("Nested Menus"), {
//     let sub5 = menu_tpl_2(menu_items!(
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//     ));

//     let sub4 = menu_tpl_2(menu_items!(
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//     ))
//     .width(200.0);

//     let sub3 = menu_tpl_2(menu_items!(
//         (debug_button_f("You can")),
//         (debug_button_f("nest menus")),
//         (submenu_button("SUB"), sub4),
//         (debug_button_f("how ever")),
//         (debug_button_f("You like")),
//         (submenu_button("SUB"), sub5),
//     ))
//     .width(180.0);

//     let sub2 = menu_tpl_2(menu_items!(
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (submenu_button("More sub menus"), sub3),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//     ))
//     .width(160.0);

//     let sub1 = menu_tpl_2(menu_items!(
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (submenu_button("Another sub menu"), sub2),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//     ))
//     .width(220.0);

//     menu_tpl_1(menu_items!(
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (submenu_button("A sub menu"), sub1),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//         (debug_button_f("Item")),
//     ))
//     .width(140.0)
// }));

// -----------------------------------------------------------------------------

// fn base_button<'a>(
//     content: impl Into<Element<'a, AppMsg>>,
//     msg: AppMsg,
// ) -> Button<'a, AppMsg> {
//     button(content)
//         .padding([4, 8])
//         .style(|theme, status| {
//             let palette = theme.extended_palette();
//             let base = ButtonStyle {
//                 text_color: palette.background.base.text,
//                 border: Border::default().rounded(6.0),
//                 ..ButtonStyle::default()
//             };
//             match status {
//                 ButtonStatus::Active => {
//                     base.with_background(Color::TRANSPARENT)
//                 }
//                 ButtonStatus::Hovered => base.with_background(Color::from_rgb(
//                     palette.primary.weak.color.r * 0.9,
//                     palette.primary.weak.color.g * 0.9,
//                     palette.primary.weak.color.b * 0.9,
//                 )),
//                 ButtonStatus::Disabled => {
//                     base.with_background(Color::from_rgb(0.5, 0.5, 0.5))
//                 }
//                 ButtonStatus::Pressed => {
//                     base.with_background(palette.primary.weak.color)
//                 }
//             }
//         })
//         .on_press(msg)
// }

// fn build_tooltip<'a>(
//     label: String,
//     content: impl Into<Element<'a, AppMsg>>,
// ) -> Element<'a, AppMsg> {
//     tooltip(
//         content,
//         container(txt(label).color(Color::WHITE)).padding(10).style(|theme| {
//             container::rounded_box(theme)
//                 .border(Border::default().rounded(8.0))
//                 .background(Color::from_rgb(0.2, 0.2, 0.2))
//         }),
//         TooltipPosition::Bottom,
//     )
//     .into()
// }

// fn tooltip_button<'a>(
//     label: String,
//     content: impl Into<Element<'a, AppMsg>>,
//     width: Option<Length>,
//     height: Option<Length>,
//     msg: AppMsg,
// ) -> Element<'a, AppMsg> {
//     build_tooltip(
//         label,
//         base_button(content, msg)
//             .width(width.unwrap_or(Length::Shrink))
//             .height(height.unwrap_or(Length::Shrink)),
//     )
// }

// fn debug_button(
//     label: &str,
//     width: Option<Length>,
//     height: Option<Length>,
// ) -> Element<'_, AppMsg> {
//     tooltip_button(
//         label.to_string(),
//         txt(label)
//             .height(height.unwrap_or(Length::Shrink))
//             .align_y(Vertical::Center),
//         width,
//         height,
//         AppMsg::Other(None),
//     )
// }

// fn debug_button_s(label: &str) -> Element<'_, AppMsg> {
//     debug_button(label, Some(Length::Shrink), Some(Length::Shrink))
// }

// fn debug_button_f(label: &str) -> Element<'_, AppMsg> {
//     debug_button(label, Some(Length::Fill), Some(Length::Shrink))
// }

// fn submenu_button(label: &str) -> Element<'_, AppMsg> {
//     tooltip_button(
//         label.to_string(),
//         iced_row![
//             txt(label).width(Length::Fill).align_y(Vertical::Center),
//             // iced_aw_font::right_open()
//             //     .width(Length::Shrink)
//             //     .align_y(Vertical::Center),
//         ]
//         .align_y(Alignment::Center),
//         Some(Length::Fill),
//         None,
//         AppMsg::Other(None),
//     )
// }

// -----------------------------------------------------------------------------
