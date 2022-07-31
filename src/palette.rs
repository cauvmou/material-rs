use std::collections::HashMap;

use crate::hct::HCT;

pub struct TonalPalette {
    cache: HashMap<u64, u32>,
    hue: f64,
    chroma: f64,
}

impl TonalPalette {
    fn new(hue: f64, chroma: f64) -> Self {
        Self { cache: HashMap::new(), hue, chroma }
    }

    pub fn tone(&mut self, tone: f64) -> u32 {
        match self.cache.get(&tone.to_bits()) {
            Some(argb) => *argb,
            None => {
                let argb = HCT::new(self.hue, self.chroma, tone).argb();
                self.cache.insert(tone.to_bits(), argb);
                argb
            },
        }
    }
}

impl From<u32> for TonalPalette {
    fn from(argb: u32) -> Self {
        let hct: HCT = argb.into();
        Self::new(hct.hue(), hct.chroma())
    }
}

pub struct CorePalette {
    pub a1: TonalPalette,
    pub a2: TonalPalette,
    pub a3: TonalPalette,
    pub n1: TonalPalette,
    pub n2: TonalPalette,
    pub error: TonalPalette,
}

impl CorePalette {

    pub fn of(argb: u32) -> Self {
        Self::new(argb, false)
    }

    pub fn content_of(argb: u32) -> Self {
        Self::new(argb, true)
    }

    fn new(argb: u32, content: bool) -> Self {
        let hct: HCT = argb.into();
        let hue = hct.hue();
        let chroma = hct.chroma();
        if content {
            Self {
                a1: TonalPalette::new(hue, chroma),
                a2: TonalPalette::new(hue, chroma / 3.0),
                a3: TonalPalette::new(hue + 60.0, chroma / 2.0),
                n1: TonalPalette::new(hue, (chroma / 12.0).min(4.0)),
                n2: TonalPalette::new(hue, (chroma / 6.0).min(8.0)),
                error: TonalPalette::new(25.0, 84.0),
            }
        } else {
            Self {
                a1: TonalPalette::new(hue, chroma.max(48.0)),
                a2: TonalPalette::new(hue, 16.0),
                a3: TonalPalette::new(hue + 60.0, 24.0),
                n1: TonalPalette::new(hue, 4.0),
                n2: TonalPalette::new(hue, 8.0),
                error: TonalPalette::new(25.0, 84.0),
            }
        }
    }
}