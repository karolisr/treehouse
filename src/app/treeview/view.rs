use super::{Ltt, TreeView, TreeViewMsg};
use crate::{
    Float,
    app::{
        BUTTON_W, LINE_H, LTT_H, PADDING, PADDING_INNER, SCROLL_BAR_W, SF, SIDE_COL_W, TEXT_SIZE,
    },
};
use iced::{
    Alignment, Background, Border, Color, Element, Font, Length, Pixels,
    alignment::{Horizontal, Vertical},
    border,
    widget::{
        Canvas, Column, PickList, Row, Rule, Scrollable, Slider, Space, Text, Theme as WidgetTheme,
        Toggler,
        button::{Button, Status as ButtonStatus, Style as ButtonStyle},
        column, container, horizontal_space,
        pick_list::{Handle as PickListHandle, Status as PickListStatus, Style as PickListStyle},
        row,
        rule::{FillMode as RuleFillMode, Style as RuleStyle},
        scrollable::{
            Direction as ScrollableDirection, Rail as ScrollBarRail, Scrollbar, Scroller,
            Status as ScrollBarStatus, Style as ScrollBarStyle,
        },
        slider::{
            Handle as SliderHandle, HandleShape as SliderHandleShape, Rail as SliderRail,
            Status as SliderStatus, Style as SliderStyle,
        },
        text, vertical_space,
    },
};
use numfmt::Formatter as NumFmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NodeOrderingOption {
    #[default]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TreeStyleOption {
    #[default]
    Phylogram,
    Fan,
}

impl std::fmt::Display for TreeStyleOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TreeStyleOption::Phylogram => "Phylogram",
            TreeStyleOption::Fan => "Fan",
        })
    }
}

const TREE_STYLE_OPTIONS: [TreeStyleOption; 2] = [TreeStyleOption::Phylogram, TreeStyleOption::Fan];

