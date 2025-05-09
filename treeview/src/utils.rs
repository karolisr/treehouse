use crate::{
    BORDER_W, Float, NODE_ORD_OPTS, NodeOrd, PI, PlotCnv, RADIUS_WIDGET, SF, TEXT_SIZE,
    TREE_LAB_FONT_NAME, TREE_STYLE_OPTS, TreeCnv, TreeState, TreeStateMsg, TreeViewMsg,
    treeview::TreeStyle,
};
use dendros::{Edge, Edges};
use iced::{
    Alignment, Background, Border, Color, Element, Font,
    Length::{self, Fill, Shrink},
    Pixels, Point, Radians, Rectangle, Theme, Vector,
    alignment::{Horizontal, Vertical},
    widget::{
        Button, Canvas, Column, PickList, Row, Rule, Scrollable, Slider, Space, Text,
        canvas::{
            Frame, LineCap, LineDash, LineJoin, Path, Stroke, Style,
            path::{Arc, Builder as PathBuilder},
        },
        container, horizontal_space,
        pick_list::Handle as PickListHandle,
        row,
        scrollable::{Direction as ScrollableDirection, Scrollbar},
        text,
        text::Alignment as TextAlignment,
        vertical_space,
    },
};
use num_traits::cast::FromPrimitive;
use numfmt::Formatter as NumFmt;
use std::{
    fmt::Display,
    ops::RangeInclusive,
    thread::{self, ScopedJoinHandle},
};
use utils::{Clr, text_width};
use widget::toggler::{Roundness as TogglerRoundness, Toggler};

const STROKE: Stroke = Stroke {
    style: Style::Solid(Color { a: 0.75, ..Clr::RED }),
    width: 1e0,
    line_cap: LineCap::Square,
    line_join: LineJoin::Round,
    line_dash: LineDash { segments: &[1e0, 2e0], offset: 0 },
};

