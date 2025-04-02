use crate::{
    Edges, Float, LINE_H, PADDING, PADDING_INNER, SCROLL_BAR_W, SF, TEXT_SIZE, Tree, flatten_tree,
    max_name_len,
};
use iced::{
    Alignment, Border, Color, Element, Font, Length, Pixels, Task,
    alignment::{Horizontal, Vertical},
    border,
    widget::{
        Button, Canvas, Column, PickList, Row, Rule, Scrollable, Slider, Theme as WidgetTheme,
        Toggler,
        canvas::Cache,
        container, horizontal_rule, horizontal_space,
        pick_list::{Handle as PickListHandle, Status as PickListStatus, Style as PickListStyle},
        row,
        rule::{FillMode as RuleFillMode, Style as RuleStyle},
        scrollable::{
            Anchor as ScrollBarAnchor, Direction as ScrollableDirection, Rail as ScrollBarRail,
            Scrollbar, Scroller, Status as ScrollBarStatus, Style as ScrollBarStyle,
        },
        slider::{
            Handle as SliderHandle, HandleShape as SliderHandleShape, Rail as SliderRail,
            Status as SliderStatus, Style as SliderStyle,
        },
        text, vertical_space,
    },
    window::Id as WinId,
};

#[derive(Debug)]

pub struct TreeView {
    pub(super) win_id: Option<WinId>,

    threads: usize,
    selected_node_ordering_option: Option<NodeOrderingOption>,
    pub(super) drawing_enabled: bool,

    pub(super) node_count: usize,
    pub(super) tip_count: usize,
    pub(super) int_node_count: usize,
    pub(super) max_name_len: usize,

    pub(super) canvas_h: Float,

    pub(super) window_w: Float,
    pub(super) window_h: Float,
    pub(super) max_available_window_h: Float,

    pub(super) node_size: Float,
    pub(super) min_node_size: Float,

    pub(super) tip_label_size: Float,
    pub(super) int_label_size: Float,
    pub(super) max_label_size: Float,
    pub(super) tip_label_w: Float,

    pub(super) tip_label_offset: Float,
    pub(super) int_label_offset: Float,

    pub(super) draw_tip_labels: bool,
    pub(super) draw_int_labels: bool,

    pub(super) edge_geom_cache: Cache,
    pub(super) tip_labels_geom_cache: Cache,
    pub(super) int_labels_geom_cache: Cache,

    pub(super) tree: Tree,
    pub(super) tree_chunked_edges: Vec<Edges>,

    tree_original: Tree,
    tree_original_chunked_edges: Option<Vec<Edges>>,
    tree_srtd_asc: Option<Tree>,
    tree_srtd_asc_chunked_edges: Option<Vec<Edges>>,
    tree_srtd_desc: Option<Tree>,
    tree_srtd_desc_chunked_edges: Option<Vec<Edges>>,
}

