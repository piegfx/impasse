use impasse::importers::Importer;

mod framework;

#[test]
fn test_gltf() {
    let gltf = impasse::importers::gltf::Gltf::import("/home/ollie/Documents/Cubebs/IMyDefaultCube2GLB.glb");
    println!("{:#?}", gltf.unwrap().images);
}