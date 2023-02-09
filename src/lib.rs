use importers::Importer;

pub mod importers;

pub struct Scene {
}

impl Scene {
    pub fn from_gltf(data: &[u8]) {
        let gltf = importers::gltf::Gltf::import(data);
    }
}

#[derive(Debug)]
pub struct Vec2(f32, f32);

#[derive(Debug)]
pub struct Vec3(f32, f32, f32);

#[derive(Debug)]
pub struct Vec4(f32, f32, f32, f32);

#[derive(Debug)]
pub struct Mat4(f32, f32, f32, f32,
                f32, f32, f32, f32,
                f32, f32, f32, f32,
                f32, f32, f32, f32
            );