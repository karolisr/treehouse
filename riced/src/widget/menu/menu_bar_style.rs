use iced::{Background, Border, Color, Padding, Shadow, Theme, Vector};

/// The Status of a widget event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuStatus {
    /// can be pressed.
    Active,
    /// can be pressed and it is being hovered.
    Hovered,
    /// is being pressed.
    Pressed,
    /// cannot be pressed.
    Disabled,
    /// is focused.
    Focused,
    /// is Selected.
    Selected,
}

/// The style function of widget.
pub type StyleFn<'a, Theme, Style> = Box<dyn Fn(&Theme, MenuStatus) -> Style + 'a>;

/// The appearance of a menu bar and its menus.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The background of the menu bar.
    pub bar_background: Background,
    /// The border of the menu bar.
    pub bar_border: Border,
    /// The shadow of the menu bar.
    pub bar_shadow: Shadow,
    /// Expand the menu bar background
    pub bar_background_expand: Padding,

    /// The background of the menus.
    pub menu_background: Background,
    /// The border of the menus.
    pub menu_border: Border,
    /// The shadow of the menus
    pub menu_shadow: Shadow,
    /// Expand the menu background
    pub menu_background_expand: Padding,

    /// The backgraound of the path
    pub path: Background,
    /// The border of the path
    pub path_border: Border,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            bar_background: Color::from([0.85; 3]).into(),
            bar_border: Border { radius: 3.0.into(), ..Default::default() },
            bar_shadow: Shadow::default(),
            bar_background_expand: 3.into(),

            menu_background: Color::from([0.85; 3]).into(),
            menu_border: Border { radius: 3.0.into(), ..Default::default() },
            menu_shadow: Shadow {
                color: Color::from([0.0, 0.0, 0.0, 0.75]),
                offset: Vector::ZERO,
                blur_radius: 10.0,
            },

            menu_background_expand: 3.into(),
            path: Color::from([0.3; 3]).into(),
            path_border: Border {
                radius: 3.0.into(),
                width: 2.0,
                color: Color { r: 1e0, g: 0e0, b: 0e0, a: 1e0 },
            },
        }
    }
}

/// The Catalog of a [`Menu`](crate::widget::menu::Menu).
pub trait MenuCatalog {
    ///Style for the trait to use.
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: MenuStatus) -> Style;
}

impl MenuCatalog for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;
    fn default<'a>() -> Self::Class<'a> { Box::new(primary) }
    fn style(&self, class: &Self::Class<'_>, status: MenuStatus) -> Style { class(self, status) }
}

/// The primary theme of a [`Menu`](crate::widget::menu::Menu).
#[must_use]
pub fn primary(theme: &Theme, _status: MenuStatus) -> Style {
    let palette = theme.extended_palette();
    Style {
        bar_background: palette.background.base.color.into(),
        menu_background: palette.background.base.color.into(),
        path: palette.primary.weak.color.into(),
        ..Default::default()
    }
}
