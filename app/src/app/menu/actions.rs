use crate::app::AppMsg;
use std::fmt::Display;
use treeview::TvMsg;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppMenuAction {
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
    Undefined,
    ContextMenuIndex(usize),
}

impl From<String> for AppMenuAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "OpenFile" => AppMenuAction::OpenFile,
            "SaveAs" => AppMenuAction::SaveAs,
            "CloseWindow" => AppMenuAction::CloseWindow,
            "Quit" => AppMenuAction::Quit,
            "ToggleSearchBar" => AppMenuAction::ToggleSearchBar,
            "ExportPdf" => AppMenuAction::ExportPdf,
            "ExportSubtree" => AppMenuAction::ExportSubtree,
            #[cfg(target_os = "windows")]
            "RegisterFileTypes" => AppMenuAction::RegisterFileTypes,
            #[cfg(target_os = "windows")]
            "UnregisterFileTypes" => AppMenuAction::UnregisterFileTypes,
            // -------------------------------------------------------------------------------------
            val if val.starts_with("ContextMenuIndex") => {
                let idx_str =
                    val.replace("ContextMenuIndex(", "").replace(")", "");
                if let Ok(idx) = idx_str.parse::<usize>() {
                    AppMenuAction::ContextMenuIndex(idx)
                } else {
                    AppMenuAction::Undefined
                }
            }
            // -------------------------------------------------------------------------------------
            _ => AppMenuAction::Undefined,
        }
    }
}

impl From<AppMenuAction> for AppMsg {
    fn from(value: AppMenuAction) -> Self {
        (&value).into()
    }
}

impl From<&AppMenuAction> for AppMsg {
    fn from(value: &AppMenuAction) -> Self {
        match value {
            AppMenuAction::OpenFile => AppMsg::OpenFile,
            AppMenuAction::SaveAs => AppMsg::SaveAs,
            AppMenuAction::Quit => AppMsg::Quit,
            AppMenuAction::CloseWindow => AppMsg::WinCloseRequested,
            AppMenuAction::ToggleSearchBar => {
                AppMsg::TvMsg(TvMsg::ToggleSearchBar)
            }
            AppMenuAction::ContextMenuIndex(idx) => {
                AppMsg::TvMsg(TvMsg::ContextMenuChosenIdx(*idx))
            }
            AppMenuAction::ExportPdf => AppMsg::ExportPdf,
            AppMenuAction::ExportSubtree => AppMsg::ExportSubtree,
            #[cfg(target_os = "windows")]
            AppMenuAction::RegisterFileTypes => AppMsg::RegisterFileTypes,
            #[cfg(target_os = "windows")]
            AppMenuAction::UnregisterFileTypes => AppMsg::UnregisterFileTypes,
            _ => AppMsg::Other(Some(value.to_string())),
        }
    }
}

impl Display for AppMenuAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
