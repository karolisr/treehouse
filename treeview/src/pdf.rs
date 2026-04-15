mod draw;
mod object_conversion;
mod utils;

use draw::*;
use object_conversion::*;
use utils::*;

use crate::edge_utils::{node_data_cart, node_data_pol};
use crate::{
    Float, NodeData, Rc, RectVals, TreSty, TreeState, ellipsize_unicode,
};
use dendros::Edge;
use num_traits::{AsPrimitive, real::Real};
use oxidize_pdf::{
    Document, Page, PdfError,
    text::{Font, measure_text},
};
use rayon::prelude::*;
use riced::fonts::JET_BRAINS_MONO_REGULAR;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn tree_to_pdf<
    T: Real + AsPrimitive<Float> + AsPrimitive<f64> + AsPrimitive<f32>,
>(
    path_buf: PathBuf,
    tre_vs: RectVals<T>,
    cnv_vs: RectVals<T>,
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    opn_angle: T,
    rot_angle: T,
    root_len: T,
    // --------------------------------
    lab_size_tip: T,
    lab_size_int: T,
    lab_size_brnch: T,
    // --------------------------------
    lab_offset_tip: T,
    lab_offset_int: T,
    lab_offset_brnch: T,
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
    let dim_max = AsPrimitive::<f64>::as_(cnv_vs.dim_max);
    let margin = 72.0 / 2.0;
    let max_page_dim_points = (72.0 * 200.0) - (margin * 2.0);
    let scaling: f64 = if dim_max >= max_page_dim_points {
        max_page_dim_points / dim_max
    } else {
        1e0
    };

    let cnv_vs_float: RectVals<Float> =
        cnv_vs.type_converted().scale(scaling as Float);

    let tre_vs_float: RectVals<Float> =
        tre_vs.type_converted().scale(scaling as Float);

    let cnv_vs_f64: RectVals<f64> = cnv_vs.type_converted().scale(scaling);
    let tre_vs_f64: RectVals<f64> = tre_vs.type_converted().scale(scaling);

    let rot_angle: f64 = rot_angle.as_();
    let opn_angle: Float = opn_angle.as_();
    let root_len: Float =
        (AsPrimitive::<f64>::as_(root_len) * scaling) as Float;

    let lab_size_tip: f64 = AsPrimitive::<f64>::as_(lab_size_tip) * scaling;
    let lab_size_int: f64 = AsPrimitive::<f64>::as_(lab_size_int) * scaling;
    let lab_size_brnch: f64 = AsPrimitive::<f64>::as_(lab_size_brnch) * scaling;
    let lab_offset_tip: f64 = AsPrimitive::<f64>::as_(lab_offset_tip) * scaling;
    let lab_offset_int: f64 = AsPrimitive::<f64>::as_(lab_offset_int) * scaling;
    let lab_offset_brnch: f64 =
        AsPrimitive::<f64>::as_(lab_offset_brnch) * scaling;

    let mut pg =
        Page::new(cnv_vs_f64.w + margin * 2e0, cnv_vs_f64.h + margin * 2e0);
    pg.set_margins(margin, margin, margin, margin);
    _ = pg.graphics().translate(margin, cnv_vs_f64.h + margin);

    // Bounds ------------------------------------------------------------------
    if draw_debug {
        draw_bounds(&cnv_vs_float, &tre_vs_float, pg.graphics());
    } // -----------------------------------------------------------------------

    match tree_style {
        TreSty::PhyGrm => {
            _ = pg.graphics().translate(tre_vs_f64.x0, -tre_vs_f64.y0);
        }
        TreSty::Fan => {
            _ = pg.graphics().translate(tre_vs_f64.cntr_x, -tre_vs_f64.cntr_y);
            _ = pg.graphics().rotate(-rot_angle);
        }
    };

    // Clade highlights --------------------------------------------------------
    if draw_clade_highlights_ok {
        draw_clade_highlights(
            tree_state.clone(),
            &cnv_vs_float,
            &tre_vs_float,
            root_len,
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
            &tre_vs_float,
            opn_angle,
            root_len,
            &root_edge,
            scaling,
            tree_style,
            pg.graphics(),
        );
    } // -----------------------------------------------------------------------

    // Tree edges --------------------------------------------------------------
    draw_edges(
        &tre_vs_float,
        tree_state.clone(),
        opn_angle,
        root_len,
        scaling,
        tree_style,
        pg.graphics(),
    ); // ----------------------------------------------------------------------

    // Text labels -------------------------------------------------------------
    let edges: &Vec<Edge> = tree_state.edges().unwrap();
    let node_data: Vec<NodeData> = edges
        .par_iter()
        .map(|edge| match tree_style {
            TreSty::PhyGrm => {
                node_data_cart(tre_vs_float.w, tre_vs_float.h, edge).into()
            }
            TreSty::Fan => node_data_pol(
                opn_angle, 0e0, tre_vs_float.radius_min, root_len, edge,
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
                        TreSty::PhyGrm => tre_vs_f64.w,
                        TreSty::Fan => tre_vs_f64.radius_min,
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
    doc.set_creator("TreeHouse");
    doc.save(path_buf)
}
