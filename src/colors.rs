use iced::Color;
pub struct SimpleColor {}

impl SimpleColor {
    pub const BLACK: Color = Color {
        r: 0e0,
        g: 0e0,
        b: 0e0,
        a: 1e0,
    };

    pub const WHITE: Color = Color {
        r: 1e0,
        g: 1e0,
        b: 1e0,
        a: 1e0,
    };

    pub const TRANSPARENT: Color = Color {
        r: 0e0,
        g: 0e0,
        b: 0e0,
        a: 0e0,
    };

    pub const RED: Color = Color {
        r: 1e0,
        g: 0e0,
        b: 0e0,
        a: 1e0,
    };

    pub const GREEN: Color = Color {
        r: 0e0,
        g: 1e0,
        b: 0e0,
        a: 1e0,
    };

    pub const BLUE: Color = Color {
        r: 0e0,
        g: 0e0,
        b: 1e0,
        a: 1e0,
    };

    pub const YELLOW: Color = Color {
        r: 1e0,
        g: 1e0,
        b: 0e0,
        a: 1e0,
    };

    pub const MAGENTA: Color = Color {
        r: 1e0,
        g: 0e0,
        b: 1e0,
        a: 1e0,
    };

    pub const CYAN: Color = Color {
        r: 0e0,
        g: 1e0,
        b: 1e0,
        a: 1e0,
    };
}
