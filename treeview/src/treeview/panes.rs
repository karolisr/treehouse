use super::{
    PlotCnv, TreeCnv, TreeViewMsg,
    styles::{sty_pane_body, sty_pane_grid, sty_pane_titlebar, sty_scrlbl},
};
use iced::{
    Element,
    Length::{self, Fill},
    Size,
    alignment::Alignment,
    widget::{
        Canvas, center, container,
        pane_grid::{
            self as pg, Configuration as PaneGridCfg, Content, DragEvent, ResizeEvent, State,
            TitleBar,
        },
        responsive,
        scrollable::{Direction as ScrollableDirection, Scrollable, Scrollbar},
        text,
    },
};

pub(crate) struct PaneGrid {
    pane_grid_state: State<Pane>,
    pub(crate) pane_cfg_tree: Option<PaneGridCfg<Pane>>,
    pub(crate) pane_cfg_lttp: Option<PaneGridCfg<Pane>>,
}

impl Default for PaneGrid {
    fn default() -> Self {
        // let pane_cfg_tree = PaneGridCfg::Pane(Pane::Tree { cnv_tree: TreeCnv::default() });
        // let pane_cfg_lttp = PaneGridCfg::Pane(Pane::LttPlot { cnv_lttp: PlotCnv::default() });
        let pane_cfg_empty = PaneGridCfg::Pane(Pane::Empty);
        let pane_grid_state = State::with_configuration(pane_cfg_empty);
        Self { pane_grid_state, ..Default::default() }
    }
}

impl PaneGrid {
    // pub(crate) fn new() -> Self {
    //     Self { ..Default::default() }
    // }

    pub(crate) fn update(&mut self, message: TreeViewMsg) {
        match message {
            TreeViewMsg::PaneDragged(drag_event) => match drag_event {
                DragEvent::Picked { pane: _pane_idx } => (),
                DragEvent::Dropped { pane: pane_idx, target } => {
                    self.pane_grid_state.drop(pane_idx, target);
                }
                DragEvent::Canceled { pane: _pane_idx } => (),
            },
            TreeViewMsg::PaneResized(ResizeEvent { split, ratio }) => {
                self.pane_grid_state.resize(split, ratio);
            }
            _ => (),
        }
    }

    // pub(crate) fn view(&self) -> Element<TreeViewMsg> {
    //     let pane_grid = pg::PaneGrid::new(&self.pane_grid_state, |pane_idx, pane, is_maximized| {
    //         Content::new(responsive(move |size| {
    //             pane.content(pane_idx, self.pane_grid_state.len(), size, is_maximized)
    //         }))
    //         .style(sty_pane_body)
    //         .title_bar(
    //             TitleBar::new(container(iced::widget::vertical_space().height(30)))
    //                 .style(sty_pane_titlebar)
    //                 .always_show_controls(),
    //         )
    //     })
    //     .width(Fill)
    //     .height(Fill)
    //     .min_size(1e2)
    //     .style(sty_pane_grid)
    //     .on_drag(TreeViewMsg::PaneDragged)
    //     .on_resize(1e1, TreeViewMsg::PaneResized);
    //     container(pane_grid).into()
    // }
}
