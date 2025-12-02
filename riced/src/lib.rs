// -------------------------------------
// #![allow(dead_code)]
// #![allow(unused_mut)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(clippy::single_match)]
// #![allow(clippy::collapsible_if)]
// #![allow(clippy::derivable_impls)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::collapsible_match)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::vec_init_then_push)]
// #![allow(clippy::needless_range_loop)]
// -------------------------------------

mod colors;
mod consts;
mod elements;
mod icons;
mod path_utils;
mod style;
#[cfg(debug_assertions)]
pub mod template_widget;
mod text_width;

use num_traits::FromPrimitive;
use std::fmt::Display;

use iced as i;
use iced::widget as w;

pub use colors::Clr;
pub use consts::*;
pub use elements::*;
pub use icons::Icon;
pub use path_utils::*;
pub use style::*;
pub use text_width::{TextWidth, text_width, text_width_line_height};

pub use i::advanced::mouse::click::{
    Click as MouseClick, Kind as MouseClickKind,
};
pub use i::alignment::{Horizontal, Vertical};
pub use i::border::{Border, Radius};
pub use i::debug::{time as timer, time_with as timer_with};
pub use i::font::{Family, Stretch, Style as FontStyle, Weight};
pub use i::futures::{
    channel::mpsc::{UnboundedSender, unbounded},
    stream::StreamExt,
};
pub use i::keyboard::{
    Event as KeyboardEvent, Key, Location as KeyLocation, Modifiers,
    key::Named as KeyName, on_key_press,
};
pub use i::mouse::{
    Button as MouseButton, Cursor, Event as MouseEvent,
    Interaction as MouseInteraction,
};
pub use i::task::{Never, Sipper, sipper};
pub use i::theme::{Style as ThemeStyle, Theme, palette::Pair as PalettePair};
pub use i::window::{
    Event as WindowEvent, Id as WindowId, Level as WindowLevel,
    Position as WindowPosition, Settings as WindowSettings,
    close as close_window, events as window_events, open as open_window,
    raw_id,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    run,
    settings::PlatformSpecific as PlatformSpecificWindowSettings,
};
pub use i::{
    Alignment, Background, Color, ContentFit, Element, Event, Font, Gradient,
    Length, Padding, Pixels, Point, Rectangle, Renderer, Result as IcedResult,
    Settings as IcedAppSettings, Shadow, Size, Subscription, Task, Vector,
    daemon, exit,
};
pub use w::button::{Button, Status as ButtonStatus, Style as ButtonStyle};
pub use w::canvas::{
    Action, Cache as CnvCache, Canvas as Cnv, Frame, Geometry, Program,
    Style as GeomStyle, Text as CnvText,
    fill::Fill as CnvFill,
    fill::Rule as FillRule,
    path::{
        Path as IcedPath, lyon_path, lyon_path::Event as LyonPathEvent,
        lyon_path::Path as LyonPath,
    },
    stroke::{LineCap, LineDash, LineJoin, Stroke as CnvStrk},
};
pub use w::center;
pub use w::checkbox::{
    Checkbox, Status as CheckboxStatus, Style as CheckboxStyle,
};
pub use w::column as iced_col;
pub use w::container;
pub use w::container::{Container, Style as ContainerStyle};
pub use w::float;
pub use w::mouse_area;
pub use w::operation::{focus, scroll_to};
pub use w::overlay::menu::Style as IcedMenuStyle;
pub use w::pane_grid::{
    Axis as PgAxis, Content as PgContent, Highlight as PgHighlight,
    Line as PgLine, Pane, PaneGrid, ResizeEvent, State as PgState,
    Style as PgStyle,
};
pub use w::pick_list::{
    Handle as PickListHandle, PickList, Status as PickListStatus,
    Style as PickListStyle,
};
pub use w::responsive;
pub use w::row as iced_row;
pub use w::rule::{
    FillMode as RuleFillMode, Rule, Style as RuleStyle,
    horizontal as horizontal_rule, vertical as vertical_rule,
};
pub use w::scrollable::{
    AbsoluteOffset, Direction as ScrollableDirection, Rail as ScrollBarRail,
    Scrollable, Scrollbar, Scroller, Status as ScrollableStatus,
    Style as ScrollableStyle, Viewport,
};
pub use w::slider::{
    Handle as SliderHandle, HandleShape as SliderHandleShape,
    Rail as SliderRail, Slider, Status as SliderStatus, Style as SliderStyle,
};
pub use w::space::{
    Space, horizontal as horizontal_space, vertical as vertical_space,
};
pub use w::svg::{
    Handle as SvgHandle, Status as SvgStatus, Style as SvgStyle, Svg,
};
pub use w::table::{
    Column as TableColumn, Style as TableStyle, Table, column as table_col,
    table,
};
pub use w::text::{Alignment as TextAlignment, LineHeight, Shaping, Text};
pub use w::text_input::{
    Status as TextInputStatus, Style as TextInputStyle, TextInput,
};
pub use w::toggler::{Status as TogglerStatus, Style as TogglerStyle, Toggler};
pub use w::{Column, Row, opaque, stack};
