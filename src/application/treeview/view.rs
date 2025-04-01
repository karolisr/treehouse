use super::Canvas;
use crate::{
    Edges, Float, PADDING, PADDING_INNER, S_BAR_W, SF, SPACING, TEXT_SIZE, Tree, flatten_tree,
    max_name_len, window_settings,
};
use iced::{
    Border, Color, Element, Font, Length, Pixels, Task,
    widget::{
        Column, PickList, Row, Scrollable, Slider, Toggler,
        canvas::Cache,
        horizontal_space,
        pick_list::{Handle as PickListHandle, Status as PickListStatus, Style as PickListStyle},
        scrollable::{Direction as ScrollableDirection, Scrollbar},
        slider::{
            Handle as SliderHandle, HandleShape as SliderHandleShape, Rail as SliderRail,
            Status as SliderStatus, Style as SliderStyle,
        },
    },
};

#[derive(Debug)]

pub struct TreeView {
    threads: usize,
    selected_node_sorting_option: Option<NodeSortingOption>,
    pub(super) drawing_enabled: bool,

    pub(super) node_count: usize,
    pub(super) tip_count: usize,
    pub(super) int_node_count: usize,
    pub(super) max_name_len: usize,

    pub(super) canvas_h: Float,

    pub(super) window_w: Float,
    pub(super) window_h: Float,

    pub(super) tip_label_size: Float,
    pub(super) int_label_size: Float,
    pub(super) node_size: Float,
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
            threads: 8,
            tree: Default::default(),
            drawing_enabled: false,
            selected_node_sorting_option: Some(NodeSortingOption::Unsorted),

            node_count: 0,
            tip_count: 0,
            int_node_count: 0,
            max_name_len: 0,

            canvas_h: window_settings().size.height * SF,
            window_w: window_settings().size.width * SF,
            window_h: window_settings().size.height * SF,

            node_size: SF * 1e1,
            tip_label_size: SF * 1e1,
            int_label_size: SF * 1e1,
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
    TreeUpdated(Tree),
    NodeSortingOptionChanged(NodeSortingOption),
    WindowResized(Float, Float),
    NodeSizeChanged(Float),
    TipLabelSizeChanged(Float),
    IntLabelSizeChanged(Float),
    TipLabelVisibilityChanged(bool),
    IntLabelVisibilityChanged(bool),
}

impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::IntLabelVisibilityChanged(state) => {
                self.draw_int_labels = state;
                Task::none()
            }

            TreeViewMsg::TipLabelVisibilityChanged(state) => {
                self.draw_tip_labels = state;
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::TipLabelSizeChanged(s) => {
                self.tip_label_size = s;
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::IntLabelSizeChanged(s) => {
                self.int_label_size = s;
                self.int_labels_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::NodeSizeChanged(s) => {
                self.node_size = s;
                self.calc_canvas_height();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::NodeSortingOptionChanged(node_sorting_option) => {
                if node_sorting_option != self.selected_node_sorting_option.unwrap() {
                    self.selected_node_sorting_option = Some(node_sorting_option);
                    self.sort();
                    self.edge_geom_cache.clear();
                    self.tip_labels_geom_cache.clear();
                    self.int_labels_geom_cache.clear();
                }
                Task::none()
            }

            TreeViewMsg::WindowResized(w, h) => {
                self.window_w = w;
                self.window_h = h;
                self.calc_canvas_height();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                Task::none()
            }

            TreeViewMsg::TreeUpdated(tree) => {
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
                self.calc_canvas_height();
                self.edge_geom_cache.clear();
                self.tip_labels_geom_cache.clear();
                self.int_labels_geom_cache.clear();
                self.drawing_enabled = true;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<TreeViewMsg> {
        if self.tip_count > 0 {
            let mut col: Column<TreeViewMsg> = Column::new();
            let mut row: Row<TreeViewMsg> = Row::new();

            col = col.push(self.sort_options_pick_list());
            col = col.push(self.draw_tip_labels_toggler());
            if self.draw_tip_labels {
                col = col.push(self.tip_label_size_slider());
            }
            col = col.push(self.draw_int_labels_toggler());
            if self.draw_int_labels {
                col = col.push(self.int_label_size_slider());
            }
            col = col.push(self.node_size_slider());
            col = col.padding(PADDING);
            col = col.spacing(SPACING);
            col = col.width(Length::Fixed(SF * 2e2));
            row = row.push(self.scrollable(self.tree_canvas()));
            row = row.push(col);

            row.into()
        } else {
            horizontal_space().into()
        }
    }

    fn calc_canvas_height(&mut self) {
        self.canvas_h = self.tip_count as Float * self.node_size;
        if self.canvas_h <= self.window_h {
            self.canvas_h = self.window_h
        }
    }

    fn tree_canvas(&self) -> Canvas<&TreeView, TreeViewMsg> {
        Canvas::new(self).height(Length::Fixed(self.canvas_h))
    }

    fn tip_label_size_slider(&self) -> Slider<Float, TreeViewMsg> {
        let mut sldr: Slider<Float, TreeViewMsg> = Slider::new(
            1e0 * SF..=2e1 * SF,
            self.tip_label_size,
            TreeViewMsg::TipLabelSizeChanged,
        );
        sldr = sldr.step(1e0);
        sldr = sldr.shift_step(2e0);
        self.apply_slider_style(sldr)
    }

    fn int_label_size_slider(&self) -> Slider<Float, TreeViewMsg> {
        let mut sldr: Slider<Float, TreeViewMsg> = Slider::new(
            1e0 * SF..=2e1 * SF,
            self.int_label_size,
            TreeViewMsg::IntLabelSizeChanged,
        );
        sldr = sldr.step(1e0);
        sldr = sldr.shift_step(2e0);
        self.apply_slider_style(sldr)
    }

    fn node_size_slider(&self) -> Slider<Float, TreeViewMsg> {
        let mut sldr: Slider<Float, TreeViewMsg> = Slider::new(
            self.window_h / self.tip_count as Float..=2e1 * SF,
            self.node_size,
            TreeViewMsg::NodeSizeChanged,
        );
        sldr = sldr.step(1e0);
        sldr = sldr.shift_step(2e0);
        self.apply_slider_style(sldr)
    }

    fn draw_tip_labels_toggler(&self) -> Toggler<'_, TreeViewMsg> {
        self.apply_toggler_settings("Tip Labels", self.draw_tip_labels)
            .on_toggle(TreeViewMsg::TipLabelVisibilityChanged)
    }

    fn draw_int_labels_toggler(&self) -> Toggler<'_, TreeViewMsg> {
        self.apply_toggler_settings("Internal Labels", self.draw_int_labels)
            .on_toggle(TreeViewMsg::IntLabelVisibilityChanged)
    }

    fn scrollable<'a>(
        &'a self,
        cnv: Canvas<&'a TreeView, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut s: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
        let mut s_bar = Scrollbar::new();
        s_bar = s_bar.width(Pixels(S_BAR_W));
        s_bar = s_bar.scroller_width(Pixels(S_BAR_W - S_BAR_W / 2e0));
        s = s.direction(ScrollableDirection::Vertical(s_bar));
        s
    }

    fn sort_options_pick_list(
        &self,
    ) -> PickList<NodeSortingOption, &[NodeSortingOption], NodeSortingOption, TreeViewMsg> {
        let h: PickListHandle<Font> = PickListHandle::Arrow {
            size: Some(Pixels(TEXT_SIZE)),
        };

        let mut pl: PickList<
            NodeSortingOption,
            &[NodeSortingOption],
            NodeSortingOption,
            TreeViewMsg,
        > = PickList::new(
            &NODE_SORTTING_OPTIONS,
            self.selected_node_sorting_option,
            TreeViewMsg::NodeSortingOptionChanged,
        );

        pl = pl.text_size(TEXT_SIZE);
        pl = pl.padding(PADDING_INNER);
        pl = pl.width(Length::Fill);
        pl = pl.handle(h);

        pl = pl.style(|theme, status| {
            let palette = theme.extended_palette();

            let active = PickListStyle {
                text_color: palette.background.weak.text,
                background: palette.background.weak.color.into(),
                placeholder_color: palette.background.strong.color,
                handle_color: palette.background.weak.text,
                border: Border {
                    radius: (2e0 * SF).into(),
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

    fn apply_toggler_settings<'a>(&self, label: &'a str, value: bool) -> Toggler<'a, TreeViewMsg> {
        let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
        tglr = tglr.label(label);
        tglr = tglr.size(TEXT_SIZE);
        tglr = tglr.text_size(TEXT_SIZE);
        tglr
    }

    fn apply_slider_style<'a, T>(
        &'a self,
        sldr: Slider<'a, T, TreeViewMsg>,
    ) -> Slider<'a, T, TreeViewMsg>
    where
        T: std::marker::Copy,
        T: std::convert::From<u8>,
        T: std::cmp::PartialOrd,
    {
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
                    width: 4.0 * SF,
                    border: Border {
                        radius: (2.0 * SF).into(),
                        width: 0.0 * SF,
                        color: Color::TRANSPARENT,
                    },
                },
                handle: SliderHandle {
                    shape: SliderHandleShape::Circle { radius: 7.0 * SF },
                    background: color.into(),
                    border_color: Color::TRANSPARENT,
                    border_width: 0.0 * SF,
                },
            }
        })
    }

    fn sort(&mut self) {
        match self.selected_node_sorting_option.unwrap() {
            NodeSortingOption::Unsorted => {
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
            NodeSortingOption::Ascending => match &self.tree_srtd_asc {
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

            NodeSortingOption::Descending => match &self.tree_srtd_desc {
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
pub enum NodeSortingOption {
    Unsorted,
    Ascending,
    Descending,
}

impl std::fmt::Display for NodeSortingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NodeSortingOption::Unsorted => "Unsorted",
            NodeSortingOption::Ascending => "Ascending",
            NodeSortingOption::Descending => "Descending",
        })
    }
}

const NODE_SORTTING_OPTIONS: [NodeSortingOption; 3] = [
    NodeSortingOption::Unsorted,
    NodeSortingOption::Ascending,
    NodeSortingOption::Descending,
];
