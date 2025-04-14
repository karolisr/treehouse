use super::{TreeView, TreeViewMsg};
use crate::app::{LINE_H, PADDING, PADDING_INNER, SCROLL_BAR_W, SF, SIDE_COL_W, TEXT_SIZE};
use iced::{
    Alignment, Border, Color, Element, Font, Length, Pixels,
    alignment::{Horizontal, Vertical},
    border,
    widget::{
        Button, Canvas, Column, PickList, Row, Rule, Scrollable, Slider, Space,
        Theme as WidgetTheme, Toggler, button, container, horizontal_space,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeOrderingOption {
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

impl TreeView {
    pub fn view(&self) -> Element<TreeViewMsg> {
        if self.tip_count == 0 {
            return container(Button::new("Open a Tree File").on_press(TreeViewMsg::OpenFile))
                .center(Length::Fill)
                .into();
        }
        let mut side_col: Column<TreeViewMsg> = Column::new();
        let mut main_row: Row<TreeViewMsg> = Row::new();

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.horizontal_rule(SF));
        side_col = side_col.push(self.horizontal_space(0, PADDING));

        if self.min_node_size_idx != self.max_node_size_idx {
            side_col = side_col.push(
                container(text!("Edge Spacing").size(TEXT_SIZE))
                    .align_x(Horizontal::Right)
                    .width(Length::Fill),
            );
            side_col = side_col.push(self.node_size_slider());
        }

        side_col = side_col.push(
            container(text!("Tree Width").size(TEXT_SIZE))
                .align_x(Horizontal::Right)
                .width(Length::Fill),
        );
        side_col = side_col.push(self.canvas_width_slider());

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.horizontal_rule(SF));
        side_col = side_col.push(self.horizontal_space(0, PADDING));

        side_col = side_col.push(self.tip_labels_toggler(self.draw_tip_branch_labels_allowed));
        if self.draw_tip_branch_labels_allowed && self.draw_tip_labels {
            side_col = side_col.push(self.tip_labels_size_slider());
        }

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(
            self.branch_labels_toggler(self.has_brlen && self.draw_tip_branch_labels_allowed),
        );
        if self.has_brlen && self.draw_tip_branch_labels_allowed && self.draw_branch_labels {
            side_col = side_col.push(self.branch_labels_size_slider());
        }

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.int_labels_toggler(true));
        if self.draw_int_labels {
            side_col = side_col.push(self.int_labels_size_slider());
        }

        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.horizontal_rule(SF));
        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(
            row![
                text!("Node Order")
                    .size(TEXT_SIZE)
                    .align_x(Alignment::Start),
                self.node_ordering_options_pick_list(),
            ]
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .spacing(PADDING),
        );
        side_col = side_col.push(self.horizontal_space(0, PADDING));
        side_col = side_col.push(self.horizontal_rule(SF));
        side_col = side_col.push(self.horizontal_space(0, PADDING));

        side_col = side_col.push(
            row![
                button("Unroot").on_press_maybe(match self.tree_original.is_rooted() {
                    true => Some(TreeViewMsg::Unroot),
                    false => None,
                }),
                button("Root").on_press_maybe({
                    if self.selected_node_ids.len() == 1 {
                        let node_id = *self.selected_node_ids.iter().last().unwrap();
                        match self.tree.can_root(node_id) {
                            true => Some(TreeViewMsg::Root(node_id)),
                            false => None,
                        }
                    } else {
                        None
                    }
                }),
            ]
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .spacing(PADDING),
        );

        side_col = side_col.width(Length::Fixed(SIDE_COL_W));

        if self.selected_node_size_idx != self.min_node_size_idx
            || self.selected_canvas_w_idx != self.min_canvas_w_idx
        {
            main_row = main_row.push(self.scrollable(self.tree_canvas()));
        } else {
            main_row = main_row.push(self.tree_canvas());
        }

        if self.selected_node_size_idx == self.min_node_size_idx {
            main_row = main_row.push(self.vertical_rule(SF));
        } else {
            main_row = main_row.push(self.vertical_space(SF, 0));
        }

        main_row = main_row.push(self.vertical_space(PADDING, 0));
        main_row = main_row.push(side_col);
        main_row = main_row.padding(PADDING);
        main_row.into()
    }

    fn tree_canvas(&self) -> Canvas<&TreeView, TreeViewMsg> {
        Canvas::new(self)
            .width(Length::Fixed(self.canvas_w))
            .height(Length::Fixed(self.canvas_h))
    }

    fn horizontal_space(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        horizontal_space().width(width).height(height)
    }

    fn vertical_space(&self, width: impl Into<Length>, height: impl Into<Length>) -> Space {
        vertical_space().width(width).height(height)
    }

    fn horizontal_rule(&self, height: impl Into<Pixels>) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::horizontal(height);
        self.apply_rule_settings(rule)
    }

    fn vertical_rule(&self, width: impl Into<Pixels>) -> Rule<'_, WidgetTheme> {
        let rule: Rule<'_, WidgetTheme> = Rule::vertical(width);
        self.apply_rule_settings(rule)
    }

    fn tip_labels_toggler(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tgl = self.apply_toggler_settings("Tip Labels", self.draw_tip_labels);
        if enabled {
            tgl = tgl.on_toggle(TreeViewMsg::TipLabelVisibilityChanged);
        }
        tgl
    }

    fn branch_labels_toggler(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tgl = self.apply_toggler_settings("Branch Lengths", self.draw_branch_labels);
        if enabled {
            tgl = tgl.on_toggle(TreeViewMsg::BranchLabelVisibilityChanged);
        }
        tgl
    }

    fn int_labels_toggler(&self, enabled: bool) -> Toggler<'_, TreeViewMsg> {
        let mut tgl = self.apply_toggler_settings("Internal Labels", self.draw_int_labels);
        if enabled {
            tgl = tgl.on_toggle(TreeViewMsg::IntLabelVisibilityChanged);
        }
        tgl
    }

    fn node_size_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_node_size_idx..=self.max_node_size_idx,
            self.selected_node_size_idx,
            TreeViewMsg::NodeSizeSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
    }

    fn canvas_width_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_canvas_w_idx..=self.max_canvas_w_idx,
            self.selected_canvas_w_idx,
            TreeViewMsg::CanvasWidthSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
    }

    fn tip_labels_size_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_label_size_idx..=self.max_label_size_idx,
            self.selected_tip_label_size_idx,
            TreeViewMsg::TipLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
    }

    fn branch_labels_size_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_label_size_idx..=self.max_label_size_idx,
            self.selected_branch_label_size_idx,
            TreeViewMsg::BranchLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
    }

    fn int_labels_size_slider(&self) -> Slider<u8, TreeViewMsg> {
        let mut sldr: Slider<u8, TreeViewMsg> = Slider::new(
            self.min_label_size_idx..=self.max_label_size_idx,
            self.selected_int_label_size_idx,
            TreeViewMsg::IntLabelSizeSelectionChanged,
        );
        sldr = sldr.step(1);
        sldr = sldr.shift_step(2);
        self.apply_slider_settings(sldr)
    }

    fn node_ordering_options_pick_list(
        &self,
    ) -> PickList<NodeOrderingOption, &[NodeOrderingOption], NodeOrderingOption, TreeViewMsg> {
        let h: PickListHandle<Font> = PickListHandle::Arrow { size: Some(Pixels(TEXT_SIZE)) };

        let mut pl: PickList<
            NodeOrderingOption,
            &[NodeOrderingOption],
            NodeOrderingOption,
            TreeViewMsg,
        > = PickList::new(
            &NODE_ORDERING_OPTIONS,
            self.selected_node_ordering_option,
            TreeViewMsg::NodeOrderingOptionChanged,
        );

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

    fn scrollable<'a>(
        &'a self,
        cnv: Canvas<&'a TreeView, TreeViewMsg>,
    ) -> Scrollable<'a, TreeViewMsg> {
        let mut scrl: Scrollable<TreeViewMsg> = Scrollable::new(cnv);

        let mut scrl_bar_h = Scrollbar::new();
        scrl_bar_h = scrl_bar_h.width(Pixels(SCROLL_BAR_W));
        scrl_bar_h = scrl_bar_h.scroller_width(Pixels(SCROLL_BAR_W));

        let mut scrl_bar_v = Scrollbar::new();
        scrl_bar_v = scrl_bar_v.width(Pixels(SCROLL_BAR_W));
        scrl_bar_v = scrl_bar_v.scroller_width(Pixels(SCROLL_BAR_W));

        scrl = scrl
            .direction(ScrollableDirection::Both { horizontal: scrl_bar_h, vertical: scrl_bar_v });
        scrl = scrl.width(self.scroll_w);
        scrl = scrl.height(self.available_vertical_space + SF);
        scrl = scrl.on_scroll(TreeViewMsg::TreeViewScrolled);
        self.apply_scroller_settings(scrl)
    }

    fn apply_scroller_settings<'a>(
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

    fn apply_toggler_settings<'a>(&self, label: &'a str, value: bool) -> Toggler<'a, TreeViewMsg> {
        let mut tglr: Toggler<TreeViewMsg> = Toggler::new(value);
        tglr = tglr.label(label);
        tglr = tglr.text_size(TEXT_SIZE);
        tglr = tglr.size(TEXT_SIZE * 1.5);
        tglr = tglr.spacing(PADDING_INNER);
        tglr = tglr.text_alignment(Alignment::End);
        tglr = tglr.width(Length::Fill);
        tglr = tglr.text_line_height(LINE_H);
        tglr
    }

    fn apply_rule_settings<'a>(&self, rule: Rule<'a, WidgetTheme>) -> Rule<'a, WidgetTheme> {
        rule.style(|theme| {
            let palette = theme.extended_palette();
            RuleStyle {
                color: palette.background.strong.color,
                width: (SF * 1e0) as u16,
                radius: (SF * 2e0).into(),
                fill_mode: RuleFillMode::Full,
            }
        })
    }

    fn apply_slider_settings<'a, T>(
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
}
