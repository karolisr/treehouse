use crate::consts::icons::*;
use crate::*;

#[derive(Debug)]
pub enum Icon {
    ArrowLeft,
    ArrowRight,
    Plot,
    AddToSelection,
    RemoveFromSelection,
    Search,
    DataTable,
}

impl From<Icon> for SvgHandle {
    fn from(icon: Icon) -> Self {
        fn i(bytes: &'static [u8]) -> SvgHandle {
            SvgHandle::from_memory(bytes)
        }
        match icon {
            Icon::ArrowLeft => i(&ARROW_LEFT_ALT),
            Icon::ArrowRight => i(&ARROW_RIGHT_ALT),
            Icon::Plot => i(&AREA_CHART),
            Icon::AddToSelection => i(&PLAYLIST_ADD),
            Icon::RemoveFromSelection => i(&PLAYLIST_REMOVE),
            Icon::Search => i(&SEARCH),
            Icon::DataTable => i(&TABLE_ROWS_NARROW),
        }
    }
}
