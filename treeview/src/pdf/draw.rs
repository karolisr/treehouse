use super::*;

use crate::consts::{STRK_EDGE, STRK_ROOT};
use crate::path_builders::{
    path_clade_highlight, path_edges_fan, path_edges_phygrm,
    path_root_edge_fan, path_root_edge_phygrm,
};
use crate::{Float, Rc, RectVals, TreSty, TreeState};
use dendros::Edge;
use oxidize_pdf::{PdfError, graphics::GraphicsContext};
use riced::{PathBuilder, Rectangle};

pub(super) fn draw_clade_highlights(
    tree_state: Rc<TreeState>,
    cnv_vs: &RectVals<Float>,
    tre_vs: &RectVals<Float>,
    root_len: Float,
    opn_angle: Float,
    tree_style: TreSty,
    gc: &mut GraphicsContext,
) -> Result<(), PdfError> {
    let highlighted_clades = tree_state.highlighted_clades();
    for node_id in tree_state.node_ids_srtd_asc() {
        if highlighted_clades.contains_key(&node_id) {
            let clade_highlight = highlighted_clades.get(&node_id).unwrap();

            let iced_path = path_clade_highlight(
                node_id, &tree_state, tre_vs.w, tre_vs.h, cnv_vs.x1,
                tre_vs.radius_min, cnv_vs.radius_min, root_len, opn_angle,
                tree_style,
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
    cnv_vs: &RectVals<Float>,
    tre_vs: &RectVals<Float>,
    gc: &mut GraphicsContext,
) {
    let path_cnv_bounds = PathBuilder::new()
        .rectangle(Rectangle {
            x: 0e0,
            y: 0e0,
            width: cnv_vs.w,
            height: cnv_vs.h,
        })
        .build();
    let path_tre_bounds = PathBuilder::new()
        .rectangle(Rectangle {
            x: tre_vs.x0,
            y: tre_vs.y0,
            width: tre_vs.w,
            height: tre_vs.h,
        })
        .build();
    _ = gc.save_state();
    _ = apply_iced_path_to_gc(path_cnv_bounds, gc);
    _ = apply_iced_path_to_gc(path_tre_bounds, gc);
    _ = gc.stroke();
    _ = gc.restore_state();
}

pub(super) fn draw_root(
    tre_vs: &RectVals<Float>,
    opn_angle: Float,
    root_len: Float,
    root_edge: &Edge,
    scaling: f64,
    tree_style: TreSty,
    gc: &mut GraphicsContext,
) {
    _ = apply_iced_path_to_gc(
        match tree_style {
            TreSty::PhyGrm => {
                path_root_edge_phygrm(tre_vs.w, tre_vs.h, root_len, root_edge)
            }
            TreSty::Fan => path_root_edge_fan(
                tre_vs.radius_min, opn_angle, root_len, root_edge,
            ),
        },
        apply_iced_stroke_to_gc(STRK_ROOT, scaling, gc),
    )
    .stroke();
}

pub(super) fn draw_edges(
    tre_vs: &RectVals<Float>,
    tree_state: Rc<TreeState>,
    opn_angle: Float,
    root_len: Float,
    scaling: f64,
    tree_style: TreSty,
    gc: &mut GraphicsContext,
) {
    if let Some(edges) = tree_state.edges() {
        _ = apply_iced_path_to_gc(
            match tree_style {
                TreSty::PhyGrm => path_edges_phygrm(edges, tre_vs.w, tre_vs.h),
                TreSty::Fan => path_edges_fan(
                    edges, opn_angle, root_len, tre_vs.radius_min,
                ),
            },
            apply_iced_stroke_to_gc(STRK_EDGE, scaling, gc),
        )
        .stroke();
    }
}
