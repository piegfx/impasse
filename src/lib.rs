use std::{io, fs, path::Path};

use importers::Importer;

use crate::binary_reader::BinaryReader;

pub mod importers;
mod binary_reader;
mod impassec;
//mod utils;

#[repr(C)]
#[derive(Debug)]
pub enum TextureType {
    Albedo,
    Normal,
    Metallic,
    Roughness,
    AmbientOcclusion,
    Emissive
}

#[repr(C)]
#[derive(Debug)]
pub struct TextureIndex {
    pub index:  usize,
    pub t_type: TextureType
}

#[repr(C)]
#[derive(Debug)]
pub enum AlphaMode {
    Opaque,
    Cutoff,
    Blend
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<VertexPositionColorTextureNormalTangentBitangent>,
    pub indices:  Vec<u32>
}

#[derive(Debug)]
pub struct Material {
    pub albedo_color:     Vec4,
    pub metallic_factor:  f32,
    pub roughness_factor: f32,
    pub emissive_factor:  Vec3,
    pub alpha_mode:       AlphaMode,
    pub alpha_cutoff:     f32,
    pub double_sided:     bool,
    pub textures:         Vec<TextureIndex>
}

#[derive(Debug)]
pub struct Scene {
    pub meshes:    Vec<Mesh>,
    pub materials: Vec<Material>
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
                    let reader = BinaryReader::new(data);

                    let name = name.to_lowercase();
                    let name = name.split('_').collect::<Vec<&str>>();

                    if (accessor.count as usize) > vertices.len() {
                        for _ in 0..(accessor.count as usize - vertices.len()) {
                            vertices.push(VertexPositionColorTextureNormalTangentBitangent {
                                position: Vec3(0.0, 0.0, 0.0),
                                color: Vec4(0.0, 0.0, 0.0, 0.0),
                                tex_coord: Vec2(0.0, 0.0),
                                normal: Vec3(0.0, 0.0, 0.0),
                                tangent: Vec3(0.0, 0.0, 0.0),
                                bitangent: Vec3(0.0, 0.0, 0.0)
                            });
                        }
                    } else if (accessor.count as usize) < vertices.len() {
                        // TODO: Don't panic - these situations (if they can happen) should be handled.
                        panic!("AAAAAAAAAAAAA IT'S NOT SUPPOSED TO BE LIKE THIS YET");
                    }
                    
                    match name[0].to_lowercase().as_str() {
                        "position" => {
                            let data = reinterpret_slice::<u8, f32>(data);
                            let mut vertex = 0;
                            for i in (0..data.len()).step_by(3) {
                                vertices[vertex].position = Vec3(data[i + 0], data[i + 1], data[i + 2]);
                                vertex += 1;
                            }
                        }

                        "normal" => {
                            let data = reinterpret_slice::<u8, f32>(data);
                            let mut vertex = 0;
                            for i in (0..data.len()).step_by(3) {
                                vertices[vertex].normal = Vec3(data[i + 0], data[i + 1], data[i + 2]);
                                vertex += 1;
                            }
                        }

                        "texcoord" => {
                            let data = reinterpret_slice::<u8, f32>(data);
                            let mut vertex = 0;
                            for i in (0..data.len()).step_by(2) {
                                vertices[vertex].tex_coord = Vec2(data[i + 0] as f32, data[i + 1] as f32);
                                vertex += 1;
                            }
                        }

                        //_ => return Err(io::Error::new(io::ErrorKind::Unsupported, format!("Unsupported attribute \"{}\"", name[0])))
                        _ => {} // Ignore
                    }
                }
                
                if let Some(prim_indices) = primitive.indices {
                    let accessor = &accessors[prim_indices as usize];
                    let view = &buffer_views[accessor.buffer_view.unwrap() as usize];
                    let data = &buffers[view.buffer as usize].data[view.byte_offset as usize..view.byte_offset as usize + view.byte_length as usize];
                    let data = reinterpret_slice::<u8, u16>(data);
                    for value in data {
                        indices.push(*value as u32);
                    }
                }
            }

            println!("{}", vertices.len());
            println!("{:?}", vertices);
            println!("{}", indices.len());
            println!("{:?}", indices);

            meshes.push(Mesh {
                vertices,
                indices
            });
        }

        let mut materials = Vec::new();

        if let Some(gltf_materials) = gltf.materials {
            for material in gltf_materials {
                let mut textures = Vec::new();

                let (base, metallic, roughness) = if let Some(pbr_mr) = material.pbr_metallic_roughness {
                    if let Some(bct) = pbr_mr.base_color_texture {
                        textures.push(TextureIndex {
                            index: bct.index as usize,
                            t_type: TextureType::Albedo
                        });
                    }

                    if let Some(mrt) = pbr_mr.metallic_roughness_texture {
                        textures.push(TextureIndex {
                            index: mrt.index as usize,
                            t_type: TextureType::Metallic
                        });

                        textures.push(TextureIndex {
                            index: mrt.index as usize,
                            t_type: TextureType::Roughness
                        });
                    }

                    (pbr_mr.base_color_factor, pbr_mr.metallic_factor, pbr_mr.roughness_factor)
                } else {
                    (Vec4(1.0, 1.0, 1.0, 1.0), 1.0, 1.0)
                };

                // TODO: Normal scale and occlusion strength.
                if let Some(nmt) = material.normal_texture {
                    textures.push(TextureIndex {
                        index: nmt.index as usize,
                        t_type: TextureType::Normal
                    });
                }

                if let Some(oct) = material.occlusion_texture {
                    textures.push(TextureIndex {
                        index: oct.index as usize,
                        t_type: TextureType::AmbientOcclusion
                    })
                }

                if let Some(emt) = material.emissive_texture {
                    textures.push(TextureIndex {
                        index: emt.index as usize,
                        t_type: TextureType::Emissive
                    });
                }

                let alpha_mode = match material.alpha_mode {
                    importers::gltf::AlphaMode::Opaque => AlphaMode::Opaque,
                    importers::gltf::AlphaMode::Mask => AlphaMode::Cutoff,
                    importers::gltf::AlphaMode::Blend => AlphaMode::Blend,
                };

                materials.push(Material {
                    albedo_color: base,
                    metallic_factor: metallic,
                    roughness_factor: roughness,
                    emissive_factor: material.emissive_factor,
                    alpha_mode,
                    alpha_cutoff: material.alpha_cutoff,
                    double_sided: material.double_sided,
                    textures
                });
            }
        }

        Ok(Scene { meshes, materials })
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
    pub color:     Vec4,
    pub tex_coord: Vec2,
    pub normal:    Vec3,
    pub tangent:   Vec3,
    pub bitangent: Vec3
}

fn reinterpret_slice<TFrom, TTo>(value: &[TFrom]) -> &[TTo] {
    unsafe { std::slice::from_raw_parts(value.as_ptr() as *const TTo, value.len() / std::mem::size_of::<TTo>()) }
}