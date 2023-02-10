use impasse::importers::Importer;

mod framework;

pub struct GltfDemo;

impl framework::Demo for GltfDemo {
    fn init(&mut self, framework: &mut framework::Framework) {
        framework.clear_color = framework::Color::from_rgba_u8(100, 149, 237, 255);
    }

    fn update(&mut self, framework: &mut framework::Framework) {
        framework.update();
    }

    fn draw(&mut self, framework: &mut framework::Framework) {
        framework.draw();
    }
}

#[test]
fn test_gltf() {
    let gltf = impasse::importers::gltf::Gltf::import(&std::fs::read("/home/ollie/Downloads/Cubebs/IMyDefaultCube2GLTFseparate.gltf").unwrap());
    println!("{:#?}", gltf);

    let mut demo = GltfDemo {  };

    let mut runner = framework::FrameworkRunner::new(&mut demo);
    runner.run();
}