use super::super::canvas::Canvas;
use crate::{Edges, Tree, flatten_tree};
use iced::{
    Element, Length, Pixels, Task,
    widget::{PickList, canvas::Cache, column, container, pick_list::Handle},
};

#[derive(Debug)]
pub struct TreeView {
    threads: usize,
    tree: Tree,
    pub(super) drawing_enabled: bool,
    selected_node_sorting_option: Option<NodeSortingOption>,

    pub(super) bg_geom_cache: Cache,
    pub(super) edge_geom_cache: Cache,

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
            selected_node_sorting_option: Some(NodeSortingOption::Original),

            bg_geom_cache: Default::default(),
            edge_geom_cache: Default::default(),

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
}

impl TreeView {
    pub fn update(&mut self, msg: TreeViewMsg) -> Task<TreeViewMsg> {
        match msg {
            TreeViewMsg::TreeUpdated(tree) => {
                self.drawing_enabled = false;
                self.edge_geom_cache.clear();
                self.tree_original = tree.clone();
                self.tree = tree;
                self.tree_srtd_asc = None;
                self.tree_srtd_desc = None;
                self.tree_srtd_asc_chunked_edges = None;
                self.tree_srtd_desc_chunked_edges = None;
                self.tree_original_chunked_edges = None;
                self.sort();
                self.drawing_enabled = true;
                Task::none()
            }
            TreeViewMsg::NodeSortingOptionChanged(node_sorting_option) => {
                self.drawing_enabled = false;
                if node_sorting_option != self.selected_node_sorting_option.unwrap() {
                    self.edge_geom_cache.clear();
                    self.selected_node_sorting_option = Some(node_sorting_option);
                    self.sort();
                }
                self.drawing_enabled = true;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<TreeViewMsg> {
        let canvas = Canvas::new(self);
        let sortop = PickList::new(
            NODE_SORTTING_OPTIONS,
            self.selected_node_sorting_option,
            TreeViewMsg::NodeSortingOptionChanged,
        );
        container(
            column![
                sortop
                    .width(6e2)
                    .text_size(6e1)
                    .text_line_height(1e0)
                    .handle(Handle::Arrow {
                        size: Some(Pixels(6e1))
                    })
                    .padding(6e1),
                container(canvas).center(Length::Fill)
            ]
            .padding(0e1)
            .spacing(0e1),
        )
        .into()
    }

    pub fn sort(&mut self) {
        match self.selected_node_sorting_option.unwrap() {
            NodeSortingOption::Original => {
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
    Original,
    Ascending,
    Descending,
}

impl std::fmt::Display for NodeSortingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NodeSortingOption::Original => "Original",
            NodeSortingOption::Ascending => "Ascending",
            NodeSortingOption::Descending => "Descending",
        })
    }
}

const NODE_SORTTING_OPTIONS: [NodeSortingOption; 3] = [
    NodeSortingOption::Original,
    NodeSortingOption::Ascending,
    NodeSortingOption::Descending,
];
