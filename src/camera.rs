use glam::{Vec2, Vec3, vec2, vec3};

use crate::math::Ray;

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    x: Vec3,
    y: Vec3,
}

impl Camera {
    const UP: Vec3 = vec3(0.0, 0.0, 1.0);

    pub fn new(pos: Vec3, target: Vec3, fov: f32, width: f32, height: f32) -> Self {
        let dir = (target - pos).normalize();
        let x = Self::UP.cross(dir).normalize();
        let y = dir.cross(x);
        let h = 2.0 * (fov / 2.0).tan();
        let w = (width / height) * h;
        Self {
            pos,
            dir,
            x: x * w,
            y: y * h,
        }
    }

    pub fn get_ray(&self, uv: Vec2) -> Ray {
        let uv = uv - 0.5;
        Ray::from(self.pos, self.dir + uv.x * self.x + uv.y * self.y)
    }
}
