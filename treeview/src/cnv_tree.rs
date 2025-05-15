use crate::Float;
use crate::RectVals;
use crate::TREE_LAB_FONT_NAME;
use crate::TreeView;
use crate::TvMsg;
use crate::cnv_utils::*;
use crate::treestate::TreeState;
use iced::Point;
use iced::Vector;
use iced::widget::text::Alignment as TextAlignment;
use iced::{
    Event, Rectangle, Renderer, Theme,
    alignment::Vertical,
    mouse::{Cursor, Interaction},
    widget::canvas::{Action, Geometry, LineCap, LineJoin, Program, Stroke, Text},
};
use utils::Clr;

#[derive(Debug)]
pub struct TreeCnvState {
    stroke_edge: Stroke<'static>,

    stroke_1: Stroke<'static>,
    stroke_2: Stroke<'static>,
    stroke_3: Stroke<'static>,

    lab_txt_template: Text,

    bounds: Rectangle,
    clip_vals: RectVals<Float>,
    tree_vals: RectVals<Float>,
}

impl Default for TreeCnvState {
    fn default() -> Self {
        Self {
            stroke_edge: Stroke {
                width: 1e0,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                ..Default::default()
            },

            stroke_1: Stroke {
                width: 6e0,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                style: Clr::RED.scale_alpha(0.5).into(),
                ..Default::default()
            },

            stroke_2: Stroke {
                width: 4e0,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                style: Clr::GRN.scale_alpha(0.5).into(),
                ..Default::default()
            },

            stroke_3: Stroke {
                width: 2e0,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                style: Clr::BLU.scale_alpha(0.5).into(),
                ..Default::default()
            },

            lab_txt_template: Text {
                font: iced::Font {
                    family: iced::font::Family::Name(TREE_LAB_FONT_NAME),
                    ..Default::default()
                },
                size: iced::Pixels(1e0),
                align_x: TextAlignment::Left,
                align_y: Vertical::Center,
                ..Default::default()
            },

            bounds: Default::default(),
            clip_vals: Default::default(),
            tree_vals: Default::default(),
        }
    }
}

impl Program<TvMsg> for TreeView {
    type State = TreeCnvState;

    fn mouse_interaction(
        &self, _state: &Self::State, _bounds: Rectangle, _cursor: Cursor,
    ) -> Interaction {
        Interaction::default()
    }

    fn update(
        &self, state: &mut Self::State, _event: &Event, bounds: Rectangle, _cursor: Cursor,
    ) -> Option<Action<TvMsg>> {
        if bounds != state.bounds {
            state.bounds = bounds;
            state.clip_vals = RectVals::clip(bounds);
            state.tree_vals = RectVals::tree(state.clip_vals, 5e1);
            return Some(Action::publish(TvMsg::RectValsChanged(state.tree_vals)));
        }
        None
    }

    fn draw(
        &self, state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut geoms: Vec<Geometry> = Vec::new();

        if !self.drawing_enabled {
            return geoms;
        }

        let tree: &TreeState;
        if let Some(t) = self.get_sel_tree() {
            tree = t;
        } else {
            return geoms;
        }

        let g_bounds = self.cache_bounds.draw(renderer, bounds.size(), |f| {
            draw_rectangle(state.clip_vals.into(), state.stroke_1, f);
            draw_rectangle(state.tree_vals.into(), state.stroke_2, f);
            draw_rectangle(self.tree_vals.into(), state.stroke_3, f);
        });
        geoms.push(g_bounds);

        let g_edge = tree.cache_edge().draw(renderer, bounds.size(), |f| {
            let paths = paths_from_edges(
                state.tree_vals.w,
                state.tree_vals.h,
                state.tree_vals.cntr_untrans,
                state.tree_vals.radius_min,
                tree.is_rooted(),
                self.rot_angle,
                self.opn_angle,
                self.sel_tree_style_opt,
                tree.edges(),
            );

            draw_edges(paths, state.stroke_edge, Some(state.tree_vals.trans), f);
        });
        geoms.push(g_edge);

        if self.tip_brnch_labs_allowed && tree.has_tip_labs() && self.draw_tip_labs {
            let g_lab_tip = tree.cache_lab_tip().draw(renderer, bounds.size(), |f| {
                let labels = node_labels(tree.all_nodepoints(), true, &state.lab_txt_template);
                draw_labels(
                    labels,
                    self.tip_lab_size,
                    Vector { x: self.tip_lab_offset, y: 0e0 },
                    Some(state.tree_vals.trans),
                    f,
                );
            });
            geoms.push(g_lab_tip);
        }

        if tree.has_int_labs() && self.draw_int_labs {
            let g_lab_int = tree.cache_lab_int().draw(renderer, bounds.size(), |f| {
                let labels = node_labels(tree.all_nodepoints(), false, &state.lab_txt_template);
                draw_labels(
                    labels,
                    self.int_lab_size,
                    Vector { x: self.int_lab_offset, y: 0e0 },
                    Some(state.tree_vals.trans),
                    f,
                );
            });
            geoms.push(g_lab_int);
        }

        if tree.has_brlen() && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            let g_lab_brnch = tree.cache_lab_brnch().draw(renderer, bounds.size(), |f| {
                let labels = branch_labels(
                    tree.all_nodepoints(),
                    state.tree_vals.w,
                    state.tree_vals.radius_min,
                    &state.lab_txt_template,
                );
                draw_labels(
                    labels,
                    self.brnch_lab_size,
                    Vector { x: 0e0, y: -self.brnch_lab_offset },
                    Some(state.tree_vals.trans),
                    f,
                );
            });
            geoms.push(g_lab_brnch);
        }
        geoms
    }
}
