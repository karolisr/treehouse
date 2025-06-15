use crate::iced::*;
use crate::*;
use lyon_path::builder;
use lyon_path::geom;

pub struct PathBuilder {
    raw: builder::WithSvg<lyon_path::path::BuilderImpl>,
}

impl PathBuilder {
    pub fn new() -> PathBuilder { PathBuilder { raw: lyon_path::Path::builder().with_svg() } }

    pub fn move_to(mut self, point: Point) -> Self {
        let _ = self.raw.move_to(geom::Point::new(point.x, point.y));
        self
    }

    pub fn line_to(mut self, point: Point) -> Self {
        let _ = self.raw.line_to(geom::Point::new(point.x, point.y));
        self
    }

    pub fn rectangle(self, rect: Rectangle) -> Self {
        let top_left = rect.position();
        self.move_to(top_left)
            .line_to(Point::new(top_left.x + rect.width, top_left.y))
            .line_to(Point::new(top_left.x + rect.width, top_left.y + rect.height))
            .line_to(Point::new(top_left.x, top_left.y + rect.height))
            .close()
    }

    pub fn circle(self, center: Point, radius: f32) -> Self {
        let start = Point { x: center.x + radius, y: center.y };
        self.move_to(start).arc_approx_line(ZERO, TAU, center, radius)
    }

    fn arc(&self, a0: f32, a1: f32, center: Point, radius: f32) -> geom::Arc<f32> {
        let center = geom::Point::new(center.x, center.y);
        let radii = geom::Vector::new(radius, radius);
        let x_rotation = geom::Angle::radians(ZERO);
        let start_angle = geom::Angle::radians(a0);
        let sweep_angle = geom::Angle::radians(a1 - a0);
        geom::Arc { center, radii, start_angle, sweep_angle, x_rotation }
    }

    pub fn arc_approx_line(mut self, a0: f32, a1: f32, center: Point, radius: f32) -> Self {
        let arc = self.arc(a0, a1, center, radius);
        arc.cast::<f64>().for_each_flattened(0.1, &mut |to| {
            let _ = self.raw.line_to(to.to_f32().to());
        });
        self
    }

    #[allow(dead_code)]
    pub fn arc_approx_quad_bezier(mut self, a0: f32, a1: f32, center: Point, radius: f32) -> Self {
        let arc = self.arc(a0, a1, center, radius);
        arc.cast::<f64>().for_each_quadratic_bezier(&mut |curve| {
            let curve = curve.cast::<f32>();
            let _ = self.raw.quadratic_bezier_to(curve.ctrl, curve.to);
        });
        self
    }

    #[allow(dead_code)]
    pub fn arc_approx_cubic_bezier(mut self, a0: f32, a1: f32, center: Point, radius: f32) -> Self {
        let arc = self.arc(a0, a1, center, radius);
        arc.cast::<f64>().for_each_cubic_bezier(&mut |curve| {
            let ctrl1 = curve.ctrl1.cast::<f32>();
            let ctrl2 = curve.ctrl2.cast::<f32>();
            let to = curve.to.cast::<f32>();
            let _ = self.raw.cubic_bezier_to(ctrl1, ctrl2, to);
        });
        self
    }

    pub fn close(mut self) -> Self {
        self.raw.close();
        self
    }

    pub fn build(self) -> IcedPath { self.raw.build().into() }
}

impl Default for PathBuilder {
    fn default() -> Self { Self::new() }
}

pub fn stroke_rect(rect: Rectangle, stroke: Strk, f: &mut Frame) {
    f.stroke(&PathBuilder::new().rectangle(rect).build(), stroke);
}

// pub fn fill_rect(rect: Rectangle, fill: CnvFill, f: &mut Frame) {
//     f.fill(&PathBuilder::new().rectangle(rect).build(), fill);
// }

pub fn stroke_circle(point: Point, stroke: Strk, radius: Float, f: &mut Frame) {
    f.stroke(&PathBuilder::new().circle(point, radius).build(), stroke);
}

pub fn fill_circle(point: Point, fill: CnvFill, radius: Float, f: &mut Frame) {
    f.fill(&PathBuilder::new().circle(point, radius).build(), fill);
}
