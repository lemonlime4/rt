use glam::{UVec2, Vec3, uvec2, vec2};

use crate::color::{linear_to_srgb_scalar, srgb_to_linear_scalar};

#[derive(Clone)]
pub struct Image {
    pixels: Vec<Vec3>,
    width: u32,
    height: u32,
}

impl Image {
    /// Zero-initialized image with specified dimensions
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: vec![Vec3::ZERO; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> UVec2 {
        uvec2(self.width, self.height)
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Vec3> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.pixels.get((y * self.width + x) as usize).copied()
        }
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Vec3> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.pixels.get_mut((y * self.width + x) as usize)
        }
    }

    pub fn at(&self, x: u32, y: u32) -> Vec3 {
        self.get(x, y).unwrap()
    }

    pub fn at_mut(&mut self, x: u32, y: u32) -> &mut Vec3 {
        self.get_mut(x, y).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (UVec2, Vec3)> {
        self.pixels.iter().copied().enumerate().map(|(i, pixel)| {
            (
                UVec2 {
                    x: i as u32 % self.width,
                    y: self.height - i as u32 / self.width,
                },
                pixel,
            )
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (UVec2, &mut Vec3)> {
        self.pixels.iter_mut().enumerate().map(|(i, pixel)| {
            (
                UVec2 {
                    x: i as u32 % self.width,
                    y: self.height - i as u32 / self.width,
                },
                pixel,
            )
        })
    }

    pub fn write(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let p = self.at(x, y);
            fn convert(x: f32) -> u8 {
                (linear_to_srgb_scalar(x.clamp(0.0, 1.0)) * 256.0)
                    .floor()
                    .clamp(0.0, 255.0) as u8
            }
            *pixel = image::Rgb([convert(p.x), convert(p.y), convert(p.z)]);
        }
        imgbuf.save(filename).unwrap()
    }
}
