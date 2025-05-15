use crate::app::AppMsg;
use std::fmt::Display;
use treeview::{SidebarPos, TvMsg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppMenuItemId {
    OpenFile,
    SaveAs,
    Quit,
    CloseWindow,
    SetSideBarPositionLeft,
    SetSideBarPositionRight,
    Undefined,
}

impl From<muda::MenuItem> for AppMenuItemId {
    fn from(value: muda::MenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<muda::CheckMenuItem> for AppMenuItemId {
    fn from(value: muda::CheckMenuItem) -> Self {
        value.id().0.clone().into()
    }
}

impl From<String> for AppMenuItemId {
    fn from(value: String) -> Self {
        match value.as_str() {
            "OpenFile" => AppMenuItemId::OpenFile,
            "SaveAs" => AppMenuItemId::SaveAs,
            "CloseWindow" => AppMenuItemId::CloseWindow,
            "Quit" => AppMenuItemId::Quit,
            "SetSideBarPositionLeft" => AppMenuItemId::SetSideBarPositionLeft,
            "SetSideBarPositionRight" => AppMenuItemId::SetSideBarPositionRight,
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
            AppMenuItemId::Quit => AppMsg::WinCloseRequested,
            AppMenuItemId::CloseWindow => AppMsg::WinCloseRequested,
            AppMenuItemId::SetSideBarPositionLeft => {
                AppMsg::TvMsg(TvMsg::SetSidebarPos(SidebarPos::Left))
            }
            AppMenuItemId::SetSideBarPositionRight => {
                AppMsg::TvMsg(TvMsg::SetSidebarPos(SidebarPos::Right))
            }
            _ => AppMsg::Other(Some(value.to_string())),
        }
    }
}

impl Display for AppMenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
