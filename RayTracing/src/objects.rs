use crate::rays::{Material, Ray, RayIntersection};
use crate::vec3::Vec3;

pub trait Renderable {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection>;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Renderable for Sphere {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction);
        let d2 = l.dot(l) - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        let d;
        if t0 >= 0.0 {
            d = t0
        } else if t1 >= 0.0 {
            d = t1
        } else {
            return None;
        }
        let hit = ray.origin + ray.direction * d;
        let normal = (hit - self.center).normalize();
        Some(RayIntersection {
            distance: t0,
            normal,
            material: &self.material,
            point: hit,
        })
    }
}

pub struct Checkerboard;

const CHECKERBOARD_MATERIAL_1: Material = Material {
    refractive_index: 1.0,
    albedo: [0.9, 0.1, 0.0, 0.0],
    diffuse_color: Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    specular_exponent: 0.0,
};

const CHECKERBOARD_MATERIAL_2: Material = Material {
    refractive_index: 1.0,
    albedo: [0.9, 0.1, 0.0, 0.0],
    diffuse_color: Vec3 {
        x: 1.0,
        y: 0.7,
        z: 0.3,
    },
    specular_exponent: 0.0,
};

impl Renderable for Checkerboard {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        if ray.direction.y.abs() < 1e-3 {
            return None;
        }

        let d = -(ray.origin.y + 4.0) / ray.direction.y;
        let pt = ray.origin + ray.direction * d;
        if d > 0.0 && pt.x.abs() < 10.0 && pt.z < -10.0 && pt.z > -30.0 {
            Some(RayIntersection {
                distance: d,
                point: pt,
                normal: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                material: if (((0.5 * pt.x) as i32) + 1000 + ((0.5 * pt.z) as i32)) % 2 == 1 {
                    &CHECKERBOARD_MATERIAL_1
                } else {
                    &CHECKERBOARD_MATERIAL_2
                },
            })
        } else {
            None
        }
    }
}
