use std::ffi::{c_char, CStr};

use crate::*;

#[repr(C)]
pub struct Mesh {
    pub vertices:     *const VertexPositionColorTextureNormalTangentBitangent,
    pub num_vertices: usize,
    pub indices:      *const u32,
    pub num_indices:  usize
    
}

#[repr(C)]
pub struct Material {
    pub albedo_color:     Vec4,
    pub metallic_factor:  f32,
    pub roughness_factor: f32,
    pub emissive_factor:  Vec3,
    pub alpha_mode:       AlphaMode,
    pub alpha_cutoff:     f32,
    pub double_sided:     bool,
    pub textures:         *const TextureIndex,
    pub num_textures:     usize
}

#[repr(C)]
pub struct Scene {
    pub meshes:        *const *const Mesh,
    pub num_meshes:    usize,
    pub materials:     *const *const Material,
    pub num_materials: usize
}

#[no_mangle]
pub unsafe extern "C" fn iaLoadScene(path: *const c_char, mut scene: *mut *mut Scene) {
    let rs_scene = crate::Scene::from_gltf(CStr::from_ptr(path).to_str().unwrap()).unwrap();
    let mut meshes = Vec::with_capacity(rs_scene.meshes.len());
    for mesh in rs_scene.meshes {
        meshes.push(Box::into_raw(Box::new(Mesh {
            num_vertices: mesh.vertices.len(),
            vertices: mesh.vertices.as_ptr(),

            num_indices: mesh.indices.len(),
            indices: mesh.indices.as_ptr()
        })) as *const _);

        std::mem::forget(mesh.vertices);
        std::mem::forget(mesh.indices);
    }

    let mut materials = Vec::with_capacity(rs_scene.materials.len());
    for material in rs_scene.materials {
        materials.push(Box::into_raw(Box::new(Material {
            albedo_color: material.albedo_color,
            metallic_factor: material.metallic_factor,
            roughness_factor: material.roughness_factor,
            emissive_factor: material.emissive_factor,
            alpha_mode: material.alpha_mode,
            alpha_cutoff: material.alpha_cutoff,
            double_sided: material.double_sided,
            textures: material.textures.as_ptr(),
            num_textures: material.textures.len(),
        })) as *const _);

        std::mem::forget(material.textures);
    }

    let scene_box = Box::new(Scene {
        meshes: meshes.as_ptr(),
        num_meshes: meshes.len(),
        materials: materials.as_ptr(),
        num_materials: materials.len()
    });

    std::mem::forget(meshes);
    std::mem::forget(materials);

    let scene_ptr = Box::into_raw(scene_box);
    *scene = scene_ptr;
}