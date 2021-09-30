use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub refractive_index: f32,
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3,
    pub specular_exponent: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RayIntersection<'a> {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}
