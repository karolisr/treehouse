use super::drawables::IndexRange;
use crate::{Float, SF, TREE_LAB_FONT_NAME};
use dendros::Edge;
use iced::{Point, Rectangle, alignment::Vertical, widget::canvas::Text};

pub struct TreeViewState {
    pub(super) tree_label_template: Text,
    pub(super) tip_idx_range: Option<IndexRange>,
    pub(super) visible_nodes: Vec<(Point, Edge)>,
    pub(super) ps: Float,
    pub(super) closest_node_point: Option<(Point, Edge)>,
    pub(super) mouse_hovering_node: bool,
    pub(super) clip_rect: Rectangle,
    pub(super) tree_rect: Rectangle,
}

impl Default for TreeViewState {
    fn default() -> Self {
        Self {
            tree_label_template: Text {
                font: iced::Font {
                    family: iced::font::Family::Name(TREE_LAB_FONT_NAME),
                    ..Default::default()
                },
                align_y: Vertical::Center,
                ..Default::default()
            },
            tip_idx_range: None,
            ps: SF * 12e0,
            visible_nodes: Vec::new(),
            closest_node_point: None,
            mouse_hovering_node: false,
            clip_rect: Default::default(),
            tree_rect: Default::default(),
        }
    }
}

impl TreeViewState {}
