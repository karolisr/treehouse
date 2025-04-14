pub fn lerp<T>(a: impl Into<T>, b: impl Into<T>, t: impl Into<T>) -> T
where
    T: std::ops::Sub
        + Copy
        + std::ops::Add<<<T as std::ops::Sub>::Output as std::ops::Mul<T>>::Output, Output = T>,
    <T as std::ops::Sub>::Output: std::ops::Mul<T>,
{
    let a = a.into();
    let b = b.into();
    let t = t.into();
    a + (b - a) * t
}
