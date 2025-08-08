#![allow(dead_code, unused_imports)]

mod camera;
mod img;
mod math;

use glam::{Vec2, Vec3, vec3};

use crate::{
    camera::Camera,
    img::Image,
    math::{Ball, Geometry, Intersection},
};

fn main() {
    let mut image = Image::new(800, 500);
    let width = image.width() as f32;
    let height = image.height() as f32;
    let camera = Camera::new(
        vec3(5.0, 3.0, 2.0), // stop removing the newlines
        vec3(0.0, 0.0, 1.0),
        1.0,
        width,
        height,
    );

    let balls = [
        Ball {
            center: vec3(0.0, 0.0, 1.0),
            radius: 1.0,
        },
        Ball {
            center: vec3(0.0, 0.0, -10000.0),
            radius: 10000.0,
        },
    ];

    let dim = image.dimensions().as_vec2();
    for (pos, pixel) in image.iter_mut() {
        let uv: Vec2 = pos.as_vec2() / dim;
        let ray = camera.get_ray(uv);
        let ints = balls.map(|ball| ball.intersect(ray));
        let int: Option<Intersection> =
            ints.iter().fold(None, |int, new_int| match (int, new_int) {
                (None, _) => *new_int,
                (_, None) => int,
                (Some(int), Some(new_int)) => Some(if int.t < new_int.t { int } else { *new_int }),
            });
        let color = match int {
            Some(int) => int.normal * 0.5 + 0.5,
            None => Vec3::ZERO,
        };
        *pixel = color;
    }

    image.write("result.png")
}
