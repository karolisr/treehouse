use crate::ZERO;
use iced::advanced::graphics::text::cosmic_text::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping,
};

#[derive(Debug)]
pub struct TextWidth<'a> {
    attrs: Attrs<'a>,
    buffer: Buffer,
    font_system: FontSystem,
}

impl TextWidth<'_> {
    pub fn font_size(&self) -> f32 { self.buffer.metrics().font_size }
    pub fn line_height(&self) -> f32 { self.buffer.metrics().line_height }

    pub fn set_font_size(&mut self, font_size: f32) {
        let metrics = Metrics::new(font_size, font_size);
        self.buffer.set_metrics(&mut self.font_system, metrics);
    }

    pub fn set_font_size_line_height(&mut self, font_size: f32, line_height: f32) {
        let metrics = Metrics::new(font_size, line_height);
        self.buffer.set_metrics(&mut self.font_system, metrics);
    }

    pub fn width(&mut self, text: &str) -> f32 {
        self.buffer.set_text(&mut self.font_system, text, &self.attrs, Shaping::Basic);
        if let Some(line_layouts) = self.buffer.line_layout(&mut self.font_system, 0) {
            if let Some(line_layout) = line_layouts.first() { line_layout.w } else { ZERO }
        } else {
            ZERO
        }
    }
}

pub fn text_width(font_size: f32, font_name: &'_ str) -> TextWidth<'_> {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(font_size, font_size);
    let buffer = Buffer::new(&mut font_system, metrics);
    let mut attrs = Attrs::new();
    attrs.family = Family::Name(font_name);
    TextWidth { attrs, buffer, font_system }
}

pub fn text_width_line_height(
    font_size: f32, line_height: f32, font_name: &'_ str,
) -> TextWidth<'_> {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(font_size, line_height);
    let buffer = Buffer::new(&mut font_system, metrics);
    let mut attrs = Attrs::new();
    attrs.family = Family::Name(font_name);
    TextWidth { attrs, buffer, font_system }
}
