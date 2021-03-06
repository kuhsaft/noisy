/// Fast f64 to int floor function.
#[inline]
pub fn fast_floor(x: f64) -> i64 {
    if x > 0.0 {
        x as i64
    } else {
        (x as i64) - 1
    }
}
