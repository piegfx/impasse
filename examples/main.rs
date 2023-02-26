use framework::Color;
use impasse::Scene;

mod framework;

struct Demo {
    scene: Scene
}

impl framework::Demo for Demo {
    fn init(&mut self, framework: &mut framework::Framework) {
        framework.clear_color = Color::from_rgba_u8(100, 149, 237, 255);
    }

    fn update(&mut self, framework: &mut framework::Framework) {
        framework.update();
    }

    fn draw(&mut self, framework: &mut framework::Framework) {
        framework.draw();
    }
}

fn main() {
    let mut demo = Demo {
        scene: Scene::from_gltf("/home/ollie/Documents/Cubebs/IMyDefaultCube2GLTFseparate.gltf").unwrap()
    };

    let mut runner = framework::FrameworkRunner::new(&mut demo);
    runner.run();
}