use super::*;

use crate::Float;
use crate::consts::STRK_EDGE_LAB_ALN;
use oxidize_pdf::{
    Page,
    text::{Font, measure_text},
};
use riced::{PathBuilder, Point};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// Top-aligned, horizontally centered label below an iced-space anchor.
/// Mirrors `draw_labels` for angle-0 `Vertical::Top` text: PDF baselines grow
/// up, so without Top→Center `adjust_h` + baseline shift, labels sit too high.
pub(super) fn write_label_below(
    text: &str,
    iced_x: f64,
    iced_y: f64,
    lab_size: f64,
    lab_offset_y: f64,
    center_width: f64,
    font: Font,
    pg: &mut Page,
) {
    // Same Top → Center vertical adjust as draw_labels.
    let magic = lab_size / 8.75;
    let adjust_h_base = lab_size / 2.0 - magic;
    let adjust_h = adjust_h_base.floor() + 2.0 - magic / 3.15;
    let iced_center_y = iced_y + lab_offset_y + adjust_h;

    // Alphabetic baseline sits below the visual center of the em box.
    let baseline_from_center = lab_size * 0.35;
    let pdf_x = iced_x - center_width / 2.0;
    let pdf_y = -iced_center_y - baseline_from_center;

    _ = pg
        .graphics()
        .save_state()
        .translate(pdf_x, pdf_y)
        .set_font(font, lab_size)
        .begin_text()
        .show_text(text)
        .unwrap()
        .end_text()
        .restore_state();
}

/// Centered (both axes) label, mirroring iced canvas `Vertical::Center` /
/// `TextAlignment::Center` text used for GTS band names. `text` may contain
/// `\n` to stack multiple lines around the vertical center, as the UI does
/// when replacing spaces in multi-word GTS names.
pub(super) fn write_label_gts_centered(
    text: &str,
    iced_x: f64,
    iced_y: f64,
    lab_size: f64,
    font: Font,
    pg: &mut Page,
) {
    let line_height = lab_size * 1.5;
    let baseline_from_center = lab_size * 0.35;
    let lines: Vec<&str> = text.split('\n').collect();
    let total_h = (lines.len() as f64 - 1.0) * line_height;

    for (i, line) in lines.iter().enumerate() {
        let line_center_y = iced_y - total_h / 2.0 + i as f64 * line_height;
        let center_width = measure_text(line, &font, lab_size);
        let pdf_x = iced_x - center_width / 2.0;
        let pdf_y = -line_center_y - baseline_from_center;

        _ = pg
            .graphics()
            .save_state()
            .translate(pdf_x, pdf_y)
            .set_font(font.clone(), lab_size)
            .begin_text()
            .show_text(line)
            .unwrap()
            .end_text()
            .restore_state();
    }
}

/// Right-aligned, vertically centered label to the left of an iced-space
/// anchor. Mirrors `draw_labels` for angle-0 `Vertical::Center` /
/// `TextAlignment::Right` text (y-axis tick labels): text ends at
/// `iced_x - lab_offset_x`, baseline centered on `iced_y`.
pub(super) fn write_label_left(
    text: &str,
    iced_x: f64,
    iced_y: f64,
    lab_size: f64,
    lab_offset_x: f64,
    font: Font,
    pg: &mut Page,
) {
    let baseline_from_center = lab_size * 0.35;
    let text_w = measure_text(text, &font, lab_size);
    let pdf_x = iced_x - lab_offset_x - text_w;
    let pdf_y = -iced_y - baseline_from_center;

    _ = pg
        .graphics()
        .save_state()
        .translate(pdf_x, pdf_y)
        .set_font(font, lab_size)
        .begin_text()
        .show_text(text)
        .unwrap()
        .end_text()
        .restore_state();
}

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
