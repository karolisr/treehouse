use iced::{Background, Border, Theme};
use utils::Clr;

const SF: f32 = 1e0;
const TEXT_SIZE: f32 = 13.0 * SF;
const BORDER_W: f32 = SF;
const RADIUS_WIDGET: f32 = 0e0 * SF;

// ------------------------------------------------------------------------------------------------
use iced::widget::container::{self, Style as ContainerStyle};
pub(crate) fn sty_cont(theme: &Theme) -> ContainerStyle {
    let pb = theme.palette();
    // let pe = theme.extended_palette();
    ContainerStyle {
        text_color: Some(pb.text),
        background: None,
        border: Border { width: 1e0, color: Clr::BLK, radius: RADIUS_WIDGET.into() },
        ..Default::default()
    }
}

pub(crate) fn sty_cont_main(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
    // .background(Clr::CYA)
}

pub(crate) fn sty_cont_sidebar(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
    // .background(Clr::YEL)
}

pub(crate) fn sty_cont_toolbar(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
    // .background(Clr::GRN)
}

pub(crate) fn sty_cont_statusbar(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
    // .background(Clr::MAG)
}
// ------------------------------------------------------------------------------------------------
use iced::widget::pane_grid::{
    Highlight as PaneGridHighlight, Line as PaneGridLine, Style as PaneGridStyle,
};
pub(crate) fn sty_pane_grid(theme: &Theme) -> PaneGridStyle {
    let pe = theme.extended_palette();
    PaneGridStyle {
        hovered_region: PaneGridHighlight {
            background: Clr::BLK.into(),
            border: Border {
                width: 1e0,
                color: pe.primary.strong.color,
                radius: RADIUS_WIDGET.into(),
            },
        },
        hovered_split: PaneGridLine { color: pe.primary.base.color, width: 2.0 },
        picked_split: PaneGridLine { color: pe.primary.strong.color, width: 2.0 },
    }
}

pub(crate) fn sty_pane_titlebar(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
    // .background(Clr::RED)
}

pub(crate) fn sty_pane_body(theme: &Theme) -> ContainerStyle {
    sty_cont(theme)
    // .background(Clr::GRN)
}
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
pub(crate) fn sty_btn(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let palette = theme.extended_palette();

    let base = ButtonStyle {
        background: Some(Background::Color(palette.primary.base.color)),
        text_color: palette.primary.base.text,
        border: Border { radius: RADIUS_WIDGET.into(), width: 0e0, ..Default::default() },
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
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
use iced::widget::pick_list::{Status as PickListStatus, Style as PickListStyle};
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
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
use iced::widget::rule::{FillMode as RuleFillMode, Style as RuleStyle};
pub(crate) fn sty_rule(theme: &Theme) -> RuleStyle {
    let palette = theme.extended_palette();
    RuleStyle {
        color: palette.background.strong.color,
        width: BORDER_W as u16,
        radius: RADIUS_WIDGET.into(),
        fill_mode: RuleFillMode::Percent(1e2),
    }
}
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
use iced::widget::scrollable::{
    Rail as ScrollBarRail, Scroller, Status as ScrollableStatus, Style as ScrollableStyle,
};
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
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
use iced::widget::slider::{
    Handle as SliderHandle, HandleShape as SliderHandleShape, Rail as SliderRail,
    Status as SliderStatus, Style as SliderStyle,
};
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
            border: Border { radius: RADIUS_WIDGET.into(), width: 1e0 * SF, color: Clr::BLK },
        },

        handle: SliderHandle {
            // shape: SliderHandleShape::Circle { radius: TEXT_SIZE / 1.75 },
            shape: SliderHandleShape::Rectangle {
                width: (TEXT_SIZE * 1.3) as u16,
                border_radius: RADIUS_WIDGET.into(),
            },
            background: color.into(),
            border_color: Clr::BLK,
            border_width: 1e0 * SF,
        },
    }
}
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
use widget::toggler::{Status as TogglerStatus, Style as TogglerStyle};
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
        background_border_width: 1e0,
        background_border_color: Clr::BLK,
        foreground: color,
        foreground_border_width: 0e0,
        foreground_border_color: Clr::TRN,
    }
}
// ------------------------------------------------------------------------------------------------
