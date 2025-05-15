const STROKE: Stroke = Stroke {
    style: Style::Solid(Color { a: 0.75, ..Clr::RED }),
    width: 1e0,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    line_dash: LineDash { segments: &[1e0, 2e0], offset: 0 },
};

pub(crate) fn visible_tip_idx_range(
    tre_cnv_y0: Float, tre_cnv_y1: Float, node_size: Float, tree_tip_edges: &Edges,
    sel_tree_style_opt: TreeStyle,
) -> Option<IndexRange> {
    match sel_tree_style_opt {
        TreeStyle::Phylogram => {
            let tip_idx_0: i64 = (tre_cnv_y0 / node_size) as i64 - 3;
            let tip_idx_1: i64 = (tre_cnv_y1 / node_size) as i64 + 3;
            let tip_idx_0: usize = tip_idx_0.max(0) as usize;
            let tip_idx_1: usize = tip_idx_1.min(tree_tip_edges.len() as i64 - 1) as usize;
            if tip_idx_0 < tip_idx_1 {
                Some(IndexRange { b: tip_idx_0, e: tip_idx_1 })
            } else {
                None
            }
        }
        TreeStyle::Fan => Some(IndexRange { b: 0, e: tree_tip_edges.len() - 1 }),
    }
}

pub(crate) fn visible_node_ranges(
    tip_idx_range: &IndexRange, tree_tip_edges: &[Edge],
) -> ChunkEdgeRange {
    let idx_0 = &tree_tip_edges[tip_idx_range.b];
    let idx_1 = &tree_tip_edges[tip_idx_range.e];

    let chnk_idx_0 = idx_0.chunk_idx;
    let edge_idx_0 = idx_0.edge_idx;

    let chnk_idx_1 = idx_1.chunk_idx;
    let edge_idx_1 = idx_1.edge_idx;

    ChunkEdgeRange {
        chnk: IndexRange { b: chnk_idx_0, e: chnk_idx_1 },
        edge: IndexRange { b: edge_idx_0, e: edge_idx_1 },
    }
}

pub(crate) fn visible_nodes(
    w: Float, h: Float, center: Point, size: Float, tip_idx_range: &IndexRange,
    tree_tip_edges: &[Edge], sel_tree_style_opt: TreeStyle, rot_angle: Float, opn_angle: Float,
    tree_edges_chunked: &[Edges],
) -> Vec<NodePoint> {
    let ChunkEdgeRange {
        chnk: IndexRange { b: chnk_idx_0, e: chnk_idx_1 },
        edge: IndexRange { b: edge_idx_0, e: edge_idx_1 },
    } = visible_node_ranges(tip_idx_range, tree_tip_edges);
    let tree_repr = sel_tree_style_opt;
    let mut points: Vec<NodePoint> = Vec::new();
    if chnk_idx_0 == chnk_idx_1 {
        let chunk = &tree_edges_chunked[chnk_idx_0];
        for e in &chunk[edge_idx_0..=edge_idx_1] {
            let mut angle: Option<Float> = None;
            let point: Point;
            match tree_repr {
                TreeStyle::Phylogram => {
                    point = node_point(w, h, e);
                }
                TreeStyle::Fan => {
                    let a = edge_angle(rot_angle, opn_angle, e);
                    point = node_point_rad(a, center, size, e);
                    angle = Some(a);
                }
            }
            points.push(NodePoint { point, edge: e.clone(), angle });
        }
    } else {
        for chnk_idx in chnk_idx_0..=chnk_idx_1 {
            let chunk = &tree_edges_chunked[chnk_idx];
            let edge_range: RangeInclusive<usize>;

            if chnk_idx == chnk_idx_0 {
                edge_range = edge_idx_0..=tree_edges_chunked[chnk_idx].len() - 1;
            } else if chnk_idx == chnk_idx_1 {
                edge_range = 0..=edge_idx_1
            } else {
                edge_range = 0..=tree_edges_chunked[chnk_idx].len() - 1;
            }

            for e in &chunk[edge_range] {
                let mut angle: Option<Float> = None;
                let point: Point;
                match tree_repr {
                    TreeStyle::Phylogram => {
                        point = node_point(w, h, e);
                    }
                    TreeStyle::Fan => {
                        let a = edge_angle(rot_angle, opn_angle, e);
                        point = node_point_rad(a, center, size, e);
                        angle = Some(a);
                    }
                }
                points.push(NodePoint { point, edge: e.clone(), angle });
            }
        }
    }

    points
}

pub(crate) fn draw_scale_bar(
    tree_height: Float, stroke: Stroke, label_template: &iced::widget::canvas::Text,
    tree_rect: &Rectangle, frame: &mut Frame,
) {
    let mut sb_len = tree_height as Float / 4e0;

    if sb_len > 1e1 {
        sb_len = sb_len.floor();
    } else {
        sb_len = (sb_len * 1e1).floor() / 1e1;
    }

    let sb_frac = sb_len / tree_height as Float;
    let sb_len_on_screen = sb_frac * tree_rect.width;
    let sb_str = format!("{sb_len}");

    let y = tree_rect.y + tree_rect.height - 3e1;

    // in the middle...
    // let p0 = Point { x: tree_rect.width / 2e0 - sb_len_on_screen / 2e0, y };
    let p0 = Point { x: 2e1, y };
    let p1 = Point { x: p0.x + sb_len_on_screen, y };

    let p_lab = Point { x: p0.x + (p1.x - p0.x) / 2e0, y: y + 1e1 };

    let mut l = label_template.clone();
    l.align_x = TextAlignment::Center;
    l.align_y = Vertical::Top;
    l.position = p_lab;
    l.content = sb_str;
    l.size = 1e1.into();

    let path = Path::new(|p| {
        p.move_to(p0);
        p.line_to(p1);
    });

    frame.stroke(&path, stroke);
    frame.fill_text(l);
}

pub(crate) fn draw_node(
    point: &Point, ps: Float, stroke: Stroke, fill: impl Into<iced::widget::canvas::Fill>,
    tree_rect: &Rectangle, frame: &mut Frame,
) {
    frame.with_save(|f| {
        f.translate(Vector { x: tree_rect.x, y: tree_rect.y });
        let path_fill = Path::new(|p| {
            p.circle(*point, ps);
        });

        let path_stroke = Path::new(|p| {
            p.circle(*point, ps - 1e0 / 2e0);
        });

        f.fill(&path_fill, fill);
        f.stroke(&path_stroke, stroke);
    });
}

use iced::widget::container::Style as ContainerStyle;
pub(crate) fn sty_cont(theme: &Theme) -> ContainerStyle {
    let pb = theme.palette();
    // let pe = theme.extended_palette();
    ContainerStyle {
        text_color: Some(pb.text),
        background: None,
        border: Border {
            width: 3e0,
            color: Clr::BLK.scale_alpha(0.125),
            radius: RADIUS_WIDGET.into(),
        },
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

pub(crate) fn toggler_cursor_line<'a>(
    enabled: bool, show_cursor_line: bool, sel_tree_style_opt: TreeStyle,
) -> Toggler<'a, TreeViewMsg> {
    let lab = match sel_tree_style_opt {
        TreeStyle::Phylogram => "Cursor Tracking Line",
        TreeStyle::Fan => "Cursor Tracking Circle",
    };

    let mut tglr = toggler(lab, show_cursor_line);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::CursorLineVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_legend<'a>(enabled: bool, draw_legend: bool) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Legend", enabled && draw_legend);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::LegendVisibilityChanged);
    }
    tglr
}
