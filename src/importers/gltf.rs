use std::{io, collections::HashMap};

use serde_json::Value;

use super::Importer;

#[derive(Debug)]
pub struct Asset {
    pub version:     String,

    pub copyright:   Option<String>,
    pub generator:   Option<String>,
    pub min_version: Option<String>
}

#[derive(Debug)]
pub struct Scene {
    pub nodes: Option<Vec<i32>>,
    pub name:  Option<String>
}

#[derive(Debug)]
pub struct Node {
    pub camera:      Option<i32>,
    pub children:    Option<Vec<i32>>,
    pub skin:        Option<i32>,
    pub matrix:      crate::Mat4,
    pub mesh:        Option<i32>,
    pub rotation:    crate::Vec4,
    pub scale:       crate::Vec3,
    pub translation: crate::Vec3,
    pub weights:     Option<Vec<f32>>,
    pub name:        Option<String>
}

#[derive(Debug)]
pub struct TextureInfo {
    pub index:     i32,
    pub tex_coord: i32,
    pub scalar:    f32
}

#[derive(Debug)]
pub struct PbrMetallicRoughness {
    pub base_color_factor:          crate::Vec4,
    pub base_color_texture:         Option<TextureInfo>,
    pub metallic_factor:            f32,
    pub roughness_factor:           f32,
    pub metallic_roughness_texture: Option<TextureInfo>
}

#[derive(Debug)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend
}

#[derive(Debug)]
pub struct Material {
    pub name:                    Option<String>,
    pub pbr_metallic_roughness:  Option<PbrMetallicRoughness>,
    pub normal_texture:          Option<TextureInfo>,
    pub occlusion_texture:       Option<TextureInfo>,
    pub emissive_texture:        Option<TextureInfo>,
    pub emissive_factor:         crate::Vec3,
    pub alpha_mode:              AlphaMode,
    pub alpha_cutoff:            f32,
    pub double_sided:            bool
}

#[derive(Debug)]
pub enum Topology {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan
}

#[derive(Debug)]
pub struct MeshPrimitive {
    pub attributes: HashMap<String, i32>,
    pub indices:    Option<i32>,
    pub material:   Option<i32>,
    pub mode:       Topology,
    // TODO: Targets, I have no idea what the JSON object looks like so unable to implement
}

#[derive(Debug)]
pub struct Mesh {
    pub primitives: Vec<MeshPrimitive>,
    pub weights:    Option<Vec<f32>>,
    pub name:       Option<String>
}

#[derive(Debug)]
pub struct Texture {
    pub sampler: Option<i32>,
    pub source:  Option<i32>,
    pub name:    Option<String>
}

#[derive(Debug)]
pub struct Image {
    pub uri:         Option<String>,
    pub mime_type:   Option<String>,
    pub buffer_view: Option<i32>,
    pub name:        Option<String>
}

#[derive(Debug)]
pub enum ComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float
}

#[derive(Debug)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4
}

#[derive(Debug)]
pub struct AccessorSparseIndices {
    pub buffer_view:    i32,
    pub byte_offset:    i32,
    pub component_type: ComponentType
}

#[derive(Debug)]
pub struct AccessorSparseValues {
    pub buffer_view: i32,
    pub byte_offset: i32
}

#[derive(Debug)]
pub struct AccessorSparse {
    pub count:   i32,
    pub indices: AccessorSparseIndices,
    pub values:  AccessorSparseValues
}

#[derive(Debug)]
pub struct Accessor {
    pub buffer_view:    Option<i32>,
    pub byte_offset:    i32,
    pub component_type: ComponentType,
    pub normalized:     bool,
    pub count:          i32,
    pub accessor_type:  AccessorType,
    pub max:            Option<Vec<f32>>,
    pub min:            Option<Vec<f32>>,
    pub sparse:         Option<AccessorSparse>,
    pub name:           Option<String>
}

