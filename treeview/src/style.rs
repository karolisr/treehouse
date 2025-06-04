use crate::iced::*;
use crate::*;

pub(crate) fn sty_cont(theme: &Theme) -> ContainerStyle {
    let pb = theme.palette();
    // let pe = theme.extended_palette();
    ContainerStyle {
        text_color: Some(pb.text),
        background: None,
        border: Border { width: ONE, color: Clr::BLK, radius: 0.into() },
        ..Default::default()
    }
}

pub(crate) fn sty_cont_main(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub(crate) fn sty_cont_sidebar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub(crate) fn sty_cont_toolbar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub(crate) fn sty_cont_statusbar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }

pub(crate) fn sty_pane_grid(theme: &Theme) -> PgStyle {
    let pe = theme.extended_palette();
    PgStyle {
        hovered_region: PgHighlight {
            background: Clr::GRN_25.into(),
            border: Border { width: ONE, color: pe.primary.strong.color, radius: 0.into() },
        },
        hovered_split: PgLine { color: pe.primary.base.color, width: ONE },
        picked_split: PgLine { color: pe.primary.strong.color, width: ONE },
    }
}

pub(crate) fn sty_pane_titlebar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub(crate) fn sty_pane_body(theme: &Theme) -> ContainerStyle { sty_cont(theme) }

pub(crate) fn sty_btn(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let palette = theme.extended_palette();

    let base = ButtonStyle {
        background: Some(Background::Color(palette.primary.base.color)),
        text_color: palette.primary.base.text,
        border: Border { radius: ZRO.into(), width: ZRO, ..Default::default() },
        ..ButtonStyle::default()
    };

    match status {
        ButtonStatus::Active | ButtonStatus::Pressed => base,
        ButtonStatus::Hovered => ButtonStyle {
            background: Some(Background::Color(palette.primary.strong.color)),
            ..base
        },
        ButtonStatus::Disabled => ButtonStyle {
            background: base.background.map(|background| background.scale_alpha(0.5)),
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}

pub(crate) fn sty_pick_lst(theme: &Theme, status: PickListStatus) -> PickListStyle {
    let palette = theme.extended_palette();

    let active = PickListStyle {
        text_color: palette.background.weak.text,
        background: palette.background.weak.color.into(),
        placeholder_color: palette.background.strong.color,
        handle_color: palette.background.weak.text,
        border: Border { radius: ZRO.into(), width: ONE, color: palette.background.strong.color },
    };

    match status {
        PickListStatus::Active => active,
        PickListStatus::Hovered | PickListStatus::Opened { .. } => PickListStyle {
            border: Border { color: palette.primary.strong.color, ..active.border },
            ..active
        },
    }
}

pub(crate) fn sty_rule(theme: &Theme) -> RuleStyle {
    let palette = theme.extended_palette();
    RuleStyle {
        color: palette.background.strong.color,
        width: ONE as u16,
        radius: ZRO.into(),
        fill_mode: RuleFillMode::Percent(1e2),
        snap: true,
    }
}

pub(crate) fn sty_scrlbl(theme: &Theme, status: ScrollableStatus) -> ScrollableStyle {
    let palette = theme.extended_palette();

    let scrollbar = ScrollBarRail {
        background: Some(palette.background.weak.color.into()),
        border: Border { radius: ZRO.into(), width: ONE, ..Default::default() },
        scroller: Scroller {
            color: palette.background.strong.color,
            border: Border { radius: ZRO.into(), width: ONE, ..Default::default() },
        },
    };

    match status {
        ScrollableStatus::Active { .. } => ScrollableStyle {
            container: container::Style::default(),
            vertical_rail: scrollbar,
            horizontal_rail: scrollbar,
            gap: None,
        },

        ScrollableStatus::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } => {
            let hovered_scrollbar = ScrollBarRail {
                scroller: Scroller { color: palette.primary.strong.color, ..scrollbar.scroller },
                ..scrollbar
            };

            ScrollableStyle {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }

        ScrollableStatus::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            ..
        } => {
            let dragged_scrollbar = ScrollBarRail {
                scroller: Scroller { color: palette.primary.base.color, ..scrollbar.scroller },
                ..scrollbar
            };

            ScrollableStyle {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
    }
}

pub(crate) fn sty_slider(theme: &Theme, status: SliderStatus) -> SliderStyle {
    let palette = theme.extended_palette();

    let color = match status {
        SliderStatus::Active => palette.primary.base.color,
        SliderStatus::Hovered => palette.primary.strong.color,
        SliderStatus::Dragged => palette.primary.weak.color,
    };

    SliderStyle {
        rail: SliderRail {
            backgrounds: (Clr::WHT.into(), Clr::WHT.into()),
            width: 6e0,
            border: Border { radius: ZRO.into(), width: ONE * SF, color: Clr::BLK },
        },

        handle: SliderHandle {
            shape: SliderHandleShape::Circle { radius: 13.0 / 1.75 },
            // shape: SliderHandleShape::Rectangle { width: (13.0 * 1.3) as u16, border_radius: ZRO.into() },
            background: color.into(),
            border_color: Clr::BLK,
            border_width: ONE * SF,
        },
    }
}

pub(crate) fn sty_toggler(theme: &Theme, status: TogglerStatus) -> TogglerStyle {
    let palette = theme.extended_palette();
    let color = match status {
        TogglerStatus::Active { is_toggled } => match is_toggled {
            true => palette.primary.base.color,
            false => palette.primary.base.color,
        },
        TogglerStatus::Hovered { is_toggled } => match is_toggled {
            true => palette.primary.strong.color,
            false => palette.primary.strong.color,
        },
        TogglerStatus::Disabled => palette.secondary.base.color,
    };

    TogglerStyle {
        background: Clr::WHT,
        background_border_width: ONE,
        background_border_color: Clr::BLK,
        foreground: color,
        foreground_border_width: ZRO,
        foreground_border_color: Clr::TRN,
    }
}
