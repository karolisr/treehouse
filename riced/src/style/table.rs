use crate::*;

pub fn sty_table_cell(theme: &Theme) -> ContainerStyle {
    let pe = theme.extended_palette();
    let base = sty_cont(theme);
    ContainerStyle {
        background: Some(Background::Color(pe.background.base.color)),
        border: Border { width: ZERO, color: Clr::TRN, radius: ZERO.into() },
        shadow: Shadow {
            color: Clr::TRN,
            offset: Vector { x: ZERO, y: ZERO },
            blur_radius: ZERO,
        },
        ..base
    }
}

pub fn sty_table_cell_selected(theme: &Theme) -> ContainerStyle {
    let pe = theme.extended_palette();
    let base = sty_table_cell(theme);
    ContainerStyle {
        background: Some(Background::Color(
            pe.primary.weak.color.scale_alpha(0.3),
        )),
        ..base
    }
}

pub fn sty_table_row_header(theme: &Theme) -> ContainerStyle {
    let pe = theme.extended_palette();
    let base = sty_table_cell(theme);
    ContainerStyle {
        background: Some(Background::Color(pe.background.strongest.color)),
        border: Border {
            radius: Radius {
                top_left: WIDGET_RADIUS,
                top_right: WIDGET_RADIUS,
                ..base.border.radius
            },
            ..base.border
        },
        ..base
    }
}

pub fn sty_table_cell_header(theme: &Theme) -> ContainerStyle {
    let pe = theme.extended_palette();
    let base = sty_table_cell(theme);
    ContainerStyle {
        background: Some(Background::Color(pe.background.weak.color)),
        ..base
    }
}

pub fn sty_table_cell_header_left(theme: &Theme) -> ContainerStyle {
    let base = sty_table_cell_header(theme);
    ContainerStyle {
        border: Border {
            radius: Radius { top_left: WIDGET_RADIUS, ..base.border.radius },
            ..base.border
        },
        ..base
    }
}

pub fn sty_table_cell_header_right(theme: &Theme) -> ContainerStyle {
    let base = sty_table_cell_header(theme);
    ContainerStyle {
        border: Border {
            radius: Radius { top_right: WIDGET_RADIUS, ..base.border.radius },
            ..base.border
        },
        ..base
    }
}