#[derive(Debug)]
pub enum Target {
    ArrayBuffer,
    ElementArrayBuffer
}

#[derive(Debug)]
pub struct BufferView {
    pub buffer:      i32,
    pub byte_offset: i32,
    pub byte_length: i32,
    pub byte_stride: Option<i32>,
    pub target:      Option<Target>,
    pub name:        Option<String>
}

#[derive(Debug)]
pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear
}

#[derive(Debug)]
pub enum TextureWrapMode {
    ClampToEdge,
    MirroredRepeat,
    Repeat
}

#[derive(Debug)]
pub struct Sampler {
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub wrap_s:     TextureWrapMode,
    pub wrap_t:     TextureWrapMode,
    pub name:       Option<String>
}

#[derive(Debug)]
pub struct Buffer {
    pub uri:         Option<String>,
    pub byte_length: i32,
    pub name:        Option<String>
}

#[derive(Debug)]
pub struct Gltf {
    pub asset:        Asset,
    pub scene:        Option<i32>,
    pub scenes:       Option<Vec<Scene>>,
    pub nodes:        Option<Vec<Node>>,
    pub materials:    Option<Vec<Material>>,
    pub meshes:       Option<Vec<Mesh>>,
    pub textures:     Option<Vec<Texture>>,
    pub images:       Option<Vec<Image>>,
    pub accessors:    Option<Vec<Accessor>>,
    pub buffer_views: Option<Vec<BufferView>>,
    pub samplers:     Option<Vec<Sampler>>,

    pub buffers:      Option<Vec<Buffer>>
}

