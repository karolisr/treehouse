use crate::consts::icons::*;
use crate::*;

pub enum Icon {
    ArrowLeft,
    ArrowRight,
    SidebarLeft,
    SidebarRight,
    ShowPlot,
    HidePlot,
    AddToSelection,
    RemoveFromSelection,
    ShowSearch,
    HideSearch,
}

impl From<Icon> for SvgHandle {
    fn from(icon: Icon) -> Self {
        fn i(bytes: &'static [u8]) -> SvgHandle {
            SvgHandle::from_memory(bytes)
        }
        match icon {
            Icon::ArrowLeft => i(&ARROW_LEFT_ALT),
            Icon::ArrowRight => i(&ARROW_RIGHT_ALT),
            Icon::SidebarLeft => i(&DOCK_TO_RIGHT),
            Icon::SidebarRight => i(&DOCK_TO_LEFT),
            Icon::ShowPlot => i(&DOCK_TO_BOTTOM),
            Icon::HidePlot => i(&CHECK_BOX_OUTLINE_BLANK),
            Icon::AddToSelection => i(&PLAYLIST_ADD),
            Icon::RemoveFromSelection => i(&PLAYLIST_REMOVE),
            Icon::ShowSearch => i(&SEARCH),
            Icon::HideSearch => i(&SEARCH_OFF),
        }
    }
}