impl Default for TreeView {
    fn default() -> Self {
        Self {
            win_id: None,
            threads: 8,
            tree: Default::default(),
            drawing_enabled: false,
            selected_node_ordering_option: Some(NodeOrderingOption::Unordered),

            node_count: 0,
            tip_count: 0,
            int_node_count: 0,
            max_name_len: 0,

            canvas_h: 1e0,
            window_w: 1e0,
            window_h: 1e0,

            max_available_window_h: 1e0,

            node_size: SF * 1e0,
            min_node_size: SF * 1e0,

            tip_label_size: SF * 1e1,
            int_label_size: SF * 1e1,
            max_label_size: SF * 3e1,
            tip_label_w: SF * 1e1,

            tip_label_offset: SF * 3e0,
            int_label_offset: SF * 3e0,

            draw_tip_labels: true,
            draw_int_labels: false,

            edge_geom_cache: Default::default(),
            tip_labels_geom_cache: Default::default(),
            int_labels_geom_cache: Default::default(),

            tree_chunked_edges: Default::default(),
            tree_original: Default::default(),
            tree_original_chunked_edges: Default::default(),
            tree_srtd_asc: Default::default(),
            tree_srtd_asc_chunked_edges: Default::default(),
            tree_srtd_desc: Default::default(),
            tree_srtd_desc_chunked_edges: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TreeViewMsg {
    SetWinId(WinId),
    TreeUpdated(Tree),
    NodeOrderingOptionChanged(NodeOrderingOption),
    WindowResized(Float, Float),
    UpdateWindowSize,
    NodeSizeChanged(Float),
    TipLabelSizeChanged(Float),
    IntLabelSizeChanged(Float),
    TipLabelVisibilityChanged(bool),
    IntLabelVisibilityChanged(bool),
    OpenFile,
}

impl TreeView {
    fn update_tip_label_w(&mut self) {
        if self.draw_tip_labels {
            self.tip_label_w =
                self.tip_label_offset + self.max_name_len as Float * self.tip_label_size / 1.75;
        } else {
            self.tip_label_w = 0e0;
        }
    }

    fn update_canvas_h_and_node_size(&mut self) {
        self.max_available_window_h = self.window_h - PADDING * 2e0 - SF;
        self.min_node_size = self.max_available_window_h / self.tip_count as Float;
        if self.node_size < self.min_node_size {
            self.node_size = self.min_node_size
        }
        self.canvas_h = self.node_size * self.tip_count as Float;
    }

    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::OpenFile => Task::none(),
            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labels = state;
                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();

                self.draw_tip_labels = state;
                self.update_tip_label_w();
                Task::none()
            }

            TreeViewMsg::TipLabelSizeChanged(s) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();

                self.tip_label_size = s;
                self.update_tip_label_w();
                Task::none()
            }

            TreeViewMsg::IntLabelSizeChanged(s) => {
                self.int_labels_geom_cache.clear();

                self.int_label_size = s;
                Task::none()
            }

            TreeViewMsg::NodeSizeChanged(s) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();

                self.node_size = s;
                self.update_canvas_h_and_node_size();
                Task::none()
            }

            TreeViewMsg::NodeOrderingOptionChanged(node_ordering_option) => {
                if node_ordering_option != self.selected_node_ordering_option.unwrap() {
                    self.edge_geom_cache.clear();
                    self.tip_labels_geom_cache.clear();
                    self.int_labels_geom_cache.clear();

                    self.selected_node_ordering_option = Some(node_ordering_option);
                    self.sort();
                }
                Task::none()
            }

            TreeViewMsg::SetWinId(id) => {
                self.win_id = Some(id);
                iced::window::get_size(id)
                    .map(|s| TreeViewMsg::WindowResized(s.width * SF, s.height * SF))
            }

            TreeViewMsg::UpdateWindowSize => match self.win_id {
                Some(id) => iced::window::get_size(id)
                    .map(|s| TreeViewMsg::WindowResized(s.width, s.height)),
                None => Task::none(),
            },

            TreeViewMsg::WindowResized(w, h) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();