impl Importer for Gltf {
    fn import(data: &[u8]) -> Result<Self, io::Error> {
        // TODO: Binary GLTF files.
        let json: Value = serde_json::from_slice(data)?;

        // Get the asset information - no need to check here, a GLTF file is required to have "asset".
        let s_asset = &json["asset"];
        let asset = Asset {
            version: s_asset["version"].as_str().unwrap().to_string(),
            copyright: if let Some(cr) = s_asset.get("copyright") { Some(cr.as_str().unwrap().to_string()) } else { None },
            generator: if let Some(gn) = s_asset.get("generator") { Some(gn.as_str().unwrap().to_string()) } else { None },
            min_version: if let Some(mv) = s_asset.get("minVersion") { Some(mv.as_str().unwrap().to_string()) } else { None },
        };

        // Get the default scene, if any.
        let scene = if let Some(sc) = json.get("scene") {
            Some(sc.as_i64().unwrap() as i32)
        } else {
            None
        };

        // Get the scenes information.
        let scenes = if let Some(s_scenes) = json.get("scenes") {
            let mut tmp_scenes = Vec::new();
            for scene in s_scenes.as_array().unwrap().into_iter() {
                let name = if let Some(nm) = scene.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };
                
                let nodes = if let Some(s_nodes) = scene.get("nodes") {
                    let s_nodes = s_nodes.as_array().unwrap();

                    let mut nodes = Vec::with_capacity(s_nodes.len());
                    for node in s_nodes {
                        nodes.push(node.as_i64().unwrap() as i32);
                    }

                    Some(nodes)
                } else {
                    None
                };

                tmp_scenes.push(Scene { 
                    name,
                    nodes
                });
            }

            Some(tmp_scenes)
        } else {
            None
        };

    
        let nodes = if let Some(s_nodes) = json.get("nodes") {
            let s_nodes = s_nodes.as_array().unwrap();

            let mut nodes = Vec::with_capacity(s_nodes.len());
            for value in s_nodes {
                let camera = if let Some(cm) = value.get("camera") {
                    Some(cm.as_i64().unwrap() as i32 )
                } else {
                    None
                };
                
                let children = if let Some(ch) = value.get("children") {
                    let ch = ch.as_array().unwrap();

                    let mut children = Vec::with_capacity(ch.len());
                    for child in ch {
                        children.push(child.as_i64().unwrap() as i32);
                    }

                    Some(children)
                } else {
                    None
                };

                let skin = if let Some(sk) = value.get("skin") {
                    Some(sk.as_i64().unwrap() as i32)
                } else {
                    None
                };

                let matrix = if let Some(mat) = value.get("matrix") {
                    let mat = mat.as_array().unwrap();

                    crate::Mat4(mat[0].as_f64().unwrap() as f32, mat[1].as_f64().unwrap() as f32, mat[2].as_f64().unwrap() as f32, mat[3].as_f64().unwrap() as f32,
                                mat[4].as_f64().unwrap() as f32, mat[5].as_f64().unwrap() as f32, mat[6].as_f64().unwrap() as f32, mat[7].as_f64().unwrap() as f32,
                                mat[8].as_f64().unwrap() as f32, mat[9].as_f64().unwrap() as f32, mat[10].as_f64().unwrap() as f32, mat[11].as_f64().unwrap() as f32,
                                mat[12].as_f64().unwrap() as f32, mat[13].as_f64().unwrap() as f32, mat[14].as_f64().unwrap() as f32, mat[15].as_f64().unwrap() as f32)
                } else {
                    crate::Mat4(1.0, 0.0, 0.0, 0.0,
                                0.0, 1.0, 0.0, 0.0,
                                0.0, 0.0, 1.0, 0.0,
                                0.0, 0.0, 0.0, 1.0
                            )
                };

                let mesh = if let Some(msh) = value.get("mesh") {
                    Some(msh.as_i64().unwrap() as i32)
                } else {
                    None
                };

                let rotation = if let Some(rot) = value.get("rotation") {
                    let rot = rot.as_array().unwrap();

                    crate::Vec4(rot[0].as_f64().unwrap() as f32, rot[1].as_f64().unwrap() as f32, rot[2].as_f64().unwrap() as f32, rot[3].as_f64().unwrap() as f32)
                } else {
                    crate::Vec4(0.0, 0.0, 0.0, 1.0)
                };

                let scale = if let Some(sc) = value.get("scale") {
                    let sc = sc.as_array().unwrap();

                    crate::Vec3(sc[0].as_f64().unwrap() as f32, sc[1].as_f64().unwrap() as f32, sc[2].as_f64().unwrap() as f32)
                } else {
                    crate::Vec3(1.0, 1.0, 1.0)
                };

                let translation = if let Some(tr) = value.get("translation") {
                    let tr = tr.as_array().unwrap();

                    crate::Vec3(tr[0].as_f64().unwrap() as f32, tr[1].as_f64().unwrap() as f32, tr[2].as_f64().unwrap() as f32)
                } else {
                    crate::Vec3(0.0, 0.0, 0.0)
                };

                let weights = if let Some(s_weights) = value.get("weights") {
                    let s_weights = s_weights.as_array().unwrap();

                    let mut weights = Vec::with_capacity(s_weights.len());
                    for weight in s_weights {
                        weights.push(weight.as_f64().unwrap() as f32);
                    }

                    Some(weights)
                } else {
                    None
                };

                let name = if let Some(nm) = value.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                nodes.push(Node {
                    camera,
                    children,
                    skin,
                    matrix,
                    mesh,
                    rotation,
                    scale,
                    translation,
                    weights,
                    name,
                });
            }

            Some(nodes)
        } else {
            None
        };
        
