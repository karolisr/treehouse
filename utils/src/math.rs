use std::ops::{Add, Mul, Sub};

pub fn lerp<T>(a: impl Into<T>, b: impl Into<T>, t: impl Into<T>) -> T
where
    T: Sub + Copy + Add<<<T as Sub>::Output as Mul<T>>::Output, Output = T>,
    <T as Sub>::Output: Mul<T>,
{
    let a = a.into();
    let b = b.into();
    let t = t.into();
    a + (b - a) * t
}
