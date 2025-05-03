use iced::Color;

pub struct Clr {}

impl Clr {
    pub const TRN: Color = Color { r: 0e0, g: 0e0, b: 0e0, a: 0e0 };
    pub const BLK: Color = Color { r: 0e0, g: 0e0, b: 0e0, a: 1e0 };
    pub const WHT: Color = Color { r: 1e0, g: 1e0, b: 1e0, a: 1e0 };

    pub const RED: Color = Color { r: 1e0, g: 0e0, b: 0e0, a: 1e0 };
    pub const GRN: Color = Color { r: 0e0, g: 1e0, b: 0e0, a: 1e0 };
    pub const BLU: Color = Color { r: 0e0, g: 0e0, b: 1e0, a: 1e0 };

    pub const CYA: Color = Color { r: 0e0, g: 1e0, b: 1e0, a: 1e0 };
    pub const MAG: Color = Color { r: 1e0, g: 0e0, b: 1e0, a: 1e0 };
    pub const YEL: Color = Color { r: 1e0, g: 1e0, b: 0e0, a: 1e0 };
}
