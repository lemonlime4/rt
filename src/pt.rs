use std::f32::consts::PI;

use glam::{Vec3, vec2};
use random::Source;

use crate::{
    camera::Camera,
    img::Image,
    math::{Ray, random_dir, random_dir_in_hemisphere},
    scene::Scene,
};

pub fn ray_color<T: random::Source>(
    source: &mut T,
    ray: Ray,
    scene: &Scene,
    remaining_depth: u32,
) -> Vec3 {
    const INT_OFFSET: f32 = 0.001;
    let int = scene.trace(ray);
    if let Some(int) = int
        && int.t > 0.001
    {
        if remaining_depth > 0 {
            // let refl_dir = random_dir_in_hemisphere(source, int.normal);
            let refl_dir = (int.normal + random_dir(source)).normalize_or(int.normal);
            // let refl_dir = 2.0 * (-ray.dir).project_onto(int.normal) + ray.dir;
            let new_ray = Ray::from(int.pos + INT_OFFSET * int.normal, refl_dir);
            let new_ray_color = ray_color(source, new_ray, scene, remaining_depth - 1);
            // let brdf = int.normal.dot(-ray.dir) / PI;
            new_ray_color * int.material.diffuse_color // * brdf
        } else {
            Vec3::ZERO
        }
    } else {
        scene.env_light.color(ray.dir)
    }
}

pub fn render(width: u32, height: u32, camera: &Camera, scene: &Scene) -> Image {
    const SAMPLES: u32 = 100;
    const DEPTH: u32 = 8;
    let mut source = random::default(0);
    let mut img = Image::new(width, height);

    let dim = img.dimensions().as_vec2();
    for (pos, pixel) in img.iter_mut() {
        for _ in 0..SAMPLES {
            let offset = vec2(source.read::<f32>(), source.read::<f32>());
            let uv = (pos.as_vec2() + offset) / dim;
            let ray = camera.get_ray(uv);
            *pixel += ray_color(&mut source, ray, scene, DEPTH);
        }
        *pixel /= SAMPLES as f32;
    }
    img
}
