use super::drawables::NodePoint;
use crate::app::{SF, TREE_LAB_FONT_NAME};
use iced::{
    Point,
    alignment::Vertical,
    widget::canvas::{
        Stroke, Text,
        stroke::{LineCap, LineJoin},
    },
};

pub struct TreeViewState {
    pub lab_txt_template: Text,
    pub closest_node_point: Option<NodePoint>,
    pub mouse_hovering_node: bool,
    pub stroke: Stroke<'static>,
    pub crosshairs: Option<Point>,
}

impl Default for TreeViewState {
    fn default() -> Self {
        Self {
            lab_txt_template: Text {
                font: iced::Font {
                    family: iced::font::Family::Name(TREE_LAB_FONT_NAME),
                    ..Default::default()
                },
                align_y: Vertical::Center,
                ..Default::default()
            },
            closest_node_point: None,
            mouse_hovering_node: false,
            stroke: Stroke {
                width: SF,
                line_cap: LineCap::Square,
                line_join: LineJoin::Round,
                ..Default::default()
            },
            crosshairs: None,
        }
    }
}
