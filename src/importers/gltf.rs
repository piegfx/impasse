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
pub struct Gltf {
    pub asset:     Asset,
    pub scene:     Option<i32>,
    pub scenes:    Option<Vec<Scene>>,
    pub nodes:     Option<Vec<Node>>,
    pub materials: Option<Vec<Material>>,
    pub meshes:    Option<Vec<Mesh>>,
    pub textures:  Option<Vec<Texture>>,

    pub buffers:   Vec<Vec<u8>>
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
        let scene = if let Some(sc) = json.get("scene") { Some(sc.as_i64().unwrap() as i32) } else { None };

        // Get the scenes information.
        let scenes = if let Some(s_scenes) = json.get("scenes") {
            let mut tmp_scenes = Vec::new();
            for scene in s_scenes.as_array().unwrap().into_iter() {
                let name = if let Some(nm) = scene.get("name") { Some(nm.as_str().unwrap().to_string()) } else { None };
                
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
                let camera = if let Some(cm) = value.get("camera") { Some(cm.as_i64().unwrap() as i32 ) } else { None };
                
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

                let skin = if let Some(sk) = value.get("skin") { Some(sk.as_i64().unwrap() as i32) } else { None };

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

                let mesh = if let Some(msh) = value.get("mesh") { Some(msh.as_i64().unwrap() as i32) } else { None };

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

                let name = if let Some(nm) = value.get("name") { Some(nm.as_str().unwrap().to_string()) } else { None };

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
                let name = if let Some(nm) = material.get("name") { Some(nm.as_str().unwrap().to_string()) } else { None };

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

                let name = if let Some(nm) = mesh.get("name") { Some(nm.as_str().unwrap().to_string()) } else { None };

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

                let name = if let Some(nm) = texture.get("name") { Some(nm.as_str().unwrap().to_string()) } else { None };

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

        let mut buffers = Vec::new();

        Ok(Gltf { asset, scene, scenes, nodes, materials, meshes, textures, buffers })
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