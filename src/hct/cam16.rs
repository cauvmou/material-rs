use crate::{utils::{linearized, argb_from_xyz}, hct::viewing_conditions};

use super::viewing_conditions::{ViewingConditions};

#[derive(Debug, Copy, Clone)]
pub struct JCh {
    j: f64,
    c: f64,
    h: f64,
}

impl JCh {
    pub fn new(j: f64, c: f64, h: f64) -> Self {
        Self { j, c, h }
    }
}

impl From<(f64, f64, f64)> for JCh {
    fn from(v: (f64, f64, f64)) -> Self {
        JCh::new(v.0, v.1, v.2)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UCS {
    jstar: f64,
    astar: f64,
    bstar: f64,
}

impl UCS {
    pub fn new(jstar: f64, astar: f64, bstar: f64) -> Self {
        Self { jstar, astar, bstar }
    }
}

impl From<(f64, f64, f64)> for UCS {
    fn from(v: (f64, f64, f64)) -> Self {
        UCS::new(v.0, v.1, v.2)
    }
}

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
#[derive(Debug, Copy, Clone)]
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
    /**
     * Google... WHAT?
     * Life is too short man...
    */
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
        let b_af = (viewing_conditions.fl) / (g_af + 27.13);

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

    pub fn from_jch_in_viewing_conditions(jch: JCh, viewing_conditions: ViewingConditions) -> Self {
        let JCh { j, c, h } = jch;
        let q = (4.0 / viewing_conditions.c) * (j / 100.0).sqrt() *
        (viewing_conditions.aw + 4.0) * viewing_conditions.f_l_root;
        let m = c * viewing_conditions.f_l_root;
        let alpha = c / (j / 100.0).sqrt();
        let s = 50.0 *
            ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();
        let hue_radians = (h * std::f64::consts::PI) / 180.0;
        let jstar = ((1.0 + 100.0 * 0.007) * j) / (1.0 + 0.007 * j);
        let mstar = (1.0 / 0.0228) * (1.0 + 0.0228 * m).log(std::f64::consts::E);
        let astar = mstar * (hue_radians).cos();
        let bstar = mstar * (hue_radians).sin();
        Cam16 {
            hue: h,
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

    pub fn from_ucs_in_viewing_conditions(ucs: UCS, viewing_conditions: ViewingConditions) -> Self {
        let UCS { jstar, astar, bstar } = ucs;
        let a = astar;
        let b = bstar;
        let m = (a * a + b * b).sqrt();
        let M = ((m * 0.0228).exp() - 1.0) / 0.0228;
        let c = M / viewing_conditions.f_l_root;
        let h = {
            let h = f64::atan2(b, a) * (180.0 / std::f64::consts::PI);
            if h < 0.0 {
                h + 360.0
            } else { h }
        };
        let j = jstar / (1.0 - (jstar - 100.0) * 0.007);
        Cam16::from_jch_in_viewing_conditions(JCh::new(j, c, h), viewing_conditions)
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let d_j = self.jstar - other.jstar;
        let d_a = self.astar - other.astar;
        let d_b = self.bstar - other.bstar;
        let d_eprime = (d_j * d_j + d_a * d_a + d_b * d_b).sqrt();
        1.41 * d_eprime.powf(0.63)
    }

    pub fn viewed(&self, viewing_conditions: ViewingConditions) -> u32 {
        let alpha = {
            if self.chroma == 0.0 || self.j == 0.0 {
                0.0
            } else {
                self.chroma / (self.j / 100.0).sqrt()
            }
        };
        let t = (alpha / (1.64 - 0.29f64.powf(viewing_conditions.n)).powf(0.73)).powf(1.0/0.9);
        let h_rad = (self.hue * std::f64::consts::PI) / 180.0;

        let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let ac = viewing_conditions.aw * (self.j / 100.0).powf(1.0 / viewing_conditions.c / viewing_conditions.z);
        let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.nc * viewing_conditions.ncb;
        let p2 = ac / viewing_conditions.nbb;

        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();

        let gamma = (23.0 * (p2 + 0.305) * t) / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

        let r_cbase = f64::max(0.0, (27.13 * r_a.abs()) / (400.0 - r_a.abs()));
        let r_c = r_a.signum() * (100.0 / viewing_conditions.fl) * r_cbase.powf( 1.0 / 0.42);

        let g_cbase = f64::max(0.0, (27.13 * g_a.abs()) / (400.0 - g_a.abs()));
        let g_c = g_a.signum() * (100.0 / viewing_conditions.fl) * g_cbase.powf( 1.0 / 0.42);

        let b_cbase = f64::max(0.0, (27.13 * b_a.abs()) / (400.0 - b_a.abs()));
        let b_c = b_a.signum() * (100.0 / viewing_conditions.fl) * b_cbase.powf( 1.0 / 0.42);

        let r_f = r_c / viewing_conditions.rgb_d[0];
        let g_f = g_c / viewing_conditions.rgb_d[1];
        let b_f = b_c / viewing_conditions.rgb_d[2];

        let x =  1.86206786 * r_f - 1.01125463 * g_f + 0.14918677 * b_f;
        let y =  0.38752654 * r_f + 0.62144744 * g_f - 0.00897398 * b_f;
        let z = -0.01584150 * r_f - 0.03412294 * g_f + 1.04996444 * b_f;

        argb_from_xyz(x, y, z)
    }
}

impl From<u32> for Cam16 {
    fn from(argb: u32) -> Self {
        Self::from_int_in_viewing_conditions(argb, ViewingConditions::default())
    }
}

impl From<JCh> for Cam16 {
    fn from(jch: JCh) -> Self {
        Self::from_jch_in_viewing_conditions(jch, ViewingConditions::default())
    }
}

impl From<UCS> for Cam16 {
    fn from(ucs: UCS) -> Self {
        Self::from_ucs_in_viewing_conditions(ucs, ViewingConditions::default())
    }
}

impl Into<u32> for Cam16 {
    fn into(self) -> u32 {
        self.viewed(ViewingConditions::default())
    }
}