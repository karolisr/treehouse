use crate::*;

pub(super) fn scrollable_cnv_plot<'a>(
    scrollable_id: &'static str,
    cnv: Cnv<&'a PlotCnv, TvMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Horizontal(scroll_bar()));
    s = s.id(scrollable_id);
    s = s.on_scroll(TvMsg::PlotCnvScrolledOrResized);
    scrollable_common(s, w, h)
}

pub(super) fn scrollable_cnv_tre<'a>(
    scrollable_id: &'static str,
    cnv: Cnv<&'a TreeCnv, TvMsg>,
    w: impl Into<Length>,
    h: impl Into<Length>,
) -> Scrollable<'a, TvMsg> {
    let mut s: Scrollable<TvMsg> = Scrollable::new(cnv);
    s = s.direction(ScrollableDirection::Both {
        horizontal: scroll_bar(),
        vertical: scroll_bar(),
    });
    s = s.id(scrollable_id);
    s = s.on_scroll(TvMsg::TreCnvScrolledOrResized);
    scrollable_common(s, w, h)
}
