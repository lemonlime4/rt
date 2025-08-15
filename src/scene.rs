use std::collections::BTreeMap;
use std::f32;

use glam::{Vec3, vec3};

use crate::{
    color::{Color, from_srgb},
    geometry::{Ball, Geometry},
    math::Ray,
};

type ObjectId = u32;
pub struct Scene {
    objects: Vec<SceneObject>,
    // lights: Vec<dyn Lgith
    // id_counter: ObjectId,
    pub env_light: Box<dyn EnvLight>,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub diffuse_color: Color,
    // pub diffuse: f32,
    // pub specular_color: Color,
    // pub metallic: f32,
}

pub struct SceneObject {
    // id: ObjectId,
    geometry: Box<dyn Geometry>,
    material: Material,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            // id_counter: 0,
            objects: Vec::new(),
            env_light: Box::new(SkyEnv),
        }
    }

    pub fn add_object(&mut self, geometry: Ball, material: Material) {
        self.objects.push(SceneObject {
            // id: self.id_counter,
            geometry: Box::new(geometry),
            material,
        });
        // self.id_counter += 1;
    }

    pub fn trace(&self, ray: Ray) -> Option<Intersection> {
        let mut min_t = f32::INFINITY;
        let mut min_obj = None;
        for obj in self.objects.iter() {
            let t = obj.geometry.intersect(ray);
            if 0.0 < t && t < min_t {
                min_t = t;
                min_obj = Some(obj);
            }
        }
        if let Some(obj) = min_obj {
            let pos = ray.at(min_t);
            Some(Intersection {
                t: min_t,
                pos,
                normal: obj.geometry.normal_at(pos),
                material: &obj.material,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait EnvLight {
    fn color(&self, dir: Vec3) -> Color;
}

struct SkyEnv;
impl EnvLight for SkyEnv {
    fn color(&self, dir: Vec3) -> Color {
        let top_color = from_srgb(0.5, 0.7, 1.0);
        let bottom_color = from_srgb(1.0, 1.0, 1.0);
        bottom_color.lerp(top_color, dir.z.clamp(0.0, 1.0).powf(0.5))
    }
}
