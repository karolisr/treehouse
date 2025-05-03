use super::style::{sty_btn, sty_pick_lst, sty_rule, sty_scrlbl, sty_slider};
use crate::{TEXT_SIZE, TreeView, TreeViewMsg};
use iced::{
    Alignment, Font,
    Length::Fill,
    Pixels, Theme,
    widget::{
        PickList, Rule, Scrollable, Slider, Toggler, button::Button,
        pick_list::Handle as PickListHandle,
    },
};
use std::{
    clone::Clone,
    cmp::{PartialEq, PartialOrd},
    convert::From,
    fmt::Display,
    marker::Copy,
};

// ------------------------------------------------------------------------------------------------

impl TreeView {
    pub(crate) fn apply_settings_btn<'a>(
        &'a self,
        btn: Button<'a, TreeViewMsg>,
    ) -> Button<'a, TreeViewMsg> {
        let mut btn = btn;
        btn = btn.style(sty_btn);
        btn
    }

    pub(crate) fn apply_settings_pick_list<'a, T: PartialEq + Display + Clone>(
        &'a self,
        pl: PickList<'a, T, &[T], T, TreeViewMsg>,
    ) -> PickList<'a, T, &'a [T], T, TreeViewMsg> {
        let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TEXT_SIZE)) };
        let mut pl = pl;
        pl = pl.handle(h);
        pl = pl.style(sty_pick_lst);
        pl
    }

    pub(crate) fn apply_settings_rule<'a>(&'a self, rule: Rule<'a, Theme>) -> Rule<'a, Theme> {
        let rule = rule.style(sty_rule);
        rule
    }

    pub(crate) fn apply_settings_scroll<'a>(
        &'a self,
        scrl: Scrollable<'a, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut scrl = scrl;
        scrl = scrl.style(sty_scrlbl);
        scrl
    }

    pub(crate) fn apply_settings_slider<'a, T>(
        &'a self,
        sldr: Slider<'a, T, TreeViewMsg>,
    ) -> Slider<'a, T, TreeViewMsg>
    where
        T: Copy,
        T: From<u8>,
        T: PartialOrd,
    {
        let mut sldr = sldr;
        sldr = sldr.style(sty_slider);
        sldr
    }

    pub(crate) fn apply_settings_toggler<'a>(
        &self,
        tglr: Toggler<'a, TreeViewMsg>,
    ) -> Toggler<'a, TreeViewMsg> {
        let mut tglr = tglr;
        tglr = tglr.text_size(TEXT_SIZE);
        tglr = tglr.size(TEXT_SIZE * 1.5);
        tglr = tglr.text_alignment(Alignment::End);
        tglr = tglr.width(Fill);
        tglr
    }
}

// ------------------------------------------------------------------------------------------------
