#![allow(unused_imports)]

mod draw;
mod object_conversion;
mod utils;

use draw::*;
use object_conversion::*;
use utils::*;

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
pub fn tree_to_pdf(
    path_buf: PathBuf,
    tre_vs: RectVals<Float>,
    cnv_vs: RectVals<Float>,
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
    draw_clade_highlights_ok: bool,
    _draw_scale_bar: bool,
    draw_debug: bool,
    // --------------------------------
) -> Result<(), PdfError> {
    let scaling: f64 = 1e0;
    let cnv_w = cnv_vs.w as f64 * scaling;
    let cnv_h = cnv_vs.h as f64 * scaling;
    let tre_w = tre_vs.w as f64 * scaling;
    let clade_highlight_max_x = cnv_vs.x1 as f64 * scaling;
    let tre_h = tre_vs.h as f64 * scaling;
    let tre_radius = tre_vs.radius_min as f64 * scaling;
    let clade_highlight_max_radius = cnv_vs.radius_min as f64 * scaling;
    let margin = tre_radius / 1e1;
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
        draw_bounds(
            &tre_vs,
            cnv_w,
            cnv_h,
            tre_w,
            tre_h,
            scaling,
            pg.graphics(),
        );
    } // -----------------------------------------------------------------------

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
    if draw_clade_highlights_ok {
        draw_clade_highlights(
            tree_state.clone(),
            tre_w,
            tre_h,
            tre_radius,
            root_len,
            clade_highlight_max_x,
            clade_highlight_max_radius,
            opn_angle,
            tree_style,
            pg.graphics(),
        )?;
    } // -----------------------------------------------------------------------

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
            tre_radius as Float,
            &root_edge,
            scaling,
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
        tre_radius as Float,
        scaling,
        pg.graphics(),
    ); // ----------------------------------------------------------------------

    // Text labels -------------------------------------------------------------
    let edges: &Vec<Edge> = tree_state.edges().unwrap();
    let node_data: Vec<NodeData> = edges
        .par_iter()
        .map(|edge| match tree_style {
            TreSty::PhyGrm => {
                node_data_cart(tre_w as Float, tre_h as Float, edge).into()
            }
            TreSty::Fan => node_data_pol(
                opn_angle, 0e0, tre_radius as Float, root_len as Float, edge,
            )
            .into(),
        })
        .collect();

    let font_data = JET_BRAINS_MONO_REGULAR.to_vec();
    let font_name = "JetBrainsMono-Regular".to_string();
    let font = Font::Custom(font_name.clone());

    for nd in node_data {
        let edge = &edges[nd.edge_idx];
        if edge.parent_node_id != edge.node_id && draw_labs_brnch {
            let text = format!("{:.3}", edge.branch_length);
            let text_w = measure_text(&text, &font, lab_size_brnch);
            write_text(
                &text,
                nd.points.p_mid.x as f64,
                -nd.points.p_mid.y as f64,
                text_w,
                lab_size_brnch,
                -text_w / 2e0,
                lab_offset_brnch,
                None,
                nd.angle as f64,
                rot_angle,
                font.clone(),
                scaling,
                &mut pg,
            );
        }

        if let Some(text) = &edge.label {
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
                        TreSty::Fan => tre_radius,
                    });
                }
            } else if draw_labs_int {
                lab_size = lab_size_int;
                lab_offset_x = lab_offset_int;
                lab_offset_y = lab_size_int / 4e0;
            } else {
                continue;
            }

            let text_w = measure_text(&text_trimmed, &font, lab_size);

            write_text(
                &text_trimmed,
                nd.points.p1.x as f64,
                -nd.points.p1.y as f64,
                text_w,
                lab_size,
                lab_offset_x,
                lab_offset_y,
                align_at,
                nd.angle as f64,
                rot_angle,
                font.clone(),
                scaling,
                &mut pg,
            );
        }
    } // -----------------------------------------------------------------------

    let mut doc = Document::new();
    _ = doc.add_font_from_bytes(font_name, font_data);
    doc.add_page(pg);
    doc.set_title("TreeHouse Exported PDF");
    doc.set_producer("TreeHouse");
    doc.save(path_buf)
}
