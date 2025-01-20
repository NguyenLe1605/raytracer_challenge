pub fn float_cmp(a: f64, b: f64) -> bool {
    f64::abs(a - b) < f64::EPSILON
}
