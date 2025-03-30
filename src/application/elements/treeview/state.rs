use iced::Rectangle;

#[derive(Default)]
pub struct TreeViewState {
    pub(super) tree_bounds: Rectangle,
}

impl TreeViewState {
    pub(super) fn set_tree_bounds(&mut self, bounds: &Rectangle) {
        self.tree_bounds = *bounds;
    }
}
