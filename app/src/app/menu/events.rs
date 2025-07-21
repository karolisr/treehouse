use crate::app::AppMsg;
use std::fmt::Display;
use treeview::{SidebarPosition, TvMsg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppMenuItemId {
    OpenFile,
    SaveAs,
    Quit,
    CloseWindow,
    SideBarPosition,
    SetSideBarPositionLeft,
    SetSideBarPositionRight,
    ToggleSearchBar,
    ExportPdf,
    Undefined,
    ContextMenuIndex(usize),
}

impl From<String> for AppMenuItemId {
    fn from(value: String) -> Self {
        match value.as_str() {
            "OpenFile" => AppMenuItemId::OpenFile,
            "SaveAs" => AppMenuItemId::SaveAs,
            "CloseWindow" => AppMenuItemId::CloseWindow,
            "Quit" => AppMenuItemId::Quit,
            "SideBarPosition" => AppMenuItemId::SideBarPosition,
            "SetSideBarPositionLeft" => AppMenuItemId::SetSideBarPositionLeft,
            "SetSideBarPositionRight" => AppMenuItemId::SetSideBarPositionRight,
            "ToggleSearchBar" => AppMenuItemId::ToggleSearchBar,
            "ExportPdf" => AppMenuItemId::ExportPdf,
            // -------------------------------------------------------------------------------------
            val if val.starts_with("ContextMenuIndex") => {
                let idx_str =
                    val.replace("ContextMenuIndex(", "").replace(")", "");
                if let Ok(idx) = idx_str.parse::<usize>() {
                    AppMenuItemId::ContextMenuIndex(idx)
                } else {
                    AppMenuItemId::Undefined
                }
            }
            // -------------------------------------------------------------------------------------
            _ => AppMenuItemId::Undefined,
        }
    }
}

impl From<AppMenuItemId> for AppMsg {
    fn from(value: AppMenuItemId) -> Self {
        (&value).into()
    }
}

impl From<&AppMenuItemId> for AppMsg {
    fn from(value: &AppMenuItemId) -> Self {
        match value {
            AppMenuItemId::OpenFile => AppMsg::OpenFile,
            AppMenuItemId::SaveAs => AppMsg::SaveAs,
            AppMenuItemId::Quit => AppMsg::Quit,
            AppMenuItemId::CloseWindow => AppMsg::WinCloseRequested,
            AppMenuItemId::SetSideBarPositionLeft => {
                AppMsg::TvMsg(TvMsg::SetSidebarPos(SidebarPosition::Left))
            }
            AppMenuItemId::SetSideBarPositionRight => {
                AppMsg::TvMsg(TvMsg::SetSidebarPos(SidebarPosition::Right))
            }
            AppMenuItemId::ToggleSearchBar => {
                AppMsg::TvMsg(TvMsg::ToggleSearchBar)
            }
            AppMenuItemId::ContextMenuIndex(idx) => {
                AppMsg::TvMsg(TvMsg::ContextMenuChosenIdx(*idx))
            }
            AppMenuItemId::ExportPdf => AppMsg::ExportPdf,
            _ => AppMsg::Other(Some(value.to_string())),
        }
    }
}

impl Display for AppMenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
