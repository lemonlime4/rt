#![allow(dead_code, unused_imports)]

mod camera;
mod color;
mod geometry;
mod img;
mod light;
mod math;
mod pt;
mod scene;
use glam::{Vec2, Vec3, vec3};

use crate::{
    camera::Camera,
    color::from_srgb,
    geometry::Ball,
    img::Image,
    scene::{Intersection, Material, Scene},
};

fn main() {
    let width = 500;
    let height = 300;
    let camera = Camera::new(
        // stop removing the newlines
        vec3(5.0, 3.0, 2.0),
        vec3(0.0, 0.0, 1.0),
        1.0,
        // vec3(1.2, 1.5, 0.3),
        // vec3(-10.0, -10.0, 5.0),
        // f32::to_radians(90.0),
        width as f32,
        height as f32,
    );

    let mut scene = Scene::new();
    scene.add_object(
        Ball::new(vec3(0.0, 0.0, 1.0), 1.0),
        Material {
            diffuse_color: from_srgb(1.0, 0.3, 0.1),
        },
    );
    scene.add_object(
        Ball::new(vec3(0.0, 0.0, -10000.0), 10000.0),
        Material {
            diffuse_color: from_srgb(0.5, 0.5, 0.5),
        },
    );

    pt::render(width, height, &camera, &scene).write("result.png");
}
