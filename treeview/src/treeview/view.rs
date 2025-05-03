use super::{
    SideBarPosition, TreeStyle,
    ui::style::{sty_cont_main, sty_cont_sidebar, sty_cont_statusbar, sty_cont_toolbar},
};
use crate::{PADDING, SIDEBAR_W, STATUSBAR_H, TreeView, TreeViewMsg};
use iced::{
    Element,
    Length::{Fill, Fixed, Shrink},
    alignment::{Horizontal, Vertical},
    widget::{Column, Row, column, container, responsive, row},
};

impl TreeView {
    pub fn view(&self) -> Element<TreeViewMsg> {
        let mut mc = Column::new();
        let mut mr = Row::new();

        if self.show_sidebar {
            match self.sidebar_position {
                SideBarPosition::Left => {
                    mr = mr.push(self.sidebar());
                    mr = mr.push(self.content());
                }
                SideBarPosition::Right => {
                    mr = mr.push(self.content());
                    mr = mr.push(self.sidebar());
                }
            }
        } else {
            mr = mr.push(self.content());
        }

        if self.show_toolbar {
            mc = mc.push(self.toolbar());
        }

        mc = mc.push(mr);

        if self.show_statusbar {
            mc = mc.push(self.statusbar());
        }

        mc.into()
    }
    // --------------------------------------------------------------------------------------------

    fn content(&self) -> Element<TreeViewMsg> {
        responsive(move |size| {
            container(self.pane_grid_main.view().map(TreeViewMsg::TreeWinPaneGridMsg))
                .width(Fixed(size.width))
                .height(Fixed(size.height))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(sty_cont_main)
                .into()
        })
        .into()
    }

    fn sidebar(&self) -> Element<TreeViewMsg> {
        let mut sc: Column<TreeViewMsg> = Column::new();

        sc = sc.push(row![
            column![
                self.txt("Tips"),
                self.txt("Nodes"),
                self.txt("Height"),
                self.txt("Rooted"),
                self.txt("Branch Lengths"),
                self.txt("Ultrametric")
            ]
            .width(Fill),
            column![
                self.txt_usize(self.tip_count),
                self.txt_usize(self.node_count),
                match self.has_brlen {
                    true => self.txt_float(self.tree_height as f32),
                    false => self.txt_usize(self.tree_height as usize),
                },
                self.txt_bool(self.is_rooted),
                self.txt_bool(self.has_brlen),
                self.txt_bool_option(self.is_ultrametric),
            ]
            .align_x(Horizontal::Right)
        ]);

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
        sc = sc.push(self.toggler_label_tip(self.tip_brnch_labs_allowed && self.has_tip_labs));
        if self.tip_brnch_labs_allowed && self.has_tip_labs && self.draw_tip_labs {
            sc = sc.push(self.slider_size_label_tip());
        }
        sc = sc.push(self.toggler_label_branch(self.has_brlen && self.tip_brnch_labs_allowed));
        if self.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            sc = sc.push(self.slider_size_label_branch());
        }
        sc = sc.push(self.toggler_label_int(self.has_int_labs));
        if self.has_int_labs && self.draw_int_labs {
            sc = sc.push(self.slider_size_label_int());
        }
        sc = sc.push(self.toggler_legend(self.has_brlen));
        sc = sc.push(self.toggler_ltt(true));
        sc = sc.push(self.toggler_cursor_line(true));
        container(sc).width(Fixed(SIDEBAR_W)).style(sty_cont_sidebar).into()
    }

    fn toolbar(&self) -> Element<TreeViewMsg> {
        let mut tr: Row<TreeViewMsg> = Row::new();

        tr = tr.spacing(PADDING / 4e0);
        tr = tr.padding(PADDING / 4e0);

        tr = tr.push(self.btn_unroot());
        tr = tr.push(self.btn_root());

        container(tr)
            .width(Fill)
            .height(Shrink)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Center)
            .style(sty_cont_toolbar)
            .into()
    }

    fn statusbar(&self) -> Element<TreeViewMsg> {
        container(iced::widget::vertical_space())
            .width(Fill)
            .height(Fixed(STATUSBAR_H))
            .style(sty_cont_statusbar)
            .into()
    }
}
