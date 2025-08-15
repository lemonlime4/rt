use glam::{Vec3, vec3};

/// Linear gamma color value
pub type Color = Vec3;

pub fn from_srgb(r: f32, g: f32, b: f32) -> Color {
    srgb_to_linear(vec3(r, g, b))
}

pub fn srgb_to_linear(x: Vec3) -> Color {
    x.map(srgb_to_linear_scalar)
}

pub fn linear_to_srgb(x: Color) -> Vec3 {
    x.map(linear_to_srgb_scalar)
}

pub fn srgb_to_linear_scalar(x: f32) -> f32 {
    // return x;
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

pub fn linear_to_srgb_scalar(x: f32) -> f32 {
    // return x;
    if x <= 0.0031308 {
        12.92 * x
    } else {
        x.powf(1.0 / 2.4) * 1.055 - 0.055
    }
}