        let materials = if let Some(s_materials) = json.get("materials") {
            let s_materials = s_materials.as_array().unwrap();

            let mut materials = Vec::with_capacity(s_materials.len());
            for material in s_materials {
                let name = if let Some(nm) = material.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                let pbr_metallic_roughness = if let Some(pbr) = material.get("pbrMetallicRoughness") {
                    let base_color_factor = if let Some(bcf) = pbr.get("baseColorFactor") {
                        crate::Vec4(bcf[0].as_f64().unwrap() as f32, bcf[1].as_f64().unwrap() as f32, bcf[2].as_f64().unwrap() as f32, bcf[3].as_f64().unwrap() as f32)
                    } else {
                        crate::Vec4(1.0, 1.0, 1.0, 1.0)
                    };

                    let base_color_texture = if let Some(bct) = pbr.get("baseColorTexture") {
                        Some(get_texture_info(bct))
                    } else {
                        None
                    };
                    
                    let metallic_factor = if let Some(mf) = pbr.get("metallicFactor") {
                        mf.as_f64().unwrap() as f32
                    } else {
                        1.0
                    };

                    let roughness_factor = if let Some(rf) = pbr.get("roughnessFactor") {
                        rf.as_f64().unwrap() as f32
                    } else {
                        1.0
                    };

                    let metallic_roughness_texture = if let Some(mft) = pbr.get("metallicRoughnessTexture") {
                        Some(get_texture_info(mft))
                    } else {
                        None
                    };

                    Some(PbrMetallicRoughness {
                        base_color_factor,
                        base_color_texture,
                        metallic_factor,
                        roughness_factor,
                        metallic_roughness_texture
                    })
                } else {
                    None
                };

                let normal_texture = if let Some(nt) = material.get("normalTexture") {
                    Some(get_texture_info(nt))
                } else {
                    None
                };

                let occlusion_texture = if let Some(ot) = material.get("occlusionTexture") {
                    Some(get_texture_info(ot))
                } else {
                    None
                };

                let emissive_texture = if let Some(et) = material.get("emissiveTexture") {
                    Some(get_texture_info(et))
                } else {
                    None
                };

                let emissive_factor = if let Some(ef) = material.get("emissiveFactor") {
                    let ef = ef.as_array().unwrap();

                    crate::Vec3(ef[0].as_f64().unwrap() as f32, ef[1].as_f64().unwrap() as f32, ef[2].as_f64().unwrap() as f32)
                } else {
                    crate::Vec3(0.0, 0.0, 0.0)
                };
                
                let alpha_mode = if let Some(am) = material.get("alphaMode") {
                    let am = am.as_str().unwrap();

                    match am {
                        "OPAQUE" => AlphaMode::Opaque,
                        "MASK" => AlphaMode::Mask,
                        "BLEND" => AlphaMode::Blend,
                        _ => AlphaMode::Opaque
                    }
                } else {
                    AlphaMode::Opaque
                };

                let alpha_cutoff = if let Some(ac) = material.get("alphaCutoff") {
                    ac.as_f64().unwrap() as f32
                } else {
                    0.5
                };

                let double_sided = if let Some(dc) = material.get("doubleSided") {
                    dc.as_bool().unwrap()
                } else {
                    false
                };

                materials.push(Material {
                    name,
                    pbr_metallic_roughness,
                    normal_texture,
                    occlusion_texture,
                    emissive_texture,
                    emissive_factor,
                    alpha_mode,
                    alpha_cutoff,
                    double_sided,
                });
            }

            Some(materials)
        } else {
            None
        };

