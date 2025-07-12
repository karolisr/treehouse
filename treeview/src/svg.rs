use crate::edge_utils::{path_builder_edges_fan, path_builder_edges_phygrm};
use crate::{Float, Rc, TreSty, TreeState};
use roarsvg::LyonWriter;
use std::path::PathBuf;
use usvg_tree::{
    Color, LineCap, LineJoin, Opacity, Paint, Stroke, StrokeMiterlimit,
    StrokeWidth, Transform,
};

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
) -> Option<()> {
    let edges = tree_state.edges_srtd_y()?;

    let pb_edges = match tree_style {
        TreSty::PhyGrm => path_builder_edges_phygrm(edges, w, h),
        TreSty::Fan => {
            path_builder_edges_fan(edges, opn_angle, root_len, radius)
        }
    };

    let mut writer = LyonWriter::new();
    let stroke_edge: Stroke = Stroke {
        paint: Paint::Color(Color::black()),
        dasharray: None,
        dashoffset: 0.0,
        miterlimit: StrokeMiterlimit::default(),
        opacity: Opacity::ONE,
        width: StrokeWidth::new(1.0).unwrap(),
        linecap: LineCap::Square,
        linejoin: LineJoin::Round,
    };
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

    _ = writer.write(path_buf);
    Some(())
}
