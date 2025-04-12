use crate::{
    Edges, Float, LINE_H, PADDING, PADDING_INNER, SCROLL_BAR_W, SF, TEXT_SIZE, Tree, flatten_tree,
    lerp, text_width,
};
use dendros::{Edge, NodeId};
use iced::{
    Alignment, Border, Color, Element, Font, Length, Pixels, Task,
    alignment::{Horizontal, Vertical},
    border,
    widget::{
        Button, Canvas, Column, PickList, Row, Rule, Scrollable, Slider, Space,
        Theme as WidgetTheme, Toggler, button,
        canvas::Cache,
        container, horizontal_space,
        pick_list::{Handle as PickListHandle, Status as PickListStatus, Style as PickListStyle},
        row,
        rule::{FillMode as RuleFillMode, Style as RuleStyle},
        scrollable::{
            Anchor as ScrollBarAnchor, Direction as ScrollableDirection, Rail as ScrollBarRail,
            Scrollbar, Scroller, Status as ScrollBarStatus, Style as ScrollBarStyle,
            Viewport as ScrollableViewport,
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

    pub(super) canvas_h: Float,
    pub(super) cnv_y0: Float,
    pub(super) cnv_y1: Float,

    pub(super) window_w: Float,
    pub(super) window_h: Float,

    pub(super) min_label_size: Float,
    pub(super) max_label_size: Float,
    pub(super) tip_label_size: Float,
    pub(super) int_label_size: Float,

    pub(super) max_count_of_tip_labels_to_draw: usize,

    min_label_size_idx: u8,
    max_label_size_idx: u8,
    selected_tip_label_size_idx: u8,
    selected_int_label_size_idx: u8,

    pub(super) available_vertical_space: Float,
    pub(super) node_size: Float,
    pub(super) min_node_size: Float,
    pub(super) max_node_size: Float,

    min_node_size_idx: u8,
    max_node_size_idx: u8,
    selected_node_size_idx: u8,

    pub(super) tip_labels_w_scale_factor: Float,
    pub(super) tip_label_w: Float,
    pub(super) tip_label_offset: Float,
    pub(super) int_label_offset: Float,

    pub(super) draw_tip_labels_allowed: bool,
    pub(super) draw_tip_labels_selection: bool,
    pub(super) draw_int_labels_selection: bool,

    pub(super) pointer_geom_cache: Cache,
    pub(super) edge_geom_cache: Cache,
    pub(super) tip_labels_geom_cache: Cache,
    pub(super) int_labels_geom_cache: Cache,

    pub(super) tree: Tree,
    pub(super) tree_tip_edges: Vec<Edge>,
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
            threads: 1,
            tree: Default::default(),
            drawing_enabled: false,
            selected_node_ordering_option: Some(NodeOrderingOption::Unordered),

            node_count: 0,
            tip_count: 0,
            int_node_count: 0,

            canvas_h: SF,
            cnv_y0: SF,
            cnv_y1: SF,

            window_w: SF,
            window_h: SF,

            min_node_size_idx: 1,
            min_label_size_idx: 1,
            max_node_size_idx: 24,
            max_label_size_idx: 24,

            selected_node_size_idx: 1,

            tip_label_size: SF * 5e0,
            selected_tip_label_size_idx: 5,

            int_label_size: SF * 8e0,
            selected_int_label_size_idx: 8,

            node_size: SF,
            min_node_size: SF,
            max_node_size: SF,
            min_label_size: SF * 1e0,
            max_label_size: SF * 24e0,

            max_count_of_tip_labels_to_draw: 200,

            available_vertical_space: SF,

            tip_labels_w_scale_factor: 1e0,
            tip_label_w: SF,
            tip_label_offset: SF * 3e0,
            int_label_offset: SF * 3e0,

            draw_tip_labels_allowed: false,
            draw_tip_labels_selection: true,
            draw_int_labels_selection: false,

            pointer_geom_cache: Default::default(),
            edge_geom_cache: Default::default(),
            tip_labels_geom_cache: Default::default(),
            int_labels_geom_cache: Default::default(),

            tree_tip_edges: Default::default(),
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
    NodeSizeSelectionChanged(u8),
    TipLabelSizeSelectionChanged(u8),
    IntLabelSizeSelectionChanged(u8),
    TipLabelVisibilityChanged(bool),
    IntLabelVisibilityChanged(bool),
    TreeViewScrolled(ScrollableViewport),
    Root(NodeId),
    Unroot,
    OpenFile,
}

impl TreeView {
    fn update_node_size(&mut self) {
        self.available_vertical_space = self.window_h - PADDING * 2e0 - SF * 2e0;
        self.min_node_size = self.available_vertical_space / self.tip_count as Float;
        self.max_node_size = Float::max(self.max_label_size, self.min_node_size);
        self.max_node_size_idx = self.max_label_size_idx;

        if self.min_node_size == self.max_node_size {
            self.max_node_size_idx = self.min_node_size_idx
        }

        if self.selected_node_size_idx > self.max_node_size_idx {
            self.selected_node_size_idx = self.max_node_size_idx
        }

        if self.selected_node_size_idx == self.min_node_size_idx {
            self.cnv_y0 = 0e0;
            self.cnv_y1 = self.cnv_y0 + self.available_vertical_space;
        }

        if self.max_node_size_idx > 1 {
            self.node_size = lerp(
                self.min_node_size,
                self.max_node_size,
                (self.selected_node_size_idx - 1) as Float / self.max_node_size_idx as Float,
            )
        } else {
            self.node_size = self.min_node_size
        }

        self.draw_tip_labels_allowed = (self.available_vertical_space / self.node_size) as usize
            <= self.max_count_of_tip_labels_to_draw;

        self.update_tip_label_w();
        self.update_canvas_h();
    }

    fn update_canvas_h(&mut self) {
        self.canvas_h = self.node_size * self.tip_count as Float;
    }

    fn calc_tip_labels_w_scale_factor(&mut self) -> Float {
        let mut max_tip_height: f64 = 0e0;
        let mut max_tip_height_name: &str = "";
        let mut max_tip_height_name_len: usize = 0;
        for edge in &self.tree_tip_edges {
            if edge.x1 >= max_tip_height - max_tip_height / 1e1 {
                max_tip_height = edge.x1;
                if let Some(name) = &edge.name {
                    if name.len() > max_tip_height_name_len {
                        max_tip_height_name = name;
                        max_tip_height_name_len = name.len();
                    }
                }
            }
        }
        text_width(max_tip_height_name, 1e0, 1e0)
    }

    fn update_tip_label_w(&mut self) {
        if self.draw_tip_labels_allowed && self.draw_tip_labels_selection {
            self.tip_label_w =
                self.tip_labels_w_scale_factor * self.tip_label_size + self.tip_label_offset;
        } else {
            self.tip_label_w = 0e0;
        }
    }

    fn merge_tip_chunks(&mut self) {
        self.tree_tip_edges = Vec::new();
        for (i_c, chunk) in self.tree_chunked_edges.iter().enumerate() {
            for (i_e, edge) in chunk.iter().enumerate() {
                if edge.is_tip {
                    let mut e = edge.clone();
                    e.chunk_idx = i_c;
                    e.edge_idx = i_e;
                    self.tree_tip_edges.push(e);
                }
            }
        }
    }

    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::OpenFile => Task::none(),

            TreeViewMsg::TreeViewScrolled(vp) => {
                self.cnv_y0 = vp.absolute_offset().y;
                self.cnv_y1 = self.cnv_y0 + vp.bounds().height;
                self.tip_labels_geom_cache.clear();
                self.pointer_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.draw_tip_labels_selection = state;
                self.update_node_size();
                Task::none()
            }

            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labels_selection = state;
                Task::none()
            }

            TreeViewMsg::TipLabelSizeSelectionChanged(idx) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.selected_tip_label_size_idx = idx;
                self.tip_label_size = self.min_label_size * idx as Float;
                self.update_node_size();
                Task::none()
            }

            TreeViewMsg::IntLabelSizeSelectionChanged(idx) => {
                self.int_labels_geom_cache.clear();
                self.selected_int_label_size_idx = idx;
                self.int_label_size = self.min_label_size * idx as Float;
                Task::none()
            }

            TreeViewMsg::NodeSizeSelectionChanged(idx) => {
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.selected_node_size_idx = idx;
                self.update_node_size();
                Task::none()
            }

            TreeViewMsg::NodeOrderingOptionChanged(node_ordering_option) => {
                if node_ordering_option != self.selected_node_ordering_option.unwrap() {
                    self.edge_geom_cache.clear();
                    self.tip_labels_geom_cache.clear();
                    self.pointer_geom_cache.clear();
                    self.int_labels_geom_cache.clear();
                    self.selected_node_ordering_option = Some(node_ordering_option);
                    self.sort();
                    self.merge_tip_chunks();
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
                self.pointer_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.window_w = w;
                self.window_h = h;
                self.update_node_size();
                Task::none()
            }

            TreeViewMsg::Root(node_id) => {
                let mut tree_to_root = self.tree.clone();
                let rslt = tree_to_root.root(node_id);

                match rslt {
                    Ok(_) => Task::done(TreeViewMsg::TreeUpdated(tree_to_root)),
                    Err(err) => {
                        println!("{err}");
                        Task::none()
                    }
                }
            }

            TreeViewMsg::Unroot => {
                self.tree_original.unroot();
                Task::done(TreeViewMsg::TreeUpdated(self.tree_original.clone()))
            }

            TreeViewMsg::TreeUpdated(tree) => {
                self.drawing_enabled = false;
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.pointer_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.tree_original = tree;
                self.tree_srtd_asc = None;
                self.tree_srtd_desc = None;
                self.tree_srtd_asc_chunked_edges = None;
                self.tree_srtd_desc_chunked_edges = None;
                self.tree_original_chunked_edges = None;
                self.node_count = self.tree_original.node_count_all();
                self.tip_count = self.tree_original.tip_count_all();
                self.int_node_count = self.tree_original.internal_node_count_all();
                self.sort();
                self.merge_tip_chunks();
                self.tip_labels_w_scale_factor = self.calc_tip_labels_w_scale_factor();
                self.update_node_size();
                self.drawing_enabled = true;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<TreeViewMsg> {
        if self.tip_count == 0 {
            return container(Button::new("Open a Tree File").on_press(TreeViewMsg::OpenFile))
                .center(Length::Fill)
                .into();
        }
        let mut side_col: Column<TreeViewMsg> = Column::new();
        let mut main_row: Row<TreeViewMsg> = Row::new();
        if self.min_node_size_idx != self.max_node_size_idx {
            side_col = side_col.push(self.horizontal_space(0, PADDING));
            side_col = side_col.push(self.horizontal_rule(SF));
            side_col = side_col.push(self.horizontal_space(0, PADDING));
            side_col = side_col.push(
                container(text!("Edge Spacing").size(TEXT_SIZE))
                    .align_x(Horizontal::Right)
                    .width(Length::Fill),
            );
            side_col = side_col.push(self.node_size_slider());
        }

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.horizontal_rule(SF));
        side_col = side_col.push(self.horizontal_space(0, PADDING));

        side_col = side_col.push(self.tip_labels_toggler(self.draw_tip_labels_allowed));
        if self.draw_tip_labels_allowed && self.draw_tip_labels_selection {
            side_col = side_col.push(self.tip_labels_size_slider());
        }

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.int_labels_toggler(true));
        if self.draw_int_labels_selection {
            side_col = side_col.push(self.int_labels_size_slider());
        }

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.horizontal_rule(SF));
        side_col = side_col.push(self.horizontal_space(0, PADDING));
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
        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.horizontal_rule(SF));
        side_col = side_col.push(self.horizontal_space(0, PADDING));

        side_col = side_col.push(
            row![
                button("Unroot").on_press_maybe(match self.tree_original.is_rooted() {
                    true => Some(TreeViewMsg::Unroot),
                    false => None,
                }),
                button("Root").on_press_maybe(None)
            ]
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .spacing(PADDING),
        );

        side_col = side_col.width(Length::Fixed(SF * 2e2));

        match self.selected_node_size_idx {
            idx if idx == self.min_node_size_idx => {
                main_row = main_row.push(self.tree_canvas());
                main_row = main_row.push(self.vertical_rule(SF))
            }
            _ => {
                main_row = {
                    main_row = main_row.push(self.scrollable(self.tree_canvas()));
                    main_row.push(self.vertical_space(SF, 0))
                }
            }
        }
        main_row = main_row.push(self.vertical_space(PADDING, 0));
        main_row = main_row.push(side_col);
        main_row = main_row.padding(PADDING);
        main_row.into()
    }

    fn tree_canvas(&self) -> Canvas<&TreeView, TreeViewMsg> {
        Canvas::new(self)
            .height(Length::Fixed(self.canvas_h))
            .width(Length::Fill)
    }

    fn horizontal_space(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        horizontal_space().width(width).height(height)
    }

    fn vertical_space(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        vertical_space().width(width).height(height)
    }

    fn horizontal_rule(&self, height: impl Into<Pixels>) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::horizontal(height);
        self.apply_rule_settings(rule)
    }

    fn vertical_rule(&self, width: impl Into<Pixels>) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::vertical(width);
        self.apply_rule_settings(rule)
    }

    fn tip_labels_toggler(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tgl = self.apply_toggler_settings("Tip Labels", self.draw_tip_labels_selection);
        if enabled {
            tgl = tgl.on_toggle(TreeViewMsg::TipLabelVisibilityChanged);
        }
        tgl
    }

    fn int_labels_toggler(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tgl =
            self.apply_toggler_settings("Internal Labels", self.draw_int_labels_selection);
        if enabled {
            tgl = tgl.on_toggle(TreeViewMsg::IntLabelVisibilityChanged);
        }
        tgl
    }

    fn node_size_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_node_size_idx..=self.max_node_size_idx,
            self.selected_node_size_idx,
            TreeViewMsg::NodeSizeSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
    }

    fn tip_labels_size_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_label_size_idx..=self.max_label_size_idx,
            self.selected_tip_label_size_idx,
            TreeViewMsg::TipLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
    }

    fn int_labels_size_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_label_size_idx..=self.max_label_size_idx,
            self.selected_int_label_size_idx,
            TreeViewMsg::IntLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
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
        scrl = scrl.height(self.available_vertical_space + SF);
        scrl = scrl.on_scroll(TreeViewMsg::TreeViewScrolled);
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
