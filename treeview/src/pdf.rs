#![allow(unused_imports)]
use crate::consts::{
    FNT_NAME, FRAC_PI_2, ONE, PI, STRK_EDGE, STRK_ROOT, TAU, TWO, ZRO,
};
use crate::path_builders::*;
use crate::{Float, Rc, TreSty, TreeState};
use dendros::Edge;
use oxidize_pdf::{
    Document, Page, PdfError,
    graphics::{Color, GraphicsContext, LineCap, LineJoin},
    text::{
        Font, FontFamily, TextAlign, TextContext, measure_char, measure_text,
    },
};
use riced::{CnvStrk, IcedPath, LyonPath, LyonPathEvent};
use std::path::PathBuf;

pub fn tree_to_pdf(
    path_buf: PathBuf,
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    rot_angle: Float,
    root_len: Float,
    radius: Float,
) -> Result<(), PdfError> {
    let mut page = Page::new(w as f64, h as f64);
    let gc: &mut GraphicsContext = page.graphics();

    match tree_style {
        TreSty::PhyGrm => {
            // _ = gc.transform(0e0, -1e0, -1e0, 0e0, 0e0, 0e0);
            // _ = gc.translate((w / TWO) as f64, (h / TWO) as f64);
            // _ = gc.rotate((-FRAC_PI_2) as f64);
        }
        TreSty::Fan => {
            _ = gc.translate((w / TWO) as f64, (h / TWO) as f64);
            _ = gc.transform(0e0, -1e0, -1e0, 0e0, 0e0, 0e0);
            _ = gc.rotate((rot_angle) as f64);
            _ = gc.rotate((-FRAC_PI_2) as f64);
        }
    };

    _ = draw_edges(
        tree_state, tree_style, w, h, opn_angle, root_len, radius, gc,
    );

    // let tc: &mut TextContext = page.text();
    let mut doc = Document::new();
    doc.set_title("TreeHouse Exported PDF");
    doc.add_page(page);
    doc.save(path_buf)
}

fn draw_edges(
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    // rot_angle: Float,
    root_len: Float,
    radius: Float,
    gc: &mut GraphicsContext,
) -> Option<()> {
    apply_iced_stroke_to_gc(STRK_EDGE, gc);

    let edges: &Vec<Edge> = tree_state.edges_srtd_y()?;

    let path = match tree_style {
        TreSty::PhyGrm => path_edges_phygrm(edges, w, h),
        TreSty::Fan => path_edges_fan(edges, opn_angle, root_len, radius),
    };

    apply_iced_path_to_gc(path, gc);

    _ = gc.stroke();

    Some(())
}

fn apply_iced_path_to_gc(iced_path: IcedPath, gc: &mut GraphicsContext) {
    apply_lyon_path_to_gc(iced_path.raw(), gc);
}

fn apply_lyon_path_to_gc(lyon_path: &LyonPath, gc: &mut GraphicsContext) {
    let mut current = None;
    for event in lyon_path.iter() {
        match event {
            LyonPathEvent::Begin { at } => {
                current = Some(at);
                _ = gc.move_to(at.x as f64, at.y as f64);
            }
            LyonPathEvent::Line { from, to } => {
                if let Some(current_point) = current {
                    if from != current_point {
                        _ = gc.move_to(from.x as f64, from.y as f64);
                    }
                }
                _ = gc.line_to(to.x as f64, to.y as f64);
                current = Some(to);
            }
            LyonPathEvent::Quadratic { from: _, ctrl: _, to } => {
                // if let Some(current_point) = current {
                //     if from != current_point {
                //         _ = gc.move_to(from.x as f64, from.y as f64);
                //     }
                // }
                _ = gc.move_to(to.x as f64, to.y as f64);
                current = Some(to);
            }
            LyonPathEvent::Cubic { from, ctrl1, ctrl2, to } => {
                if let Some(current_point) = current {
                    if from != current_point {
                        _ = gc.move_to(from.x as f64, from.y as f64);
                    }
                }
                _ = gc.curve_to(
                    ctrl1.x as f64, ctrl1.y as f64, ctrl2.x as f64,
                    ctrl2.y as f64, to.x as f64, to.y as f64,
                );
                current = Some(to);
            }
            LyonPathEvent::End { last, first, close } => {
                if let Some(current_point) = current {
                    if last != current_point {
                        _ = gc.move_to(last.x as f64, last.y as f64);
                    }
                }
                if close {
                    _ = gc.line_to(first.x as f64, first.y as f64);
                    _ = gc.close_path();
                }
                current = Some(last);
            }
        }
    }
}

fn color_from_iced_color(color_iced: riced::Color) -> Color {
    let c: [f32; 4] = color_iced.into_linear();
    Color::Rgb(c[0] as f64, c[1] as f64, c[2] as f64)
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

fn apply_iced_stroke_to_gc(stroke_iced: CnvStrk, gc: &mut GraphicsContext) {
    let line_cap = line_cap_from_iced(stroke_iced.line_cap);
    let line_join = line_join_from_iced(stroke_iced.line_join);
    let line_width = stroke_iced.width as f64;
    let stroke_color = color_from_iced_stroke(stroke_iced);

    _ = gc
        .set_line_cap(line_cap)
        .set_line_join(line_join)
        .set_line_width(line_width)
        .set_stroke_color(stroke_color);
}
