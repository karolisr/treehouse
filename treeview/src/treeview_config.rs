use serde::{Deserialize, Serialize};

use super::{TreNodeOrd, TreSty, TreUnit};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct TreeViewConfig {
    pub align_tip_labs: bool,
    pub draw_cursor_line: bool,
    pub draw_gts: bool,
    pub draw_labs_brnch: bool,
    pub draw_labs_int: bool,
    pub draw_labs_tip: bool,
    pub draw_ltt: bool,
    pub draw_root: bool,
    pub full_width_scale_bar: bool,
    pub node_ord_opt: TreNodeOrd,
    pub selection_lock: bool,
    pub show_nodes_table: bool,
    pub show_plot: bool,
    pub show_scale_bar: bool,
    pub show_search_bar: bool,
    pub show_side_bar: bool,
    pub show_tool_bar: bool,
    pub tip_only_search: bool,
    pub tre_sty: TreSty,
    pub tre_unit: TreUnit,
    pub trim_tip_labs: bool,
    pub lab_size_idx_tip: u16,
    pub lab_size_idx_int: u16,
    pub lab_size_idx_brnch: u16,
    pub root_len_idx: u16,
    pub opn_angle_idx: u16,
    pub rot_angle_idx: u16,
}

impl Default for TreeViewConfig {
    fn default() -> Self {
        Self {
            align_tip_labs: false,
            draw_cursor_line: true,
            draw_gts: false,
            draw_labs_brnch: false,
            draw_labs_int: false,
            draw_labs_tip: true,
            draw_ltt: false,
            draw_root: true,
            full_width_scale_bar: false,
            node_ord_opt: TreNodeOrd::Ascending,
            selection_lock: false,
            show_nodes_table: false,
            show_plot: false,
            show_scale_bar: false,
            show_search_bar: false,
            show_side_bar: true,
            show_tool_bar: true,
            tip_only_search: false,
            tre_sty: TreSty::PhyGrm,
            tre_unit: TreUnit::MillionYears,
            trim_tip_labs: false,
            lab_size_idx_tip: 8,
            lab_size_idx_int: 8,
            lab_size_idx_brnch: 8,
            root_len_idx: 25,
            opn_angle_idx: 345,
            rot_angle_idx: 360,
        }
    }
}

impl TreeViewConfig {
    pub fn x_axis_is_reversed(&self) -> bool {
        match self.tre_unit {
            TreUnit::Unitless => false,
            TreUnit::Substitutions => false,
            TreUnit::MillionYears => true,
            TreUnit::CoalescentUnits => false,
        }
    }
}
