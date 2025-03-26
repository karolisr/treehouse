use iced_core::{
    Clipboard, Element, Length, Rectangle, Shell, Size, Theme as ThemeCore, Vector, Widget,
    event::{Event, Status as EventStatus},
    layout::{self, Layout, Limits, Node as LayoutNode},
    mouse::{Cursor, Interaction},
    renderer::Style,
    widget::tree::{State as WidgetTreeState, Tag as WidgetTreeTag, Tree as WidgetTree},
    window::{Event as WindowEvent, RedrawRequest},
};
use iced_graphics::geometry::Renderer as GeometryRenderer;
use iced_renderer::Renderer as RendererRenderer;
use iced_widget::canvas::Program;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Canvas<P, Message, Theme = ThemeCore, Renderer = RendererRenderer>
where
    Renderer: GeometryRenderer,
    P: Program<Message, Theme, Renderer>,
{
    width: Length,
    height: Length,
    program: P,
    message: PhantomData<Message>,
    theme: PhantomData<Theme>,
    renderer: PhantomData<Renderer>,
    last_mouse_interaction: Option<Interaction>,
}

impl<P, Message, Theme, Renderer> Canvas<P, Message, Theme, Renderer>
where
    Renderer: GeometryRenderer,
    P: Program<Message, Theme, Renderer>,
{
    const DEFAULT_SIZE: f32 = 100.0;

    pub fn new(program: P) -> Self {
        Canvas {
            width: Length::Fixed(Self::DEFAULT_SIZE),
            height: Length::Fixed(Self::DEFAULT_SIZE),
            program,
            message: PhantomData,
            theme: PhantomData,
            renderer: PhantomData,
            last_mouse_interaction: None,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<P, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Canvas<P, Message, Theme, Renderer>
where
    Renderer: GeometryRenderer,
    P: Program<Message, Theme, Renderer>,
{
    fn tag(&self) -> WidgetTreeTag {
        struct Tag<T>(T);
        WidgetTreeTag::of::<Tag<P::State>>()
    }

    fn state(&self) -> WidgetTreeState {
        WidgetTreeState::new(P::State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, _tree: &mut WidgetTree, _renderer: &Renderer, limits: &Limits) -> LayoutNode {
        layout::atomic(limits, self.width, self.height)
    }

    fn update(
        &mut self,
        tree: &mut WidgetTree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let state = tree.state.downcast_mut::<P::State>();
        let is_redraw_request = matches!(event, Event::Window(WindowEvent::RedrawRequested(_now)),);

        if let Some(action) = self.program.update(state, event, bounds, cursor) {
            let (message, redraw_request, event_status) = action.into_inner();

            shell.request_redraw_at(redraw_request);

            if let Some(message) = message {
                shell.publish(message);
            }

            if event_status == EventStatus::Captured {
                shell.capture_event();
            }
        }

        if shell.redraw_request() != RedrawRequest::NextFrame {
            let mouse_interaction =
                self.mouse_interaction(tree, layout, cursor, viewport, renderer);

            if is_redraw_request {
                self.last_mouse_interaction = Some(mouse_interaction);
            } else if self
                .last_mouse_interaction
                .is_some_and(|last_mouse_interaction| last_mouse_interaction != mouse_interaction)
            {
                shell.request_redraw();
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &WidgetTree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> Interaction {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<P::State>();
        self.program.mouse_interaction(state, bounds, cursor)
    }

    fn draw(
        &self,
        tree: &WidgetTree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }
        let state = tree.state.downcast_ref::<P::State>();
        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            let layers = self.program.draw(state, renderer, theme, bounds, cursor);

            for layer in layers {
                renderer.draw_geometry(layer);
            }
        });
    }
}

impl<'a, P, Message, Theme, Renderer> From<Canvas<P, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + GeometryRenderer,
    P: 'a + Program<Message, Theme, Renderer>,
{
    fn from(canvas: Canvas<P, Message, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(canvas)
    }
}
