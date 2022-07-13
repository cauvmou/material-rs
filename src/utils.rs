const e:     f64 = 0.008856451679035631;
const kappa: f64 = 903.2962962962963;

pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

pub fn lab_f(t: f64) -> f64 {
    if t > e {
        t.powf(1.0 / 3.0)
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

pub fn lerp(v1: f64, v2: f64, f: f64) -> f64 {
    v1 + (v2 - v1) * f
}