        let meshes = if let Some(s_meshes) = json.get("meshes") {
            let s_meshes = s_meshes.as_array().unwrap();
            let mut meshes = Vec::with_capacity(s_meshes.len());
            
            for mesh in s_meshes {
                let s_primitives = mesh["primitives"].as_array().unwrap();
                let mut primitives = Vec::with_capacity(s_primitives.len());

                for primitive in s_primitives {
                    let s_attributes = primitive["attributes"].as_object().unwrap();
                    let mut attributes = HashMap::with_capacity(s_attributes.len());

                    for (key, value) in s_attributes {
                        attributes.insert(key.to_string(), value.as_i64().unwrap() as i32);
                    }

                    let indices = if let Some(id) = primitive.get("indices") {
                        Some(id.as_i64().unwrap() as i32)
                    } else {
                        None
                    };

                    let material = if let Some(mat) = primitive.get("material") {
                        Some(mat.as_i64().unwrap() as i32)
                    } else {
                        None
                    };

                    let mode = if let Some(md) = primitive.get("mode") {
                        match md.as_i64().unwrap() {
                            0 => Topology::Points,
                            1 => Topology::Lines,
                            2 => Topology::LineLoop,
                            3 => Topology::LineStrip,
                            4 => Topology::Triangles,
                            5 => Topology::TriangleStrip,
                            6 => Topology::TriangleFan,
                            _ => Topology::Triangles
                        }
                    } else {
                        Topology::Triangles
                    };

                    primitives.push(MeshPrimitive {
                        attributes,
                        indices,
                        material,
                        mode,
                    });
                }

                let weights = if let Some(s_weights) = mesh.get("weights") {
                    let s_weights = s_weights.as_array().unwrap();

                    let mut weights = Vec::with_capacity(s_weights.len());
                    for weight in s_weights {
                        weights.push(weight.as_f64().unwrap() as f32);
                    }

                    Some(weights)
                } else {
                    None
                };

                let name = if let Some(nm) = mesh.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                meshes.push(Mesh {
                    primitives,
                    weights,
                    name
                });
            }

            Some(meshes)
        } else {
            None
        };

        let textures = if let Some(s_textures) = json.get("textures") {
            let s_textures = s_textures.as_array().unwrap();

            let mut textures = Vec::with_capacity(s_textures.len());
            for texture in s_textures {
                let sampler = if let Some(smp) = texture.get("sampler") {
                    Some(smp.as_i64().unwrap() as i32)
                } else {
                    None
                };

                let source = if let Some(src) = texture.get("source") {
                    Some(src.as_i64().unwrap() as i32)
                } else {
                    None
                };

                let name = if let Some(nm) = texture.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None 
                };

                textures.push(Texture {
                    sampler,
                    source,
                    name
                });
            }

