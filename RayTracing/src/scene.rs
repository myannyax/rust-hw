use crate::objects::Renderable;
use crate::rays::{Ray, RayIntersection};
use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

pub struct Scene {
    pub objects: Vec<Box<dyn Renderable>>,
    pub lights: Vec<Light>,
}

fn reflect(i: &Vec3, n: Vec3) -> Vec3 {
    *i - n * 2.0 * (i.dot(n))
}

impl Renderable for Vec<Box<dyn Renderable>> {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        self.iter()
            .map(|obj| obj.ray_intersect(ray))
            .flatten()
            .min_by(|i, j| i.distance.partial_cmp(&j.distance).unwrap())
    }
}

fn refract(i: &Vec3, n: Vec3, eta_t: f32, eta_i: f32) -> Vec3 {
    let cos = -i.dot(n).clamp(-1.0, 1.0);
    if cos < 0.0 {
        refract(i, -n, eta_i, eta_t)
    } else {
        let eta = eta_i / eta_t;
        let k = 1.0 - eta * eta * (1.0 - cos * cos);
        if k < 0.0 {
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            *i * eta + n * (eta * cos - k.sqrt())
        }
    }
}

impl Scene {
    pub fn cast_ray(&self, ray: &Ray, depth: i32) -> Vec3 {
        let intersection = self.objects.ray_intersect(ray);
        if depth <= 0 || intersection.is_none() {
            return Vec3 {
                x: 0.2,
                y: 0.7,
                z: 0.8,
            }; // background color
        }
        let m = intersection.unwrap();

        let reflect_dir = reflect(&ray.direction, m.normal).normalize();
        let refract_dir =
            refract(&ray.direction, m.normal, m.material.refractive_index, 1.0).normalize();
        let reflect_orig = if reflect_dir.dot(m.normal) < 0.0 {
            m.point - m.normal * 1e-3
        } else {
            m.point + m.normal * 1e-3
        };
        let refract_orig = if refract_dir.dot(m.normal) < 0.0 {
            m.point - m.normal * 1e-3
        } else {
            m.point + m.normal * 1e-3
        };
        let reflect_color = self.cast_ray(
            &Ray {
                origin: reflect_orig,
                direction: reflect_dir,
            },
            depth - 1,
        );
        let refract_color = self.cast_ray(
            &Ray {
                origin: refract_orig,
                direction: refract_dir,
            },
            depth - 1,
        );

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        for i in 0..(self.lights.len()) {
            let light_dir = (self.lights[i].position - m.point).normalize();

            let shadow_orig = if light_dir.dot(m.normal) < 0.0 {
                m.point - m.normal * 1e-3
            } else {
                m.point + m.normal * 1e-3
            };

            let shadow = self.objects.ray_intersect(&Ray {
                origin: shadow_orig,
                direction: light_dir,
            });

            if shadow.is_some()
                && shadow.unwrap().distance < (self.lights[i].position - m.point).norm()
            {
                continue;
            }

            diffuse_light_intensity += self.lights[i].intensity * light_dir.dot(m.normal).max(0.0);
            specular_light_intensity += (reflect(&(light_dir * -1.0), m.normal) * -1.0)
                .dot(ray.direction)
                .max(0.0)
                .powf(m.material.specular_exponent)
                * self.lights[i].intensity;
        }
        m.material.diffuse_color * diffuse_light_intensity * m.material.albedo[0]
            + Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            } * specular_light_intensity
                * m.material.albedo[1]
            + reflect_color * m.material.albedo[2]
            + refract_color * m.material.albedo[3]
    }
}
