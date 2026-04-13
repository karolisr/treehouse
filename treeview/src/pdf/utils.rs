use super::*;

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

#[allow(clippy::too_many_arguments)]
pub(super) fn write_text(
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
    scaling: f64,
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
                apply_iced_stroke_to_gc(
                    STRK_EDGE_LAB_ALN,
                    scaling,
                    pg.graphics(),
                ),
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
        .graphics()
        .save_state()
        .translate(
            x + (cos * lab_offset_x - sin * lab_offset_y),
            y - (sin * lab_offset_x + cos * lab_offset_y),
        )
        .rotate(-angle)
        .set_font(font, lab_size)
        .begin_text()
        .show_text(text)
        .unwrap()
        .end_text()
        .restore_state();
}
