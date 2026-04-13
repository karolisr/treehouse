use crate::consts::{STRK_EDGE, STRK_EDGE_LAB_ALN, STRK_ROOT};
use crate::edge_utils::{node_data_cart, node_data_pol};
use crate::path_builders::{
    path_clade_highlight, path_edges_fan, path_edges_phygrm,
    path_root_edge_fan, path_root_edge_phygrm,
};
use crate::{
    Float, NodeData, Rc, RectVals, TreSty, TreeState, ellipsize_unicode,
};
use dendros::Edge;
use oxidize_pdf::{
    Document, Page, PdfError,
    graphics::{Color, GraphicsContext, LineCap, LineDashPattern, LineJoin},
    text::{Font, measure_text},
};
use rayon::prelude::*;
use riced::fonts::JET_BRAINS_MONO_REGULAR;
use riced::{
    CnvStrk, IcedPath, LyonPath, LyonPathEvent, PathBuilder, Point, Rectangle,
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::path::PathBuf;

pub(super) fn color_from_iced_color(color_iced: riced::Color) -> Color {
    let c: [f32; 4] = color_iced.into_linear();
    Color::Rgb(c[0] as f64, c[1] as f64, c[2] as f64)
}

pub(super) fn alpha_from_iced_color(color_iced: riced::Color) -> f64 {
    let c: [f32; 4] = color_iced.into_linear();
    c[3] as f64
}

fn color_from_iced_stroke(stroke_iced: CnvStrk) -> Color {
    let color_iced = match stroke_iced.style {
        riced::GeomStyle::Solid(color) => color,
        riced::GeomStyle::Gradient(_gradient) => riced::Color::BLACK,
    };
    color_from_iced_color(color_iced)
}

fn line_cap_from_iced(line_cap_iced: riced::LineCap) -> LineCap {
    match line_cap_iced {
        riced::LineCap::Butt => LineCap::Butt,
        riced::LineCap::Square => LineCap::Square,
        riced::LineCap::Round => LineCap::Round,
    }
}

fn line_join_from_iced(line_join_iced: riced::LineJoin) -> LineJoin {
    match line_join_iced {
        riced::LineJoin::Miter => LineJoin::Miter,
        riced::LineJoin::Round => LineJoin::Round,
        riced::LineJoin::Bevel => LineJoin::Bevel,
    }
}

fn line_dash_from_iced(
    line_dash_iced: riced::LineDash,
    scaling: f64,
) -> LineDashPattern {
    LineDashPattern {
        array: line_dash_iced
            .segments
            .iter()
            .map(|seg| *seg as f64 * scaling)
            .collect(),
        phase: line_dash_iced.offset as f64 * scaling,
    }
}

pub(super) fn apply_iced_stroke_to_gc<'a>(
    stroke_iced: CnvStrk<'a>,
    scaling: f64,
    gc: &'a mut GraphicsContext,
) -> &'a mut GraphicsContext {
    let line_cap = line_cap_from_iced(stroke_iced.line_cap);
    let line_join = line_join_from_iced(stroke_iced.line_join);
    let line_width = stroke_iced.width as f64;
    let stroke_color = color_from_iced_stroke(stroke_iced);
    let line_dash = line_dash_from_iced(stroke_iced.line_dash, scaling);
    gc.set_line_cap(line_cap)
        .set_line_join(line_join)
        .set_line_width(line_width)
        .set_line_dash_pattern(line_dash)
        .set_stroke_color(stroke_color)
}

pub(super) fn apply_iced_path_to_gc(
    iced_path: IcedPath,
    gc: &mut GraphicsContext,
) -> &mut GraphicsContext {
    apply_lyon_path_to_gc(iced_path.raw(), gc);
    gc
}

fn apply_lyon_path_to_gc(lyon_path: &LyonPath, gc: &mut GraphicsContext) {
    let mut current = None;
    for event in lyon_path.iter() {
        match event {
            LyonPathEvent::Begin { at } => {
                if let Some(current) = current {
                    if at != current {
                        _ = gc.move_to(at.x as f64, -at.y as f64);
                    }
                } else {
                    _ = gc.move_to(at.x as f64, -at.y as f64);
                }

                current = Some(at);
            }

            LyonPathEvent::Line { from, to } => {
                if let Some(current) = current {
                    if from != current {
                        _ = gc.move_to(from.x as f64, -from.y as f64);
                    }

                    if to != current {
                        _ = gc.line_to(to.x as f64, -to.y as f64);
                    }
                }

                current = Some(to);
            }

            LyonPathEvent::Quadratic { from, ctrl: _, to } => {
                if let Some(current) = current {
                    if from != current {
                        _ = gc.move_to(from.x as f64, -from.y as f64);
                    }

                    if to != current {
                        _ = gc.move_to(to.x as f64, -to.y as f64);
                    }
                }

                current = Some(to);
            }

            LyonPathEvent::Cubic { from, ctrl1, ctrl2, to } => {
                if let Some(current) = current {
                    if from != current {
                        _ = gc.move_to(from.x as f64, -from.y as f64);
                    }

                    if to != current {
                        _ = gc.curve_to(
                            ctrl1.x as f64, -ctrl1.y as f64, ctrl2.x as f64,
                            -ctrl2.y as f64, to.x as f64, -to.y as f64,
                        );
                    }
                }

                current = Some(to);
            }

            LyonPathEvent::End { last, first, close } => {
                if let Some(current) = current {
                    if last != current {
                        _ = gc.move_to(last.x as f64, -last.y as f64);
                    }

                    if close {
                        if first != current {
                            _ = gc.line_to(first.x as f64, -first.y as f64);
                        }

                        _ = gc.close_path();
                    }
                }

                current = Some(last);
            }
        }
    }
}
