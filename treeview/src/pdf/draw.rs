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
pub(super) fn draw_clade_highlights(
    tree_state: Rc<TreeState>,
    tre_w: f64,
    tre_h: f64,
    tre_radius: f64,
    root_len: f64,
    clade_highlight_max_x: f64,
    clade_highlight_max_radius: f64,
    opn_angle: Float,
    tree_style: TreSty,
    gc: &mut GraphicsContext,
) -> Result<(), PdfError> {
    let highlighted_clades = tree_state.highlighted_clades();
    for node_id in tree_state.node_ids_srtd_asc() {
        if highlighted_clades.contains_key(&node_id) {
            let clade_highlight = highlighted_clades.get(&node_id).unwrap();

            let iced_path = path_clade_highlight(
                node_id, &tree_state, tre_w as Float, tre_h as Float,
                clade_highlight_max_x as Float, tre_radius as Float,
                clade_highlight_max_radius as Float, root_len as Float,
                opn_angle, tree_style,
            );

            let color = color_from_iced_color(clade_highlight.color);
            let alpha = alpha_from_iced_color(clade_highlight.color);

            _ = gc.save_state();
            _ = apply_iced_path_to_gc(iced_path.clone(), gc);
            _ = gc.set_fill_color(color).set_alpha_fill(alpha)?;
            _ = gc.fill();
            _ = gc.restore_state();
        }
    }
    Ok(())
}

pub(super) fn draw_bounds(
    tre_vs: &RectVals<Float>,
    cnv_w: f64,
    cnv_h: f64,
    tre_w: f64,
    tre_h: f64,
    scaling: f64,
    gc: &mut GraphicsContext,
) {
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
    _ = gc.save_state();
    _ = apply_iced_path_to_gc(path_cnv_bounds, gc);
    _ = apply_iced_path_to_gc(path_tre_bounds, gc);
    _ = gc.stroke();
    _ = gc.restore_state();
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_root(
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    root_len: Float,
    radius: Float,
    root_edge: &Edge,
    scaling: f64,
    gc: &mut GraphicsContext,
) {
    _ = apply_iced_path_to_gc(
        match tree_style {
            TreSty::PhyGrm => path_root_edge_phygrm(w, h, root_len, root_edge),
            TreSty::Fan => {
                path_root_edge_fan(radius, opn_angle, root_len, root_edge)
            }
        },
        apply_iced_stroke_to_gc(STRK_ROOT, scaling, gc),
    )
    .stroke();
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_edges(
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    root_len: Float,
    radius: Float,
    scaling: f64,
    gc: &mut GraphicsContext,
) {
    if let Some(edges) = tree_state.edges() {
        _ = apply_iced_path_to_gc(
            match tree_style {
                TreSty::PhyGrm => path_edges_phygrm(edges, w, h),
                TreSty::Fan => {
                    path_edges_fan(edges, opn_angle, root_len, radius)
                }
            },
            apply_iced_stroke_to_gc(STRK_EDGE, scaling, gc),
        )
        .stroke();
    }
}
