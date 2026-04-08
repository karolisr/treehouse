use crate::*;

#[allow(dead_code)]
pub(super) fn pick_list_plot_x_axis_scale_type<'a>(
    axis_scale_type: AxisScaleType,
) -> Row<'a, TvMsg> {
    let mut pl: PickList<
        AxisScaleType,
        &[AxisScaleType],
        AxisScaleType,
        TvMsg,
    > = PickList::new(
        &AXIS_SCALE_TYPE_OPTS,
        Some(axis_scale_type),
        TvMsg::PlotXAxisScaleTypeChanged,
    );
    pl = pick_list_common(pl);
    iced_row![txt("X-Axis Scale").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}

pub(super) fn pick_list_plot_y_axis_scale_type<'a>(
    axis_scale_type: AxisScaleType,
) -> Row<'a, TvMsg> {
    let mut pl: PickList<
        AxisScaleType,
        &[AxisScaleType],
        AxisScaleType,
        TvMsg,
    > = PickList::new(
        &AXIS_SCALE_TYPE_OPTS,
        Some(axis_scale_type),
        TvMsg::PlotYAxisScaleTypeChanged,
    );
    pl = pick_list_common(pl);
    iced_row![txt("Y-Axis Scale").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}

pub(super) fn pick_list_node_ordering<'a>(
    node_ord: TreNodeOrd,
) -> Row<'a, TvMsg> {
    let mut pl: PickList<TreNodeOrd, &[TreNodeOrd], TreNodeOrd, TvMsg> =
        PickList::new(
            &TRE_NODE_ORD_OPTS,
            Some(node_ord),
            TvMsg::TreNodeOrdOptChanged,
        );
    pl = pick_list_common(pl);
    iced_row![txt("Node Order").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}

pub(super) fn pick_list_tre_sty<'a>(tre_sty: TreSty) -> Row<'a, TvMsg> {
    let mut pl: PickList<TreSty, &[TreSty], TreSty, TvMsg> =
        PickList::new(&TRE_STY_OPTS, Some(tre_sty), TvMsg::TreStyOptChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Style").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}

pub(super) fn pick_list_tree_unit<'a>(tre_units: TreUnit) -> Row<'a, TvMsg> {
    let mut pl: PickList<TreUnit, &[TreUnit], TreUnit, TvMsg> =
        PickList::new(&TRE_UNIT_OPTS, Some(tre_units), TvMsg::TreUnitChanged);
    pl = pick_list_common(pl);
    iced_row![txt("Distance Unit").width(Length::FillPortion(9)), pl]
        .align_y(Vertical::Center)
}
