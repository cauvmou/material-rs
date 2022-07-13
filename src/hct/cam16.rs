use crate::utils::linearized;

use super::viewing_conditions::{ViewingConditions};


/**
 * CAM16, a color appearance model. Colors are not just defined by their hex
 * code, but rather, a hex code and viewing conditions.
 *
 * CAM16 instances also have coordinates in the CAM16-UCS space, called J*, a*,
 * b*, or jstar, astar, bstar in code. CAM16-UCS is included in the CAM16
 * specification, and should be used when measuring distances between colors.
 *
 * In traditional color spaces, a color can be identified solely by the
 * observer's measurement of the color. Color appearance models such as CAM16
 * also use information about the environment where the color was
 * observed, known as the viewing conditions.
 *
 * For example, white under the traditional assumption of a midday sun white
 * point is accurately measured as a slightly chromatic blue by CAM16. (roughly,
 * hue 203, chroma 3, lightness 100)
 */
pub struct Cam16 {
    pub hue: f64,
    pub chroma: f64,
    pub j: f64,    
    pub q: f64,    
    pub m: f64,    
    pub s: f64,    
    pub jstar: f64,
    pub astar: f64,
    pub bstar: f64,
}

impl Cam16 {
    pub fn from_int_in_viewing_conditions(argb: u32, viewing_conditions: ViewingConditions) -> Self {
        let red = (argb & 0x00ff0000) >> 16;
        let green = (argb & 0x0000ff00) >> 8;
        let blue = argb & 0x000000ff;
        let red_l = linearized(red);
        let green_l = linearized(green);
        let blue_l = linearized(blue);
        let x = 0.41233895 * red_l + 0.35762064 * green_l + 0.18051042 * blue_l;
        let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
        let z = 0.01932141 * red_l + 0.11916382 * green_l + 0.95034478 * blue_l;

        let r_c = 0.401288 * x + 0.650173 * y - 0.051461 * z;
        let g_c = -0.250268 * x + 1.204414 * y + 0.045854 * z;
        let b_c = -0.002079 * x + 0.048952 * y + 0.953127 * z;

        let r_d = viewing_conditions.rgb_d[0] * r_c;
        let g_d = viewing_conditions.rgb_d[1] * g_c;
        let b_d = viewing_conditions.rgb_d[2] * b_c;

        let r_af = ((viewing_conditions.fl * r_d.abs()) / 100.0).powf(0.42);
        let g_af = ((viewing_conditions.fl * g_d.abs()) / 100.0).powf(0.42);
        let b_af = ((viewing_conditions.fl * b_d.abs()) / 100.0).powf(0.42);
 
        let r_a = (r_d.signum() * 400.0 * r_af) / (r_af + 27.13);
        let g_a = (g_d.signum() * 400.0 * g_af) / (g_af + 27.13);
        let b_a = (b_d.signum() * 400.0 * b_af) / (b_af + 27.13);

        let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
        let b = (r_a + g_a - 2.0 * b_a) / 9.0;
        let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
        let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;
        let atan2 = f64::atan2(b, a);
        let atan_degrees = (atan2 * 180.0) / std::f64::consts::PI;
        let hue = if atan_degrees < 0.0 { atan_degrees + 360.0 }
            else if atan_degrees >= 360.0 { atan_degrees - 360.0 }
            else { atan_degrees };
        let hue_radians = (hue * std::f64::consts::PI) / 180.0;

        let ac = p2 * viewing_conditions.nbb;
        let j = 100.0 * (ac / viewing_conditions.aw).powf(viewing_conditions.c * viewing_conditions.z);
        let q = (4.0 / viewing_conditions.c) * (j / 100.0).sqrt() *
                    (viewing_conditions.aw + 4.0) * viewing_conditions.f_l_root;
        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = 0.25 * (((hue_prime * std::f64::consts::PI) / 180.0 + 2.0).cos() + 3.8);
        let p1 = (50000.0 / 13.0) * e_hue * viewing_conditions.nc * viewing_conditions.ncb;
        let t = (p1 * (a * a + b * b).sqrt()) / (u + 0.305);
        let alpha = t.powf(0.9) * (1.64 - 0.29f64.powf(viewing_conditions.n)).powf(0.73);
        let c = alpha * (j / 100.0).sqrt();
        let m = c * viewing_conditions.f_l_root;
        let s = 50.0 *((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        let jstar = ((1.0 + 100.0 * 0.007) * j) / (1.0 + 0.007 * j);
        let mstar = (1.0 / 0.0228) * (1.0 + 0.0228 * m).log(std::f64::consts::E);
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();

        Cam16 {
            hue,
            chroma: c,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }
}

impl From<u32> for Cam16 {
    fn from(argb: u32) -> Self {
        Self::from_int_in_viewing_conditions(argb, ViewingConditions::default())
    }
}