            Some(textures)
        } else {
            None
        };

        let images = if let Some(s_images) = json.get("images") {
            let s_images = s_images.as_array().unwrap();

            let mut images = Vec::with_capacity(s_images.len());
            for image in s_images {
                let uri = if let Some(ui) = image.get("uri") {
                    Some(ui.as_str().unwrap().to_string())
                } else {
                    None
                };

                let mime_type = if let Some(mt) = image.get("mimeType") {
                    Some(mt.as_str().unwrap().to_string())
                } else {
                    None
                };

                let buffer_view = if let Some(bv) = image.get("bufferView") {
                    Some(bv.as_i64().unwrap() as i32)
                } else {
                    None
                };

                let name = if let Some(nm) = image.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                images.push(Image {
                    uri,
                    mime_type,
                    buffer_view,
                    name
                });
            }

            Some(images)
        } else {
            None
        };

        let accessors = if let Some(s_accessors) = json.get("accessors") {
            let s_accessors = s_accessors.as_array().unwrap();

            let mut accessors = Vec::with_capacity(s_accessors.len());
            for accessor in s_accessors {
                let buffer_view = if let Some(bv) = accessor.get("bufferView") {
                    Some(bv.as_i64().unwrap() as i32)
                } else {
                    None
                };

                let byte_offset = if let Some(bo) = accessor.get("byteOffset") {
                    bo.as_i64().unwrap() as i32
                } else {
                    0
                };

                let component_type = match accessor["componentType"].as_i64().unwrap() {
                    5120 => ComponentType::Byte,
                    5121 => ComponentType::UnsignedByte,
                    5122 => ComponentType::Short,
                    5123 => ComponentType::UnsignedShort,
                    5125 => ComponentType::UnsignedInt,
                    5126 => ComponentType::Float,
                    ct => panic!("Unrecognized component type {ct}")
                };

                let normalized = if let Some(nm) = accessor.get("normalized") {
                    nm.as_bool().unwrap()
                } else {
                    false
                };

                let count = accessor["count"].as_i64().unwrap() as i32;
                
                let accessor_type = match accessor["type"].as_str().unwrap() {
                    "SCALAR" => AccessorType::Scalar,
                    "VEC2" => AccessorType::Vec2,
                    "VEC3" => AccessorType::Vec3,
                    "VEC4" => AccessorType::Vec4,
                    "MAT2" => AccessorType::Mat2,
                    "MAT3" => AccessorType::Mat3,
                    "MAT4" => AccessorType::Mat4,
                    at => panic!("Unrecognized accessor type \"{at}\".")
                };

                let max = if let Some(mx) = accessor.get("max") {
                    let mx = mx.as_array().unwrap();
                    
                    let mut max = Vec::with_capacity(mx.len());
                    for value in mx {
                        max.push(value.as_f64().unwrap() as f32);
                    }

                    Some(max)
                } else {
                    None
                };

                let min = if let Some(mn) = accessor.get("min") {
                    let mn = mn.as_array().unwrap();
                    
                    let mut min = Vec::with_capacity(mn.len());
                    for value in mn {
                        min.push(value.as_f64().unwrap() as f32);
                    }

                    Some(min)
                } else {
                    None
                };

                let sparse = if let Some(s_sparce) = accessor.get("sparse") {
                    let count = s_sparce["count"].as_i64().unwrap() as i32;

                    let s_indices = &s_sparce["indices"];

                    let buffer_view = s_indices["bufferView"].as_i64().unwrap() as i32;

                    let byte_offset = if let Some(bo) = s_indices.get("byteOffset") {
                        bo.as_i64().unwrap() as i32
                    } else {
                        0
                    };

                    let component_type = match s_indices["componentType"].as_i64().unwrap() {
                        5121 => ComponentType::UnsignedByte,
                        5123 => ComponentType::UnsignedShort,
                        5125 => ComponentType::UnsignedInt,
                        ct => panic!("Unsupported component type {ct}.")
                    };

                    let indices = AccessorSparseIndices {
                        buffer_view,
                        byte_offset,
                        component_type
                    };

                    let s_values = &s_sparce["values"];

                    let buffer_view = s_values["bufferView"].as_i64().unwrap() as i32;

                    let byte_offset = if let Some(bo) = s_values.get("byteOffset") {
                        bo.as_i64().unwrap() as i32
                    } else {
                        0
                    };

                    let values = AccessorSparseValues {
                        buffer_view,
                        byte_offset
                    };

                    Some(AccessorSparse {
                        count,
                        indices,
                        values
                    })
                } else {
                    None
                };

                let name = if let Some(nm) = accessor.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                accessors.push(Accessor {
                    buffer_view,
                    byte_offset,
                    component_type,
                    normalized,
                    count,
                    accessor_type,
                    max,
                    min,
                    sparse,
                    name,
                });
            }

            Some(accessors)
        } else {
            None
        };

        let buffer_views = if let Some(s_buffer_views) = json.get("bufferViews") {
            let s_buffer_views = s_buffer_views.as_array().unwrap();

            let mut buffer_views = Vec::with_capacity(s_buffer_views.len());
            for view in s_buffer_views { 
                let buffer = view["buffer"].as_i64().unwrap() as i32;
                
                let byte_offset = if let Some(bo) = view.get("byteOffset") {
                    bo.as_i64().unwrap() as i32
                } else {
                    0
                };

                let byte_length = view["byteLength"].as_i64().unwrap() as i32;

                let byte_stride = if let Some(bs) = view.get("byteStride") {
                    Some(bs.as_i64().unwrap() as i32)
                } else {
                    None
                };

                let target = if let Some(tg) = view.get("target") {
                    Some(match tg.as_i64().unwrap() {
                        34962 => Target::ArrayBuffer,
                        34963 => Target::ElementArrayBuffer,
                        tg => panic!("Unrecognized target {tg}.")
                    })
                } else {
                    None
                };

                let name = if let Some(nm) = view.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                buffer_views.push(BufferView {
                    buffer,
                    byte_offset,
                    byte_length,
                    byte_stride,
                    target,
                    name
                });
            }

            Some(buffer_views)
        } else {
            None
        };

        let samplers = if let Some(s_samplers) = json.get("samplers") {
            let s_samplers = s_samplers.as_array().unwrap();

            let mut samplers = Vec::with_capacity(s_samplers.len());
            for sampler in s_samplers {
                let mag_filter = if let Some(mf) = sampler.get("magFilter") {
                    Some(get_texture_filter(mf.as_i64().unwrap()))
                } else {
                    None
                };

                let min_filter = if let Some(mf) = sampler.get("minFilter") {
                    Some(get_texture_filter(mf.as_i64().unwrap()))
                } else {
                    None
                };

                let wrap_s = if let Some(ws) = sampler.get("wrapS") {
                    get_wrap_mode(ws.as_i64().unwrap())
                } else {
                    TextureWrapMode::Repeat
                };

                let wrap_t = if let Some(wt) = sampler.get("wrapT") {
                    get_wrap_mode(wt.as_i64().unwrap())
                } else {
                    TextureWrapMode::Repeat
                };

                let name = if let Some(nm) = sampler.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                samplers.push(Sampler {
                    mag_filter,
                    min_filter,
                    wrap_s,
                    wrap_t,
                    name,
                });
            }

            Some(samplers)
        } else {
            None
        };

        let buffers = if let Some(s_buffers) = json.get("buffers") {
            let s_buffers = s_buffers.as_array().unwrap();

            let mut buffers = Vec::with_capacity(s_buffers.len());
            for buffer in s_buffers {
                let uri = if let Some(ui) = buffer.get("uri") {
                    Some(ui.as_str().unwrap().to_string())
                } else {
                    None
                };

                let byte_length = buffer["byteLength"].as_i64().unwrap() as i32;

                let name = if let Some(nm) = buffer.get("name") {
                    Some(nm.as_str().unwrap().to_string())
                } else {
                    None
                };

                buffers.push(Buffer {
                    uri,
                    byte_length,
                    name
                });
            }

            Some(buffers)
        } else {
            None
        };

        Ok(Gltf {
            asset,
            scene,
            scenes,
            nodes,
            materials,
            meshes,
            textures,
            images,
            accessors,
            buffer_views,
            samplers,
            buffers
        })
    }
}

