use serde::Serialize;

use crate::palette::CorePalette;

#[derive(Serialize)]
pub struct Scheme {
    primary: u32,
    #[serde(rename = "onPrimary")]
    on_primary: u32,
    #[serde(rename = "primaryContainer")]
    primary_container: u32,
    #[serde(rename = "onPrimaryContainer")]
    on_primary_container: u32,
    secondary: u32,
    #[serde(rename = "onSecondary")]
    on_secondary: u32,
    #[serde(rename = "secondaryContainer")]
    secondary_container: u32,
    #[serde(rename = "onSecondaryContainer")]
    on_secondary_container: u32,
    tertiary: u32,
    #[serde(rename = "onTertiary")]
    on_tertiary: u32,
    #[serde(rename = "tertiaryContainer")]
    tertiary_container: u32,
    #[serde(rename = "onTertiaryContainer")]
    on_tertiary_container: u32,
    error: u32,
    #[serde(rename = "onError")]
    on_error: u32,
    #[serde(rename = "errorContainer")]
    error_container: u32,
    #[serde(rename = "onErrorContainer")]
    on_error_container: u32,
    background: u32,
    #[serde(rename = "onBackground")]
    on_background: u32,
    surface: u32,
    #[serde(rename = "onSurface")]
    on_surface: u32,
    #[serde(rename = "surfaceVariant")]
    surface_variant: u32,
    #[serde(rename = "onSurfaceVariant")]
    on_surface_variant: u32,
    outline: u32,
    shadow: u32,
    #[serde(rename = "inverseSurface")]
    inverse_surface: u32,
    #[serde(rename = "inverseOnSurface")]
    inverse_on_surface: u32,
    #[serde(rename = "inversePrimary")]
    inverse_primary: u32
}

impl Scheme {
    pub fn light(argb: u32) -> Self {
        Self::light_from_core_palette(&mut CorePalette::of(argb))
    }

    pub fn light_content(argb: u32) -> Self {
        Self::light_from_core_palette(&mut CorePalette::content_of(argb))
    }

    pub fn light_from_core_palette(core: &mut CorePalette) -> Self {
        Self {
            primary:                core.a1.tone(40.0),
            on_primary:             core.a1.tone(100.0),
            primary_container:      core.a1.tone(90.0),
            on_primary_container:   core.a1.tone(10.0),
            secondary:              core.a2.tone(40.0),
            on_secondary:           core.a2.tone(100.0),
            secondary_container:    core.a2.tone(90.0),
            on_secondary_container: core.a2.tone(10.0),
            tertiary:               core.a3.tone(40.0),
            on_tertiary:            core.a3.tone(100.0),
            tertiary_container:     core.a3.tone(90.0),
            on_tertiary_container:  core.a3.tone(10.0),
            error:                  core.error.tone(40.0),
            on_error:               core.error.tone(100.0),
            error_container:        core.error.tone(90.0),
            on_error_container:     core.error.tone(10.0),
            background:             core.n1.tone(99.0),
            on_background:          core.n1.tone(10.0),
            surface:                core.n1.tone(99.0),
            on_surface:             core.n1.tone(10.0),
            surface_variant:        core.n2.tone(90.0),
            on_surface_variant:     core.n2.tone(30.0),
            outline:                core.n2.tone(50.0),
            shadow:                 core.n1.tone(0.0),
            inverse_surface:        core.n1.tone(20.0),
            inverse_on_surface:     core.n1.tone(95.0),
            inverse_primary:        core.a1.tone(80.0)
        }
    }

    pub fn dark(argb: u32) -> Self {
        Self::dark_from_core_palette(&mut CorePalette::of(argb))
    }

    pub fn dark_content(argb: u32) -> Self {
        Self::dark_from_core_palette(&mut CorePalette::content_of(argb))
    }

    pub fn dark_from_core_palette(core: &mut CorePalette) -> Self {
        Self {
            primary:                core.a1.tone(80.0),
            on_primary:             core.a1.tone(20.0),
            primary_container:      core.a1.tone(30.0),
            on_primary_container:   core.a1.tone(90.0),
            secondary:              core.a2.tone(80.0),
            on_secondary:           core.a2.tone(20.0),
            secondary_container:    core.a2.tone(30.0),
            on_secondary_container: core.a2.tone(90.0),
            tertiary:               core.a3.tone(80.0),
            on_tertiary:            core.a3.tone(20.0),
            tertiary_container:     core.a3.tone(30.0),
            on_tertiary_container:  core.a3.tone(90.0),
            error:                  core.error.tone(80.0),
            on_error:               core.error.tone(20.0),
            error_container:        core.error.tone(30.0),
            on_error_container:     core.error.tone(80.0),
            background:             core.n1.tone(10.0),
            on_background:          core.n1.tone(90.0),
            surface:                core.n1.tone(10.0),
            on_surface:             core.n1.tone(90.0),
            surface_variant:        core.n2.tone(30.0),
            on_surface_variant:     core.n2.tone(80.0),
            outline:                core.n2.tone(60.0),
            shadow:                 core.n1.tone(0.0),
            inverse_surface:        core.n1.tone(90.0),
            inverse_on_surface:     core.n1.tone(20.0),
            inverse_primary:        core.a1.tone(40.0)
        }
    }

    pub fn json(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(self)
    }
}