use crate::*;

pub(super) fn btn_prev_tre(enabled: bool) -> Button<'static, TvMsg> {
    btn_svg(
        Icon::ArrowLeft,
        match enabled {
            true => Some(TvMsg::PrevTre),
            false => None,
        },
    )
    .width(BTN_H2)
    .height(BTN_H2)
}

pub(super) fn btn_next_tre<'a>(enabled: bool) -> Button<'a, TvMsg> {
    btn_svg(
        Icon::ArrowRight,
        match enabled {
            true => Some(TvMsg::NextTre),
            false => None,
        },
    )
    .width(BTN_H2)
    .height(BTN_H2)
}

#[allow(dead_code)]
pub(super) fn btn_clade_highlight<'a>(
    sel_tre: Rc<TreeState>,
) -> Button<'a, TvMsg> {
    let (lab, msg) = match sel_tre.sel_node_ids().len() == 1 {
        true => {
            let &node_id = sel_tre.sel_node_ids().iter().last().unwrap();
            match sel_tre.clade_has_highlight(node_id) {
                false => (
                    "Highlight Clade",
                    Some(TvMsg::AddCladeHighlight((node_id, Clr::BLU_25))),
                ),
                true => (
                    "Remove Clade Highlight",
                    Some(TvMsg::RemoveCladeHighlight(node_id)),
                ),
            }
        }
        false => ("Highlight Clade", None),
    };
    btn_txt(lab, msg).width(BTN_H1 * 5.0)
}

pub(super) fn btn_root<'a>(sel_tre: Rc<TreeState>) -> Button<'a, TvMsg> {
    btn_txt("Root", {
        if sel_tre.sel_node_ids().len() == 1 {
            let &node_id = sel_tre.sel_node_ids().iter().last().unwrap();
            match sel_tre.is_valid_potential_outgroup_node(node_id)
                && !sel_tre.is_subtree_view_active()
            {
                true => Some(TvMsg::Root(node_id)),
                false => None,
            }
        } else {
            None
        }
    })
    .width(BTN_H1 * TWO)
}

pub(super) fn btn_unroot<'a>(sel_tre: Rc<TreeState>) -> Button<'a, TvMsg> {
    btn_txt(
        "Unroot",
        match sel_tre.is_rooted() && !sel_tre.is_subtree_view_active() {
            true => Some(TvMsg::Unroot),
            false => None,
        },
    )
    .width(BTN_H1 * TWO)
}

#[allow(dead_code)]
pub(super) fn btn_set_subtree_view<'a>(
    sel_tre: Rc<TreeState>,
) -> Button<'a, TvMsg> {
    btn_txt("Subtree", {
        if sel_tre.sel_node_ids().len() == 1 {
            let &node_id = sel_tre.sel_node_ids().iter().last().unwrap();
            match sel_tre.is_valid_potential_subtree_view_node(node_id) {
                true => Some(TvMsg::SetSubtreeView(node_id)),
                false => None,
            }
        } else {
            None
        }
    })
    .width(BTN_H1 * TWO)
}

pub(super) fn btn_clear_subtree_view<'a>(
    sel_tre: Rc<TreeState>,
) -> Button<'a, TvMsg> {
    btn_txt(
        "Close Subtree",
        match sel_tre.is_subtree_view_active() {
            true => Some(TvMsg::ClearSubtreeView),
            false => None,
        },
    )
    .width(BTN_H1 * 3.0)
}
