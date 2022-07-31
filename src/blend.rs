use crate::{hct::{HCT, cam16::{Cam16, UCS}}, utils::{math::{difference_degrees, sanitize_degrees_double, rotation_direction}, color::lstar_from_argb}};

pub fn harmonize(design_color: u32, source_color: u32) -> u32 {
    let from_hct: HCT = design_color.into();
    let to_hct: HCT = source_color.into();
    let difference_degrees = difference_degrees(from_hct.hue(), to_hct.hue());
    let rotation_degrees = (difference_degrees * 0.5).min(15.0);
    let output_hue = sanitize_degrees_double(
        from_hct.hue() + rotation_degrees * rotation_direction(from_hct.hue(), to_hct.hue())
    );
    HCT::new(output_hue, from_hct.chroma(), from_hct.tone()).argb()
}

pub fn hct_hue(from: u32, to: u32, amount: f64) -> u32 {
    let ucs = cam16_ucs(from, to, amount);
    let ucs_cam: Cam16 = ucs.into();
    let from_cam: Cam16 = from.into();
    HCT::new(ucs_cam.hue, from_cam.chroma, lstar_from_argb(from)).argb()
}

pub fn cam16_ucs(from: u32, to: u32, amount: f64) -> UCS {
    let from: Cam16 = from.into();
    let to: Cam16 = to.into();
    let jstar = from.jstar + (to.jstar - from.jstar) * amount;
    let astar = from.astar + (to.astar - from.astar) * amount;
    let bstar = from.bstar * (to.bstar - from.bstar) * amount;
    (jstar, astar, bstar).into()
}