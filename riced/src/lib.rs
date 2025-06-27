#![feature(const_float_round_methods)]
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

pub use i::alignment::{Horizontal, Vertical};
pub use i::border::{Border, Radius};
pub use i::debug::{time as timer, time_with as timer_with};
pub use i::font::{Family, Stretch, Style as FontStyle, Weight};
pub use i::futures::{
    channel::mpsc::{UnboundedSender, unbounded},
    stream::StreamExt,
};
pub use i::keyboard::{
    Event as KeyboardEvent, Key, Location as KeyLocation, Modifiers, on_key_press,
};
pub use i::mouse::{Button as MouseButton, Cursor, Event as MouseEvent, Interaction};
pub use i::task::{Never, Sipper, sipper};
pub use i::theme::{Theme, palette::Pair as PalettePair};
pub use i::window::{
    Event as WindowEvent, Id as WindowId, Level as WindowLevel, Position as WindowPosition,
    Settings as WindowSettings, close as close_window, events as window_events,
    open as open_window, settings::PlatformSpecific as PlatformSpecificWindowSettings,
};
pub use i::{
    Alignment, Background, Color, Element, Event, Font, Length, Padding, Pixels, Point, Rectangle,
    Renderer, Result as IcedResult, Settings as IcedAppSettings, Shadow, Size, Subscription, Task,
    Vector, daemon, exit,
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
pub use w::checkbox::{Checkbox, Status as CheckboxStatus, Style as CheckboxStyle};
pub use w::column as iced_col;
pub use w::container;
pub use w::container::{Container, Style as ContainerStyle};
pub use w::horizontal_space;
pub use w::pane_grid::{
    Axis as PgAxis, Content as PgContent, Highlight as PgHighlight, Line as PgLine, Pane,
    ResizeEvent, State as PgState, Style as PgStyle,
};
pub use w::pick_list::{
    Handle as PickListHandle, Status as PickListStatus, Style as PickListStyle,
};
pub use w::responsive;
pub use w::row as iced_row;
pub use w::rule::{FillMode as RuleFillMode, Style as RuleStyle};
pub use w::scrollable::{
    AbsoluteOffset, Direction as ScrollableDirection, Rail as ScrollBarRail, Scrollbar, Scroller,
    Status as ScrollableStatus, Style as ScrollableStyle, Viewport, scroll_to,
};
pub use w::slider::{
    Handle as SliderHandle, HandleShape as SliderHandleShape, Rail as SliderRail,
    Status as SliderStatus, Style as SliderStyle,
};
pub use w::svg::{Handle as SvgHandle, Status as SvgStatus, Style as SvgStyle, Svg};
pub use w::text::{Alignment as TextAlignment, LineHeight, Shaping};
pub use w::text_input::{
    Status as TextInputStatus, Style as TextInputStyle, TextInput, focus as focus_text_input,
};
pub use w::toggler::{Status as TogglerStatus, Style as TogglerStyle, Toggler};
pub use w::vertical_space;
pub use w::{Button, Column, PaneGrid, PickList, Row, Rule, Scrollable, Slider, Space, Text};
