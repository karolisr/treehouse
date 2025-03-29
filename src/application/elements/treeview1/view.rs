// #[cfg(not(debug_assertions))]
use super::super::canvas::Canvas;
use crate::{Edges, Tree, flatten_tree, window_settings};
use iced::{
    Element, Length, Task,
    alignment::{Horizontal, Vertical},
    widget::{
        canvas::Cache, column, container, pick_list, row, scrollable, slider, text, vertical_space,
    },
};

type Float = f32;

#[derive(Debug, Default)]
pub struct TreeView1 {
    pub(super) tree: Tree,
    tree_orig: Tree,
    pub(super) cache: Cache,
    node_sort_selection: Option<NodeSortOptions>,
    canvas_height: Float,
    window_height: Float,
    pub(super) lab_size: Float,
    pub(super) node_size: Float,
    pub(super) tree_height: Float,
    pub(super) tip_count: usize,
    node_count: usize,
    pub(super) edges_chunks: Vec<Edges>,
}
#[derive(Debug, Clone)]
pub enum TreeView1Msg {
    CacheClearRequested,
    TreeUpdated(Tree),
    NodeSortOptionChanged(NodeSortOptions),
    CanvasHeightChanged(Float),
    WindowHeightChanged(Float),
    LabelSizeChanged(Float),
    NodeSizeChanged(Float),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeSortOptions {
    Original,
    Ascending,
    Descending,
}

impl std::fmt::Display for NodeSortOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NodeSortOptions::Original => "Original",
            NodeSortOptions::Ascending => "Ascending",
            NodeSortOptions::Descending => "Descending",
        })
    }
}

impl TreeView1 {
    const NODE_SORT_OPTIONS: [NodeSortOptions; 3] = [
        NodeSortOptions::Original,
        NodeSortOptions::Ascending,
        NodeSortOptions::Descending,
    ];

    pub fn new() -> Self {
        Self {
            tree: Tree::default(),
            tree_orig: Tree::default(),
            cache: Cache::new(),
            node_sort_selection: Some(NodeSortOptions::Ascending),
            canvas_height: window_settings().size.height,
            window_height: window_settings().size.height - 2e1,
            lab_size: 1e0,
            node_size: 1e0,
            ..Default::default()
        }
    }

    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    pub fn update(&mut self, msg: TreeView1Msg) -> Task<TreeView1Msg> {
        match msg {
            TreeView1Msg::CacheClearRequested => {
                self.cache.clear();
                Task::none()
            }
            TreeView1Msg::NodeSizeChanged(s) => {
                self.node_size = s;
                Task::none()
            }
            TreeView1Msg::LabelSizeChanged(s) => {
                self.lab_size = s;
                Task::done(TreeView1Msg::CacheClearRequested)
            }
            TreeView1Msg::CanvasHeightChanged(h) => {
                self.canvas_height = h;
                Task::done(TreeView1Msg::CacheClearRequested)
            }
            TreeView1Msg::WindowHeightChanged(h) => {
                self.window_height = h - 2e1;
                self.node_size = self.window_height / self.tip_count as Float;
                if self.node_size < 4e0 {
                    self.lab_size = 4e0;
                } else {
                    self.lab_size = self.node_size;
                }
                Task::none()
            }
            TreeView1Msg::TreeUpdated(tree) => {
                self.tree_orig = tree.clone();
                self.tree = tree;

                match self.node_sort_selection.unwrap() {
                    NodeSortOptions::Original => self.tree = self.tree_orig.clone(),
                    NodeSortOptions::Ascending => self.tree.sort(false),
                    NodeSortOptions::Descending => self.tree.sort(true),
                };

                self.tree_height = self.tree.height() as Float;
                self.tip_count = self.tree.tip_count_all();
                self.node_count = self.tree.node_count_all();
                self.node_size = self.window_height / self.tip_count as Float;
                if self.node_size < 4e0 {
                    self.lab_size = 4e0;
                } else {
                    self.lab_size = self.node_size;
                }
                self.edges_chunks = flatten_tree(&self.tree, 8);
                Task::done(TreeView1Msg::CacheClearRequested)
            }
            TreeView1Msg::NodeSortOptionChanged(option) => {
                match option {
                    NodeSortOptions::Original => self.tree = self.tree_orig.clone(),
                    NodeSortOptions::Ascending => self.tree.sort(false),
                    NodeSortOptions::Descending => self.tree.sort(true),
                };
                self.node_sort_selection = Some(option);
                self.edges_chunks = flatten_tree(&self.tree, 8);
                Task::done(TreeView1Msg::CacheClearRequested)
            }
        }
    }

    pub fn view(&self) -> Element<TreeView1Msg> {
        if self.tree.tip_count_all() > 0 {
            let cnv = Canvas::new(self)
                .width(Length::Fill)
                .height(self.canvas_height);
            let text_size = 13;
            let padding: u32 = 10;
            let sidebar_width = 225;
            row![
                container(scrollable(cnv).spacing(padding / 2)).align_y(Vertical::Center),
                column![
                    column![
                        row![
                            column![
                                text!("Tips:").size(text_size).align_y(Vertical::Center),
                                text!("Nodes:",).size(text_size).align_y(Vertical::Center),
                                text!("Height:").size(text_size).align_y(Vertical::Center),
                            ]
                            .align_x(Horizontal::Right)
                            .spacing(padding / 2),
                            column![
                                text!("{:<7}", self.tip_count)
                                    .size(text_size)
                                    .align_y(Vertical::Center),
                                text!("{:<7}", self.node_count)
                                    .size(text_size)
                                    .align_y(Vertical::Center),
                                text!("{:<7.4}", self.tree_height)
                                    .size(text_size)
                                    .align_y(Vertical::Center),
                            ]
                            .align_x(Horizontal::Left)
                            .spacing(padding / 2),
                        ]
                        .padding(padding as Float)
                        .spacing(padding / 2)
                        .align_y(Vertical::Center),
                    ]
                    .align_x(Horizontal::Center)
                    .padding(padding as Float)
                    .width(sidebar_width),
                    row![
                        text!("Label Size:").size(text_size),
                        slider(
                            self.window_height / self.tip_count as Float..=1e1,
                            self.lab_size,
                            TreeView1Msg::LabelSizeChanged
                        )
                        .height(text_size)
                    ]
                    .spacing(padding / 2)
                    .padding(padding as Float)
                    .align_y(Vertical::Center)
                    .width(sidebar_width),
                    row![
                        text!("Node Size:").size(text_size),
                        slider(
                            self.window_height / self.tip_count as Float..=2e1,
                            self.node_size,
                            TreeView1Msg::NodeSizeChanged
                        )
                        .height(text_size)
                    ]
                    .spacing(padding / 2)
                    .padding(padding as Float)
                    .align_y(Vertical::Center)
                    .width(sidebar_width),
                    row![
                        text!("Sort:").size(text_size),
                        pick_list(
                            Self::NODE_SORT_OPTIONS,
                            self.node_sort_selection,
                            TreeView1Msg::NodeSortOptionChanged
                        )
                        .text_size(text_size)
                        .width(sidebar_width)
                    ]
                    .spacing(padding / 2)
                    .padding(padding as Float)
                    .align_y(Vertical::Center)
                    .width(sidebar_width),
                    vertical_space()
                ]
                .align_x(Horizontal::Center)
                .spacing(padding / 2)
                .padding(0),
            ]
            .align_y(Vertical::Top)
            .into()
        } else {
            container(
                text!("No Tree Loaded")
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            )
            .center(Length::Fill)
            .into()
        }
    }
}
