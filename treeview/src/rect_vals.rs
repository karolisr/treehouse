// -------------------------------------
// #![allow(dead_code)]
// #![allow(unused_mut)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(clippy::single_match)]
// #![allow(clippy::collapsible_if)]
// #![allow(clippy::derivable_impls)]
// #![allow(clippy::type_complexity)]
// #![allow(clippy::collapsible_match)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::vec_init_then_push)]
// #![allow(clippy::needless_range_loop)]
// -------------------------------------

use std::fmt::{Debug, Display, Formatter, Result};

use num_traits::real::Real;

use riced::Rectangle;
use riced::Vector;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RectVals<T> {
    pub x0: T,
    pub y0: T,
    pub x1: T,
    pub y1: T,
    pub w: T,
    pub h: T,
    pub dim_min: T,
    pub dim_max: T,
    pub radius_min: T,
    pub radius_max: T,
    pub cntr_x: T,
    pub cntr_y: T,
    pub cntr: Vector<T>,
    pub trans: Vector<T>,
}

#[inline]
fn zero<T: Real>() -> T {
    T::from(0).expect(
        "Conversion: 'i32' -> 'num_traits::real::Real' should NOT fail.",
    )
}

#[inline]
fn two<T: Real>() -> T {
    T::from(2).expect(
        "Conversion: 'i32' -> 'num_traits::real::Real' should NOT fail.",
    )
}

impl<T: Real> RectVals<T> {
    pub fn cnv(bounds: Rectangle<T>) -> Self {
        let x = zero::<T>();
        let y = zero::<T>();
        let w = bounds.width;
        let h = bounds.height;
        Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn wh(w: T, h: T) -> Self {
        let x = zero::<T>();
        let y = zero::<T>();
        Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn corners(x0: T, y0: T, x1: T, y1: T) -> Self {
        let x = x0;
        let y = y0;
        let w = x1 - x0;
        let h = y1 - y0;
        Rectangle { x, y, width: w, height: h }.into()
    }

    pub fn type_converted<U: Real>(&self) -> RectVals<U> {
        let x = U::from(self.x0).unwrap();
        let y = U::from(self.y0).unwrap();
        let width = U::from(self.w).unwrap();
        let height = U::from(self.h).unwrap();
        Rectangle { x, y, width, height }.into()
    }

    pub fn scale(&self, scaling: T) -> RectVals<T> {
        let x = self.x0 * scaling;
        let y = self.y0 * scaling;
        let width = self.w * scaling;
        let height = self.h * scaling;
        Rectangle { x, y, width, height }.into()
    }

    pub fn padded(&self, left: T, right: T, top: T, bottom: T) -> RectVals<T> {
        let x = self.x0 + left;
        let y = self.y0 + top;
        let width = self.w - right - left;
        let height = self.h - bottom - top;
        Rectangle { x, y, width, height }.into()
    }

    pub fn transfer_x_from(&self, other: &RectVals<T>) -> RectVals<T> {
        let x = other.x0;
        let y = self.y0;
        let width = other.w;
        let height = self.h;
        Rectangle { x, y, width, height }.into()
    }

    pub fn transfer_y_from(&self, other: &RectVals<T>) -> RectVals<T> {
        let x = self.x0;
        let y = other.y0;
        let width = self.w;
        let height = other.h;
        Rectangle { x, y, width, height }.into()
    }
}

impl<T: Real> From<Rectangle<T>> for RectVals<T> {
    fn from(r: Rectangle<T>) -> Self {
        let x0 = r.x;
        let y0 = r.y;
        let w = r.width;
        let h = r.height;
        let x1 = x0 + w;
        let y1 = y0 + h;

        let dim_min = w.min(h);
        let dim_max = w.max(h);
        let radius_min = dim_min / two::<T>();
        let radius_max = dim_min.hypot(dim_max);

        let cntr_untrans_x = w / two::<T>();
        let cntr_untrans_y = h / two::<T>();

        let cntr_x = cntr_untrans_x + x0;
        let cntr_y = cntr_untrans_y + y0;
        let cntr = Vector { x: cntr_x, y: cntr_y };

        let trans = Vector { x: x0, y: y0 };

        RectVals {
            x0,
            y0,
            x1,
            y1,
            w,
            h,
            dim_min,
            dim_max,
            radius_min,
            radius_max,
            cntr_x,
            cntr_y,
            cntr,
            trans,
        }
    }
}

impl<T: Real> From<RectVals<T>> for Rectangle<T> {
    fn from(v: RectVals<T>) -> Self {
        Rectangle { x: v.x0, y: v.y0, width: v.w, height: v.h }
    }
}

impl<T: Real + Display> Display for RectVals<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({:7.2}, {:7.2}), ({:7.2}, {:7.2})",
            self.x0, self.y0, self.x1, self.y1
        )
    }
}
