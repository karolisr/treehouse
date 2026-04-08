use crate::*;

pub(super) fn toggler_cursor_line<'a>(
    enabled: bool,
    draw_cursor_line: bool,
    tre_sty: TreSty,
) -> Toggler<'a, TvMsg> {
    let lab = match tre_sty {
        TreSty::PhyGrm => "Cursor Tracking Line",
        TreSty::Fan => "Cursor Tracking Circle",
    };
    let mut tglr = toggler(lab, draw_cursor_line);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::CursorLineVisChanged);
    }
    tglr
}

pub(super) fn toggler_label_branch<'a>(
    enabled: bool,
    draw_brnch_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Branch Lengths", draw_brnch_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::BrnchLabVisChanged);
    }
    tglr
}

pub(super) fn toggler_label_int<'a>(
    enabled: bool,
    draw_int_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Internal Labels", draw_int_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::IntLabVisChanged);
    }
    tglr
}

pub(super) fn toggler_label_tip<'a>(
    enabled: bool,
    draw_tip_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Tip Labels", draw_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabVisChanged);
    }
    tglr
}

pub(super) fn toggler_label_tip_align<'a>(
    enabled: bool,
    align_tip_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Align Tip Labels", align_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabAlignOptChanged);
    }
    tglr
}

pub(super) fn toggler_label_tip_trim<'a>(
    enabled: bool,
    trim_tip_labs: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Trim Tip Labels", trim_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TipLabTrimOptChanged);
    }
    tglr
}

pub(super) fn toggler_scale_bar<'a>(
    enabled: bool,
    draw_scale_bar: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Scale Bar", draw_scale_bar);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::ScaleBarVisChanged);
    }
    tglr
}

pub(super) fn toggler_full_width_scale_bar<'a>(
    enabled: bool,
    full_width_scale_bar: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Full Width Scale Bar", full_width_scale_bar);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::FullWidthScaleBarSelected);
    }
    tglr
}

pub(super) fn toggler_root<'a>(
    enabled: bool,
    draw_root: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Root", draw_root);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::RootVisChanged);
    }
    tglr
}

pub(super) fn toggler_selection_lock<'a>(
    enabled: bool,
    selection_lock: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Selection Lock", selection_lock);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::SelectionLockChanged);
    }
    tglr
}

#[allow(dead_code)]
pub(super) fn toggler_plot<'a>(
    enabled: bool,
    show_plot: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Show Plot", show_plot);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::TogglePlot);
    }
    tglr
}

pub(super) fn toggler_ltt<'a>(
    enabled: bool,
    show_ltt: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("LTT", show_ltt);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::ToggleLtt);
    }
    tglr
}

pub(super) fn toggler_gts<'a>(
    enabled: bool,
    show_gts: bool,
) -> Toggler<'a, TvMsg> {
    let mut tglr = toggler("Geological Time Scale", show_gts);
    if enabled {
        tglr = tglr.on_toggle(TvMsg::ToggleGts);
    }
    tglr
}
