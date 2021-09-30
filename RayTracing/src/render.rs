use std::io::{BufWriter, Write};

use crate::rays::Ray;
use crate::scene::Scene;
use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Params {
    pub field_of_view: f32,
    pub width: u32,
    pub height: u32,
    pub depth: i32,
}

pub(crate) fn render(scene: &Scene, params: &Params) {
    let width = params.width;
    let height = params.height;
    let mut frame_buffer = vec![Vec3::default(); (width * height) as usize];
    for j in 0..height {
        for i in 0..width {
            let x = (2.0 * (i as f32 + 0.5) / (width as f32) - 1.0)
                * (params.field_of_view / 2.0).tan()
                * (width as f32)
                / (height as f32);
            let y = -(2.0 * (j as f32 + 0.5) / (height as f32) - 1.0)
                * (params.field_of_view / 2.0).tan();
            let dir = Vec3 { x, y, z: -1.0 }.normalize();
            frame_buffer[(i + j * width) as usize] = scene.cast_ray(
                &Ray {
                    origin: Vec3::default(),
                    direction: dir,
                },
                params.depth,
            );
        }
    }

    let mut bytes = Vec::with_capacity((width * height * 3) as usize);
    for mut vector in frame_buffer {
        let max_coordinate = vector[0].max(vector[1]).max(vector[2]);
        if max_coordinate > 1.0 {
            vector = vector * (1.0 / max_coordinate);
        }
        for i in 0..3 {
            bytes.push((255.0 * vector[i].clamp(0.0, 1.0)) as u8);
        }
    }

    let file = std::fs::File::create("./out.ppm").unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(&mut writer, "P6\n{} {}\n255", width, height).unwrap();
    writer.write_all(&bytes).unwrap();
}
