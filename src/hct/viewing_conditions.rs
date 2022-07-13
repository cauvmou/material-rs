use crate::utils::{self, lerp};


const PI_FRAC_200: f64 = 200.0 / std::f64::consts::PI;

/**
 * In traditional color spaces, a color can be identified solely by the
 * observer's measurement of the color. Color appearance models such as CAM16
 * also use information about the environment where the color was
 * observed, known as the viewing conditions.
 *
 * For example, white under the traditional assumption of a midday sun white
 * point is accurately measured as a slightly chromatic blue by CAM16. (roughly,
 * hue 203, chroma 3, lightness 100)
 *
 * This class caches intermediate values of the CAM16 conversion process that
 * depend only on viewing conditions, enabling speed ups.
 */
pub struct ViewingConditions {
    pub n: f64,
    pub aw: f64,
    pub nbb: f64,
    pub ncb: f64,
    pub c: f64,
    pub nc: f64,
    pub rgb_d: f64,
    pub fi: f64,
    pub f_l_root: f64,
    pub z: f64,
}

pub struct ViewingConditionsBuilder {
    white_point: [f64; 3],
    adapting_luminance: f64,
    background_lstar: f64,
    surround: f64,
    discounting_illumination: bool,
}

impl ViewingConditionsBuilder {
    pub fn new() -> Self {
        Self {
            white_point: [95.047, 100.0, 108.883],
            adapting_luminance: PI_FRAC_200 * utils::y_from_lstar(50.0) / 100.0,
            background_lstar: 50.0,
            surround: 2.0,
            discounting_illumination: false,
        }
    }

    pub fn build(self) -> ViewingConditions {
        let xyz = self.white_point;
        let rW = xyz[0] * 0.401288 + xyz[1] * 0.650173 + xyz[2] * -0.051461;
        let gW = xyz[0] * -0.250268 + xyz[1] * 1.204414 + xyz[2] * 0.045854;
        let bW = xyz[0] * -0.002079 + xyz[1] * 0.048952 + xyz[2] * 0.953127;
        let f = 0.8 + self.surround / 10.0;
        let c = {
            if f >= 0.9 {
                lerp(0.59, 0.69, (f - 0.9) * 10.0)
            } else {
                lerp(0.525, 0.59, (f - 0.8) * 10.0)
            }
        }
    }
}