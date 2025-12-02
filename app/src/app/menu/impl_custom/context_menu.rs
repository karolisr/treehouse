use riced::Button;
use riced::Column;
use riced::Container;
use riced::Element;
use riced::PADDING;
use riced::Point;
use riced::SF;
use riced::Task;
use riced::WindowId;
use riced::btn_txt;
use riced::container;
use riced::context_menu_element;
use riced::sty_cont_message;

use treeview::TvContextMenuListing;

use crate::AppMsg;

use super::AppMenu;
use super::AppMenuAction;
use super::MenuItem;

pub fn show_context_menu(
    tree_view_context_menu_listing: TvContextMenuListing,
    _winid: WindowId,
) -> Task<AppMsg> {
    Task::done(AppMsg::SetCustomContextMenu(
        tree_view_context_menu_listing.into(),
    ))
}

#[derive(Default, Clone, Debug)]
pub struct ContextMenu {
    menu: AppMenu,
    position: Point,
}

impl ContextMenu {
    pub fn new(items: Vec<MenuItem>, position: Point) -> Self {
        let menu = AppMenu::with_items(items);
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
        let mut btns: Vec<Element<'_, AppMsg>> = Vec::new();
        for item in &self.menu.items {
            let btn: Button<'_, AppMsg> = btn_txt(
                &item.label,
                match item.enabled {
                    true => Some(AppMsg::MenuEvent(item.id.clone())),
                    false => None,
                },
            );
            btns.push(btn.width(SF * 100.0).into());
        }
        container(Column::from_vec(btns).spacing(PADDING).padding(PADDING))
            .style(sty_cont_message)
    }
}

impl From<TvContextMenuListing> for ContextMenu {
    fn from(tv_context_menu_listing: TvContextMenuListing) -> Self {
        let mut menu_items: Vec<MenuItem> = Vec::new();
        tv_context_menu_listing.items().iter().enumerate().for_each(
            |(idx, item)| {
                let menu_item = MenuItem::new(
                    AppMenuAction::ContextMenuIndex(idx),
                    item.label.clone(),
                    item.enabled,
                );
                menu_items.push(menu_item);
            },
        );
        ContextMenu::new(menu_items, tv_context_menu_listing.position())
    }
}
