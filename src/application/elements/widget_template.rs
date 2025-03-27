use crate::SimpleColor;
use iced::{
    Element, Length, Point, Rectangle, Renderer as RendererRenderer, Size, Theme as IcedTheme,
    Vector,
    advanced::{
        Clipboard, Shell,
        graphics::geometry::{Cache, Frame, Renderer as GeometryRenderer},
        layout::{self, Layout, Limits, Node as LayoutNode},
        renderer::Style,
        widget::{
            Widget,
            tree::{State as WidgetTreeState, Tag as WidgetTreeTag, Tree as WidgetTree},
        },
    },
    event::{Event, Status as EventStatus},
    mouse::{Cursor, Interaction},
    widget::Action,
    window::{Event as WindowEvent, RedrawRequest},
};
use std::marker::PhantomData;
pub type Geometry<Renderer = RendererRenderer> = <Renderer as GeometryRenderer>::Geometry;

pub struct State<Renderer>
where
    Renderer: GeometryRenderer + 'static,
{
    cache: Cache<Renderer>,
}

impl<Renderer> Default for State<Renderer>
where
    Renderer: GeometryRenderer + 'static,
{
    fn default() -> Self {
        Self {
            cache: Cache::new(),
        }
    }
}

pub struct WidgetTemplate<Message, Theme = IcedTheme, Renderer = RendererRenderer>
where
    Renderer: GeometryRenderer + 'static,
{
    width: Length,
    height: Length,
    message: PhantomData<Message>,
    theme: PhantomData<Theme>,
    renderer: PhantomData<Renderer>,
    last_mouse_interaction: Option<Interaction>,
}

impl<Message, Theme, Renderer> Default for WidgetTemplate<Message, Theme, Renderer>
where
    Renderer: GeometryRenderer + 'static,
{
    fn default() -> Self {
        Self {
            width: Length::Fixed(6e2),
            height: Length::Fixed(6e2),
            message: PhantomData,
            theme: PhantomData,
            renderer: PhantomData,
            last_mouse_interaction: None,
        }
    }
}

impl<Message, Theme, Renderer> WidgetTemplate<Message, Theme, Renderer>
where
    Renderer: GeometryRenderer + 'static,
{
    pub fn new() -> Self {
        println!("new");
        Self::default()
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

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for WidgetTemplate<Message, Theme, Renderer>
where
    Renderer: GeometryRenderer + 'static,
{
    fn tag(&self) -> WidgetTreeTag {
        struct Tag<T>(T);
        WidgetTreeTag::of::<Tag<State<Renderer>>>()
    }

    fn state(&self) -> WidgetTreeState {
        WidgetTreeState::new::<State<Renderer>>(State::default())
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
        // let bounds = layout.bounds();
        // let state = tree.state.downcast_mut::<State<Renderer>>();
        // let action: Option<Action<Message>> = Some(Action::request_redraw());
        let action: Option<Action<Message>> = None;
        if let Some(action) = action {
            let (message, redraw_request, event_status) = action.into_inner();
            shell.request_redraw_at(redraw_request);
            if let Some(message) = message {
                shell.publish(message);
            }
            if event_status == EventStatus::Captured {
                shell.capture_event();
            }
        }

        let is_redraw_request = matches!(event, Event::Window(WindowEvent::RedrawRequested(_now)));

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
        _tree: &WidgetTree,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> Interaction {
        // let bounds = layout.bounds();
        Interaction::default()
    }

    fn draw(
        &self,
        tree: &WidgetTree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }
        let state = tree.state.downcast_ref::<State<Renderer>>();
        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            let geometry: Geometry<Renderer> =
                state
                    .cache
                    .draw(renderer, bounds.size(), |frame: &mut Frame<Renderer>| {
                        println!("Cache Invalidated.");
                        frame.fill_rectangle(
                            Point::new(0e0, 0e0),
                            bounds.size(),
                            SimpleColor::BLUE,
                        );
                    });
            let layers: Vec<Geometry<Renderer>> = vec![geometry];
            for layer in layers {
                renderer.draw_geometry(layer);
            }
        });
    }
}

impl<'a, Message, Theme, Renderer> From<WidgetTemplate<Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + GeometryRenderer,
{
    fn from(
        widget: WidgetTemplate<Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(widget)
    }
}
