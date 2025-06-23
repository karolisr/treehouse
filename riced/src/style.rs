use crate::*;

pub(crate) fn sty_checkbox(theme: &Theme, status: CheckboxStatus) -> CheckboxStyle {
    let palette = theme.extended_palette();

    fn styled(
        icon_color: Color, border_color: Color, base: PalettePair, accent: PalettePair,
        is_checked: bool,
    ) -> CheckboxStyle {
        CheckboxStyle {
            background: Background::Color(if is_checked { accent.color } else { base.color }),
            icon_color,
            border: Border {
                radius: WIDGET_RADIUS.into(),
                width: BORDER_W,
                color: if is_checked { accent.color } else { border_color },
            },
            text_color: None,
        }
    }

    match status {
        CheckboxStatus::Active { is_checked } => styled(
            palette.primary.strong.text, palette.background.strongest.color,
            palette.background.base, palette.primary.base, is_checked,
        ),
        CheckboxStatus::Hovered { is_checked } => styled(
            palette.primary.strong.text, palette.background.strongest.color,
            palette.background.weak, palette.primary.strong, is_checked,
        ),
        CheckboxStatus::Disabled { is_checked } => styled(
            palette.primary.strong.text, palette.background.weak.color, palette.background.weak,
            palette.background.strong, is_checked,
        ),
    }
}

pub(crate) fn sty_text_input(theme: &Theme, status: TextInputStatus) -> TextInputStyle {
    let pe = theme.extended_palette();

    let active = TextInputStyle {
        background: Background::Color(pe.background.base.color),
        border: Border {
            radius: WIDGET_RADIUS.into(),
            width: BORDER_W,
            color: pe.background.strongest.color,
        },
        icon: pe.background.weak.text,
        placeholder: pe.background.strongest.color,
        value: pe.background.base.text,
        selection: pe.primary.weak.color,
    };

    match status {
        TextInputStatus::Active => active,
        TextInputStatus::Hovered => TextInputStyle {
            border: Border { color: pe.background.base.text, ..active.border },
            ..active
        },
        TextInputStatus::Focused { .. } => TextInputStyle {
            border: Border { color: pe.primary.strong.color, ..active.border },
            ..active
        },
        TextInputStatus::Disabled => TextInputStyle {
            background: Background::Color(pe.background.weak.color),
            value: active.placeholder,
            ..active
        },
    }
}

pub(crate) fn sty_svg(theme: &Theme, status: SvgStatus) -> SvgStyle {
    let pe = theme.extended_palette();
    let color = match status {
        SvgStatus::Idle => pe.background.base.color,
        SvgStatus::Hovered => pe.background.base.color,
    };
    SvgStyle { color: Some(color) }
}

pub fn sty_cont(theme: &Theme) -> ContainerStyle {
    let pb = theme.palette();
    ContainerStyle {
        text_color: Some(pb.text),
        background: Some(Background::Color(Clr::BLK.scale_alpha(0.02))),
        border: Border {
            width: BORDER_W,
            color: Clr::BLK.scale_alpha(0.15),
            radius: WIDGET_RADIUS.into(),
        },
        shadow: Shadow {
            color: Clr::BLK_25,
            offset: Vector { x: ZERO, y: ZERO },
            blur_radius: PADDING - PADDING / FOUR,
        },
        snap: cfg!(feature = "crisp"),
    }
}

pub fn sty_cont_bottom(theme: &Theme) -> ContainerStyle {
    let base = sty_cont(theme);
    ContainerStyle {
        border: Border {
            radius: Radius {
                bottom_right: MAC_OS_WINDOW_BORDER_RADIUS,
                bottom_left: MAC_OS_WINDOW_BORDER_RADIUS,
                ..base.border.radius
            },
            ..base.border
        },
        ..base
    }
}

pub fn sty_cont_bottom_left(theme: &Theme) -> ContainerStyle {
    let base = sty_cont(theme);
    ContainerStyle {
        border: Border {
            radius: Radius { bottom_left: MAC_OS_WINDOW_BORDER_RADIUS, ..base.border.radius },
            ..base.border
        },
        ..base
    }
}

