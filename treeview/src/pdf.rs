use crate::consts::{STRK_EDGE, STRK_EDGE_LAB_ALN, STRK_ROOT};
use crate::edge_utils::{node_data_cart, node_data_rad};
use crate::path_builders::{
    path_edges_fan, path_edges_phygrm, path_root_edge_fan,
    path_root_edge_phygrm,
};
use crate::{
    Float, NodeData, Rc, RectVals, TreSty, TreeState, ellipsize_unicode,
};
use dendros::Edge;
use oxidize_pdf::{
    Document, Page, PdfError,
    graphics::{Color, GraphicsContext, LineCap, LineJoin},
    text::{Font, measure_text},
};
use rayon::prelude::*;
use riced::{
    CnvStrk, IcedPath, LyonPath, LyonPathEvent, PathBuilder, Point, Rectangle,
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn tree_to_pdf(
    path_buf: PathBuf,
    tre_vs: RectVals<Float>,
    cnv_w: Float,
    cnv_h: Float,
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    opn_angle: Float,
    rot_angle: Float,
    root_len: Float,
    // --------------------------------
    lab_size_tip: Float,
    lab_size_int: Float,
    lab_size_brnch: Float,
    // --------------------------------
    lab_offset_tip: Float,
    lab_offset_int: Float,
    lab_offset_brnch: Float,
    // --------------------------------
    align_tip_labs: bool,
    trim_tip_labs: bool,
    trim_tip_labs_to_nchar: u16,
    // --------------------------------
    draw_labs_tip: bool,
    draw_labs_int: bool,
    draw_labs_brnch: bool,
    _draw_clade_labs: bool,
    _draw_legend: bool,
    draw_debug: bool,
    // --------------------------------
) -> Result<(), PdfError> {
    let scaling: f64 = 1e0;
    let cnv_w = cnv_w as f64 * scaling;
    let cnv_h = cnv_h as f64 * scaling;
    let tre_w = tre_vs.w as f64 * scaling;
    let tre_h = tre_vs.h as f64 * scaling;
    let radius = tre_vs.radius_min as f64 * scaling;
    let margin = radius / 1e1;
    let root_len = root_len as f64 * scaling;
    let rot_angle = rot_angle as f64;
    let lab_size_tip = lab_size_tip as f64 * scaling;
    let lab_size_int = lab_size_int as f64 * scaling;
    let lab_size_brnch = lab_size_brnch as f64 * scaling;
    let lab_offset_tip = lab_offset_tip as f64 * scaling;
    let lab_offset_int = lab_offset_int as f64 * scaling;
    let lab_offset_brnch = lab_offset_brnch as f64 * scaling;

    let mut pg = Page::new(cnv_w + margin * 2e0, cnv_h + margin * 2e0);
    pg.set_margins(margin, margin, margin, margin);

    _ = pg.graphics().translate(margin, cnv_h + margin);

    // Bounds ------------------------------------------------------------------
    if draw_debug {
        let path_cnv_bounds = PathBuilder::new()
            .rectangle(Rectangle {
                x: 0e0,
                y: 0e0,
                width: cnv_w as Float,
                height: cnv_h as Float,
            })
            .build();
        let path_tre_bounds = PathBuilder::new()
            .rectangle(Rectangle {
                x: tre_vs.x0 * scaling as Float,
                y: tre_vs.y0 * scaling as Float,
                width: tre_w as Float,
                height: tre_h as Float,
            })
            .build();
        _ = pg.graphics().save_state();
        _ = apply_iced_path_to_gc(path_cnv_bounds, pg.graphics());
        _ = apply_iced_path_to_gc(path_tre_bounds, pg.graphics());
        _ = pg.graphics().stroke();
        _ = pg.graphics().restore_state();
    }
    // -------------------------------------------------------------------------

    match tree_style {
        TreSty::PhyGrm => {
            _ = pg.graphics().translate(
                tre_vs.x0 as f64 * scaling,
                -tre_vs.y0 as f64 * scaling,
            );
        }
        TreSty::Fan => {
            _ = pg.graphics().translate(
                tre_vs.cntr_x as f64 * scaling,
                -tre_vs.cntr_y as f64 * scaling,
            );
            _ = pg.graphics().rotate(-rot_angle);
        }
    };

    // Clade highlights --------------------------------------------------------
    // -------------------------------------------------------------------------

    // Root edge ---------------------------------------------------------------
    if let Some(root_edge) = tree_state.edge_root()
        && root_len > 0.0
    {
        draw_root(
            tree_style,
            tre_w as Float,
            tre_h as Float,
            opn_angle,
            root_len as Float,
            radius as Float,
            &root_edge,
            pg.graphics(),
        );
    } // -----------------------------------------------------------------------

    // Tree edges --------------------------------------------------------------
    draw_edges(
        tree_state.clone(),
        tree_style,
        tre_w as Float,
        tre_h as Float,
        opn_angle,
        root_len as Float,
        radius as Float,
        pg.graphics(),
    ); // ----------------------------------------------------------------------

    // Text labels -------------------------------------------------------------
    let edges: &Vec<Edge> = tree_state.edges_srtd_y().unwrap();
    let node_data: Vec<NodeData> = edges
        .par_iter()
        .map(|edge| match tree_style {
            TreSty::PhyGrm => {
                node_data_cart(tre_w as Float, tre_h as Float, edge).into()
            }
            TreSty::Fan => node_data_rad(
                opn_angle, 0e0, radius as Float, root_len as Float, edge,
            )
            .into(),
        })
        .collect();

    let font = Font::Courier.with_recommended_encoding();

    for nd in node_data {
        let edge = &edges[nd.edge_idx];

        let mut angle = 0e0;

        if let Some(a) = nd.angle {
            angle = a as f64;
        }

        if edge.parent_node_id.is_some() && draw_labs_brnch {
            let text = format!("{:.3}", edge.brlen);
            let text_w = measure_text(&text, font.font, lab_size_brnch);
            write_text(
                &text,
                nd.points.p_mid.x as f64,
                -nd.points.p_mid.y as f64,
                text_w,
                lab_size_brnch,
                -text_w / 2e0,
                lab_offset_brnch,
                None,
                angle,
                rot_angle,
                font.font,
                &mut pg,
            );
        }

        if let Some(text) = &edge.name {
            let lab_size;
            let lab_offset_x;
            let lab_offset_y;
            let mut align_at: Option<f64> = None;
            let mut text_trimmed: String = text.to_string();
            if edge.is_tip && draw_labs_tip {
                if trim_tip_labs {
                    text_trimmed = ellipsize_unicode(
                        text_trimmed,
                        trim_tip_labs_to_nchar.into(),
                    );
                }

                lab_size = lab_size_tip;
                lab_offset_x = lab_offset_tip;
                lab_offset_y = lab_size_tip / 4e0;
                if align_tip_labs {
                    align_at = Some(match tree_style {
                        TreSty::PhyGrm => tre_w,
                        TreSty::Fan => radius,
                    });
                }
            } else if draw_labs_int {
                lab_size = lab_size_int;
                lab_offset_x = lab_offset_int;
                lab_offset_y = lab_size_int / 4e0;
            } else {
                continue;
            }

            let text_w = measure_text(&text_trimmed, font.font, lab_size);
            write_text(
                &text_trimmed, nd.points.p1.x as f64, -nd.points.p1.y as f64,
                text_w, lab_size, lab_offset_x, lab_offset_y, align_at, angle,
                rot_angle, font.font, &mut pg,
            );
        }
    }
    // -------------------------------------------------------------------------

    let mut doc = Document::new();
    doc.set_default_font_encoding(font.encoding);
    doc.add_page(pg);
    doc.set_title("TreeHouse Exported PDF");
    doc.set_producer("TreeHouse");
    doc.save(path_buf)
}

#[allow(clippy::too_many_arguments)]
fn write_text(
    text: &str,
    x: f64,
    y: f64,
    text_w: f64,
    lab_size: f64,
    lab_offset_x: f64,
    lab_offset_y: f64,
    align_at: Option<f64>,
    angle: f64,
    rot_angle: f64,
    font: Font,
    pg: &mut Page,
) {
    let mut lab_offset_x = lab_offset_x;
    let mut lab_offset_y = lab_offset_y;
    let mut x = x;
    let mut y = y;

    let mut x_orig = x;
    let mut y_orig = y;

    let mut angle = angle;
    let angle_orig = angle;

    if let Some(align_at) = align_at {
        if angle == 0.0 {
            x = align_at;
        } else {
            let (sin, cos) = (-angle_orig).sin_cos();
            x = align_at * cos;
            y = align_at * sin;
        }
    }

    if align_at.is_some() {
        let (sin, cos) = (angle_orig).sin_cos();
        x_orig += cos * lab_offset_x;
        y_orig -= sin * lab_offset_x;

        let pt_lab = Point { x: x as Float, y: -y as Float };
        let pt_edge = Point { x: x_orig as Float, y: -y_orig as Float };
        if pt_lab.distance(pt_edge) > lab_offset_x as Float * 2e0 {
            let path =
                PathBuilder::new().move_to(pt_lab).line_to(pt_edge).build();

            _ = apply_iced_path_to_gc(
                path,
                apply_iced_stroke_to_gc(STRK_EDGE_LAB_ALN, pg.graphics()),
            );

            _ = pg.graphics().stroke();
        }
    }

    let (sin, cos) = angle.sin_cos();
    // = Rotate labels on the left side of the circle by 180 degrees
    let a = (angle + rot_angle) % TAU;
    if a > FRAC_PI_2 && a < PI + FRAC_PI_2 {
        angle += PI;
        lab_offset_x += text_w;
        lab_offset_y = -lab_offset_y;
    } // ===========================================================
    _ = pg
        .text()
        .set_text_angle(-angle)
        .set_font(font, lab_size)
        .at(
            x + (cos * lab_offset_x - sin * lab_offset_y),
            y - (sin * lab_offset_x + cos * lab_offset_y),
        )
        .write(text);
}

fn draw_root(
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    root_len: Float,
    radius: Float,
    root_edge: &Edge,
    gc: &mut GraphicsContext,
) {
    _ = apply_iced_path_to_gc(
        match tree_style {
            TreSty::PhyGrm => path_root_edge_phygrm(w, h, root_len, root_edge),
            TreSty::Fan => {
                path_root_edge_fan(radius, opn_angle, root_len, root_edge)
            }
        },
        apply_iced_stroke_to_gc(STRK_ROOT, gc),
    );
}

fn draw_edges(
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    root_len: Float,
    radius: Float,
    gc: &mut GraphicsContext,
) {
    if let Some(edges) = tree_state.edges_srtd_y() {
        _ = apply_iced_path_to_gc(
            match tree_style {
                TreSty::PhyGrm => path_edges_phygrm(edges, w, h),
                TreSty::Fan => {
                    path_edges_fan(edges, opn_angle, root_len, radius)
                }
            },
            apply_iced_stroke_to_gc(STRK_EDGE, gc),
        )
        .stroke();
    }
}

fn apply_iced_path_to_gc(
    iced_path: IcedPath,
    gc: &mut GraphicsContext,
) -> &mut GraphicsContext {
    apply_lyon_path_to_gc(iced_path.raw(), gc);
    gc
}

fn apply_iced_stroke_to_gc<'a>(
    stroke_iced: CnvStrk<'a>,
    gc: &'a mut GraphicsContext,
) -> &'a mut GraphicsContext {
    let line_cap = line_cap_from_iced(stroke_iced.line_cap);
    let line_join = line_join_from_iced(stroke_iced.line_join);
    let line_width = stroke_iced.width as f64;
    let stroke_color = color_from_iced_stroke(stroke_iced);
    gc.set_line_cap(line_cap)
        .set_line_join(line_join)
        .set_line_width(line_width)
        .set_stroke_color(stroke_color)
}

