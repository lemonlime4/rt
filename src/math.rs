use glam::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self {
            orig,
            dir: dir.normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub pos: Vec3,
    pub t: f32,
    pub normal: Vec3,
}

// impl Intersection {
//     fn from_ray(ray: Ray, t: f32) -> Self {
//         Self { pos: ray.at(t), t }
//     }
// }

pub trait Geometry {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub center: Vec3,
    pub radius: f32,
}

impl Geometry for Ball {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let oc = ray.orig - self.center;
        let b = oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let h = b * b - c;
        if h < 0.0 {
            return None;
        }
        let h = h.sqrt();
        let t0 = -b - h;
        let t1 = -b + h;
        let t = match (t0 < 0.0, t1 < 0.0) {
            (false, _) => t0,
            (true, false) => t1,
            _ => None?,
        };
        let pos = ray.at(t);
        let normal = (pos - self.center).normalize();
        Some(Intersection { pos, normal, t })
    }
}
