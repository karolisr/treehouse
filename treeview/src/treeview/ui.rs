pub(crate) mod style;
mod widget;
mod widget_settings;

use super::{SidebarLocation, TreeState, TreeStyle, TreeView, TreeViewMsg};
use iced::{
    Element,
    Length::{Fill, Fixed, Shrink},
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Row, column, container, responsive},
};
use style::{sty_cont_main, sty_cont_sidebar, sty_cont_statusbar, sty_cont_toolbar};

impl TreeView {
    pub(crate) fn content(&self, _ts: &TreeState) -> Element<TreeViewMsg> {
        responsive(move |size| {
            container(self.pane_grid_main.view().map(TreeViewMsg::PaneGridMsg))
                .width(Fixed(size.width))
                .height(Fixed(size.height))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(sty_cont_main)
                .into()
        })
        .into()
    }

    pub(crate) fn sidebar(&self, ts: &TreeState) -> Element<TreeViewMsg> {
        let mut sc: Column<TreeViewMsg> = Column::new();

        sc = sc.push(self.stats(ts));
        sc = sc.push(self.pick_list_tree_style());
        sc = sc.push(self.pick_list_node_ordering());

        match self.sel_tree_style_opt {
            TreeStyle::Phylogram => {
                sc = sc.push(self.slider_width_canvas());
                if self.min_node_size_idx != self.max_node_size_idx {
                    sc = sc.push(self.slider_size_node());
                }
            }
            TreeStyle::Fan => {
                sc = sc.push(self.slider_width_canvas());
                sc = sc.push(self.slider_angle_opn());
                sc = sc.push(self.slider_angle_rot());
            }
        }

        sc = sc.push(self.toggler_label_tip(self.tip_brnch_labs_allowed && ts.has_tip_labs));
        if self.tip_brnch_labs_allowed && ts.has_tip_labs && self.draw_tip_labs {
            sc = sc.push(self.slider_size_label_tip());
        }

        sc = sc.push(self.toggler_label_branch(ts.has_brlen && self.tip_brnch_labs_allowed));
        if ts.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            sc = sc.push(self.slider_size_label_branch());
        }

        sc = sc.push(self.toggler_label_int(ts.has_int_labs));
        if ts.has_int_labs && self.draw_int_labs {
            sc = sc.push(self.slider_size_label_int());
        }

        sc = sc.push(self.toggler_legend(ts.has_brlen));
        sc = sc.push(self.toggler_ltt(true));
        sc = sc.push(self.toggler_cursor_line(true));

        sc = sc.padding(6e0);
        sc = sc.spacing(6e0);

        container(sc).width(260).style(sty_cont_sidebar).into()
    }

    pub(crate) fn toolbar(&self, ts: &TreeState) -> Element<TreeViewMsg> {
        let mut tbr: Row<TreeViewMsg> = Row::new();

        tbr = tbr.spacing(2e0);
        tbr = tbr.padding(6e0);

        tbr = tbr.push(self.btn_unroot(ts));
        tbr = tbr.push(self.btn_root(ts));

        let btn_sb_pos: Button<'_, TreeViewMsg> = match self.sidebar_position {
            SidebarLocation::Left => {
                self.btn("SBR", Some(TreeViewMsg::SetSidebarLocation(SidebarLocation::Right)))
            }
            SidebarLocation::Right => {
                self.btn("SBL", Some(TreeViewMsg::SetSidebarLocation(SidebarLocation::Left)))
            }
        };

        tbr = tbr.push(btn_sb_pos);

        container(tbr)
            .width(Fill)
            .height(Shrink)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Center)
            .style(sty_cont_toolbar)
            .into()
    }

    pub(crate) fn statusbar(&self, _ts: &TreeState) -> Element<TreeViewMsg> {
        container(iced::widget::vertical_space())
            .width(Fill)
            .height(30)
            .style(sty_cont_statusbar)
            .into()
    }

    pub(crate) fn stats(&self, ts: &TreeState) -> Row<TreeViewMsg> {
        let mut sr: Row<TreeViewMsg> = Row::new();

        let lc: Column<TreeViewMsg> = column![
            self.txt("Tips"),
            self.txt("Nodes"),
            self.txt("Height"),
            self.txt("Rooted"),
            self.txt("Branch Lengths"),
            self.txt("Ultrametric")
        ]
        .width(Fill);

        sr = sr.push(lc);

        let rc: Column<TreeViewMsg> = column![
            self.txt_usize(ts.tip_count),
            self.txt_usize(ts.node_count),
            match ts.has_brlen {
                true => self.txt_float(ts.tree_height),
                false => self.txt_usize(ts.tree_height as usize),
            },
            self.txt_bool(ts.is_rooted),
            self.txt_bool(ts.has_brlen),
            self.txt_bool_option(ts.is_ultrametric),
        ]
        .align_x(Horizontal::Right);
        sr = sr.push(rc);

        sr
    }
}
