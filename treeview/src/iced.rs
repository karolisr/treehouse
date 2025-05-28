use iced as i;
use iced::widget as w;

pub use i::alignment::{Horizontal, Vertical};
pub use i::font::{Family, Stretch, Style as FontStyle, Weight};
pub use i::mouse::{Button as MouseButton, Cursor, Event as MouseEvent, Interaction};
pub use i::{
    Alignment, Background, Border, Element, Event, Font, Length, Padding, Pixels, Point, Rectangle, Renderer, Size,
    Task, Theme, Vector,
};

pub use w::button::{Status as ButtonStatus, Style as ButtonStyle};
pub use w::canvas::{
    Action, Cache, Canvas as Cnv, Frame, Geometry, Program,
    Style::Solid,
    Text as CnvText,
    fill::Fill as CnvFill,
    fill::Rule as FillRule,
    path::{Path as IcedPath, lyon_path},
    stroke::{LineCap, LineDash, LineJoin, Stroke as Strk},
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
pub use w::pick_list::{Status as PickListStatus, Style as PickListStyle};
pub use w::responsive;
pub use w::row as iced_row;
pub use w::rule::{FillMode as RuleFillMode, Style as RuleStyle};
pub use w::scrollable::{
    AbsoluteOffset, Direction as ScrollableDirection, Rail as ScrollBarRail, Scrollbar, Scroller,
    Status as ScrollableStatus, Style as ScrollableStyle, Viewport, scroll_to,
};
pub use w::slider::{
    Handle as SliderHandle, HandleShape as SliderHandleShape, Rail as SliderRail, Status as SliderStatus,
    Style as SliderStyle,
};
pub use w::text::{Alignment as TextAlignment, LineHeight, Shaping};
pub use w::vertical_space;
pub use w::{Button, Column, PaneGrid, PickList, Row, Rule, Scrollable, Slider, Space, Text};
