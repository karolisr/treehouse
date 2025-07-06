use iced::{
    Element, Length, Rectangle, Size, Vector,
    advanced::{
        Clipboard, Renderer as RendererTrait, Shell,
        layout::{Layout, Limits, Node},
        mouse::{Cursor, Interaction as MouseInteraction},
        overlay,
        renderer::Style,
        widget::{
            Operation, Widget,
            tree::{State, Tag, Tree},
        },
    },
    event::Event,
};

pub struct TheWidget {
    width: Length,
    height: Length,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for TheWidget
where
    Renderer: RendererTrait,
{
    fn update(
        &mut self, _tree: &mut Tree, _event: &Event, _layout: Layout<'_>, _cursor: Cursor,
        _renderer: &Renderer, _clipboard: &mut dyn Clipboard, _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(limits.min())
    }

    fn draw(
        &self, _tree: &Tree, _renderer: &mut Renderer, _theme: &Theme, _style: &Style,
        _layout: Layout<'_>, _cursor: Cursor, _viewport: &Rectangle,
    ) {
    }

    fn overlay<'a>(
        &'a mut self, _tree: &'a mut Tree, _layout: Layout<'a>, _renderer: &Renderer,
        _viewport: &Rectangle, _translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> {
        None
    }

    fn mouse_interaction(
        &self, _tree: &Tree, _layout: Layout<'_>, _cursor: Cursor, _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> MouseInteraction {
        MouseInteraction::None
    }

    fn operate(
        &self, _tree: &mut Tree, _layout: Layout<'_>, _renderer: &Renderer,
        _operation: &mut dyn Operation,
    ) {
    }

    fn size(&self) -> Size<Length> {
        Size { width: self.width, height: self.height }
    }

    fn tag(&self) -> Tag {
        Tag::stateless()
    }

    fn state(&self) -> State {
        State::None
    }

    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.children.clear();
    }
}

impl<'a, Message, Theme, Renderer> From<TheWidget> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: RendererTrait + 'a,
{
    fn from(w: TheWidget) -> Self {
        Element::new(w)
    }
}