pub fn sty_cont_bottom_right(theme: &Theme) -> ContainerStyle {
    let base = sty_cont(theme);
    ContainerStyle {
        border: Border {
            radius: Radius { bottom_right: MAC_OS_WINDOW_BORDER_RADIUS, ..base.border.radius },
            ..base.border
        },
        ..base
    }
}

pub fn sty_cont_tool_bar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub fn sty_cont_search_bar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }

pub fn sty_pane_grid(theme: &Theme) -> PgStyle {
    let pe = theme.extended_palette();
    PgStyle {
        hovered_region: PgHighlight {
            background: Clr::GRN_25.into(),
            border: Border { width: BORDER_W, color: pe.primary.strong.color, radius: ZERO.into() },
        },
        hovered_split: PgLine { color: pe.primary.base.color, width: SF * TWO },
        picked_split: PgLine { color: pe.primary.strong.color, width: SF * TWO },
    }
}

pub fn sty_pane_titlebar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub fn sty_pane_body(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub fn sty_pane_body_bottom(theme: &Theme) -> ContainerStyle { sty_cont_bottom(theme) }

pub(crate) fn sty_btn(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let ep = theme.extended_palette();

    let base = ButtonStyle {
        background: Some(Background::Color(ep.primary.base.color)),
        text_color: ep.primary.base.text,
        border: Border { radius: WIDGET_RADIUS.into(), width: BORDER_W, color: Clr::TRN },
        ..ButtonStyle::default()
    };

    match status {
        ButtonStatus::Active | ButtonStatus::Pressed => base,
        ButtonStatus::Hovered => {
            ButtonStyle { background: Some(Background::Color(ep.primary.strong.color)), ..base }
        }
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
        border: Border {
            radius: WIDGET_RADIUS.into(),
            width: BORDER_W,
            color: palette.background.strong.color,
        },
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
        width: BORDER_W as u16,
        radius: WIDGET_RADIUS.into(),
        fill_mode: RuleFillMode::Percent(1e2),
        snap: true,
    }
}

pub(crate) fn sty_scrlbl(theme: &Theme, status: ScrollableStatus) -> ScrollableStyle {
    let palette = theme.extended_palette();

    let scrollbar = ScrollBarRail {
        background: Some(palette.background.weak.color.into()),
        border: Border { radius: WIDGET_RADIUS.into(), width: ZERO, color: Clr::TRN },
        scroller: Scroller {
            color: palette.background.strong.color,
            border: Border { radius: WIDGET_RADIUS.into(), width: ZERO, color: Clr::TRN },
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
            backgrounds: (color.into(), palette.background.strong.color.into()),
            width: (SLIDER_H + SF * TWO) / THREE,
            border: Border { radius: WIDGET_RADIUS.into(), width: BORDER_W, color: Clr::TRN },
        },

        handle: SliderHandle {
            shape: SliderHandleShape::Rectangle {
                width: SLIDER_H as u16,
                border_radius: WIDGET_RADIUS.into(),
            },
            background: color.into(),
            border_color: Clr::TRN,
            border_width: BORDER_W,
        },
    }
}

pub(crate) fn sty_toggler(theme: &Theme, status: TogglerStatus) -> TogglerStyle {
    let palette = theme.extended_palette();

    let background = match status {
        TogglerStatus::Active { is_toggled } | TogglerStatus::Hovered { is_toggled } => {
            if is_toggled {
                palette.primary.strong.color
            } else {
                palette.background.strong.color
            }
        }
        TogglerStatus::Disabled => palette.background.weak.color,
    };

    let foreground = match status {
        TogglerStatus::Active { is_toggled } => {
            if is_toggled {
                palette.primary.strong.text
            } else {
                palette.background.base.color
            }
        }
        TogglerStatus::Hovered { is_toggled } => {
            if is_toggled {
                Color { a: 0.75, ..palette.primary.strong.text }
            } else {
                palette.background.weak.color
            }
        }
        TogglerStatus::Disabled => palette.background.base.color,
    };

    TogglerStyle {
        foreground,
        background,
        foreground_border_width: SF * TWO,
        background_border_width: ZERO,
        foreground_border_color: Clr::TRN,
        background_border_color: Clr::TRN,
        border_radius: WIDGET_RADIUS,
    }
}
