use super::drawables::{IndexRange, NodePoint};
use crate::{
    Float,
    app::{SF, TREE_LAB_FONT_NAME},
};
use iced::{Rectangle, alignment::Vertical, widget::canvas::Text};

pub struct TreeViewState {
    pub tree_label_template: Text,
    pub tip_idx_range: Option<IndexRange>,
    pub visible_nodes: Vec<NodePoint>,
    pub ps: Float,
    pub closest_node_point: Option<NodePoint>,
    pub mouse_hovering_node: bool,
    pub clip_rect: Rectangle,
    pub tree_rect: Rectangle,
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
            ps: SF * 1e1,
            visible_nodes: Vec::new(),
            closest_node_point: None,
            mouse_hovering_node: false,
            clip_rect: Default::default(),
            tree_rect: Default::default(),
        }
    }
}
