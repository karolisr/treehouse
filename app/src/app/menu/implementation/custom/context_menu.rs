use riced::BTN_H_MENU;
use riced::Button;
use riced::Column;
use riced::Container;
use riced::Element;
use riced::PADDING;
use riced::Point;
use riced::SF;
use riced::Task;
use riced::Vector;
use riced::WindowId;
use riced::container;
use riced::context_menu_element;
use riced::horizontal_rule;
use riced::sty_cont_context_menu;

use treeview::TvContextMenuSpecification;

use crate::AppMsg;

use super::super::super::AppMenuItemId;
use super::super::super::menu_model::Menu;
use super::super::super::menu_model::MenuItem;

use super::ui::btn_menu_item;

pub fn show_tv_context_menu(
    specification: TvContextMenuSpecification,
    _window_id: WindowId,
) -> Task<AppMsg> {
    let offset = -3e0 * PADDING - BTN_H_MENU;
    let spec = specification.set_position_offset(Vector { x: 0.0, y: offset });
    Task::done(AppMsg::SetCustomContextMenu(spec.into()))
}

#[derive(Default, Clone, Debug)]
pub struct ContextMenu {
    menu: Menu,
    position: Point,
}

impl ContextMenu {
    pub fn new(items: Vec<MenuItem>, position: Point) -> Self {
        let menu = Menu::with_items(items);
        Self { menu, position }
    }

    pub fn element<'a>(
        &'a self,
        base: Element<'a, AppMsg>,
    ) -> Element<'a, AppMsg> {
        context_menu_element(
            base,
            self.context_menu_container(),
            self.position,
            AppMsg::HideContextMenu,
        )
    }

    fn context_menu_container(&self) -> Container<'_, AppMsg> {
        let mut items: Vec<Element<'_, AppMsg>> = Vec::new();
        for item in self.menu.items() {
            match item {
                MenuItem::Item { id, label, enabled, .. } => {
                    let btn: Button<'_, AppMsg> = btn_menu_item(
                        label.clone(),
                        None,
                        match enabled {
                            true => Some(AppMsg::MenuEvent(id.clone().into())),
                            false => None,
                        },
                    );
                    items.push(btn.into());
                }
                MenuItem::Submenu { .. } => todo!(),
                MenuItem::Separator => {
                    items.push(horizontal_rule(SF).into());
                }
            }
        }
        let cm_column = Column::from_vec(items);
        container(cm_column).style(sty_cont_context_menu).padding(PADDING)
    }
}

impl From<TvContextMenuSpecification> for ContextMenu {
    fn from(tv_context_menu_listing: TvContextMenuSpecification) -> Self {
        let mut menu_items: Vec<MenuItem> = Vec::new();
        tv_context_menu_listing.items().iter().enumerate().for_each(
            |(idx, item)| {
                let menu_item = MenuItem::item(
                    item.label.clone(),
                    item.enabled,
                    AppMenuItemId::ContextMenuIndex(idx),
                    None,
                );
                menu_items.push(menu_item);
            },
        );
        ContextMenu::new(menu_items, tv_context_menu_listing.position())
    }
}
