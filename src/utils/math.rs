pub fn lerp(v1: f64, v2: f64, f: f64) -> f64 {
    v1 + (v2 - v1) * f
}