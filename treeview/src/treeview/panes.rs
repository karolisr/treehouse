use super::ui::style::{sty_pane_body, sty_pane_grid, sty_pane_titlebar};
use iced::{
    Element,
    Length::Fill,
    Size,
    alignment::Alignment,
    widget::{
        center, container,
        pane_grid::{
            self as pg, Configuration as PaneGridCfg, Content, DragEvent, ResizeEvent, State,
            TitleBar,
        },
        responsive, text,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Pane {
    Tree,
    LttPlot,
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub enum PaneGridMsg {
    Dragged(DragEvent),
    Resized(ResizeEvent),
}

pub(crate) struct PaneGrid {
    pane_grid_state: State<Pane>,
    pane_cfg_lttp: PaneGridCfg<Pane>,
}

impl Default for PaneGrid {
    fn default() -> Self {
        let pane_cfg_tree = PaneGridCfg::Pane(Pane::Tree);
        let pane_cfg_lttp = PaneGridCfg::Pane(Pane::LttPlot);
        let pane_grid_state = State::with_configuration(pane_cfg_tree);
        Self { pane_grid_state, pane_cfg_lttp }
    }
}

impl PaneGrid {
    pub(crate) fn new() -> Self {
        Self { ..Default::default() }
    }

    pub(crate) fn update(&mut self, message: PaneGridMsg) {
        match message {
            PaneGridMsg::Dragged(drag_event) => match drag_event {
                DragEvent::Picked { pane: _pane_idx } => (),
                DragEvent::Dropped { pane: pane_idx, target } => {
                    self.pane_grid_state.drop(pane_idx, target);
                }
                DragEvent::Canceled { pane: _pane_idx } => (),
            },
            PaneGridMsg::Resized(ResizeEvent { split, ratio }) => {
                self.pane_grid_state.resize(split, ratio);
            }
        }
    }

    pub(crate) fn view(&self) -> Element<PaneGridMsg> {
        let pane_count = self.pane_grid_state.len();
        let pane_grid = pg::PaneGrid::new(&self.pane_grid_state, |pane_idx, pane, is_maximized| {
            Content::new(responsive(move |size| {
                view_content(pane, pane_idx, pane_count, size, is_maximized)
            }))
            .style(sty_pane_body)
            .title_bar(
                TitleBar::new(container(iced::widget::vertical_space().height(30)))
                    .style(sty_pane_titlebar)
                    .always_show_controls(),
            )
        })
        .width(Fill)
        .height(Fill)
        .min_size(1e2)
        .style(sty_pane_grid)
        .on_drag(PaneGridMsg::Dragged)
        .on_resize(1e1, PaneGridMsg::Resized);

        container(pane_grid).into()
    }
}

fn view_content<'a>(
    pane: &Pane,
    pane_idx: pg::Pane,
    pane_count: usize,
    size: Size,
    is_maximized: bool,
) -> Element<'a, PaneGridMsg> {
    let w = size.width;
    let h = size.height;

    let mut content = text!("{w} x {h} | {pane:?} | {pane_idx:?} | {pane_count} | {is_maximized}");

    content = content.align_x(Alignment::Center);
    content = content.align_y(Alignment::Center);

    center(content).into()
}
