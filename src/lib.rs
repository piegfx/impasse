use std::{io, fs, path::Path};

use importers::Importer;

pub mod importers;

#[repr(C)]
pub struct Mesh {
    pub vertices: VertexPositionColorTextureNormalTangentBitangent,
    pub indices:  u32
}

pub struct Scene {
    pub meshes: Vec<Mesh>
}

impl Scene {
    pub fn from_gltf(path: &str) -> Result<Scene, io::Error> {
        let gltf = importers::gltf::Gltf::import(&fs::read(path)?)?;

        if gltf.buffers.is_none() || gltf.accessors.is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "glTF does not contain enough information to load anything useful."));
        }

        let gltf_buffers = gltf.buffers.as_ref().unwrap();
        let accessors = gltf.accessors.as_ref().unwrap();
        let buffer_views = gltf.buffer_views.unwrap();

        let mut buffers = Vec::with_capacity(gltf_buffers.len());

        let working_dir = Path::new(path).parent().unwrap();
        for buffer in gltf_buffers.iter() {
            let full_uri = working_dir.join(buffer.uri.as_ref().unwrap());
            buffers.push(fs::read(full_uri)?);
        }

        let mut meshes = Vec::new();

        for mesh in gltf.meshes.unwrap().iter() {
            for primitive in mesh.primitives.iter() {
                for (name, index) in &primitive.attributes {
                    let accessor = &accessors[*index as usize];
                    let view = &buffer_views[accessor.buffer_view.unwrap() as usize];
                    let data = &buffers[view.buffer as usize][view.byte_offset as usize..view.byte_offset as usize + view.byte_length as usize];
                    
                }
            }
        }

        Ok(Scene { meshes })
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Vec2(pub f32, pub f32);

#[derive(Debug)]
#[repr(C)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[derive(Debug)]
#[repr(C)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

#[derive(Debug)]
#[repr(C)]
pub struct Mat4(pub f32, pub f32, pub f32, pub f32,
                pub f32, pub f32, pub f32, pub f32,
                pub f32, pub f32, pub f32, pub f32,
                pub f32, pub f32, pub f32, pub f32
            );

#[derive(Debug)]
#[repr(C)]
pub struct VertexPositionColorTextureNormalTangentBitangent {
    pub position:  Vec3,
    pub color:     Option<Vec4>,
    pub tex_coord: Option<Vec2>,
    pub normal:    Option<Vec3>,
    pub tangent:   Option<Vec3>,
    pub bitangent: Option<Vec3>
}

fn reinterpret_slice<TFrom, TTo>(value: &[TFrom]) -> &[TTo] {
    unsafe { std::slice::from_raw_parts(value.as_ptr() as *const TTo, value.len() / std::mem::size_of::<TTo>()) }
}