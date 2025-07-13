use crate::consts::{FNT_NAME, STRK_EDGE, STRK_ROOT, ZRO};
use crate::path_builders::*;
use crate::{Float, Rc, TreSty, TreeState};
use dendros::Edge;
use riced::PathBuilder;
use roarsvg::{
    FontProvider, LyonWriter, NoText, SvgTransform, create_text_node, fill,
    stroke,
};
use std::path::PathBuf;
use usvg_tree::{
    Color, DominantBaseline, Fill, FillRule, LineCap, LineJoin, Opacity, Paint,
    Stroke, StrokeMiterlimit, StrokeWidth, Transform,
};

#[allow(clippy::too_many_arguments)]
pub fn svg_writer_tree(
    path_buf: PathBuf,
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    rot_angle: Float,
    root_len: Float,
    radius: Float,
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
) -> Option<()> {
    let mut writer: LyonWriter<NoText> = LyonWriter::new();

    // Clade labels ------------------------------------------------------------
    writer = clade_labels_writer(
        tree_state.clone(),
        tree_style,
        w,
        h,
        opn_angle,
        rot_angle,
        root_len,
        radius,
        writer,
    )?;
    // -------------------------------------------------------------------------

    // Root edge ---------------------------------------------------------------
    if let Some(root_edge) = tree_state.edge_root()
        && root_len > ZRO
    {
        writer = root_edge_writer(
            tree_style, w, h, opn_angle, rot_angle, root_len, radius,
            &root_edge, writer,
        )?;
    }
    // -------------------------------------------------------------------------

    // Tree edges --------------------------------------------------------------
    writer = edges_writer(
        tree_state.clone(),
        tree_style,
        w,
        h,
        opn_angle,
        rot_angle,
        root_len,
        radius,
        writer,
    )?;
    // -------------------------------------------------------------------------

    // Tip labels --------------------------------------------------------------
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    let mut writer = writer.add_fonts(fontdb);
    let edges: &Vec<Edge> = tree_state.edges_srtd_y()?;
    let font_families: Vec<String> = vec![FNT_NAME.to_string()];
    let font_size: Float = lab_size_int;

    for edge in edges {
        if let Some(text) = &edge.name {
            let text = text.to_string();
            writer
                .push_text(
                    text,
                    font_families.clone(),
                    font_size,
                    SvgTransform::from_translate(
                        edge.x1 as Float * w,
                        edge.y as Float * h + font_size / 2e0,
                    ),
                    Some(Fill::default()),
                    Some(Stroke::default()),
                    DominantBaseline::Auto,
                )
                .ok()?;
        }
    }
    // -------------------------------------------------------------------------

    let res = writer.write(path_buf);
    match res {
        Ok(_) => Some(()),
        Err(err) => {
            println!("ERR: {err:?}");
            None
        }
    }
}

fn clade_labels_writer(
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    rot_angle: Float,
    root_len: Float,
    radius: Float,
    mut writer: LyonWriter<NoText>,
) -> Option<LyonWriter<NoText>> {
    let labeled_clades = tree_state.labeled_clades();
    for (node_id, clade_label) in labeled_clades {
        let pb_clade_label = match tree_style {
            TreSty::PhyGrm => {
                path_builder_clade_highlight_phygrm(*node_id, &tree_state, w, h)
            }
            TreSty::Fan => path_builder_clade_highlight_fan(
                *node_id, &tree_state, radius, root_len, opn_angle,
            ),
        };

        let stroke_clade_label =
            stroke_usvg_from_iced(&STRK_EDGE.with_color(clade_label.color));

        let fill_clade_label = fill_usvg_from_iced(&riced::CnvFill {
            style: riced::GeomStyle::Solid(clade_label.color),
            rule: riced::FillRule::EvenOdd,
        });

        writer
            .push(
                pb_clade_label.build().raw(),
                Some(fill_clade_label),
                Some(stroke_clade_label),
                // None,
                match tree_style {
                    TreSty::PhyGrm => None,
                    TreSty::Fan => {
                        Some(Transform::from_rotate(rot_angle.to_degrees()))
                    }
                },
            )
            .ok()?;
    }
    Some(writer)
}

