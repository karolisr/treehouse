use crate::{
    PlotCnv, TreeCnv, TreeState, TreeStateMsg, TreeView, TreeViewMsg,
    treeview::{NODE_ORD_OPTS, NodeOrd, TREE_STYLE_OPTS, TreeStyle},
};
use iced::{
    Length::{self, Fill, Fixed, Shrink},
    Pixels, Theme,
    alignment::{Horizontal, Vertical},
    widget::{
        Button, Canvas, PickList, Row, Rule, Scrollable, Slider, Space, Text, Toggler, container,
        horizontal_space, row, scrollable::Direction as ScrollableDirection, scrollable::Scrollbar,
        text, vertical_space,
    },
};
use numfmt::Formatter as NumFmt;

impl TreeView {
    pub(crate) fn btn<'a>(
        &'a self,
        lab: &'a str,
        msg: Option<TreeViewMsg>,
    ) -> Button<'a, TreeViewMsg> {
        let mut txt = Text::new(lab);
        txt = txt.align_x(Horizontal::Center);
        txt = txt.align_y(Vertical::Center);
        let mut btn = Button::new(txt);
        btn = btn.on_press_maybe(msg);
        btn = self.apply_settings_btn(btn);
        btn
    }

    pub(crate) fn btn_root(&self, ts: &TreeState) -> Button<TreeViewMsg> {
        self.btn("Root", {
            if ts.sel_node_ids.len() == 1 {
                let node_id = *ts.sel_node_ids.iter().last().unwrap();
                match ts.can_root(node_id) {
                    true => Some(TreeViewMsg::TreeStateMsg(TreeStateMsg::Root(node_id))),
                    false => None,
                }
            } else {
                None
            }
        })
    }

    pub(crate) fn btn_unroot(&self, ts: &TreeState) -> Button<TreeViewMsg> {
        self.btn(
            "Unroot",
            match ts.is_rooted {
                true => Some(TreeViewMsg::TreeStateMsg(TreeStateMsg::Unroot)),
                false => None,
            },
        )
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn canvas_ltt(&self) -> Canvas<&PlotCnv, TreeViewMsg> {
        Canvas::new(&self.ltt_cnv).width(Fixed(self.ltt_cnv_w)).height(Fixed(1e2))
    }

    pub(crate) fn canvas_tree(&self) -> Canvas<&TreeCnv, TreeViewMsg> {
        Canvas::new(&self.tre_cnv).width(Fixed(self.tre_cnv_w)).height(Fixed(self.tre_cnv_h))
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn pick_list_node_ordering(&self) -> Row<TreeViewMsg> {
        let mut pl: PickList<NodeOrd, &[NodeOrd], NodeOrd, TreeViewMsg> = PickList::new(
            &NODE_ORD_OPTS,
            Some(self.sel_node_ord_opt),
            TreeViewMsg::NodeOrdOptChanged,
        );
        pl = self.apply_settings_pick_list(pl);
        row![text!("Node Order").width(Fill), pl].align_y(Vertical::Center)
    }

    pub(crate) fn pick_list_tree_style(&self) -> Row<TreeViewMsg> {
        let mut pl: PickList<TreeStyle, &[TreeStyle], TreeStyle, TreeViewMsg> = PickList::new(
            &TREE_STYLE_OPTS,
            Some(self.sel_tree_style_opt),
            TreeViewMsg::TreeStyleOptionChanged,
        );
        pl = self.apply_settings_pick_list(pl);
        row![text!("Style").width(Fill), pl].align_y(Vertical::Center)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn rule_h(&self, height: impl Into<Pixels>) -> Rule<'_, Theme> {
        let mut rule: Rule<'_, Theme> = Rule::horizontal(height);
        rule = self.apply_settings_rule(rule);
        rule
    }

    pub(crate) fn rule_v(&self, width: impl Into<Pixels>) -> Rule<'_, Theme> {
        let mut rule: Rule<'_, Theme> = Rule::vertical(width);
        rule = self.apply_settings_rule(rule);
        rule
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn scroll_bar(&self) -> Scrollbar {
        Scrollbar::new()
    }

    pub(crate) fn scroll_canvas_ltt<'a>(
        &'a self,
        cnv: Canvas<&'a PlotCnv, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
        scrl = scrl.direction(ScrollableDirection::Horizontal(self.scroll_bar()));
        scrl = scrl.width(self.tree_scroll_w);
        scrl = scrl.height(1e2);
        scrl = scrl.on_scroll(TreeViewMsg::LttCnvScrolled);
        scrl = scrl.id("ltt");
        self.apply_settings_scroll(scrl)
    }

    pub(crate) fn scroll_canvas_tree<'a>(
        &'a self,
        cnv: Canvas<&'a TreeCnv, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);

        scrl = scrl.direction(ScrollableDirection::Both {
            horizontal: self.scroll_bar(),
            vertical: self.scroll_bar(),
        });

        scrl = scrl.width(self.tree_scroll_w);
        scrl = scrl.height(self.tree_scroll_h);
        scrl = scrl.on_scroll(TreeViewMsg::TreCnvScrolled);
        scrl = scrl.id("tre");

        self.apply_settings_scroll(scrl)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn slider<'a>(
        &'a self,
        lab: &str,
        slider: Slider<'a, u16, TreeViewMsg>,
    ) -> Row<'a, TreeViewMsg> {
        let slider = self.apply_settings_slider(slider);
        row![
            slider,
            container(text!("{lab}"))
                .align_x(Horizontal::Right)
                .align_y(Vertical::Center)
                .width(Fill),
        ]
        .align_y(Vertical::Center)
    }

    pub(crate) fn slider_angle_opn(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_opn_angle_idx..=self.max_opn_angle_idx,
            self.sel_opn_angle_idx,
            TreeViewMsg::OpnAngleSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider("Opening Angle", sldr)
    }

    pub(crate) fn slider_angle_rot(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_rot_angle_idx..=self.max_rot_angle_idx,
            self.sel_rot_angle_idx,
            TreeViewMsg::RotAngleSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider("Rotation Angle", sldr)
    }

    pub(crate) fn slider_size_label_branch(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_brnch_lab_size_idx,
            TreeViewMsg::BranchLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider("Branch Label Size", sldr)
    }

    pub(crate) fn slider_size_label_int(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_int_lab_size_idx,
            TreeViewMsg::IntLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider("Internal Label Size", sldr)
    }

    pub(crate) fn slider_size_label_tip(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_tip_lab_size_idx,
            TreeViewMsg::TipLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider("Tip Label Size", sldr)
    }

    pub(crate) fn slider_size_node(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_node_size_idx..=self.max_node_size_idx,
            self.sel_node_size_idx,
            TreeViewMsg::NodeSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider("Edge Spacing", sldr)
    }

    pub(crate) fn slider_width_canvas(&self) -> Row<TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_tre_cnv_w_idx..=self.max_tre_cnv_w_idx,
            self.sel_tre_cnv_w_idx,
            TreeViewMsg::CanvasWidthSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.slider("Zoom", sldr)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn space_h(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        horizontal_space().width(width).height(height)
    }

    pub(crate) fn space_v(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        vertical_space().width(width).height(height)
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn toggler<'a>(&self, label: &'a str, value: bool) -> Toggler<'a, TreeViewMsg> {
        let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
        tglr = tglr.label(label);
        tglr = self.apply_settings_toggler(tglr);
        tglr
    }

    pub(crate) fn toggler_cursor_line(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let lab = match self.sel_tree_style_opt {
            TreeStyle::Phylogram => "Cursor Tracking Line",
            TreeStyle::Fan => "Cursor Tracking Circle",
        };

        let mut tglr = self.toggler(lab, self.show_cursor_line);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::CursorLineVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_label_branch(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Branch Lengths", enabled && self.draw_brnch_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::BranchLabelVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_label_int(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Internal Labels", enabled && self.draw_int_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::IntLabelVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_label_tip(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Tip Labels", enabled && self.draw_tip_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::TipLabelVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_legend(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Legend", enabled && self.draw_legend);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::LegendVisibilityChanged);
        }
        tglr
    }

    pub(crate) fn toggler_ltt(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("LTT Plot", self.show_ltt);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::LttPlotVisibilityChanged);
        }
        tglr
    }

    // --------------------------------------------------------------------------------------------

    pub(crate) fn txt(&self, s: impl Into<String>) -> Text {
        Text::new(s.into()).align_x(Horizontal::Right).align_y(Vertical::Center).width(Shrink)
    }

    pub(crate) fn txt_bool(&self, b: bool) -> Text {
        let s = match b {
            true => "Yes",
            false => "No",
        };
        self.txt(s)
    }

    pub(crate) fn txt_bool_option(&self, ob: Option<bool>) -> Text {
        match ob {
            Some(b) => self.txt_bool(b),
            None => self.txt("N/A"),
        }
    }

    pub(crate) fn txt_float(&self, n: impl Into<f32>) -> Text {
        let mut num_fmt = NumFmt::new();
        num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
        num_fmt = num_fmt.separator(',').unwrap();
        let s = num_fmt.fmt2(n.into());
        self.txt(s)
    }

    pub(crate) fn txt_usize(&self, n: impl Into<usize>) -> Text {
        let mut num_fmt = NumFmt::new();
        num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
        num_fmt = num_fmt.separator(',').unwrap();
        let s = num_fmt.fmt2(n.into());
        self.txt(s)
    }
}
