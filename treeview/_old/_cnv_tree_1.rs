impl Program<TreeViewMsg> for TreeCnv {
    fn draw(&self, st: &Self::State, rndr: &Renderer, thm: &Theme, bnds: Rectangle, crsr: Cursor) -> Vec<Geometry> {
        let palette = thm.palette();
        let palette_ex = thm.extended_palette();
        let color_text = palette.text;
        let color_bg_weakest = palette_ex.background.weakest.color;
        let color_bg_weak = palette_ex.background.weak.color;
        let color_bg_base = palette_ex.background.base.color;
        let color_bg_strong = palette_ex.background.strong.color;
        let color_bg_strongest = palette_ex.background.strongest.color;
        let color_primary_weak = palette_ex.primary.weak.color;
        let color_primary_base = palette_ex.primary.base.color;
        let color_primary_strong = palette_ex.primary.strong.color;
        let color_secondary_weak = palette_ex.secondary.weak.color;
        let color_secondary_base = palette_ex.secondary.base.color;
        let color_secondary_strong = palette_ex.secondary.strong.color;
        let color_success_base = palette_ex.success.base.color;
        let color_warning_base = palette_ex.warning.base.color;
        let color_danger_base = palette_ex.danger.base.color;

        #[cfg(debug_assertions)]
        {
            let g_palette = self.g_palette.draw(rndr, bnds.size(), |f| {
                let colors_bg = [color_bg_base, color_bg_weakest, color_bg_weak, color_bg_strong, color_bg_strongest];
                let colors_primary = [color_primary_base, color_primary_weak, color_primary_strong, color_text];
                let colors_secondary = [color_secondary_base, color_secondary_weak, color_secondary_strong];
                let colors_other = [color_success_base, color_warning_base, color_danger_base];
                let color_rect_size = SF * 15e0;
                let palette_rect_w = 2e0 * PADDING + color_rect_size * 5e0;
                let palette_rect_h = 2e0 * PADDING + color_rect_size * 4e0;
                let palette_rect_x = st.tree_rect.x + PADDING;
                let palette_rect_y = st.tree_rect.y + st.tree_rect.height - palette_rect_h - PADDING;

                f.fill_rectangle(
                    Point { x: palette_rect_x, y: palette_rect_y },
                    iced::Size { width: palette_rect_w, height: palette_rect_h },
                    color_bg_base,
                );

                f.stroke_rectangle(
                    Point { x: palette_rect_x + SF / 2e0, y: palette_rect_y + SF / 2e0 },
                    iced::Size {
                        width: 2e0 * PADDING + color_rect_size * 5e0 - SF,
                        height: 2e0 * PADDING + color_rect_size * 4e0 - SF,
                    },
                    stroke.with_color(color_text),
                );

                for (i, c) in colors_bg.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_primary.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * ONE,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_secondary.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * TWO,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }

                for (i, c) in colors_other.iter().enumerate() {
                    f.fill_rectangle(
                        Point {
                            x: palette_rect_x + PADDING + color_rect_size * i as Float,
                            y: palette_rect_y + PADDING + color_rect_size * 3e0,
                        },
                        iced::Size { width: color_rect_size, height: color_rect_size },
                        *c,
                    );
                }
            });
            geoms.push(g_palette);
        }
    }
}
