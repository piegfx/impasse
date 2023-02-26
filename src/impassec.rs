use std::ffi::{c_char, CStr};

use crate::VertexPositionColorTextureNormalTangentBitangent;

#[repr(C)]
pub struct Mesh {
    pub vertices:     *const VertexPositionColorTextureNormalTangentBitangent,
    pub num_vertices: usize,
    pub indices:      *const u32,
    pub num_indices:  usize
}

#[repr(C)]
pub struct Scene {
    pub meshes:     *const *const Mesh,
    pub num_meshes: usize
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

    let scene_box = Box::new(Scene {
        meshes: meshes.as_ptr(),
        num_meshes: meshes.len()
    });

    std::mem::forget(meshes);

    let scene_ptr = Box::into_raw(scene_box);
    *scene = scene_ptr;
}