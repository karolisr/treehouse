use crate::iced::*;
use crate::*;

pub(crate) fn sty_cont(theme: &Theme) -> ContainerStyle {
    let pb = theme.palette();
    // let pe = theme.extended_palette();
    ContainerStyle {
        text_color: Some(pb.text),
        background: None,
        border: Border { width: 1e0, color: Clr::BLK, radius: 0.into() },
        ..Default::default()
    }
}

pub(crate) fn sty_pane_grid(theme: &Theme) -> PgStyle {
    let pe = theme.extended_palette();
    PgStyle {
        hovered_region: PgHighlight {
            background: Clr::GRN_25.into(),
            border: Border { width: 1e0, color: pe.primary.strong.color, radius: 0.into() },
        },
        hovered_split: PgLine { color: pe.primary.base.color, width: 1e0 },
        picked_split: PgLine { color: pe.primary.strong.color, width: 1e0 },
    }
}

pub(crate) fn sty_pane_titlebar(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
pub(crate) fn sty_pane_body(theme: &Theme) -> ContainerStyle { sty_cont(theme) }
