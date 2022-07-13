const e:     f64 = 0.008856451679035631;
const kappa: f64 = 903.2962962962963;

pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

pub fn lab_f(t: f64) -> f64 {
    if t > e {
        t.cbrt()
    } else {
        (kappa * t + 16.0) / 116.0
    }
}

pub fn lab_invf(ft: f64) -> f64 {
    let ft3 = ft * ft * ft;
    if ft3 > e {
        ft3
    } else {
        (116.0 * ft - 16.0) / kappa
    }
}

pub fn linearized(rgbComponent: u32) -> f64 {
    let normalized = rgbComponent as f64 / 255.0;
    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

pub fn lerp(v1: f64, v2: f64, f: f64) -> f64 {
    v1 + (v2 - v1) * f
}