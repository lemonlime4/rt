use std::f32::consts::TAU;

use glam::{Vec3, vec3};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn from(orig: Vec3, dir: Vec3) -> Self {
        Self {
            orig,
            dir: dir.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}

pub fn random_dir<S: random::Source>(source: &mut S) -> Vec3 {
    let (sin, cos) = f32::sin_cos(source.read::<f32>() * TAU);
    let u = source.read::<f32>();
    let r = (1.0 - u * u).sqrt();
    vec3(cos * r, sin * r, u * 2.0 - 1.0)
}

pub fn random_dir_in_hemisphere<S: random::Source>(source: &mut S, normal: Vec3) -> Vec3 {
    let dir = random_dir(source);
    dir * dir.dot(normal).signum()
}
