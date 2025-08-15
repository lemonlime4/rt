use glam::Vec3;

use crate::math::Ray;

pub trait Geometry {
    /// Returns negative if no intersection
    fn intersect(&self, ray: Ray) -> f32;

    /// Returns unit vector
    fn normal_at(&self, pos: Vec3) -> Vec3;
}

#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub center: Vec3,
    pub radius: f32,
}

impl Ball {
    pub fn new(center: Vec3, radius: f32) -> Self {
        assert!(radius > f32::EPSILON);
        Self { center, radius }
    }
}

impl Geometry for Ball {
    fn intersect(&self, ray: Ray) -> f32 {
        let oc = ray.orig - self.center;
        let b = oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let h = b * b - c;
        if h < 0.0 {
            return -1.0;
        }
        let h = h.sqrt();
        let t0 = -b - h;
        let t1 = -b + h;
        match t0 < 0.0 {
            false => t0,
            true => t1,
        }
    }

    fn normal_at(&self, pos: Vec3) -> Vec3 {
        (pos - self.center).normalize()
    }
}
