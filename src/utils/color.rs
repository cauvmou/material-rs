use super::math::matrix_multiply;

const e: f64 = 0.008856451679035631;
const kappa: f64 = 903.2962962962963;

const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.41233895, 0.35762064, 0.18051042],
    [0.2126, 0.7152, 0.0722],
    [0.01932141, 0.11916382, 0.95034478],
];

const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [
        3.2413774792388685,
        -1.5376652402851851,
        -0.49885366846268053,
    ],
    [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
    [
        0.05562093689691305,
        -0.20395524564742123,
        1.0571799111220335,
    ],
];

pub fn argb_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    255u32 << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

pub fn argb_from_linrgb(linrgb: [f64; 3]) -> u32 {
    argb_from_rgb(
        delinearized(linrgb[0]),
        delinearized(linrgb[1]),
        delinearized(linrgb[2]),
    )
}

pub fn argb_from_lstar(lstar: f64) -> u32 {
    let y = y_from_lstar(lstar);
    let component = delinearized(y);
    argb_from_rgb(component, component, component)
}

pub fn argb_from_xyz(x: f64, y: f64, z: f64) -> u32 {
    let matrix = XYZ_TO_SRGB;
    let linearR = matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z;
    let linearG = matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z;
    let linearB = matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z;
    let r = delinearized(linearR);
    let g = delinearized(linearG);
    let b = delinearized(linearB);
    argb_from_rgb(r, g, b)
}

pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

pub fn xyz_from_argb(argb: u32) -> [f64; 3] {
    let r = linearized(argb & 0x00f00000);
    let g = linearized(argb & 0x0000ff00);
    let b = linearized(argb & 0x000000ff);
    matrix_multiply([r, g, b], SRGB_TO_XYZ)
}

pub fn lstar_from_argb(argb: u32) -> f64 {
    let y = xyz_from_argb(argb)[1];
    116.0 * lab_f(y / 100.0) - 16.0
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

pub fn delinearized(rgbComponent: f64) -> u8 {
    let normalized = rgbComponent / 100.0;
    let delinearized = {
        if normalized <= 0.0031308 {
            normalized * 12.92
        } else {
            1.055 * normalized.powf(1.0 / 2.4) - 0.055
        }
    };
    (delinearized * 255.0).round() as u8
}
