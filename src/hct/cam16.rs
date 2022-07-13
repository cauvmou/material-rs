
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
    pub fn from_int_in_viewing_conditions() {
        
    }
}

impl From<u32> for Cam16 {

}