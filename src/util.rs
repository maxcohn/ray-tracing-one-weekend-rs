#[inline]
/// Clamp the given value between the upper and lower bounds
pub fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val > min {
        if val < max {
            val
        } else {
            max
        }
    } else {
        min
    }
}