fn root_edge_writer(
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    rot_angle: Float,
    root_len: Float,
    radius: Float,
    root_edge: &Edge,
    mut writer: LyonWriter<NoText>,
) -> Option<LyonWriter<NoText>> {
    let pb_root = match tree_style {
        TreSty::PhyGrm => {
            path_builder_root_edge_phygrm(w, h, root_len, root_edge)
        }
        TreSty::Fan => {
            path_builder_root_edge_fan(radius, opn_angle, root_len, root_edge)
        }
    };

    let stroke_root = stroke_usvg_from_iced(&STRK_ROOT);

    writer
        .push(
            pb_root.build().raw(),
            None,
            Some(stroke_root),
            match tree_style {
                TreSty::PhyGrm => None,
                TreSty::Fan => {
                    Some(Transform::from_rotate(rot_angle.to_degrees()))
                }
            },
        )
        .ok()?;
    Some(writer)
}

fn edges_writer(
    tree_state: Rc<TreeState>,
    tree_style: TreSty,
    w: Float,
    h: Float,
    opn_angle: Float,
    rot_angle: Float,
    root_len: Float,
    radius: Float,
    mut writer: LyonWriter<NoText>,
) -> Option<LyonWriter<NoText>> {
    let edges: &Vec<Edge> = tree_state.edges_srtd_y()?;
    let pb_edges: PathBuilder = match tree_style {
        TreSty::PhyGrm => path_builder_edges_phygrm(edges, w, h),
        TreSty::Fan => {
            path_builder_edges_fan(edges, opn_angle, root_len, radius)
        }
    };
    let stroke_edge = stroke_usvg_from_iced(&STRK_EDGE);
    writer
        .push(
            pb_edges.build().raw(),
            None,
            Some(stroke_edge),
            match tree_style {
                TreSty::PhyGrm => None,
                TreSty::Fan => {
                    Some(Transform::from_rotate(rot_angle.to_degrees()))
                }
            },
        )
        .ok()?;
    Some(writer)
}

fn color_usvg_from_iced(color_iced: &riced::Color) -> (Color, Opacity) {
    let c: [u8; 4] = color_iced.into_rgba8();
    (Color { red: c[0], green: c[1], blue: c[2] }, Opacity::new_u8(c[3]))
}

fn paint_usvg_from_iced_geom_style(
    geom_style_iced: &riced::GeomStyle,
) -> Paint {
    match geom_style_iced {
        riced::GeomStyle::Solid(color) => {
            Paint::Color(color_usvg_from_iced(color).0)
        }
        riced::GeomStyle::Gradient(_gradient) => Paint::Color(Color::black()),
    }
}

fn stroke_usvg_from_iced(stroke_iced: &riced::CnvStrk) -> Stroke {
    let color_iced = match stroke_iced.style {
        riced::GeomStyle::Solid(color) => color,
        riced::GeomStyle::Gradient(_gradient) => riced::Color::BLACK,
    };

    let (color, opacity) = color_usvg_from_iced(&color_iced);

    let linecap = match stroke_iced.line_cap {
        riced::LineCap::Butt => LineCap::Butt,
        riced::LineCap::Square => LineCap::Square,
        riced::LineCap::Round => LineCap::Round,
    };

    let linejoin = match stroke_iced.line_join {
        riced::LineJoin::Miter => LineJoin::Miter,
        riced::LineJoin::Round => LineJoin::Round,
        riced::LineJoin::Bevel => LineJoin::Bevel,
    };

    let riced::LineDash { segments, offset } = stroke_iced.line_dash;
    let dasharray: Option<Vec<f32>> =
        if segments.is_empty() { None } else { Some(segments.to_vec()) };

    let dashoffset = offset as f32;

    Stroke {
        paint: Paint::Color(color),
        dasharray,
        dashoffset,
        miterlimit: StrokeMiterlimit::default(),
        opacity,
        width: StrokeWidth::new(stroke_iced.width).unwrap(),
        linecap,
        linejoin,
    }
}

fn fill_usvg_from_iced(fill_iced: &riced::CnvFill) -> Fill {
    let paint = paint_usvg_from_iced_geom_style(&fill_iced.style);
    let color_iced = match fill_iced.style {
        riced::GeomStyle::Solid(color) => color,
        riced::GeomStyle::Gradient(_gradient) => riced::Color::BLACK,
    };
    let (_color, opacity) = color_usvg_from_iced(&color_iced);
    let rule = match fill_iced.rule {
        riced::FillRule::NonZero => FillRule::NonZero,
        riced::FillRule::EvenOdd => FillRule::EvenOdd,
    };
    Fill { paint, opacity, rule }
}
