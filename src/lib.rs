use std::{io, fs, path::Path};

use importers::Importer;

pub mod importers;

mod binary_reader;

#[repr(C)]
pub struct Mesh {
    pub vertices: Vec<VertexPositionColorTextureNormalTangentBitangent>,
    pub indices:  Vec<u32>
}

pub struct Scene {
    pub meshes: Vec<Mesh>
}

impl Scene {
    pub fn from_gltf(path: &str) -> Result<Scene, io::Error> {
        let gltf = importers::gltf::Gltf::import(path)?;

        if gltf.buffers.is_none() || gltf.accessors.is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "glTF does not contain enough information to load anything useful."));
        }

        let buffers = gltf.buffers.as_ref().unwrap();
        let accessors = gltf.accessors.as_ref().unwrap();
        let buffer_views = gltf.buffer_views.unwrap();

        let mut meshes = Vec::new();

        for mesh in gltf.meshes.unwrap().iter() {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();

            for primitive in mesh.primitives.iter() {
                for (name, index) in &primitive.attributes {
                    let accessor = &accessors[*index as usize];
                    let view = &buffer_views[accessor.buffer_view.unwrap() as usize];
                    let data = &buffers[view.buffer as usize].data[view.byte_offset as usize..view.byte_offset as usize + view.byte_length as usize];
                    
                    let name = name.to_lowercase();
                    let name = name.split('_').collect::<Vec<&str>>();
                    
                    match name[0].to_lowercase().as_str() {
                        "position" => {

                        }

                        _ => return Err(io::Error::new(io::ErrorKind::Unsupported, format!("Unsupported attribute \"{}\"", name[0])))
                    }
                }
            }

            meshes.push(Mesh {
                vertices,
                indices
            });
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