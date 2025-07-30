mod draw;
mod program;
mod state;

use crate::*;
use state::St;

#[derive(Debug)]
pub(super) struct TreeCnv {
    pub(super) tre_sty: TreSty,
    // -------------------------------------------------------------------------
    cache_bnds: CnvCache,
    cache_tip_lab_w_resize_area: CnvCache,
    cache_legend: CnvCache,
    cache_hovered_node: CnvCache,
    cache_cursor_line: CnvCache,
    cache_palette: CnvCache,
    // -------------------------------------------------------------------------
    pub(super) padd_l: Float,
    pub(super) padd_r: Float,
    pub(super) padd_t: Float,
    pub(super) padd_b: Float,
    // -------------------------------------------------------------------------
    pub(super) crsr_x_rel: Option<Float>,
    // -------------------------------------------------------------------------
    pub(super) vis_x0: Float,
    pub(super) vis_y0: Float,
    pub(super) vis_x1: Float,
    pub(super) vis_y1: Float,
    pub(super) vis_x_mid: Float,
    pub(super) vis_y_mid: Float,
    pub(super) vis_x_mid_rel: Float,
    pub(super) vis_y_mid_rel: Float,
    // -------------------------------------------------------------------------
    pub(super) draw_debug: bool,
    pub(super) draw_cursor_line: bool,
    pub(super) draw_labs_allowed: bool,
    pub(super) draw_labs_brnch: bool,
    pub(super) draw_labs_int: bool,
    pub(super) draw_labs_tip: bool,
    pub(super) draw_clade_labs: bool,
    pub(super) draw_legend: bool,
    pub(super) draw_root: bool,
    pub(super) drawing_enabled: bool,
    // -------------------------------------------------------------------------
    pub(super) tip_labs_vis_max: usize,
    // pub(super) node_labs_vis_max: usize,
    // -------------------------------------------------------------------------
    pub(super) lab_size_min: Float,
    pub(super) lab_size_max: Float,
    pub(super) lab_size_tip: Float,
    pub(super) lab_size_int: Float,
    pub(super) lab_size_brnch: Float,
    // -------------------------------------------------------------------------
    pub(super) lab_offset_tip: Float,
    pub(super) lab_offset_int: Float,
    pub(super) lab_offset_brnch: Float,
    // -------------------------------------------------------------------------
    pub(super) clade_labs_w: Float,
    // -------------------------------------------------------------------------
    pub(super) opn_angle: Float,
    pub(super) rot_angle: Float,
    // -------------------------------------------------------------------------
    pub(super) tre_vs: RectVals<Float>,
    pub(super) root_len_frac: Float,
    pub(super) stale_tre_rect: bool,
    // -------------------------------------------------------------------------
    pub(super) tree_state: Option<Rc<TreeState>>,
    // -------------------------------------------------------------------------
    pub(super) align_tip_labs: bool,
    pub(super) trim_tip_labs: bool,
    pub(super) trim_tip_labs_to_nchar: u16,
    // -------------------------------------------------------------------------
    pub(crate) tip_w_set_by_user: Option<Float>,
    pub(crate) selection_lock: bool,
}

