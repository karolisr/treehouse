// #[cfg(not(debug_assertions))]
use crate::{TreeView1, window_settings};
use iced::{
    Color, Padding, Point, Rectangle, Size,
    widget::canvas::{Frame, Path, Stroke},
};

type Float = f32;

#[derive(Debug)]
pub struct TreeView1State {
    // pub(super) tip_count: usize,
    // tree_height: Float,
    pub(super) scale_factor_x: Float,
    pub(super) scale_factor_y: Float,
    pub(super) bounds_global: Rectangle,
    pub(super) bounds_full: Rectangle,
    pub(super) bounds_tree: Rectangle,
    pub(super) bounds_tl_sep: Rectangle,
    pub(super) bounds_tip_labels: Rectangle,
    // tip_names: Vec<String>,
    pub(super) label_width: Float,
    pub(super) label_height: Float,
    pub(super) scale_factor_y_min: Float,
    label_offset: Float,
    // pub(super) tip_label_rects: Vec<Rectangle>,
    pub(super) dragging_tl_sep: bool,
    pub(super) drag_start_x: Float,
    pub(super) drag_start_y: Float,
    pub(super) label_width_prev: Float,
    pub(super) height: Float,
    pub(super) height_prev: Float,
    pub(super) height_win: Float,
}

// #[cfg(debug_assertions)]
// static mut COUNTER: usize = 0;

impl Default for TreeView1State {
    fn default() -> Self {
        Self {
            label_width: 1e1,
            label_width_prev: 1e1,
            scale_factor_y: 2e-3,
            scale_factor_y_min: 2e-3,
            label_height: 2e-3,
            label_offset: 5e0,
            height: 0e0,
            height_prev: 0e0,
            height_win: window_settings().size.height,
            // tip_count: Default::default(),
            // tree_height: Default::default(),
            scale_factor_x: Default::default(),
            bounds_global: Default::default(),
            bounds_full: Default::default(),
            bounds_tree: Default::default(),
            bounds_tl_sep: Default::default(),
            bounds_tip_labels: Default::default(),
            // tip_names: Default::default(),
            // tip_label_rects: Default::default(),
            dragging_tl_sep: Default::default(),
            drag_start_x: Default::default(),
            drag_start_y: Default::default(),
        }
    }
}

impl TreeView1State {
    pub(super) fn cache_tree_state(&mut self, tree_view: &TreeView1, bounds: &Rectangle) {
        let offset: Float = 1e1;
        self.bounds_global = Rectangle::new(
            Point { x: 0e0, y: 0e0 },
            Size {
                width: bounds.width,
                height: bounds.height,
            },
        );
        self.bounds_full = self.bounds_global.shrink(Padding::new(offset));

        self.bounds_tree = self
            .bounds_full
            .shrink(Padding::new(0e0).right(self.label_width + self.label_offset));

        self.bounds_tl_sep = self.bounds_full.shrink(
            Padding::new(0e0)
                .left(self.bounds_tree.width)
                .right(self.label_width),
        );

        self.bounds_tip_labels = self
            .bounds_full
            .shrink(Padding::new(0e0).left(self.bounds_tree.width + self.bounds_tl_sep.width));

        self.scale_factor_x = self.bounds_tree.width / tree_view.tree_height as Float;
        // self.scale_factor_y = self.bounds_tree.height / tree_view.tip_count as Float;
        self.scale_factor_y = self.scale_factor_y_min;
        self.height = offset * 2e0 + self.scale_factor_y * tree_view.tip_count as Float;
    }

    // fn child_heights(&self, node_id: usize, tree: &Tree) -> Vec<Float> {
    //     tree.tip_node_counts_for_children(node_id)
    //         .iter()
    //         .map(|&count| count as Float * self.scale_factor_y)
    //         .collect()
    // }

    // fn child_bounds(
    //     &self,
    //     node_id: usize,
    //     tree: &Tree,
    //     parent_bounds: &Rectangle,
    //     parent_branch_length: Float,
    //     child_index: usize,
    //     child_heights: &[Float],
    // ) -> Rectangle {
    //     Rectangle {
    //         x: parent_bounds.x + parent_branch_length * self.scale_factor_x,
    //         y: parent_bounds.y + child_heights[..child_index].iter().sum::<Float>(),
    //         width: tree.branch_length(node_id) as Float * self.scale_factor_x,
    //         height: child_heights[child_index],
    //     }
    // }

    // fn draw_node(
    //     &self,
    //     node_id: usize,
    //     tree: &Tree,
    //     frame: &mut Frame,
    //     bounds: &Rectangle,
    //     child_heights: &[Float],
    // ) {
    //     // #[cfg(debug_assertions)]
    //     // unsafe {
    //     //     COUNTER += 1
    //     // }
    //     // #[cfg(not(debug_assertions))]
    //     let color = SimpleColor::BLACK;
    //     // #[allow(static_mut_refs)]
    //     // #[cfg(debug_assertions)]
    //     // let color = unsafe {
    //     //     Color {
    //     //         r: COUNTER as Float / self.tip_count as Float,
    //     //         g: 5e-1,
    //     //         b: 1e-1,
    //     //         a: 1e0,
    //     //     }
    //     // };
    //     let child_count: usize = tree.child_node_count(node_id);
    //     let is_tip: bool = tree.is_tip(node_id);

    //     let mut y: Float = bounds.center_y();
    //     let h: Float = bounds.height;

    //     if !is_tip {
    //         y = bounds.center_y();
    //     }

    //     let mut coords_node: Point = Point {
    //         x: bounds.x + bounds.width,
    //         y,
    //     };

