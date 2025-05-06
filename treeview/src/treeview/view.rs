use super::{SidebarLocation, TreeState};
use crate::{TreeView, TreeViewMsg};
use iced::{
    Element,
    widget::{Column, Row},
};

impl TreeView {
    pub fn view(&self) -> Element<TreeViewMsg> {
        let ts: &TreeState;

        if let Some(idx) = self.sel_tree_idx {
            ts = &self.trees[idx];
        } else {
            return iced::widget::vertical_space().into();
        }

        let mut mc = Column::new();
        let mut mr = Row::new();

        mc = mc.padding(6e0);
        mc = mc.spacing(6e0);
        mr = mr.spacing(6e0);

        if self.show_sidebar {
            match self.sidebar_position {
                SidebarLocation::Left => {
                    mr = mr.push(self.sidebar(ts));
                    mr = mr.push(self.content(ts));
                }
                SidebarLocation::Right => {
                    mr = mr.push(self.content(ts));
                    mr = mr.push(self.sidebar(ts));
                }
            }
        } else {
            mr = mr.push(self.content(ts));
        }

        if self.show_toolbar {
            mc = mc.push(self.toolbar(ts));
        }

        mc = mc.push(mr);

        if self.show_statusbar {
            mc = mc.push(self.statusbar(ts));
        }

        mc.into()
    }
}
