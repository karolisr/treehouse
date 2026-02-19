use super::menu_model::MenuItemId;
use crate::AppMsg;
use std::fmt::Display;
use treeview::TvMsg;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppMenuItemId {
    About,
    Settings,
    OpenFile,
    SaveAs,
    Quit,
    CloseWindow,
    Find,
    ExportPdf,
    ExportSubtree,
    #[cfg(target_os = "windows")]
    RegisterFileTypes,
    #[cfg(target_os = "windows")]
    UnregisterFileTypes,
    Submenu,
    Undefined,
    ContextMenuIndex(usize),
}

impl From<String> for AppMenuItemId {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Settings" => AppMenuItemId::Settings,
            "About" => AppMenuItemId::About,
            "OpenFile" => AppMenuItemId::OpenFile,
            "SaveAs" => AppMenuItemId::SaveAs,
            "CloseWindow" => AppMenuItemId::CloseWindow,
            "Quit" => AppMenuItemId::Quit,
            "Find" => AppMenuItemId::Find,
            "ExportPdf" => AppMenuItemId::ExportPdf,
            "ExportSubtree" => AppMenuItemId::ExportSubtree,
            "Submenu" => AppMenuItemId::Submenu,
            #[cfg(target_os = "windows")]
            "RegisterFileTypes" => AppMenuItemId::RegisterFileTypes,
            #[cfg(target_os = "windows")]
            "UnregisterFileTypes" => AppMenuItemId::UnregisterFileTypes,
            // -----------------------------------------------------------------
            val if val.starts_with("ContextMenuIndex") => {
                let idx_str =
                    val.replace("ContextMenuIndex(", "").replace(")", "");
                if let Ok(idx) = idx_str.parse::<usize>() {
                    AppMenuItemId::ContextMenuIndex(idx)
                } else {
                    AppMenuItemId::Undefined
                }
            }
            // -----------------------------------------------------------------
            _ => AppMenuItemId::Undefined,
        }
    }
}

impl From<AppMenuItemId> for AppMsg {
    fn from(app_menu_item_id: AppMenuItemId) -> Self {
        (&app_menu_item_id).into()
    }
}

impl From<&AppMenuItemId> for AppMsg {
    fn from(app_menu_item_id: &AppMenuItemId) -> Self {
        match app_menu_item_id {
            AppMenuItemId::Settings => AppMsg::ShowSettings,
            AppMenuItemId::OpenFile => AppMsg::OpenFile,
            AppMenuItemId::SaveAs => AppMsg::SaveAs,
            AppMenuItemId::Quit => AppMsg::Quit,
            AppMenuItemId::CloseWindow => AppMsg::WinCloseRequested,
            AppMenuItemId::Find => AppMsg::TvMsg(TvMsg::ShowSearchBar),
            AppMenuItemId::ContextMenuIndex(idx) => {
                AppMsg::TvMsg(TvMsg::ContextMenuChosenIdx(*idx))
            }
            AppMenuItemId::ExportPdf => AppMsg::ExportPdf,
            AppMenuItemId::ExportSubtree => AppMsg::ExportSubtree,
            #[cfg(target_os = "windows")]
            AppMenuItemId::RegisterFileTypes => AppMsg::RegisterFileTypes,
            #[cfg(target_os = "windows")]
            AppMenuItemId::UnregisterFileTypes => AppMsg::UnregisterFileTypes,
            _ => AppMsg::Other(Some(app_menu_item_id.to_string())),
        }
    }
}

impl Display for AppMenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<AppMenuItemId> for MenuItemId {
    fn from(app_menu_item_id: AppMenuItemId) -> Self {
        MenuItemId { id: app_menu_item_id.to_string() }
    }
}

impl From<MenuItemId> for AppMenuItemId {
    fn from(menu_item_id: MenuItemId) -> Self {
        menu_item_id.id.into()
    }
}

impl From<MenuItemId> for AppMsg {
    fn from(menu_item_id: MenuItemId) -> Self {
        let app_menu_item_id: AppMenuItemId = menu_item_id.into();
        app_menu_item_id.into()
    }
}
