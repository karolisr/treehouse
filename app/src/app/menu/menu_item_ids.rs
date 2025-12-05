use crate::app::AppMsg;
use std::fmt::Display;
use treeview::TvMsg;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum MenuItemId {
    OpenFile,
    SaveAs,
    Quit,
    CloseWindow,
    ToggleSearchBar,
    ExportPdf,
    ExportSubtree,
    #[cfg(target_os = "windows")]
    RegisterFileTypes,
    #[cfg(target_os = "windows")]
    UnregisterFileTypes,
    Separator,
    Submenu,
    Undefined,
    ContextMenuIndex(usize),
}

impl From<String> for MenuItemId {
    fn from(value: String) -> Self {
        match value.as_str() {
            "OpenFile" => MenuItemId::OpenFile,
            "SaveAs" => MenuItemId::SaveAs,
            "CloseWindow" => MenuItemId::CloseWindow,
            "Quit" => MenuItemId::Quit,
            "ToggleSearchBar" => MenuItemId::ToggleSearchBar,
            "ExportPdf" => MenuItemId::ExportPdf,
            "ExportSubtree" => MenuItemId::ExportSubtree,
            #[cfg(target_os = "windows")]
            "RegisterFileTypes" => MenuItemId::RegisterFileTypes,
            #[cfg(target_os = "windows")]
            "UnregisterFileTypes" => MenuItemId::UnregisterFileTypes,
            "Separator" => MenuItemId::Separator,
            "Submenu" => MenuItemId::Submenu,
            // -----------------------------------------------------------------
            val if val.starts_with("ContextMenuIndex") => {
                let idx_str =
                    val.replace("ContextMenuIndex(", "").replace(")", "");
                if let Ok(idx) = idx_str.parse::<usize>() {
                    MenuItemId::ContextMenuIndex(idx)
                } else {
                    MenuItemId::Undefined
                }
            }
            // -----------------------------------------------------------------
            _ => MenuItemId::Undefined,
        }
    }
}

impl From<MenuItemId> for AppMsg {
    fn from(value: MenuItemId) -> Self {
        (&value).into()
    }
}

impl From<&MenuItemId> for AppMsg {
    fn from(value: &MenuItemId) -> Self {
        match value {
            MenuItemId::OpenFile => AppMsg::OpenFile,
            MenuItemId::SaveAs => AppMsg::SaveAs,
            MenuItemId::Quit => AppMsg::Quit,
            MenuItemId::CloseWindow => AppMsg::WinCloseRequested,
            MenuItemId::ToggleSearchBar => {
                AppMsg::TvMsg(TvMsg::ToggleSearchBar)
            }
            MenuItemId::ContextMenuIndex(idx) => {
                AppMsg::TvMsg(TvMsg::ContextMenuChosenIdx(*idx))
            }
            MenuItemId::ExportPdf => AppMsg::ExportPdf,
            MenuItemId::ExportSubtree => AppMsg::ExportSubtree,
            #[cfg(target_os = "windows")]
            MenuItemId::RegisterFileTypes => AppMsg::RegisterFileTypes,
            #[cfg(target_os = "windows")]
            MenuItemId::UnregisterFileTypes => AppMsg::UnregisterFileTypes,
            _ => AppMsg::Other(Some(value.to_string())),
        }
    }
}

impl Display for MenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
