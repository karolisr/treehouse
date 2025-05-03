use crate::PADDING;
use iced::{
    Element, Length, Size,
    alignment::Alignment,
    widget::{
        center, container,
        pane_grid::{Configuration, Content, DragEvent, PaneGrid, ResizeEvent, State},
        responsive, text,
    },
};

use super::ui::style::sty_pane_grid;

#[derive(Debug)]
pub(crate) enum MainContentPane {
    Tree,
    LttPlot,
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub enum PaneGridMainMsg {
    Dragged(DragEvent),
    Resized(ResizeEvent),
}

pub(crate) struct PaneGridMain {
    pane_grid_state: State<MainContentPane>,
}

impl Default for PaneGridMain {
    fn default() -> Self {
        // ----------------------------------------------------------------------------------------
        // let pane_cfg_tree = Configuration::Pane(MainContentPane::Tree);
        // let pane_cfg_lttp = Configuration::Pane(MainContentPane::LttPlot);
        // let pane_cfg_main = Configuration::Split {
        //     axis: iced::widget::pane_grid::Axis::Horizontal,
        //     ratio: 3e0 / 4e0,
        //     a: Box::new(pane_cfg_tree),
        //     b: Box::new(pane_cfg_lttp),
        // };
        // let pane_grid_state = State::with_configuration(pane_cfg_main);
        // ----------------------------------------------------------------------------------------
        let pane_cfg_empty = Configuration::Pane(MainContentPane::Empty);
        let pane_grid_state = State::with_configuration(pane_cfg_empty);
        // ----------------------------------------------------------------------------------------
        Self { pane_grid_state }
    }
}

impl PaneGridMain {
    pub(crate) fn new() -> Self {
        Self { ..Default::default() }
    }

    pub(crate) fn update(&mut self, message: PaneGridMainMsg) {
        match message {
            PaneGridMainMsg::Dragged(drag_event) => match drag_event {
                DragEvent::Picked { pane: _pane_idx } => (),
                DragEvent::Dropped { pane: pane_idx, target } => {
                    self.pane_grid_state.drop(pane_idx, target);
                }
                DragEvent::Canceled { pane: _pane_idx } => (),
            },
            PaneGridMainMsg::Resized(ResizeEvent { split, ratio }) => {
                self.pane_grid_state.resize(split, ratio);
            }
        }
    }

    pub(crate) fn view(&self) -> Element<PaneGridMainMsg> {
        let pane_count = self.pane_grid_state.len();
        let pane_grid = PaneGrid::new(&self.pane_grid_state, |pane_idx, pane, is_maximized| {
            Content::new(responsive(move |size| {
                view_content(pane, pane_idx, pane_count, size, is_maximized)
            }))
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .min_size(1e2)
        .style(sty_pane_grid)
        .on_resize(PADDING, PaneGridMainMsg::Resized);
        container(pane_grid).into()
    }
}

fn view_content<'a>(
    pane: &MainContentPane,
    pane_idx: iced::widget::pane_grid::Pane,
    pane_count: usize,
    size: Size,
    is_maximized: bool,
) -> Element<'a, PaneGridMainMsg> {
    let w = size.width;
    let h = size.height;

    let mut content = text!("{w} x {h} | {pane:?} | {pane_idx:?} | {pane_count} | {is_maximized}");

    content = content.align_x(Alignment::Center);
    content = content.align_y(Alignment::Center);

    center(content).into()
}