impl TreeCnv {
    pub fn new() -> Self {
        Self {
            tre_sty: TreSty::PhyGrm,
            // -----------------------------------------------------------------
            padd_l: TREE_PADDING,
            padd_r: TREE_PADDING,
            padd_t: TREE_PADDING,
            padd_b: TREE_PADDING,
            // -----------------------------------------------------------------
            draw_debug: false,
            drawing_enabled: false,
            draw_root: true,
            draw_labs_allowed: false,
            draw_labs_tip: false,
            draw_clade_labs: true,
            draw_labs_int: false,
            draw_labs_brnch: false,
            draw_legend: true,
            draw_cursor_line: false,
            // -----------------------------------------------------------------
            tip_labs_vis_max: 1000,
            // node_labs_vis_max: 900,
            // -----------------------------------------------------------------
            opn_angle: ZRO,
            rot_angle: ZRO,
            // -----------------------------------------------------------------
            lab_size_min: SF,
            lab_size_max: SF,
            lab_size_tip: SF,
            lab_size_int: SF,
            lab_size_brnch: SF,
            // -----------------------------------------------------------------
            lab_offset_tip: ZRO,
            lab_offset_int: ZRO,
            lab_offset_brnch: ZRO,
            // -----------------------------------------------------------------
            clade_labs_w: ZRO,
            // -----------------------------------------------------------------
            cache_bnds: Default::default(),
            cache_tip_lab_w_resize_area: Default::default(),
            cache_legend: Default::default(),
            cache_hovered_node: Default::default(),
            cache_cursor_line: Default::default(),
            cache_palette: Default::default(),
            // -----------------------------------------------------------------
            crsr_x_rel: None,
            // -----------------------------------------------------------------
            vis_x0: ZRO,
            vis_y0: ZRO,
            vis_x1: ZRO,
            vis_y1: ZRO,
            vis_x_mid: ZRO,
            vis_y_mid: ZRO,
            vis_x_mid_rel: ZRO,
            vis_y_mid_rel: ZRO,
            // -----------------------------------------------------------------
            tre_vs: RectVals::default(),
            root_len_frac: ZRO,
            stale_tre_rect: false,
            // -----------------------------------------------------------------
            tree_state: None,
            // -----------------------------------------------------------------
            align_tip_labs: false,
            trim_tip_labs: false,
            trim_tip_labs_to_nchar: 20,
            // -----------------------------------------------------------------
            tip_w_set_by_user: None,
            selection_lock: false,
        }
    }

    pub(super) fn clear_cache_bnds(&self) {
        self.cache_bnds.clear();
    }

    pub(super) fn clear_cache_tip_lab_w_resize_area(&self) {
        self.cache_tip_lab_w_resize_area.clear();
    }

    pub(super) fn clear_cache_cache_palette(&self) {
        self.cache_palette.clear();
    }

    pub(super) fn clear_cache_cursor_line(&self) {
        self.cache_cursor_line.clear();
    }

    pub(super) fn clear_cache_hovered_node(&self) {
        self.cache_hovered_node.clear();
    }

    pub(super) fn clear_cache_legend(&self) {
        self.cache_legend.clear();
    }

    pub(super) fn clear_caches_all(&self) {
        self.clear_cache_bnds();
        self.clear_cache_tip_lab_w_resize_area();
        self.clear_cache_cache_palette();
        self.clear_cache_cursor_line();
        self.clear_cache_hovered_node();
        self.clear_cache_legend();
    }

