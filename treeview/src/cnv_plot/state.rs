use crate::*;

#[derive(Debug, Default)]
pub struct St {
    pub(super) bnds: Rectangle<Float>,
    pub(super) cursor_tracking_point: Option<Point>,
    pub(super) translation: Vector,
    pub(super) plt_vs: RectVals<Float>,
    pub(super) plt_rect: Rectangle<Float>,
    pub(super) plt_padd_l: Float,
    pub(super) plt_padd_r: Float,
    pub(super) plt_padd_t: Float,
    pub(super) plt_padd_b: Float,
    // pub(super) text_w: Option<TextWidth<'static>>,
    pub(super) text_size: Float,
}

impl St {
    pub(super) fn cursor_tracking_point(
        &mut self,
        crsr: Cursor,
    ) -> Option<Action<TvMsg>> {
        if let Some(mouse) = crsr.position_in(self.bnds) {
            let adj = mouse - self.translation;
            let crsr_x_rel = adj.x / self.plt_vs.w;
            if (ZRO - EPSILON..=ONE + EPSILON).contains(&crsr_x_rel)
                && (ZRO - EPSILON..=self.plt_vs.h + EPSILON).contains(&adj.y)
            {
                self.cursor_tracking_point = Some(adj);
                Some(Action::publish(TvMsg::CursorOnLttCnv {
                    x: Some(crsr_x_rel),
                }))
            } else {
                self.cursor_tracking_point = None;
                Some(Action::publish(TvMsg::CursorOnLttCnv { x: None }))
            }
        } else {
            self.cursor_tracking_point = None;
            None
        }
    }
}
