use iced::{
    Element, Length, Rectangle, Size, Vector,
    advanced::{
        self, Clipboard, Shell, layout, mouse, overlay, renderer,
        widget::{self, tree},
    },
    event,
};

pub struct TheWidget {
    width: Length,
    height: Length,
}

impl<Message, Theme, Renderer> widget::Widget<Message, Theme, Renderer> for TheWidget
where
    Renderer: advanced::Renderer,
{
    fn size(&self) -> Size<Length> { Size { width: self.width, height: self.height } }

    fn layout(
        &self, _tree: &mut tree::Tree, _renderer: &Renderer, limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.min())
    }

    fn draw(
        &self, _tree: &tree::Tree, _renderer: &mut Renderer, _theme: &Theme,
        _style: &renderer::Style, _layout: layout::Layout<'_>, _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
    }

    fn tag(&self) -> tree::Tag { tree::Tag::stateless() }

    fn state(&self) -> tree::State { tree::State::None }

    fn children(&self) -> Vec<tree::Tree> { Vec::new() }

    fn diff(&self, tree: &mut tree::Tree) { tree.children.clear(); }

    fn operate(
        &self, _state: &mut tree::Tree, _layout: layout::Layout<'_>, _renderer: &Renderer,
        _operation: &mut dyn widget::Operation,
    ) {
    }

    fn update(
        &mut self, _state: &mut tree::Tree, _event: &event::Event, _layout: layout::Layout<'_>,
        _cursor: mouse::Cursor, _renderer: &Renderer, _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>, _viewport: &Rectangle,
    ) {
    }

    fn mouse_interaction(
        &self, _state: &tree::Tree, _layout: layout::Layout<'_>, _cursor: mouse::Cursor,
        _viewport: &Rectangle, _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }

    fn overlay<'a>(
        &'a mut self, _state: &'a mut tree::Tree, _layout: layout::Layout<'a>,
        _renderer: &Renderer, _viewport: &Rectangle, _translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> {
        None
    }
}

impl<'a, Message, Theme, Renderer> From<TheWidget> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: advanced::Renderer + 'a,
{
    fn from(w: TheWidget) -> Self { Element::new(w) }
}
