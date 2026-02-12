use riced::Point;
use riced::Vector;

use crate::{Clr, Display, Formatter, NodeId, Result, TreeState, TvMsg};

#[derive(Debug, Clone)]
pub struct TvContextMenuItem {
    pub msg: TvMsg,
    pub label: String,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone)]
pub struct TvContextMenuSpecification {
    items: Vec<TvContextMenuItem>,
    position: Point,
}

impl TvContextMenuSpecification {
    pub(crate) fn for_node(
        node_id: NodeId,
        tree_state: &TreeState,
        position: Point,
    ) -> Self {
        Self::default()
            .push(TvMsg::SetSubtreeView(node_id), Some(tree_state))
            .push(
                TvMsg::AddRemoveCladeHighlight((node_id, Clr::GRN_25)),
                Some(tree_state),
            )
            .push(TvMsg::Root(node_id), Some(tree_state))
            .push(TvMsg::RemoveNode(node_id), Some(tree_state))
            .set_position(position)
    }

    pub(crate) fn for_tip_lab_w_resize_area(position: Point) -> Self {
        Self::default()
            .push(TvMsg::TipLabWidthSetByUser(None), None)
            .set_position(position)
    }

    pub fn items(&self) -> &[TvContextMenuItem] {
        &self.items
    }

    pub fn position(&self) -> Point {
        self.position
    }

    fn set_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }

    pub fn set_position_offset(mut self, offset: Vector) -> Self {
        self.position += offset;
        self
    }

    fn push(mut self, tv_msg: TvMsg, tree_state: Option<&TreeState>) -> Self {
        struct Values<'a> {
            enabled: bool,
            label: &'a str,
        }

        let enabled: bool;
        let label: &str;

        if let Some(tree_state) = tree_state {
            Values { enabled, label } = match tv_msg {
                TvMsg::Root(node_id) => Values {
                    enabled: tree_state
                        .is_valid_potential_outgroup_node(node_id)
                        && !tree_state.is_subtree_view_active(),
                    label: "Root Here",
                },

                TvMsg::SetSubtreeView(node_id) => Values {
                    enabled: tree_state
                        .is_valid_potential_subtree_view_node(node_id),
                    label: "View Subtree",
                },

                TvMsg::AddRemoveCladeHighlight((node_id, _)) => {
                    let label = match tree_state.clade_has_highlight(node_id) {
                        true => "Remove Clade Highlight",
                        false => "Highlight Clade",
                    };
                    Values { enabled: true, label }
                }

                TvMsg::RemoveNode(node_id) => Values {
                    enabled: tree_state.can_node_be_removed(node_id),
                    label: "Drop This Node",
                },

                _ => return self,
            };
        } else {
            Values { enabled, label } = match tv_msg {
                TvMsg::TipLabWidthSetByUser(None) => {
                    Values { enabled: true, label: "Reset tip position" }
                }
                _ => return self,
            }
        }

        let item: TvContextMenuItem = TvContextMenuItem {
            msg: tv_msg,
            label: label.to_string(),
            enabled,
        };

        self.items.push(item);
        self
    }
}

impl Display for TvContextMenuSpecification {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s: String = String::new();

        s.push_str(
            format!(
                "    position: {:0.2}, {:0.2}\n    items:",
                self.position.x, self.position.y
            )
            .as_str(),
        );

        self.items().iter().enumerate().for_each(|(i, item)| {
            s.push_str(
                format!(
                    "\n      {i}: {} [enabled={}] {:?}",
                    item.label, item.enabled, item.msg
                )
                .as_str(),
            );
        });
        write!(f, "{s}")
    }
}