                self.window_w = w;
                self.window_h = h;
                self.update_canvas_h_and_node_size();
                Task::none()
            }

            TreeViewMsg::TreeUpdated(tree) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();

                self.drawing_enabled = false;
                self.tree_original = tree;
                self.tree_srtd_asc = None;
                self.tree_srtd_desc = None;
                self.tree_srtd_asc_chunked_edges = None;
                self.tree_srtd_desc_chunked_edges = None;
                self.tree_original_chunked_edges = None;
                self.node_count = self.tree_original.node_count_all();
                self.tip_count = self.tree_original.tip_count_all();
                self.int_node_count = self.node_count - self.tip_count;
                self.max_name_len = max_name_len(&self.tree_original);

                self.sort();
                self.update_canvas_h_and_node_size();
                self.update_tip_label_w();

                self.drawing_enabled = true;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<TreeViewMsg> {
        if self.tip_count > 0 {
            let mut side_col: Column<TreeViewMsg> = Column::new();
            let mut main_row: Row<TreeViewMsg> = Row::new();

            side_col = side_col.push(horizontal_space().height(Length::Fixed(PADDING)));
            side_col = side_col.push(
                container(text!("Edge Spacing").size(TEXT_SIZE))
                    .align_x(Horizontal::Right)
                    .width(Length::Fill),
            );
            side_col = side_col.push(self.node_size_slider());

            side_col = side_col.push(horizontal_space().height(Length::Fixed(PADDING)));
            side_col = side_col.push(self.tip_labels_toggler());
            if self.draw_tip_labels {
                side_col = side_col.push(self.tip_labels_size_slider());
            }

            side_col = side_col.push(horizontal_space().height(Length::Fixed(PADDING)));
            side_col = side_col.push(self.int_labels_toggler());
            if self.draw_int_labels {
                side_col = side_col.push(self.int_labels_size_slider());
            }

            side_col = side_col.push(horizontal_space().height(Length::Fixed(PADDING)));

            side_col = side_col.push(horizontal_rule(PADDING));
            side_col = side_col.push(horizontal_space().height(Length::Fixed(PADDING)));

            side_col = side_col.push(
                row![
                    text!("Node Order")
                        .size(TEXT_SIZE)
                        .align_x(Alignment::Start),
                    self.node_ordering_options_pick_list(),
                ]
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .spacing(PADDING),
            );

            side_col = side_col.width(Length::Fixed(SF * 2e2));

            if self.canvas_h > self.max_available_window_h {
                main_row = main_row.push(self.scrollable(self.tree_canvas()));
                main_row = main_row.push(vertical_space().width(PADDING));
            } else {
                main_row = main_row.push(self.tree_canvas());
                main_row = main_row.push(self.vertical_rule());
                main_row = main_row.push(vertical_space().width(PADDING + 1e0));
            }

            main_row = main_row.push(side_col);
            main_row = main_row.padding(PADDING);
            main_row.into()
        } else {
            container(Button::new("Open a Tree File").on_press(TreeViewMsg::OpenFile))
                .center(Length::Fill)
                .into()
        }
    }

    fn tree_canvas(&self) -> Canvas<&TreeView, TreeViewMsg> {
        Canvas::new(self)
            .height(Length::Fixed(self.canvas_h))
            .width(Length::Fill)
    }

    fn vertical_rule(&self) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::vertical(1);
        self.apply_rule_settings(rule)
    }

    fn node_ordering_options_pick_list(
        &self,
    ) -> PickList<NodeOrderingOption, &[NodeOrderingOption], NodeOrderingOption, TreeViewMsg> {
        let h: PickListHandle<Font> = PickListHandle::Arrow {
            size: Some(Pixels(TEXT_SIZE)),
        };

        let mut pl: PickList<
            NodeOrderingOption,
            &[NodeOrderingOption],
            NodeOrderingOption,
            TreeViewMsg,
        > = PickList::new(
            &NODE_ORDERING_OPTIONS,
            self.selected_node_ordering_option,
            TreeViewMsg::NodeOrderingOptionChanged,
        );

        pl = pl.text_size(TEXT_SIZE);
        pl = pl.padding(PADDING_INNER);
        pl = pl.width(Length::Fill);
        pl = pl.handle(h);
        pl = pl.text_line_height(LINE_H);

        pl = pl.style(|theme, status| {
            let palette = theme.extended_palette();

            let active = PickListStyle {
                text_color: palette.background.weak.text,
                background: palette.background.weak.color.into(),
                placeholder_color: palette.background.strong.color,
                handle_color: palette.background.weak.text,
                border: Border {
                    radius: (SF * 2e0).into(),
                    width: 1e0 * SF,
                    color: palette.background.strong.color,
                },
            };

            match status {
                PickListStatus::Active => active,
                PickListStatus::Hovered | PickListStatus::Opened { .. } => PickListStyle {
                    border: Border {
                        color: palette.primary.strong.color,
                        ..active.border
                    },
                    ..active
                },
            }
        });
        pl
    }

    #[allow(dead_code)]
    fn horizontal_rule(&self) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::horizontal(1);
        self.apply_rule_settings(rule)
    }

    fn tip_labels_toggler(&self) -> Toggler<'_, TreeViewMsg> {
        self.apply_toggler_settings("Tip Labels", self.draw_tip_labels)
            .on_toggle(TreeViewMsg::TipLabelVisibilityChanged)
    }

    fn tip_labels_size_slider(&self) -> Slider<Float, TreeViewMsg> {
        let mut sldr: Slider<Float, TreeViewMsg> = Slider::new(
            3e0 * SF..=self.max_label_size,
            self.tip_label_size,
            TreeViewMsg::TipLabelSizeChanged,
        );
        sldr = sldr.step(1e0 * SF);
        sldr = sldr.shift_step(5e0 * SF);
        self.apply_slider_settings(sldr)
    }

    fn int_labels_toggler(&self) -> Toggler<'_, TreeViewMsg> {
        self.apply_toggler_settings("Internal Labels", self.draw_int_labels)
            .on_toggle(TreeViewMsg::IntLabelVisibilityChanged)
    }

    fn int_labels_size_slider(&self) -> Slider<Float, TreeViewMsg> {
        let mut sldr: Slider<Float, TreeViewMsg> = Slider::new(
            3e0 * SF..=self.max_label_size,
            self.int_label_size,
            TreeViewMsg::IntLabelSizeChanged,
        );
        sldr = sldr.step(1e0 * SF);
        sldr = sldr.shift_step(5e0 * SF);
        self.apply_slider_settings(sldr)
    }

    fn node_size_slider(&self) -> Slider<Float, TreeViewMsg> {
        let mut sldr: Slider<Float, TreeViewMsg> = Slider::new(
            self.min_node_size..=self.max_label_size,
            self.node_size,
            TreeViewMsg::NodeSizeChanged,
        );
        sldr = sldr.step(1e0 * SF);
        sldr = sldr.shift_step(5e0 * SF);
        self.apply_slider_settings(sldr)
    }

    fn scrollable<'a>(
        &'a self,
        cnv: Canvas<&'a TreeView, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
        let mut scrl_bar = Scrollbar::new();
        scrl_bar = scrl_bar.width(Pixels(SCROLL_BAR_W));
        scrl_bar = scrl_bar.scroller_width(Pixels(SCROLL_BAR_W));
        scrl_bar = scrl_bar.anchor(ScrollBarAnchor::Start);
        scrl = scrl.direction(ScrollableDirection::Vertical(scrl_bar));
        scrl = scrl.height(self.max_available_window_h + SF);
        self.apply_scroller_settings(scrl)
    }

    fn apply_scroller_settings<'a>(
        &'a self,
        scrl: Scrollable<'a, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        scrl.style(|theme, status| {
            let palette = theme.extended_palette();

            let scrollbar = ScrollBarRail {
                background: Some(palette.background.weak.color.into()),
                border: border::rounded(2e1 * SF),
                scroller: Scroller {
                    color: palette.background.strong.color,
                    border: border::rounded(2e1 * SF),
                },
            };

            match status {
                ScrollBarStatus::Active { .. } => ScrollBarStyle {
                    container: container::Style::default(),
                    vertical_rail: scrollbar,
                    horizontal_rail: scrollbar,
                    gap: None,
                },
                ScrollBarStatus::Hovered {
                    is_horizontal_scrollbar_hovered,
                    is_vertical_scrollbar_hovered,
                    ..
                } => {
                    let hovered_scrollbar = ScrollBarRail {
                        scroller: Scroller {
                            color: palette.primary.strong.color,
                            ..scrollbar.scroller
                        },
                        ..scrollbar
                    };

                    ScrollBarStyle {
                        container: container::Style::default(),
                        vertical_rail: if is_vertical_scrollbar_hovered {
                            hovered_scrollbar
                        } else {
                            scrollbar
                        },
                        horizontal_rail: if is_horizontal_scrollbar_hovered {
                            hovered_scrollbar
                        } else {
                            scrollbar
                        },
                        gap: None,
                    }
                }
                ScrollBarStatus::Dragged {
                    is_horizontal_scrollbar_dragged,
                    is_vertical_scrollbar_dragged,
                    ..
                } => {
                    let dragged_scrollbar = ScrollBarRail {
                        scroller: Scroller {
                            color: palette.primary.base.color,
                            ..scrollbar.scroller
                        },
                        ..scrollbar
                    };

                    ScrollBarStyle {
                        container: container::Style::default(),
                        vertical_rail: if is_vertical_scrollbar_dragged {
                            dragged_scrollbar
                        } else {
                            scrollbar
                        },
                        horizontal_rail: if is_horizontal_scrollbar_dragged {
                            dragged_scrollbar
                        } else {
                            scrollbar
                        },
                        gap: None,
                    }
                }
            }
        })
    }

    fn apply_toggler_settings<'a>(&self, label: &'a str, value: bool) -> Toggler<'a, TreeViewMsg> {
        let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
        tglr = tglr.label(label);
        tglr = tglr.text_size(TEXT_SIZE);
        tglr = tglr.size(TEXT_SIZE * 1.5);
        tglr = tglr.spacing(PADDING_INNER);
        tglr = tglr.text_alignment(Alignment::End);
        tglr = tglr.width(Length::Fill);
        tglr = tglr.text_line_height(LINE_H);
        tglr
    }

    fn apply_rule_settings<'a>(&self, rule: Rule<'a, WidgetTheme>) -> Rule<'a, WidgetTheme> {
        rule.style(|theme| {
            let palette = theme.extended_palette();
            RuleStyle {
                color: palette.background.strong.color,
                width: (SF * 1e0) as u16,
                radius: (SF * 2e0).into(),
                fill_mode: RuleFillMode::Full,
            }
        })
    }

    fn apply_slider_settings<'a, T>(
        &'a self,
        sldr: Slider<'a, T, TreeViewMsg>,
    ) -> Slider<'a, T, TreeViewMsg>
    where
        T: std::marker::Copy,
        T: std::convert::From<u8>,
        T: std::cmp::PartialOrd,
    {
        let sldr = sldr.height(TEXT_SIZE + PADDING);

        sldr.style(|theme, status| {
            let palette = theme.extended_palette();

            let color = match status {
                SliderStatus::Active => palette.primary.base.color,
                SliderStatus::Hovered => palette.primary.strong.color,
                SliderStatus::Dragged => palette.primary.weak.color,
            };

            SliderStyle {
                rail: SliderRail {
                    backgrounds: (color.into(), palette.background.strong.color.into()),
                    width: TEXT_SIZE / 3e0,
                    border: Border {
                        radius: (SF * 2e0).into(),
                        width: 0e0 * SF,
                        color: Color::TRANSPARENT,
                    },
                },
                handle: SliderHandle {
                    shape: SliderHandleShape::Circle {
                        radius: TEXT_SIZE / 1.75,
                    },
                    background: color.into(),
                    border_color: Color::TRANSPARENT,
                    border_width: 0e0 * SF,
                },
            }
        })
    }

    fn sort(&mut self) {
        match self.selected_node_ordering_option.unwrap() {
            NodeOrderingOption::Unordered => {
                self.tree = self.tree_original.clone();
                self.tree_chunked_edges = match &self.tree_original_chunked_edges {
                    Some(chunked_edges) => chunked_edges.clone(),
                    None => {
                        self.tree_original_chunked_edges =
                            Some(flatten_tree(&self.tree, self.threads));
                        self.tree_original_chunked_edges.clone().unwrap()
                    }
                };
            }
            NodeOrderingOption::Ascending => match &self.tree_srtd_asc {
                Some(tree_srtd_asc) => {
                    self.tree = tree_srtd_asc.clone();
                    self.tree_chunked_edges = self.tree_srtd_asc_chunked_edges.clone().unwrap();
                }
                None => {
                    let mut tmp = self.tree_original.clone();
                    tmp.sort(false);
                    self.tree_srtd_asc = Some(tmp);
                    self.tree = self.tree_srtd_asc.clone().unwrap();
                    self.tree_srtd_asc_chunked_edges = Some(flatten_tree(&self.tree, self.threads));
                    self.tree_chunked_edges = self.tree_srtd_asc_chunked_edges.clone().unwrap();
                }
            },

            NodeOrderingOption::Descending => match &self.tree_srtd_desc {
                Some(tree_srtd_desc) => {
                    self.tree = tree_srtd_desc.clone();
                    self.tree_chunked_edges = self.tree_srtd_desc_chunked_edges.clone().unwrap();
                }
                None => {
                    let mut tmp = self.tree_original.clone();
                    tmp.sort(true);
                    self.tree_srtd_desc = Some(tmp);
                    self.tree = self.tree_srtd_desc.clone().unwrap();
                    self.tree_srtd_desc_chunked_edges =
                        Some(flatten_tree(&self.tree, self.threads));
                    self.tree_chunked_edges = self.tree_srtd_desc_chunked_edges.clone().unwrap();
                }
            },
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeOrderingOption {
    Unordered,
    Ascending,
    Descending,
}

impl std::fmt::Display for NodeOrderingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NodeOrderingOption::Unordered => "Unordered",
            NodeOrderingOption::Ascending => "Ascending",
            NodeOrderingOption::Descending => "Descending",
        })
    }
}

const NODE_ORDERING_OPTIONS: [NodeOrderingOption; 3] = [
    NodeOrderingOption::Unordered,
    NodeOrderingOption::Ascending,
    NodeOrderingOption::Descending,
];
