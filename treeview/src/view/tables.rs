use crate::*;

pub(super) fn table_node_data<'a>(
    tv: &'a TreeView,
    w: Float,
    h: Float,
) -> Element<'a, TvMsg> {
    if let Some(ts) = tv.sel_tre() {
        iced_row![
            nodes_table(tv, ts.clone(), w / TWO, h),
            attributes_table(tv, ts.clone(), w / TWO, h)
        ]
        .into()
    } else {
        txt("No tree loaded").into()
    }
}

// pub(super) fn table_nodes<'a>(
//     tv: &'a TreeView,
//     w: Float,
//     h: Float,
// ) -> Element<'a, TvMsg> {
//     if let Some(ts) = tv.sel_tre() {
//         nodes_table(tv, ts, w, h)
//     } else {
//         txt("No tree loaded").into()
//     }
// }

// pub(super) fn table_attributes<'a>(
//     tv: &'a TreeView,
//     w: Float,
//     h: Float,
// ) -> Element<'a, TvMsg> {
//     if let Some(ts) = tv.sel_tre() {
//         attributes_table(tv, ts, w, h)
//     } else {
//         txt("No tree loaded").into()
//     }
// }
