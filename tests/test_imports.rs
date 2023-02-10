use framework::{pie::{GraphicsBuffer, Shader, InputLayout, BufferType, ShaderAttachment, ShaderStage, InputLayoutDescription, Format}, VertexPositionColor, Color};
use impasse::{importers::Importer, Vec3};

mod framework;

pub struct GltfDemo {
    pub v_buffer: Option<GraphicsBuffer>,
    pub i_buffer: Option<GraphicsBuffer>,
    pub shader:   Option<Shader>,
    pub layout:   Option<InputLayout>
}

impl framework::Demo for GltfDemo {
    fn init(&mut self, framework: &mut framework::Framework) {
        framework.clear_color = framework::Color::from_rgba_u8(100, 149, 237, 255);

        let device = &mut framework.graphics.device;

        let vertices = [
            VertexPositionColor { position: Vec3(0.0, 0.5, 0.0), color: Color::from_rgba_f32(1.0, 0.0, 0.0, 1.0) },
            VertexPositionColor { position: Vec3(0.5, -0.5, 0.0), color: Color::from_rgba_f32(0.0, 1.0, 0.0, 1.0) },
            VertexPositionColor { position: Vec3(-0.5, -0.5, 0.0), color: Color::from_rgba_f32(0.0, 0.0, 1.0, 1.0) }
        ];

        let indices = [
            0, 1, 2
        ];

        self.v_buffer = Some(device.create_buffer(BufferType::VertexBuffer, &vertices, false));
        self.i_buffer = Some(device.create_buffer(BufferType::IndexBuffer, &indices, false));

        let v_shader = "#version 330 core
        
        layout (location = 0) in vec3 aPosition;
        layout (location = 1) in vec4 aColor;
        
        out vec4 frag_color;
        
        void main() {
            gl_Position = vec4(aPosition, 1.0);
            frag_color = aColor;
        }";

        let f_shader = "#version 330 core
        
        in vec4 frag_color;
        
        out vec4 out_color;
        
        void main() {
            out_color = frag_color;   
        }";

        let attachments = [
            ShaderAttachment { stage: ShaderStage::Vertex, source: v_shader },
            ShaderAttachment { stage: ShaderStage::Fragment, source: f_shader }
        ];

        self.shader = Some(device.create_shader(&attachments).unwrap());
        
        let descriptions = [
            InputLayoutDescription { format: Format::R32G32B32Float, offset: 0 },
            InputLayoutDescription { format: Format::R32G32B32A32Float, offset: 12 },
        ];

        self.layout = Some(device.create_input_layout(&descriptions));
    }

    fn update(&mut self, framework: &mut framework::Framework) {
        framework.update();
    }

    fn draw(&mut self, framework: &mut framework::Framework) {
        framework.draw();
        
        let device = &mut framework.graphics.device;

        let v_buffer = self.v_buffer.as_ref().unwrap();
        let i_buffer = self.i_buffer.as_ref().unwrap();
        let shader = self.shader.as_ref().unwrap();
        let layout = self.layout.as_ref().unwrap();
        device.set_vertex_buffer(v_buffer, 28, layout);
        device.set_index_buffer(i_buffer);
        device.set_shader(shader);
        device.draw_indexed(3);
    }
}

#[test]
fn test_gltf() {
    let gltf = impasse::importers::gltf::Gltf::import(&std::fs::read("/home/ollie/Downloads/Cubebs/IMyDefaultCube2GLTFseparate.gltf").unwrap());
    println!("{:#?}", gltf);

    let mut demo = GltfDemo { v_buffer: None, i_buffer: None, shader: None, layout: None  };

    let mut runner = framework::FrameworkRunner::new(&mut demo);
    runner.run();
}