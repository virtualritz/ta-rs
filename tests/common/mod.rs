pub mod data;

#[inline]
pub fn clamp_to_two_digits(v: f64) -> f64 {
    (v * 100.0).floor() * 0.01
}