    pub(super) fn calc_tre_vs(
        &self,
        cnv_vs: &RectVals<Float>,
        edges_tip_tallest: &[Edge],
        is_rooted: bool,
        has_clade_labels: bool,
        text_w_tip: &mut TextWidth<'static>,
    ) -> (RectVals<Float>, Float) {
        let tre_vs_prelim =
            cnv_vs.padded(self.padd_l, self.padd_r, self.padd_t, self.padd_b);
        let mut tip_w: Float = ZRO;
        let trim_to = match self.trim_tip_labs {
            true => Some(self.trim_tip_labs_to_nchar as usize),
            false => None,
        };
        if self.draw_labs_tip && self.draw_labs_allowed {
            tip_w = calc_tip_w(
                self.tre_sty, &tre_vs_prelim, self.tip_w_set_by_user,
                edges_tip_tallest, self.lab_offset_tip, trim_to, text_w_tip,
            );
        }
        let mut offset_due_to_clade_lab = ZRO;
        if has_clade_labels && self.draw_clade_labs {
            // offset_due_to_clade_lab = self.clade_labs_w + self.lab_offset_tip + SF;
            offset_due_to_clade_lab = self.clade_labs_w;
        }
        let mut root_len = ZRO;
        match self.tre_sty {
            TreSty::PhyGrm => {
                let mut offset_due_to_brnch_lab = ZRO;
                let mut offset_due_to_tip_lab = ZRO;
                if self.draw_labs_allowed && self.draw_labs_tip {
                    offset_due_to_tip_lab = self.lab_size_tip / TWO;
                }
                if self.draw_labs_allowed && self.draw_labs_brnch {
                    offset_due_to_brnch_lab =
                        self.lab_size_brnch + self.lab_offset_brnch.abs();
                }
                let right = tip_w + offset_due_to_clade_lab;
                let top = (offset_due_to_tip_lab).max(offset_due_to_brnch_lab);
                let bottom = offset_due_to_tip_lab;
                let mut tre_vs = tre_vs_prelim.padded(ZRO, right, top, bottom);
                if is_rooted {
                    root_len = tre_vs.w * self.root_len_frac;
                    tre_vs.w -= root_len;
                    tre_vs.x0 += root_len;
                    tre_vs.trans.x += root_len;
                    tre_vs.dim_min = tre_vs.w.min(tre_vs.h);
                    tre_vs.dim_max = tre_vs.w.max(tre_vs.h);
                    tre_vs.radius_min = tre_vs.dim_min / TWO;
                    tre_vs.radius_max = tre_vs.dim_min.hypot(tre_vs.dim_max);
                    let cntr_untrans_x = tre_vs.w / TWO;
                    let cntr_untrans_y = tre_vs.h / TWO;
                    tre_vs.cntr_x = cntr_untrans_x + tre_vs.x0;
                    tre_vs.cntr_y = cntr_untrans_y + tre_vs.y0;
                    tre_vs.cntr = Vector { x: tre_vs.cntr_x, y: tre_vs.cntr_y };
                }

                (tre_vs, root_len)
            }
            TreSty::Fan => {
                let p = tip_w + offset_due_to_clade_lab;
                let tre_vs = tre_vs_prelim.padded(p, p, p, p);
                if is_rooted {
                    root_len = tre_vs.radius_min * self.root_len_frac;
                }
                (tre_vs, root_len)
            }
        }
    }
}

fn calc_tip_w(
    tre_sty: TreSty,
    tre_vs: &RectVals<Float>,
    tip_w_set_by_user: Option<Float>,
    edges_to_consider: &[Edge],
    lab_offset_tip: Float,
    trim_to: Option<usize>,
    text_w_tip: &mut TextWidth,
) -> Float {
    let tre_vs_w = match tre_sty {
        TreSty::PhyGrm => tre_vs.w,
        TreSty::Fan => tre_vs.radius_min,
    };

    let tip_w: Float = if let Some(tip_w_set_by_user) = tip_w_set_by_user {
        tip_w_set_by_user
    } else {
        lab_offset_tip
            + calc_tip_lab_extra_w(
                tre_vs_w, edges_to_consider, trim_to, text_w_tip,
            )
    };

    tip_w.min(tre_vs_w / TWO).max(PADDING)
}

fn calc_tip_lab_extra_w(
    tre_vs_w: Float,
    edges_tip_tallest: &[Edge],
    trim_to: Option<usize>,
    text_w_tip: &mut TextWidth,
) -> Float {
    let mut max_w: Float = ZRO;
    let mut max_offset: Float = ZRO;
    for edge in edges_tip_tallest {
        if let Some(name) = &edge.name {
            let offset = edge.x1 as Float * tre_vs_w;
            if offset >= max_offset {
                max_offset = offset;
            };
            let mut name_trimmed = name.to_string();
            if let Some(nchar) = trim_to {
                name_trimmed = ellipsize_unicode(name_trimmed, nchar);
            }
            let tip_name_w = text_w_tip.width(&name_trimmed);
            let curr_max_w =
                tip_name_w + (max_offset + offset) / TWO - tre_vs_w;
            if curr_max_w >= max_w {
                max_w = curr_max_w;
            }
        }
    }
    max_w
}