    //     if *bounds != self.bounds_tree {
    //         // Bounds =================================
    //         // self.draw_bounds(bounds, frame);
    //         // ========================================
    //         // Edges ==================================
    //         let coords_edge_start: Point = Point { x: bounds.x, y };
    //         let path = Path::new(|p| {
    //             p.move_to(coords_node);
    //             p.line_to(coords_edge_start);
    //         });
    //         frame.stroke(&path, Stroke::default().with_color(color));
    //         // ========================================
    //     } else {
    //         coords_node.x = bounds.x
    //     }

    //     // Verticals ==============================
    //     if !is_tip {
    //         let coords_top: Point = Point {
    //             x: coords_node.x,
    //             y: bounds.y + child_heights[0] / 2e0,
    //         };
    //         let coords_bottom: Point = Point {
    //             x: coords_node.x,
    //             y: bounds.y + h - child_heights[child_count - 1] / 2e0,
    //         };
    //         let path = Path::new(|p| {
    //             p.move_to(coords_top);
    //             p.line_to(coords_bottom);
    //         });
    //         frame.stroke(&path, Stroke::default().with_color(color));
    //     }
    //     // ========================================
    // }

    // pub(super) fn draw_tree(
    //     &self,
    //     node_id: usize,
    //     tree: &Tree,
    //     frame: &mut Frame,
    //     bounds: &Rectangle,
    // ) {
    //     let branch_length = tree.branch_length(node_id) as Float;
    //     let child_heights: Vec<Float> = self.child_heights(node_id, tree);
    //     let child_node_ids: &[usize] = tree.child_node_ids(node_id);
    //     let child_bounds_vec: Vec<Rectangle> = child_node_ids
    //         .iter()
    //         .enumerate()
    //         .map(|(i, &node_id)| {
    //             self.child_bounds(node_id, tree, bounds, branch_length, i, &child_heights)
    //         })
    //         .collect();

    //     self.draw_node(node_id, tree, frame, bounds, &child_heights);
    //     for (i, &node_id) in child_node_ids.iter().enumerate() {
    //         self.draw_tree(node_id, tree, frame, &child_bounds_vec[i]);
    //     }
    // }

    #[allow(dead_code)]
    fn draw_bounds(&self, bounds: &Rectangle, frame: &mut Frame) {
        let offset: Float = 1e0;
        let path = Path::new(|p| {
            p.rectangle(
                Point {
                    x: bounds.x + offset,
                    y: bounds.y + offset,
                },
                Size {
                    width: bounds.width - offset * 2e0,
                    height: bounds.height - offset * 2e0,
                },
            );
        });
        frame.stroke(
            &path,
            Stroke::default().with_color(Color {
                r: 0e0,
                g: 0e0,
                b: 1e0,
                a: 3e-1,
            }),
        );
    }

    #[allow(dead_code)]
    pub fn draw_bg(&self, bounds: &Rectangle, color: &Color, frame: &mut Frame) {
        let line_width: Float = 1e0;

        let top_left = Point {
            x: bounds.x + line_width,
            y: bounds.y + line_width,
        };

        let size = Size {
            width: bounds.size().width - line_width * 2e0,
            height: bounds.size().height - line_width * 2e0,
        };

        frame.fill_rectangle(top_left, size, *color);

        frame.stroke_rectangle(
            top_left,
            size,
            Stroke::default()
                .with_color((*color).scale_alpha(1e1))
                .with_width(line_width),
        );
    }

    // pub(super) fn prepare_tip_label_rects(&mut self) {
    //     self.tip_label_rects = Vec::new();
    //     for i in 0..self.tip_count {
    //         let label_bounds: Rectangle = Rectangle {
    //             x: self.bounds_tip_labels.x,
    //             y: self.bounds_tip_labels.y
    //                 + self.scale_factor_y / 4e0
    //                 + self.scale_factor_y * i as Float,
    //             width: self.label_width,
    //             height: self.scale_factor_y / 2e0,
    //         };
    //         self.tip_label_rects.push(label_bounds);
    //     }
    // }

    // fn draw_tip_label(&self, name: &str, bounds: &Rectangle, frame: &mut Frame, _cursor: &Cursor) {
    //     let mut lab = Text::from(name);
    //     lab.size = Pixels(self.label_height);
    //     // let mut color = SimpleColor::BLACK;
    //     // if cursor.is_over(*bounds) {
    //     //     color = SimpleColor::RED;
    //     // }
    //     lab.color = SimpleColor::BLACK;
    //     lab.align_y = Vertical::Center;
    //     lab.position = Point::new(bounds.x, bounds.y + bounds.height / 2e0);
    //     frame.fill_text(lab);
    // }

    // pub(super) fn draw_tip_labels(
    //     &self,
    //     frame: &mut Frame,
    //     cursor: &Cursor,
    //     tip_label_rects: Vec<Rectangle>,
    //     tip_names: Vec<String>,
    // ) {
    //     for (i, name) in tip_names.iter().enumerate() {
    //         // self.draw_tip_label(name, &self.tip_label_rects[i], frame, cursor);
    //         self.draw_tip_label(name, &tip_label_rects[i], frame, cursor);
    //     }
    // }

    // fn display(&self) -> String {
    //     let mut rv: String = String::new();
    //     rv.push_str(&format!(
    //         "{:4} {:5.2} | {:5.2} {:5.2}",
    //         self.tip_count, self.tree_height, self.scale_factor_x, self.scale_factor_y
    //     ));
    //     rv
    // }
}

// impl std::fmt::Display for TreeViewState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.display())
//     }
// }
