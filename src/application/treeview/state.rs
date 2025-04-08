use iced::{alignment::Vertical, widget::canvas::Text};

use crate::TREE_LAB_FONT_NAME;

pub struct TreeViewState {
    pub(super) tree_label_template: Text,
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
        }
    }
}

impl TreeViewState {}
