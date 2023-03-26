#[test]
pub fn test_scene() {
    let scene = impasse::Scene::from_gltf("/home/ollie/Downloads/ionthrusterconcept01.gltf").unwrap();

    println!("{:#?}", scene);
}