use std::io;

use serde_json::Value;

use super::Importer;

#[derive(Debug)]
pub struct GltfAsset {
    version:     String,

    copyright:   Option<String>,
    generator:   Option<String>,
    min_version: Option<String>
}

#[derive(Debug)]
pub struct GltfScene {
    pub nodes: Option<Vec<i32>>,
    pub name:  Option<String>
}

#[derive(Debug)]
pub struct GltfNode {
    pub camera:      Option<i32>,
    pub children:    Option<Vec<i32>>,
    pub skin:        Option<i32>,
    pub matrix:      crate::Mat4,
    pub mesh:        Option<i32>,
    pub rotation:    crate::Vec4,
    pub scale:       crate::Vec3,
    pub translation: crate::Vec3,
    pub weights:     Option<Vec<i32>>,
    pub name:        Option<String>
}

#[derive(Debug)]
pub struct Gltf {
    pub asset:   GltfAsset,
    pub scene:   Option<i32>,
    pub scenes:  Option<Vec<GltfScene>>,
    pub nodes:   Option<Vec<GltfNode>>,

    pub buffers: Vec<Vec<u8>>
}

impl Importer for Gltf {
    fn import(data: &[u8]) -> Result<Self, io::Error> {
        // TODO: Binary GLTF files.
        let json: Value = serde_json::from_slice(data)?;

        // Get the asset information - no need to check here, a GLTF file is required to have "asset".
        let s_asset = &json["asset"];
        let asset = GltfAsset {
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
            for value in s_scenes.as_array().unwrap().into_iter() {
                let name = if let Some(nm) = value.get("name") { Some(nm.as_str().unwrap().to_string()) } else { None };
                
                let nodes = if let Some(s_nodes) = value.get("nodes") {
                    let s_nodes = s_nodes.as_array().unwrap();

                    let mut nodes = Vec::with_capacity(s_nodes.len());
                    for node in s_nodes {
                        nodes.push(node.as_i64().unwrap() as i32);
                    }

                    Some(nodes)
                } else {
                    None
                };

                tmp_scenes.push(GltfScene { 
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
                        weights.push(weight.as_i64().unwrap() as i32);
                    }

                    Some(weights)
                } else {
                    None
                };

                let name = if let Some(nm) = value.get("name") { Some(nm.as_str().unwrap().to_string()) } else { None };

                nodes.push(GltfNode {
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

        let mut buffers = Vec::new();

        Ok(Gltf { asset, scene, scenes, nodes, buffers })
    }
}