fn apply_lyon_path_to_gc(lyon_path: &LyonPath, gc: &mut GraphicsContext) {
    let mut current = None;
    for event in lyon_path.iter() {
        match event {
            LyonPathEvent::Begin { at } => {
                current = Some(at);
                _ = gc.move_to(at.x as f64, -at.y as f64);
            }
            LyonPathEvent::Line { mut from, to } => {
                from.y *= -1e0;
                if let Some(current) = current {
                    if from != current {
                        _ = gc.move_to(from.x as f64, from.y as f64);
                    }
                }
                _ = gc.line_to(to.x as f64, -to.y as f64);
                current = Some(to);
            }
            LyonPathEvent::Quadratic { from: _, ctrl: _, to } => {
                // if let Some(current) = current {
                //     from.y *= -1e0;
                //     if from != current {
                //         _ = gc.move_to(from.x as f64, from.y as f64);
                //     }
                // }
                _ = gc.move_to(to.x as f64, -to.y as f64);
                current = Some(to);
            }
            LyonPathEvent::Cubic { mut from, ctrl1, ctrl2, to } => {
                if let Some(current) = current {
                    from.y *= -1e0;
                    if from != current {
                        _ = gc.move_to(from.x as f64, from.y as f64);
                    }
                }
                _ = gc.curve_to(
                    ctrl1.x as f64, -ctrl1.y as f64, ctrl2.x as f64,
                    -ctrl2.y as f64, to.x as f64, -to.y as f64,
                );
                current = Some(to);
            }
            LyonPathEvent::End { mut last, first, close } => {
                if let Some(current) = current {
                    last.y *= -1e0;
                    if last != current {
                        _ = gc.move_to(last.x as f64, last.y as f64);
                    }
                }
                if close {
                    _ = gc.line_to(first.x as f64, -first.y as f64);
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
