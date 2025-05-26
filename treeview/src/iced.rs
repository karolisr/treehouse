use iced as i;
use iced::widget as w;

// pub use i::widget::canvas::path::lyon_path;

pub use i::alignment::{Horizontal, Vertical};
pub use i::font::{Family, Stretch, Style as FontStyle, Weight};
pub use i::mouse::{Cursor, Event as MouseEvent, Interaction};
pub use i::window::Event as WindowEvent;
pub use i::{
    Alignment, Border, Element, Event, Font, Length, Padding, Pixels, Point, Rectangle, Renderer, Size, Task, Theme,
    Vector,
};
pub use w::canvas::{
    Action, Cache, Canvas as Cnv, Frame, Geometry, Program, Text as CnvText,
    path::{Path, lyon_path},
    stroke::{LineCap, LineDash, LineJoin, Stroke as Strk, Style::Solid},
};
pub use w::center;
pub use w::column as iced_col;
pub use w::container;
pub use w::container::Style as ContainerStyle;
pub use w::horizontal_space;
pub use w::pane_grid::{
    Axis, Content as PgContent, Highlight as PgHighlight, Line as PgLine, Pane, ResizeEvent, State as PgState,
    Style as PgStyle,
};
pub use w::responsive;
pub use w::row as iced_row;
pub use w::scrollable::{AbsoluteOffset, Direction as ScrollableDirection, Scrollbar, Viewport, scroll_to};
pub use w::text::{Alignment as TextAlignment, LineHeight, Shaping};
pub use w::vertical_space;
pub use w::{Button, Column, PaneGrid, PickList, Row, Rule, Scrollable, Slider, Space, Text};