const RADIUS: f32 = 1e1;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct IndexRange {
    pub(crate) b: usize,
    pub(crate) e: usize,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NodePoint {
    pub(crate) point: iced::Point,
    pub(crate) edge: dendros::Edge,
    pub(crate) angle: Option<Float>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Label {
    pub(crate) text: iced::widget::canvas::Text,
    pub(crate) angle: Option<Float>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct EdgePoints {
    pub(crate) pt_0: iced::Point,
    pub(crate) pt_1: iced::Point,
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ChunkEdgeRange {
    pub(crate) chnk: IndexRange,
    pub(crate) edge: IndexRange,
}

pub(crate) fn clip_rect_from_bounds(bounds: Rectangle) -> Rectangle {
    Rectangle { x: 1e0, y: 1e0, width: bounds.width - 2e0, height: bounds.height - 2e0 }
}

pub(crate) fn draw_point(point: Point, frame: &mut Frame) {
    let path = Path::circle(point, RADIUS);
    frame.stroke(&path, STROKE);
}

pub(crate) fn draw_rectangle(rect: Rectangle, frame: &mut Frame) {
    frame.stroke_rectangle(Point { x: rect.x, y: rect.y }, rect.size(), STROKE);
}

pub(crate) fn edge_path_phylogram(
    w: Float,
    h: Float,
    edge: &Edge,
    pb: &mut PathBuilder,
    draw_root: bool,
) {
    let EdgePoints { pt_0, pt_1 } = edge_points(w, h, edge);
    pb.move_to(pt_1);
    pb.line_to(pt_0);
    if let Some(y_parent) = edge.y_parent {
        let pt_parent = Point { x: pt_0.x, y: y_parent as Float * h };
        pb.line_to(pt_parent)
    } else if draw_root && edge.parent_node_id.is_none() {
        let pt_parent = Point { x: 1e1 * -1e0, y: edge.y as Float * h };
        pb.line_to(pt_parent)
    }
}

pub(crate) fn edge_path_fan(
    rot_angle: Float,
    opn_angle: Float,
    center: Point,
    size: Float,
    edge: &Edge,
    pb: &mut PathBuilder,
    draw_root: bool,
) {
    let angle = edge_angle(rot_angle, opn_angle, edge);
    let EdgePoints { pt_0, pt_1 } = edge_points_rad(angle, center, size, edge);
    pb.move_to(pt_1);
    pb.line_to(pt_0);
    if let Some(y_parent) = edge.y_parent {
        let angle_parent = rot_angle + y_parent as Float * opn_angle;
        let p_arc = Arc {
            center,
            radius: center.distance(pt_0),
            start_angle: Radians(angle),
            end_angle: Radians(angle_parent),
        };
        pb.arc(p_arc);
    } else if draw_root && edge.parent_node_id.is_none() {
        let x0 = center.x - (1e1 * 1e0) * angle.cos();
        let y0 = center.y - (1e1 * 1e0) * angle.sin();
        let pt_parent = Point { x: x0, y: y0 };
        pb.line_to(pt_parent)
    }
}

pub(crate) fn edge_point(w: Float, h: Float, edge: &Edge) -> Point {
    let x = edge.x0 as Float * w;
    let y = edge.y as Float * h;
    Point { x, y }
}

pub(crate) fn edge_point_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> Point {
    let x0 = edge.x0 as Float * angle.cos() * size;
    let y0 = edge.x0 as Float * angle.sin() * size;
    Point { x: center.x + x0, y: center.y + y0 }
}

pub(crate) fn node_point(w: Float, h: Float, edge: &Edge) -> Point {
    let x = edge.x1 as Float * w;
    let y = edge.y as Float * h;
    Point { x, y }
}

pub(crate) fn node_point_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> Point {
    let x1 = edge.x1 as Float * angle.cos() * size;
    let y1 = edge.x1 as Float * angle.sin() * size;
    Point { x: center.x + x1, y: center.y + y1 }
}

pub(crate) fn edge_points(w: Float, h: Float, edge: &Edge) -> EdgePoints {
    let pt_0 = edge_point(w, h, edge);
    let pt_1 = node_point(w, h, edge);
    EdgePoints { pt_0, pt_1 }
}

pub(crate) fn edge_points_rad(angle: Float, center: Point, size: Float, edge: &Edge) -> EdgePoints {
    let pt_0 = edge_point_rad(angle, center, size, edge);
    let pt_1 = node_point_rad(angle, center, size, edge);
    EdgePoints { pt_0, pt_1 }
}

pub(crate) fn edge_angle(rot_angle: Float, opn_angle: Float, edge: &Edge) -> Float {
    rot_angle + edge.y as Float * opn_angle
}

pub(crate) fn paths_from_chunks(
    w: Float,
    h: Float,
    center: Point,
    size: Float,
    draw_root: bool,
    rot_angle: Float,
    opn_angle: Float,
    sel_tree_style_opt: TreeStyle,
    tree_edges_chunked: &[Edges],
) -> Vec<Path> {
    let repr: TreeStyle = sel_tree_style_opt;
    let node_count: usize = tree_edges_chunked.iter().map(|x| x.len()).sum();
    let mut paths: Vec<Path> = Vec::with_capacity(node_count);
    thread::scope(|thread_scope| {
        let mut handles: Vec<ScopedJoinHandle<'_, Path>> = Vec::new();
        for chunk in tree_edges_chunked {
            let handle = thread_scope.spawn(move || {
                let mut pb = PathBuilder::new();
                for edge in chunk {
                    match repr {
                        TreeStyle::Phylogram => edge_path_phylogram(w, h, edge, &mut pb, draw_root),
                        TreeStyle::Fan => edge_path_fan(
                            rot_angle, opn_angle, center, size, edge, &mut pb, draw_root,
                        ),
                    }
                }
                pb.build()
            });
            handles.push(handle);
        }
        for j in handles {
            let path = j.join().unwrap();
            paths.push(path);
        }
    });
    paths
}

pub(crate) fn node_labels(
    nodes: &Vec<NodePoint>,
    tips: bool,
    label_text_template: &iced::widget::canvas::Text,
) -> Vec<Label> {
    let mut labels: Vec<Label> = Vec::with_capacity(nodes.len());
    for NodePoint { point, edge, angle } in nodes {
        if (tips && !edge.is_tip) || (!tips && edge.is_tip) {
            continue;
        }
        if let Some(name) = &edge.name {
            let mut text = label_text_template.clone();
            text.content = name.to_string();
            text.position = *point;
            labels.push(Label { text, angle: *angle });
        }
    }
    labels
}

pub(crate) fn branch_labels(
    size: Float,
    visible_nodes: &[NodePoint],
    label_text_template: &iced::widget::canvas::Text,
) -> Vec<Label> {
    let mut label_text_template = label_text_template.clone();
    label_text_template.align_x = TextAlignment::Center;
    label_text_template.align_y = Vertical::Bottom;
    let mut labels: Vec<Label> = Vec::with_capacity(visible_nodes.len());
    for NodePoint { point, edge, angle } in visible_nodes {
        if edge.parent_node_id.is_none() {
            continue;
        }
        let mut text = label_text_template.clone();
        let mut node_point = *point;

        let adj = edge.brlen_normalized as Float * size / 2e0;
        if let Some(angle) = angle {
            node_point.x -= angle.cos() * adj;
            node_point.y -= angle.sin() * adj;
        } else {
            node_point.x -= adj;
        }

        text.position = node_point;
        text.content = format!("{:.3}", edge.brlen);
        labels.push(Label { text, angle: *angle });
    }
    labels
}

pub(crate) fn visible_tip_idx_range(
    tre_cnv_y0: Float,
    tre_cnv_y1: Float,
    node_size: Float,
    tree_tip_edges: &Edges,
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
    tip_idx_range: &IndexRange,
    tree_tip_edges: &[Edge],
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
    w: Float,
    h: Float,
    center: Point,
    size: Float,
    tip_idx_range: &IndexRange,
    tree_tip_edges: &[Edge],
    sel_tree_style_opt: TreeStyle,
    rot_angle: Float,
    opn_angle: Float,
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

pub(crate) fn draw_edges(
    paths: Vec<Path>,
    stroke: Stroke,
    tree_rect: &Rectangle,
    frame: &mut Frame,
) {
    frame.with_save(|f| {
        f.translate(Vector { x: tree_rect.x, y: tree_rect.y });
        for p in paths {
            f.stroke(&p, stroke);
        }
    })
}

pub(crate) fn draw_scale_bar(
    tree_height: Float,
    stroke: Stroke,
    label_template: &iced::widget::canvas::Text,
    tree_rect: &Rectangle,
    frame: &mut Frame,
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
    point: &Point,
    ps: Float,
    stroke: Stroke,
    fill: impl Into<iced::widget::canvas::Fill>,
    tree_rect: &Rectangle,
    frame: &mut Frame,
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

pub(crate) fn draw_labels(
    labels: Vec<Label>,
    text_size: Float,
    offset: Point,
    tree_rect: &Rectangle,
    clip: &Rectangle,
    frame: &mut Frame,
) {
    let zero_point = Point { x: 0e0, y: 0e0 };
    let mut text_w = text_width(text_size, text_size, TREE_LAB_FONT_NAME);
    let text_size: Pixels = text_size.into();
    frame.with_clip(*clip, |f| {
        f.translate(Vector { x: tree_rect.x + offset.x, y: tree_rect.y + offset.y });
        for Label { mut text, angle } in labels {
            text.size = text_size;
            if let Some(mut angle) = angle {
                let mut adjust_w = offset.x;
                // = Rotate labels on the left side of the circle by 180 degrees ==============
                let a = angle % (2e0 * PI);
                if a > PI / 2e0 && a < PI * 1.5 {
                    angle += PI;
                    match text.align_x {
                        TextAlignment::Left => adjust_w = -text_w.width(&text.content) - offset.x,
                        TextAlignment::Center => {}
                        TextAlignment::Right => adjust_w = text_w.width(&text.content) + offset.x,
                        _ => {}
                    }
                } // ==========================================================================
                f.push_transform();
                f.translate(Vector {
                    x: text.position.x - offset.x + angle.cos() * adjust_w,
                    y: text.position.y - offset.y + angle.sin() * adjust_w,
                });
                f.rotate(angle);
                text.position = zero_point;
                f.fill_text(text);
                f.pop_transform();
            } else {
                f.fill_text(text);
            }
        }
    });
}

// ------------------------------------------------------------------------------------------------
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

// --------------------------------------------------------------------------------------------

pub(crate) fn btn(lab: &str, msg: Option<TreeViewMsg>) -> Button<TreeViewMsg> {
    let mut txt = Text::new(lab);
    txt = txt.align_x(Horizontal::Center);
    txt = txt.align_y(Vertical::Center);
    let mut btn = Button::new(txt);
    btn = btn.on_press_maybe(msg);
    // btn = btn.style(sty_btn);
    btn
}

pub(crate) fn btn_root(ts: &TreeState) -> Button<TreeViewMsg> {
    btn("Root", {
        if ts.sel_node_ids.len() == 1 {
            let node_id = *ts.sel_node_ids.iter().last().unwrap();
            match ts.can_root(node_id) {
                true => Some(TreeViewMsg::TreeStateMsg(TreeStateMsg::Root(node_id))),
                false => None,
            }
        } else {
            None
        }
    })
}

pub(crate) fn btn_unroot(ts: &TreeState) -> Button<TreeViewMsg> {
    btn(
        "Unroot",
        match ts.is_rooted {
            true => Some(TreeViewMsg::TreeStateMsg(TreeStateMsg::Unroot)),
            false => None,
        },
    )
}

// --------------------------------------------------------------------------------------------

pub(crate) fn pick_list_common<'a, T: PartialEq + Display + Clone>(
    pl: PickList<'a, T, &[T], T, TreeViewMsg>,
) -> PickList<'a, T, &'a [T], T, TreeViewMsg> {
    let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TEXT_SIZE)) };
    let mut pl = pl;
    pl = pl.handle(h);
    // pl = pl.style(sty_pick_lst);
    pl
}

pub(crate) fn pick_list_node_ordering<'a>(sel_node_ord_opt: NodeOrd) -> Row<'a, TreeViewMsg> {
    let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TreeViewMsg> =
        PickList::new(&NODE_ORD_OPTS, Some(sel_node_ord_opt), TreeViewMsg::NodeOrdOptChanged);
    pl = pick_list_common(pl);
    row![text!("Node Order").width(Fill), pl].align_y(Vertical::Center)
}

pub(crate) fn pick_list_tree_style<'a>(sel_tree_style_opt: TreeStyle) -> Row<'a, TreeViewMsg> {
    let mut pl: PickList<TreeStyle, &[TreeStyle], TreeStyle, TreeViewMsg> = PickList::new(
        &TREE_STYLE_OPTS,
        Some(sel_tree_style_opt),
        TreeViewMsg::TreeStyleOptionChanged,
    );
    pl = pick_list_common(pl);
    row![text!("Style").width(Fill), pl].align_y(Vertical::Center)
}

// --------------------------------------------------------------------------------------------

pub(crate) fn rule_common(rule: Rule<Theme>) -> Rule<Theme> {
    // let rule = rule.style(sty_rule);
    rule
}

pub(crate) fn rule_h<'a>(height: impl Into<Pixels>) -> Rule<'a, Theme> {
    let mut r: Rule<'_, Theme> = Rule::horizontal(height);
    r = rule_common(r);
    r
}

pub(crate) fn rule_v<'a>(width: impl Into<Pixels>) -> Rule<'a, Theme> {
    let mut r: Rule<'_, Theme> = Rule::vertical(width);
    r = rule_common(r);
    r
}

// --------------------------------------------------------------------------------------------

pub(crate) fn scrollable_common(
    scrl: Scrollable<TreeViewMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<TreeViewMsg> {
    let mut s = scrl;
    // s = s.style(sty_scrlbl);
    s = s.width(w.into());
    s = s.height(h.into());
    s
}

pub(crate) fn scrollable_v<'a>(
    content: impl Into<Element<'a, TreeViewMsg>>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<'a, TreeViewMsg> {
    let mut s: Scrollable<TreeViewMsg> = Scrollable::new(content);
    s = s.direction(ScrollableDirection::Vertical(Scrollbar::new()));
    scrollable_common(s, w, h)
}

pub(crate) fn scrollable_cnv_ltt(
    cnv: Canvas<&PlotCnv, TreeViewMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<TreeViewMsg> {
    let mut s: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Horizontal(Scrollbar::new()));
    s = s.on_scroll(TreeViewMsg::LttCnvScrolled);
    s = s.id("ltt");
    scrollable_common(s, w, h)
}

pub(crate) fn scrollable_cnv_tree(
    cnv: Canvas<&TreeCnv, TreeViewMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<TreeViewMsg> {
    let mut s: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
    let sb = Scrollbar::new();
    s = s.direction(ScrollableDirection::Both { horizontal: sb, vertical: sb });
    s = s.on_scroll(TreeViewMsg::TreCnvScrolled);
    s = s.id("tre");
    scrollable_common(s, w, h)
}

// --------------------------------------------------------------------------------------------

pub(crate) fn slider<'a, T>(
    lab: Option<&str>,
    min: T,
    max: T,
    sel: T,
    msg: impl 'a + Fn(T) -> TreeViewMsg,
) -> Element<'a, TreeViewMsg>
where
    f64: From<T>,
    T: 'a + PartialOrd + From<u8> + Copy + FromPrimitive,
{
    let mut slider: Slider<T, TreeViewMsg> = Slider::new(min..=max, sel, msg);
    slider = slider.step(1);
    slider = slider.shift_step(2);
    slider = slider.height(TEXT_SIZE * 1.2);
    // slider = slider.style(sty_slider);

    if let Some(lab) = lab {
        let mut lab = container(text!("{lab}"));
        lab = lab.align_x(Horizontal::Right);
        lab = lab.align_y(Vertical::Center);
        lab = lab.width(Fill);

        let mut c: Column<TreeViewMsg> = Column::new();
        c = c.push(lab);
        c = c.push(slider);
        c = c.align_x(Horizontal::Center);
        c = c.spacing(3e0);
        c.into()
    } else {
        slider.into()
    }
}

// --------------------------------------------------------------------------------------------

pub(crate) fn space_h(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    horizontal_space().width(w).height(h)
}

pub(crate) fn space_v(w: impl Into<Length>, h: impl Into<Length>) -> Space {
    vertical_space().width(w).height(h)
}

// --------------------------------------------------------------------------------------------

pub(crate) fn toggler(label: &str, value: bool) -> Toggler<TreeViewMsg> {
    let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
    tglr = tglr.label(label);
    tglr = tglr.text_size(TEXT_SIZE);
    tglr = tglr.size(TEXT_SIZE * 1.5);
    tglr = tglr.text_alignment(Alignment::End);
    tglr = tglr.width(Fill);
    // tglr = tglr.roundness(TogglerRoundness::Radius(RADIUS_WIDGET));
    // tglr = tglr.style(sty_toggler);
    tglr
}

pub(crate) fn toggler_cursor_line<'a>(
    enabled: bool,
    show_cursor_line: bool,
    sel_tree_style_opt: TreeStyle,
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

pub(crate) fn toggler_label_branch<'a>(
    enabled: bool,
    draw_brnch_labs: bool,
) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Branch Lengths", enabled && draw_brnch_labs);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::BranchLabelVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_label_int<'a>(
    enabled: bool,
    draw_int_labs: bool,
) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Internal Labels", enabled && draw_int_labs);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::IntLabelVisibilityChanged);
    }
    tglr
}

