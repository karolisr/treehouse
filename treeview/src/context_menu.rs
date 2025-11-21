use crate::{Clr, Display, Formatter, NodeId, Result, TreeState, TvMsg};

#[derive(Debug, Clone)]
pub struct TvContextMenuItem {
    pub msg: TvMsg,
    pub label: String,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone)]
pub struct TvContextMenuListing {
    items: Vec<TvContextMenuItem>,
}

impl TvContextMenuListing {
    pub(crate) fn for_node(node_id: NodeId, tree_state: &TreeState) -> Self {
        Self::default().push(TvMsg::Root(node_id), Some(tree_state)).push(
            TvMsg::AddRemoveCladeLabel((node_id, Clr::GRN_25)),
            Some(tree_state),
        )
    }

    pub(crate) fn for_tip_lab_w_resize_area() -> Self {
        Self::default().push(TvMsg::TipLabWidthSetByUser(None), None)
    }

    pub fn items(&self) -> &[TvContextMenuItem] {
        &self.items
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
                    label: "Root here",
                },

                TvMsg::AddRemoveCladeLabel((node_id, _)) => {
                    let label = match tree_state.clade_has_label(node_id) {
                        true => "Unlabel",
                        false => "Label",
                    };
                    Values { enabled: true, label }
                }
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

impl Display for TvContextMenuListing {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s: String = String::new();
        self.items().iter().enumerate().for_each(|(i, item)| {
            s.push_str(
                format!(
                    "\n\t{i}: {} [enabled={}] {:?}",
                    item.label, item.enabled, item.msg
                )
                .as_str(),
            );
        });
        writeln!(f, "{s}")
    }
}
