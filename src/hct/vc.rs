use crate::utils::{self, math::lerp, color::y_from_lstar};


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
    pub rgb_d: [f64; 3],
    pub fl: f64,
    pub f_l_root: f64,
    pub z: f64,
}

impl Default for ViewingConditions {
    fn default() -> Self {
        ViewingConditionsBuilder::new().build()
    }
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
            adapting_luminance: PI_FRAC_200 * y_from_lstar(50.0) / 100.0,
            background_lstar: 50.0,
            surround: 2.0,
            discounting_illumination: false,
        }
    }

    pub fn with_white_point(mut self, white_point: [f64; 3]) -> Self {
        self.white_point = white_point;
        self
    }

    pub fn with_adapting_luminance(mut self, adapting_luminance: f64) -> Self {
        self.adapting_luminance = adapting_luminance;
        self
    }

    pub fn with_background_lstar(mut self, background_lstar: f64) -> Self {
        self.background_lstar = background_lstar;
        self
    }

    pub fn with_surround(mut self, surround: f64) -> Self {
        self.surround = surround;
        self
    }

    pub fn with_discounting_illumination(mut self, discounting_illumination: bool) -> Self {
        self.discounting_illumination = discounting_illumination;
        self
    }

    pub fn build(self) -> ViewingConditions {
        let xyz = self.white_point;
        let rW = xyz[0] * 0.401288 + xyz[1] * 0.650173 + xyz[2] * -0.051461;
        let gW = xyz[0] * -0.250268 + xyz[1] * 1.204414 + xyz[2] * 0.045854;
        let bW = xyz[0] * -0.002079 + xyz[1] * 0.048952 + xyz[2] * 0.953127;
        let f = 0.8 + self.surround / 10.0;
        let c =
            if f >= 0.9 {
                lerp(0.59, 0.69, (f - 0.9) * 10.0)
            } else {
                lerp(0.525, 0.59, (f - 0.8) * 10.0)
            };
        let d = 
            if self.discounting_illumination {
                1.0
            } else {
                let d = f * (1.0 - (1.0 / 3.6) * ((-self.adapting_luminance - 42.0) / 92.0)).exp();
                if d > 1.0 {
                    1.0
                } else if d < 0.0 {
                    0.0
                } else {
                    d
                }
            };
        let nc = f;
        let rgb_d = [
            d * (100.0 / rW) + 1.0 - d,
            d * (100.0 / gW) + 1.0 - d,
            d * (100.0 / bW) + 1.0 - d,
        ];
        let k = 1.0 / (5.0 * self.adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4_f = 1.0 - k4;
        let fl = k4 * self.adapting_luminance + 0.1 * k4_f * k4_f * (0.5 * self.adapting_luminance).cbrt();
        let n = y_from_lstar(self.background_lstar) / self.white_point[1];
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 / n.powf(0.2);
        let ncb = nbb;
        let rgb_afactors = [
            ((fl * rgb_d[0] * rW) / 100.0).powf(0.42),
            ((fl * rgb_d[1] * gW) / 100.0).powf(0.42),
            ((fl * rgb_d[2] * bW) / 100.0).powf(0.42),
        ];
        let rgbA = [
            (400.0 * rgb_afactors[0]) / (rgb_afactors[0] + 27.13),
            (400.0 * rgb_afactors[1]) / (rgb_afactors[1] + 27.13),
            (400.0 * rgb_afactors[2]) / (rgb_afactors[2] + 27.13),
        ];
        let aw = (2.0 * rgbA[0] + rgbA[1] + 0.05 * rgbA[2]) * nbb;
        ViewingConditions {
            n,
            aw,
            nbb,
            ncb,
            c,
            nc,
            rgb_d,
            fl,
            f_l_root: fl.powf(0.25),
            z,
        }
    }
}