use crate::app::AppMsg;
use dendros::NodeId;
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
    Undefined,
    Root,
    AddCladeLabel,
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
            // -------------------------------------------------------------------------------------
            "Root" => AppMenuItemId::Root,
            "AddCladeLabel" => AppMenuItemId::AddCladeLabel,
            // -------------------------------------------------------------------------------------
            _ => AppMenuItemId::Undefined,
        }
    }
}

impl From<AppMenuItemId> for AppMsg {
    fn from(value: AppMenuItemId) -> Self { (&value).into() }
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
            AppMenuItemId::ToggleSearchBar => AppMsg::TvMsg(TvMsg::ToggleSearchBar),
            AppMenuItemId::Root => AppMsg::Other(Some("Root".to_string())),
            AppMenuItemId::AddCladeLabel => AppMsg::Other(Some("AddCladeLabel".to_string())),
            _ => AppMsg::Other(Some(value.to_string())),
        }
    }
}

impl Display for AppMenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:?}") }
}
