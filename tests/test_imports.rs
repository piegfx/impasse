use impasse::importers::Importer;

#[test]
fn test_gltf() {
    let gltf = impasse::importers::gltf::Gltf::import(&std::fs::read("/home/ollie/Downloads/Cubebs/IMyDefaultCube2GLTFseparate.gltf").unwrap());

    println!("{:#?}", gltf);
}