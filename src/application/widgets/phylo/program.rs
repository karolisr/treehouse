use super::canvas::Cache;
use super::canvas::Canvas2;
// use super::canvas::Frame;
use super::canvas::Geometry;
use super::canvas::Program;
// use super::canvas::Stroke;
use crate::MainWinMsg;
use crate::SimpleColor;
// use iced::Alignment::Center;
// use iced::Element;
// use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Size;
use iced::Theme;
// use iced::advanced::Widget;
use iced::mouse::Cursor;

#[derive(Default, Debug)]
pub struct Phylo {
    cache: Cache,
}

impl Phylo {
    pub fn view(&self) -> Canvas2<&Phylo, MainWinMsg> {
        Canvas2::new(self)
    }
}

#[derive(Default, Debug)]
pub struct PhyloState {}
impl Program<MainWinMsg> for Phylo {
    type State = PhyloState;

    fn draw(
        &self,
        _state: &PhyloState,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        // let mut frame = Frame::new(renderer, bounds.size());
        // let geometry = frame.into_geometry();
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // println!("geometry changed {:?}", bounds);
            frame.fill_rectangle(
                Point { x: 0e0, y: 0e0 },
                Size {
                    width: bounds.width,
                    height: bounds.height,
                },
                SimpleColor::GREEN,
            );
        });
        vec![geometry]
    }
}