pub(crate) fn toggler_label_tip<'a>(
    enabled: bool,
    draw_tip_labs: bool,
) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("Tip Labels", enabled && draw_tip_labs);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::TipLabelVisibilityChanged);
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

pub(crate) fn toggler_ltt<'a>(enabled: bool, show_ltt: bool) -> Toggler<'a, TreeViewMsg> {
    let mut tglr = toggler("LTT Plot", show_ltt);
    if enabled {
        tglr = tglr.on_toggle(TreeViewMsg::LttPlotVisibilityChanged);
    }
    tglr
}

// --------------------------------------------------------------------------------------------

pub(crate) fn txt(s: impl Into<String>) -> Text<'static> {
    Text::new(s.into()).align_x(Horizontal::Right).align_y(Vertical::Center).width(Shrink)
}

pub(crate) fn txt_bool(b: bool) -> Text<'static> {
    let s = match b {
        true => "Yes",
        false => "No",
    };
    txt(s)
}

pub(crate) fn txt_bool_option(ob: Option<bool>) -> Text<'static> {
    match ob {
        Some(b) => txt_bool(b),
        None => txt("N/A"),
    }
}

pub(crate) fn txt_float(n: impl Into<f32>) -> Text<'static> {
    let mut num_fmt = NumFmt::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

pub(crate) fn txt_usize(n: impl Into<usize>) -> Text<'static> {
    let mut num_fmt = NumFmt::new();
    num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
    num_fmt = num_fmt.separator(',').unwrap();
    let s = num_fmt.fmt2(n.into());
    txt(s)
}

// ------------------------------------------------------------------------------------------------
