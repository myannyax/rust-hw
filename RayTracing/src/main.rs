use crate::objects::{Checkerboard, Renderable, Sphere};
use crate::rays::Material;
use crate::render::{render, Params};
use crate::scene::{Light, Scene};
use crate::vec3::Vec3;

mod objects;
mod rays;
mod render;
mod scene;
mod vec3;

fn main() {
    let params = Params {
        field_of_view: std::f32::consts::PI / 3.0,
        width: 1024,
        height: 768,
        depth: 5,
    };

    let ivory = Material {
        refractive_index: 1.0,
        albedo: [0.6, 0.3, 0.1, 0.0],
        diffuse_color: Vec3 {
            x: 0.4,
            y: 0.4,
            z: 0.3,
        },
        specular_exponent: 50.0,
    };

    let red_rubber = Material {
        refractive_index: 1.0,
        albedo: [0.9, 0.1, 0.0, 0.0],
        diffuse_color: Vec3 {
            x: 0.3,
            y: 0.1,
            z: 0.1,
        },
        specular_exponent: 10.0,
    };

    let mirror = Material {
        refractive_index: 1.0,
        albedo: [0.0, 10.0, 0.8, 0.0],
        diffuse_color: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        specular_exponent: 1425.0,
    };

    let glass = Material {
        refractive_index: 1.5,
        albedo: [0.0, 0.5, 0.1, 0.8],
        diffuse_color: Vec3 {
            x: 0.6,
            y: 0.7,
            z: 0.8,
        },
        specular_exponent: 125.0,
    };

    let objects: Vec<Box<dyn Renderable>> = vec![
        Box::new(Sphere {
            center: Vec3 {
                x: -3.0,
                y: 0.0,
                z: -16.0,
            },
            radius: 2.0,
            material: ivory,
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: -1.0,
                y: -1.5,
                z: -12.0,
            },
            radius: 2.0,
            material: glass,
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: 1.5,
                y: -0.5,
                z: -18.0,
            },
            radius: 3.0,
            material: red_rubber,
        }),
        Box::new(Sphere {
            center: Vec3 {
                x: 7.0,
                y: 5.0,
                z: -18.0,
            },
            radius: 4.0,
            material: mirror,
        }),
        Box::new(Checkerboard),
    ];

    let lights = vec![
        Light {
            position: Vec3 {
                x: -20.0,
                y: 20.0,
                z: 20.0,
            },
            intensity: 1.5,
        },
        Light {
            position: Vec3 {
                x: 30.0,
                y: 50.0,
                z: -25.0,
            },
            intensity: 1.8,
        },
        Light {
            position: Vec3 {
                x: 30.0,
                y: 20.0,
                z: 30.0,
            },
            intensity: 1.7,
        },
    ];

    let scene = Scene { objects, lights };

    render(&scene, &params);
}
