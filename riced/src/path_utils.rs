use core::f32;

use crate::*;
use lyon_path::builder;
use lyon_path::geom;

#[allow(missing_debug_implementations)]
pub struct PathBuilder {
    raw: builder::WithSvg<lyon_path::path::BuilderImpl>,
}

fn point_from_angle_and_radius(
    angle: f32,
    radius: f32,
    radius_scaling_factor: f32,
) -> Point {
    let (sin, cos) = angle.sin_cos();
    let x = radius_scaling_factor * cos * radius;
    let y = radius_scaling_factor * sin * radius;
    Point { x, y }
}

impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder { raw: lyon_path::Path::builder().with_svg() }
    }

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
            .line_to(Point::new(
                top_left.x + rect.width,
                top_left.y + rect.height,
            ))
            .line_to(Point::new(top_left.x, top_left.y + rect.height))
            .close()
    }

    pub fn circle(self, center: Point, radius: f32) -> Self {
        let start = Point { x: center.x + radius, y: center.y };
        self.move_to(start).arc(ZERO, f32::consts::TAU, center, radius)
    }

    pub fn thick_arc(
        self,
        a0: f32,
        a1: f32,
        center: Point,
        inner_radius: f32,
        width: f32,
    ) -> Self {
        let outer_radius = inner_radius + width;
        let p0 = point_from_angle_and_radius(a0, inner_radius, ONE);
        let p1 = point_from_angle_and_radius(
            a1,
            inner_radius,
            outer_radius / inner_radius,
        );
        self.move_to(p0)
            .arc(a0, a1, center, inner_radius)
            .line_to(p1)
            .arc(a1, a0, center, outer_radius)
            .close()
    }

    pub fn arc(self, a0: f32, a1: f32, center: Point, radius: f32) -> Self {
        self.arc_approx_line(a0, a1, center, radius)
    }

    pub fn arc_approx_line(
        mut self,
        a0: f32,
        a1: f32,
        center: Point,
        radius: f32,
    ) -> Self {
        let arc = self.prepare_arc_segment(a0, a1, center, radius);
        arc.cast::<f64>().for_each_flattened(0.1, &mut |to| {
            let _ = self.raw.line_to(to.to_f32().to());
        });
        self
    }

    pub fn arc_approx_quad_bezier(
        mut self,
        a0: f32,
        a1: f32,
        center: Point,
        radius: f32,
    ) -> Self {
        let arc = self.prepare_arc_segment(a0, a1, center, radius);
        arc.cast::<f64>().for_each_quadratic_bezier(&mut |curve| {
            let curve = curve.cast::<f32>();
            let _ = self.raw.quadratic_bezier_to(curve.ctrl, curve.to);
        });
        self
    }

    pub fn arc_approx_cubic_bezier(
        mut self,
        a0: f32,
        a1: f32,
        center: Point,
        radius: f32,
    ) -> Self {
        let arc = self.prepare_arc_segment(a0, a1, center, radius);
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

    pub fn build(self) -> IcedPath {
        self.raw.build().into()
    }

    // ---------------------------------------------------------------------------------------------

    fn prepare_arc_segment(
        &self,
        a0: f32,
        a1: f32,
        center: Point,
        radius: f32,
    ) -> geom::Arc<f32> {
        let center = geom::Point::new(center.x, center.y);
        let radii = geom::Vector::new(radius, radius);
        let x_rotation = geom::Angle::radians(ZERO);
        let start_angle = geom::Angle::radians(a0);
        let sweep_angle = geom::Angle::radians(a1 - a0);
        geom::Arc { center, radii, start_angle, sweep_angle, x_rotation }
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn stroke_rect(rect: Rectangle, stroke: Strk, f: &mut Frame) {
    f.stroke(&PathBuilder::new().rectangle(rect).build(), stroke);
}

pub fn fill_rect(rect: Rectangle, fill: CnvFill, f: &mut Frame) {
    f.fill(&PathBuilder::new().rectangle(rect).build(), fill);
}

pub fn stroke_circle(point: Point, stroke: Strk, radius: f32, f: &mut Frame) {
    f.stroke(&PathBuilder::new().circle(point, radius).build(), stroke);
}

pub fn fill_circle(point: Point, fill: CnvFill, radius: f32, f: &mut Frame) {
    f.fill(&PathBuilder::new().circle(point, radius).build(), fill);
}
