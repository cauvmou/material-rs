use crate::utils::color::lstar_from_argb;

use self::{cam16::Cam16, solver::solve_to_int};

pub mod cam16;
pub mod solver;
pub mod vc;

struct HCT {
    hue: f64,
    chroma: f64,
    tone: f64,
    argb: u32,
}

impl HCT {
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        solve_to_int(hue, chroma, tone).into()
    }

    fn set(&mut self, argb: u32) {
        let target: HCT = argb.into();
        self.argb = argb;
        self.hue = target.hue;
        self.chroma = target.chroma;
        self.tone = target.tone;
    }

    pub fn hue(&self) -> f64 {
        self.hue
    }

    pub fn set_hue(&mut self, hue: f64) {
        self.set(solve_to_int(hue, self.chroma, self.tone))
    }

    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    pub fn set_chroma(&mut self, chroma: f64) {
        self.set(solve_to_int(self.hue, chroma, self.tone))
    }

    pub fn tone(&self) -> f64 {
        self.tone
    }

    pub fn set_tone(&mut self, tone: f64) {
        self.set(solve_to_int(self.hue, self.chroma, tone))
    }

    pub fn argb(&self) -> u32 {
        self.argb
    }

    pub fn set_argb(&mut self, argb: u32) {
        self.set(argb)
    }
}

impl From<u32> for HCT {
    fn from(argb: u32) -> Self {
        let Cam16 {
            hue,
            chroma,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        } = argb.into();
        Self {
            hue,
            chroma,
            tone: lstar_from_argb(argb),
            argb,
        }
    }
}
