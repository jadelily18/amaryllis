// taken from https://docs.rs/colorgrad/latest/colorgrad/#colored-noise
pub fn remap(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    (t - a) * ((d - c) / (b - a)) + c
}
