use iced::advanced::graphics::text::cosmic_text::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping,
};

pub struct TextWidth<'a> {
    attrs: Attrs<'a>,
    buffer: Buffer,
    font_system: FontSystem,
}

impl TextWidth<'_> {
    pub fn width(&mut self, text: &str) -> f32 {
        self.buffer
            .set_text(&mut self.font_system, text, &self.attrs, Shaping::Basic);
        self.buffer
            .line_layout(&mut self.font_system, 0)
            .unwrap()
            .first()
            .unwrap()
            .w
    }
}

pub fn text_width(font_size: f32, line_height: f32, font_name: &str) -> TextWidth {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(font_size, line_height);
    let buffer = Buffer::new(&mut font_system, metrics);
    let mut attrs = Attrs::new();
    attrs.family = Family::Name(font_name);
    TextWidth { attrs, buffer, font_system }
}