fn get_texture_info(value: &Value) -> TextureInfo {
    let index = value["index"].as_i64().unwrap() as i32;
    let tex_coord = if let Some(tc) = value.get("texCoord") { tc.as_i64().unwrap() as i32 } else { 0 };

    let scalar = if let Some(scale) = value.get("scale") { 
        scale.as_f64().unwrap() as f32
    } else if let Some(strength) = value.get("strength") {
        strength.as_f64().unwrap() as f32
    } else {
        1.0
    };

    TextureInfo { index, tex_coord, scalar }
}

fn get_texture_filter(value: i64) -> TextureFilter {
    match value {
        9728 => TextureFilter::Nearest,
        9729 => TextureFilter::Linear,
        9984 => TextureFilter::NearestMipmapNearest,
        9985 => TextureFilter::LinearMipmapNearest,
        9986 => TextureFilter::NearestMipmapLinear,
        9987 => TextureFilter::LinearMipmapLinear,
        tf => panic!("Invalid texture filter {tf}")
    }
}

fn get_wrap_mode(value: i64) -> TextureWrapMode {
    match value {
        33071 => TextureWrapMode::ClampToEdge,
        33648 => TextureWrapMode::MirroredRepeat,
        10497 => TextureWrapMode::Repeat,
        wm => panic!("Invalid wrap mode.")
    }
}