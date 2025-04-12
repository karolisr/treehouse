use iced::advanced::graphics::text::cosmic_text::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping,
};

pub fn lerp<T>(a: impl Into<T>, b: impl Into<T>, t: impl Into<T>) -> T
where
    T: std::ops::Sub
        + Copy
        + std::ops::Add<<<T as std::ops::Sub>::Output as std::ops::Mul<T>>::Output, Output = T>,
    <T as std::ops::Sub>::Output: std::ops::Mul<T>,
{
    let a = a.into();
    let b = b.into();
    let t = t.into();
    a + (b - a) * t
}

pub fn text_width(s: &str, font_size: f32, line_height: f32, font_name: &str) -> f32 {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(font_size, line_height);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    let mut attrs = Attrs::new();
    attrs.family = Family::Name(font_name);
    buffer.set_text(s, &attrs, Shaping::Basic);
    buffer.line_layout(0).unwrap().first().unwrap().w
}
