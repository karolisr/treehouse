use crate::{BORDER_W, RADIUS_WIDGET, SF, TEXT_SIZE};
use iced::{
    Background, Border, Color, Theme,
    widget::{
        button::{Status as ButtonStatus, Style as ButtonStyle},
        container::{self, Style as ContainerStyle},
        pane_grid::{Line, Style as PaneGridStyle},
        pick_list::{Status as PickListStatus, Style as PickListStyle},
        rule::{FillMode as RuleFillMode, Style as RuleStyle},
        scrollable::{
            Rail as ScrollBarRail, Scroller, Status as ScrollableStatus, Style as ScrollableStyle,
        },
        slider::{
            Handle as SliderHandle, HandleShape as SliderHandleShape, Rail as SliderRail,
            Status as SliderStatus, Style as SliderStyle,
        },
    },
};

// ------------------------------------------------------------------------------------------------

pub(crate) fn sty_cont(theme: &Theme) -> ContainerStyle {
    let pb = theme.palette();
    let pe = theme.extended_palette();
    ContainerStyle {
        text_color: Some(pb.text),
        background: Some(pe.background.weakest.color.into()),
        ..Default::default()
    }
}

pub(crate) fn sty_cont_main(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
}

pub(crate) fn sty_cont_sidebar(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
}

pub(crate) fn sty_cont_toolbar(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
}

pub(crate) fn sty_cont_statusbar(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
}

// ------------------------------------------------------------------------------------------------

pub(crate) fn sty_pane_grid(theme: &Theme) -> PaneGridStyle {
    PaneGridStyle {
        hovered_split: Line { color: theme.extended_palette().secondary.weak.color, width: 2e0 },
        picked_split: Line { color: theme.extended_palette().secondary.weak.color, width: 2e0 },
        ..iced::widget::pane_grid::default(theme)
    }
}

// ------------------------------------------------------------------------------------------------

pub(crate) fn sty_btn(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let palette = theme.extended_palette();

    let base = ButtonStyle {
        background: Some(Background::Color(palette.primary.base.color)),
        text_color: palette.primary.base.text,
        border: Border { radius: RADIUS_WIDGET.into(), width: BORDER_W, ..Default::default() },
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
        border: Border {
            radius: RADIUS_WIDGET.into(),
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
        radius: RADIUS_WIDGET.into(),
        fill_mode: RuleFillMode::Percent(1e2),
    }
}

pub(crate) fn sty_scrlbl(theme: &Theme, status: ScrollableStatus) -> ScrollableStyle {
    let palette = theme.extended_palette();

    let scrollbar = ScrollBarRail {
        background: Some(palette.background.weak.color.into()),
        border: Border { radius: RADIUS_WIDGET.into(), width: BORDER_W, ..Default::default() },
        scroller: Scroller {
            color: palette.background.strong.color,
            border: Border { radius: RADIUS_WIDGET.into(), width: BORDER_W, ..Default::default() },
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
            width: TEXT_SIZE / 3e0,
            border: Border {
                radius: RADIUS_WIDGET.into(),
                width: 0e0 * SF,
                color: Color::TRANSPARENT,
            },
        },

        handle: SliderHandle {
            shape: SliderHandleShape::Circle { radius: TEXT_SIZE / 1.75 },
            // shape: SliderHandleShape::Rectangle {
            //     width: TEXT_SIZE as u16,
            //     border_radius: RADIUS_WIDGET.into(),
            // },
            background: color.into(),
            border_color: Color::TRANSPARENT,
            border_width: 0e0 * SF,
        },
    }
}

// ------------------------------------------------------------------------------------------------