impl TreeView {
    pub fn view(&self) -> Element<TreeViewMsg> {
        if self.tip_count == 0 {
            return container(
                self.btn("Open a Tree File", Some(TreeViewMsg::OpenFile))
                    .width(SF * 2e2),
            )
            .center(Length::Fill)
            .into();
        }

        // Main Row:
        let mut mr: Row<TreeViewMsg> = Row::new();

        // Main Column:
        let mut mc: Column<TreeViewMsg> = Column::new();

        // Tree Canvas Row:
        let mut tcr: Row<TreeViewMsg> = Row::new();

        // Side Column:
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
            .align_x(Horizontal::Left)
            .width(Length::Fill),
            column![
                self.txt_usize(self.tip_count),
                self.txt_usize(self.node_count),
                match self.has_brlen {
                    true => self.txt_float(self.tree_height),
                    false => self.txt_usize(self.tree_height as usize),
                },
                self.txt_bool(self.is_rooted),
                self.txt_bool(self.has_brlen),
                self.txt_bool_option(self.is_ultrametric),
            ]
            .align_x(Horizontal::Right)
        ]);

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.rule_h(SF));
        sc = sc.push(self.space_h(0, PADDING));

        sc = sc.push(
            row![
                text!("Style").size(TEXT_SIZE).align_x(Alignment::Start),
                self.pick_list_tree_style(),
            ]
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .spacing(PADDING),
        );

        sc = sc.push(self.space_h(0, PADDING));

        sc = sc.push(
            row![
                text!("Node Order")
                    .size(TEXT_SIZE)
                    .align_x(Alignment::Start),
                self.pick_list_node_ordering(),
            ]
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .spacing(PADDING),
        );

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.rule_h(SF));
        sc = sc.push(self.space_h(0, PADDING));

        match self.sel_tree_style_opt {
            TreeStyleOption::Phylogram => {
                sc = sc.push(self.slider("Width", self.slider_width_canvas()));
                if self.min_node_size_idx != self.max_node_size_idx {
                    sc = sc.push(self.slider("Edge Spacing", self.slider_size_node()));
                }
            }
            TreeStyleOption::Fan => {
                sc = sc.push(self.slider("Zoom", self.slider_width_canvas()));
                sc = sc.push(self.slider("Opening Angle", self.slider_angle_opn()));
                sc = sc.push(self.slider("Rotation Angle", self.slider_angle_rot()));
            }
        }

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.rule_h(SF));
        sc = sc.push(self.space_h(0, PADDING));

        sc = sc.push(self.toggler_label_tip(self.tip_brnch_labs_allowed && self.has_tip_labs));
        if self.tip_brnch_labs_allowed && self.has_tip_labs && self.draw_tip_labs {
            sc = sc.push(self.slider_size_label_tip());
        }

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.toggler_label_branch(self.has_brlen && self.tip_brnch_labs_allowed));
        if self.has_brlen && self.tip_brnch_labs_allowed && self.draw_brnch_labs {
            sc = sc.push(self.slider_size_label_branch());
        }

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.toggler_label_int(self.has_int_labs));
        if self.has_int_labs && self.draw_int_labs {
            sc = sc.push(self.slider_size_label_int());
        }

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.toggler_legend(self.has_brlen));

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.toggler_ltt(true));

        sc = sc.push(self.space_h(0, PADDING));
        sc = sc.push(self.rule_h(SF));
        sc = sc.push(self.space_h(0, PADDING));

        sc = sc.push(
            container(
                row![self.btn_root(), self.btn_unroot()]
                    .align_y(Vertical::Center)
                    .spacing(PADDING),
            )
            .align_x(Horizontal::Center)
            .align_y(Vertical::Top)
            .width(Length::Fill)
            .height(Length::Shrink),
        );

        sc = sc.width(SIDE_COL_W);

        if (self.sel_tree_style_opt == TreeStyleOption::Phylogram
            && self.sel_node_size_idx != self.min_node_size_idx)
            || self.sel_tre_cnv_w_idx != self.min_tre_cnv_w_idx
        {
            tcr = tcr.push(self.scroll_canvas_tree(self.canvas_tree()));
        } else {
            tcr = tcr.push(self.canvas_tree());
        }

        // if (self.sel_tree_style_opt == TreeStyleOption::Phylogram
        //     && self.sel_node_size_idx == self.min_node_size_idx)
        //     || (self.sel_tree_style_opt == TreeStyleOption::Fan
        //         && self.sel_tre_cnv_w_idx == self.min_tre_cnv_w_idx)
        // {
        //     tcr = tcr.push(self.rule_v(SF));
        // } else {
        //     tcr = tcr.push(self.space_v(SF, 0));
        // }

        mc = mc.push(tcr);

        if self.show_ltt {
            if self.sel_tree_style_opt == TreeStyleOption::Phylogram
                && self.sel_tre_cnv_w_idx != self.min_tre_cnv_w_idx
            {
                mc = mc.push(self.scroll_canvas_ltt(self.canvas_ltt()));
            } else {
                mc = mc.push(self.canvas_ltt());
            }
        }

        mc = mc.spacing(PADDING);
        mr = mr.push(mc);
        mr = mr.push(self.space_v(PADDING, 0));
        mr = mr.push(sc);
        mr = mr.padding(PADDING);

        mr.into()
    }

    // --------------------------------------------------------------------------------------------

    fn btn<'a>(&'a self, lab: &'a str, msg: Option<TreeViewMsg>) -> Button<'a, TreeViewMsg> {
        let txt = Text::new(lab)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .size(TEXT_SIZE);
        let mut btn = Button::new(txt);
        btn = btn.on_press_maybe(msg);
        btn = self.apply_settings_btn(btn);
        btn
    }

    fn btn_root(&self) -> Button<TreeViewMsg> {
        self.btn("Root", {
            if self.sel_node_ids.len() == 1 {
                let node_id = *self.sel_node_ids.iter().last().unwrap();
                match self.tree.can_root(node_id) {
                    true => Some(TreeViewMsg::Root(node_id)),
                    false => None,
                }
            } else {
                None
            }
        })
    }

    fn btn_unroot(&self) -> Button<TreeViewMsg> {
        self.btn(
            "Unroot",
            match self.tree_orig.is_rooted() {
                true => Some(TreeViewMsg::Unroot),
                false => None,
            },
        )
    }

    // --------------------------------------------------------------------------------------------

    fn canvas_ltt(&self) -> Canvas<&Ltt, TreeViewMsg> {
        Canvas::new(&self.ltt)
            .width(Length::Fixed(self.ltt_cnv_w))
            .height(Length::Fixed(LTT_H))
    }

    fn canvas_tree(&self) -> Canvas<&TreeView, TreeViewMsg> {
        Canvas::new(self)
            .width(Length::Fixed(self.tre_cnv_w))
            .height(Length::Fixed(self.tre_cnv_h))
    }

    // --------------------------------------------------------------------------------------------

    fn pick_list_node_ordering(
        &self,
    ) -> PickList<NodeOrderingOption, &[NodeOrderingOption], NodeOrderingOption, TreeViewMsg> {
        let pl: PickList<
            NodeOrderingOption,
            &[NodeOrderingOption],
            NodeOrderingOption,
            TreeViewMsg,
        > = PickList::new(
            &NODE_ORDERING_OPTIONS,
            Some(self.sel_node_ord_opt),
            TreeViewMsg::NodeOrderingOptionChanged,
        );
        self.apply_settings_pick_list(pl)
    }

    fn pick_list_tree_style(
        &self,
    ) -> PickList<TreeStyleOption, &[TreeStyleOption], TreeStyleOption, TreeViewMsg> {
        let pl: PickList<TreeStyleOption, &[TreeStyleOption], TreeStyleOption, TreeViewMsg> =
            PickList::new(
                &TREE_STYLE_OPTIONS,
                Some(self.sel_tree_style_opt),
                TreeViewMsg::TreeReprOptionChanged,
            );
        self.apply_settings_pick_list(pl)
    }

    // --------------------------------------------------------------------------------------------

    fn rule_h(&self, height: impl Into<Pixels>) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::horizontal(height);
        self.apply_settings_rule(rule)
    }

    #[allow(dead_code)]
    fn rule_v(&self, width: impl Into<Pixels>) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::vertical(width);
        self.apply_settings_rule(rule)
    }

    // --------------------------------------------------------------------------------------------

    fn scroll_bar(&self) -> Scrollbar {
        let mut sb = Scrollbar::new();
        sb = sb.width(Pixels(SCROLL_BAR_W));
        sb = sb.scroller_width(Pixels(SCROLL_BAR_W));
        sb
    }

    fn scroll_canvas_ltt<'a>(
        &'a self,
        cnv: Canvas<&'a Ltt, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);
        scrl = scrl.direction(ScrollableDirection::Horizontal(self.scroll_bar()));
        scrl = scrl.width(self.tree_scroll_w);
        scrl = scrl.height(LTT_H);
        scrl = scrl.on_scroll(TreeViewMsg::LttCanvasScrolled);
        self.apply_settings_scroll(scrl)
    }

    fn scroll_canvas_tree<'a>(
        &'a self,
        cnv: Canvas<&'a TreeView, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);

        scrl = scrl.direction(ScrollableDirection::Both {
            horizontal: self.scroll_bar(),
            vertical: self.scroll_bar(),
        });

        scrl = scrl.width(self.tree_scroll_w);
        scrl = scrl.height(self.tree_scroll_h);
        scrl = scrl.on_scroll(TreeViewMsg::TreeCanvasScrolled);

        self.apply_settings_scroll(scrl)
    }

    // --------------------------------------------------------------------------------------------

    fn slider<'a>(
        &self,
        lab: &str,
        slider: Slider<'a, u16, TreeViewMsg>,
    ) -> Column<'a, TreeViewMsg> {
        column![
            container(text!("{lab}").size(TEXT_SIZE))
                .align_x(Horizontal::Right)
                .width(Length::Fill),
            slider
        ]
    }

    fn slider_angle_opn(&self) -> Slider<u16, TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_opn_angle_idx..=self.max_opn_angle_idx,
            self.sel_opn_angle_idx,
            TreeViewMsg::OpnAngleSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.apply_settings_slider(sldr)
    }

    fn slider_angle_rot(&self) -> Slider<u16, TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_rot_angle_idx..=self.max_rot_angle_idx,
            self.sel_rot_angle_idx,
            TreeViewMsg::RotAngleSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.apply_settings_slider(sldr)
    }

    fn slider_size_label_branch(&self) -> Slider<u16, TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_brnch_lab_size_idx,
            TreeViewMsg::BranchLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.apply_settings_slider(sldr)
    }

    fn slider_size_label_int(&self) -> Slider<u16, TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_int_lab_size_idx,
            TreeViewMsg::IntLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.apply_settings_slider(sldr)
    }

    fn slider_size_label_tip(&self) -> Slider<u16, TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_lab_size_idx..=self.max_lab_size_idx,
            self.sel_tip_lab_size_idx,
            TreeViewMsg::TipLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.apply_settings_slider(sldr)
    }

    fn slider_size_node(&self) -> Slider<u16, TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_node_size_idx..=self.max_node_size_idx,
            self.sel_node_size_idx,
            TreeViewMsg::NodeSizeSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.apply_settings_slider(sldr)
    }

    fn slider_width_canvas(&self) -> Slider<u16, TreeViewMsg> {
        let mut sldr: Slider<u16, TreeViewMsg> = Slider::new(
            self.min_tre_cnv_w_idx..=self.max_tre_cnv_w_idx,
            self.sel_tre_cnv_w_idx,
            TreeViewMsg::CanvasWidthSelectionChanged,
        );
        sldr = sldr.step(1_u16);
        sldr = sldr.shift_step(2_u16);
        self.apply_settings_slider(sldr)
    }

    // --------------------------------------------------------------------------------------------

    fn space_h(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        horizontal_space().width(width).height(height)
    }

    fn space_v(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        vertical_space().width(width).height(height)
    }

    // --------------------------------------------------------------------------------------------

    fn toggler<'a>(&self, label: &'a str, value: bool) -> Toggler<'a, TreeViewMsg> {
        let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
        tglr = tglr.label(label);
        tglr = self.apply_settings_toggler(tglr);
        tglr
    }

    fn toggler_label_branch(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Branch Lengths", self.has_brlen && self.draw_brnch_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::BranchLabelVisibilityChanged);
        }
        tglr
    }

    fn toggler_label_int(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Internal Labels", self.has_int_labs && self.draw_int_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::IntLabelVisibilityChanged);
        }
        tglr
    }

    fn toggler_label_tip(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Tip Labels", self.has_tip_labs && self.draw_tip_labs);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::TipLabelVisibilityChanged);
        }
        tglr
    }

    fn toggler_legend(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("Legend", self.has_brlen && self.draw_legend);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::LegendVisibilityChanged);
        }
        tglr
    }

    fn toggler_ltt(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tglr = self.toggler("LTT Plot", self.show_ltt);
        if enabled {
            tglr = tglr.on_toggle(TreeViewMsg::LttVisibilityChanged);
        }
        tglr
    }

    // --------------------------------------------------------------------------------------------

    fn txt(&self, s: impl Into<String>) -> Text {
        Text::new(s.into())
            .align_x(Horizontal::Right)
            .align_y(Vertical::Center)
            .width(Length::Shrink)
            .size(TEXT_SIZE)
    }

    fn txt_bool(&self, b: bool) -> Text {
        let s = match b {
            true => "Yes",
            false => "No",
        };
        self.txt(s)
    }

    fn txt_bool_option(&self, ob: Option<bool>) -> Text {
        match ob {
            Some(b) => self.txt_bool(b),
            None => self.txt("N/A"),
        }
    }

    fn txt_float(&self, n: impl Into<Float>) -> Text {
        let mut num_fmt = NumFmt::new();
        num_fmt = num_fmt.precision(numfmt::Precision::Decimals(3));
        num_fmt = num_fmt.separator(',').unwrap();
        let s = num_fmt.fmt2(n.into());
        self.txt(s)
    }

    fn txt_usize(&self, n: impl Into<usize>) -> Text {
        let mut num_fmt = NumFmt::new();
        num_fmt = num_fmt.precision(numfmt::Precision::Decimals(0));
        num_fmt = num_fmt.separator(',').unwrap();
        let s = num_fmt.fmt2(n.into());
        self.txt(s)
    }

    // --------------------------------------------------------------------------------------------

    fn apply_settings_btn<'a>(
        &'a self,
        mut btn: Button<'a, TreeViewMsg>,
    ) -> Button<'a, TreeViewMsg> {
        btn = btn.width(BUTTON_W);
        btn = btn.height(TEXT_SIZE * 1.5 + PADDING_INNER * 2e0);
        btn = btn.style(|theme, status| {
            let palette = theme.extended_palette();
            let base = ButtonStyle {
                background: Some(Background::Color(palette.primary.base.color)),
                text_color: palette.primary.base.text,
                border: Border {
                    radius: (SF * 3e0).into(),
                    width: 1e0 * SF,
                    ..Default::default()
                },
                ..ButtonStyle::default()
            };
            match status {
                ButtonStatus::Active | ButtonStatus::Pressed => base,
                ButtonStatus::Hovered => ButtonStyle {
                    background: Some(Background::Color(palette.primary.strong.color)),
                    ..base
                },
                ButtonStatus::Disabled => ButtonStyle {
                    background: base
                        .background
                        .map(|background| background.scale_alpha(0.5)),
                    text_color: base.text_color.scale_alpha(0.5),
                    ..base
                },
            }
        });
        btn
    }

    fn apply_settings_pick_list<
        'a,
        T: std::cmp::PartialEq + std::fmt::Display + std::clone::Clone,
    >(
        &'a self,
        mut pl: PickList<'a, T, &[T], T, TreeViewMsg>,
    ) -> PickList<'a, T, &'a [T], T, TreeViewMsg> {
        let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TEXT_SIZE)) };
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
                    border: Border { color: palette.primary.strong.color, ..active.border },
                    ..active
                },
            }
        });

        pl
    }

    fn apply_settings_rule<'a>(&self, rule: Rule<'a, WidgetTheme>) -> Rule<'a, WidgetTheme> {
        rule.style(|theme| {
            let palette = theme.extended_palette();
            RuleStyle {
                color: palette.background.strong.color,
                width: (SF * 1e0) as u16,
                radius: (SF * 2e0).into(),
                fill_mode: RuleFillMode::Percent(1e2),
            }
        })
    }

    fn apply_settings_scroll<'a>(
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

    fn apply_settings_slider<'a, T>(
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
                    shape: SliderHandleShape::Circle { radius: TEXT_SIZE / 1.75 },
                    background: color.into(),
                    border_color: Color::TRANSPARENT,
                    border_width: 0e0 * SF,
                },
            }
        })
    }

    fn apply_settings_toggler<'a>(
        &self,
        mut tglr: Toggler<'a, TreeViewMsg>,
    ) -> Toggler<'a, TreeViewMsg> {
        tglr = tglr.text_size(TEXT_SIZE);
        tglr = tglr.size(TEXT_SIZE * 1.5);
        tglr = tglr.spacing(PADDING_INNER);
        tglr = tglr.text_alignment(Alignment::End);
        tglr = tglr.width(Length::Fill);
        tglr = tglr.text_line_height(LINE_H);
        tglr
    }
}
