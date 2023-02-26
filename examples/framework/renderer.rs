use impasse::Mat4;

use super::{pie::{GraphicsDevice, Shader}, graphics::Renderable};

const SHADER_VERTEX: &str = "
#version 430

layout (location = 0) in vec3 aPosition;

void main() {
    gl_Position = aPosition;
}
";

const SHADER_FRAGMENT: &str = "
#version 430

layout (location = 0) out vec4 out_color;

void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}
";

struct TransformedRenderable {
    pub renderable: Renderable,
    pub transform:  Mat4
}

pub struct Renderer {
    shader:  Shader,

    opaques: Vec<TransformedRenderable>
}

impl Renderer {
    pub fn new(device: &GraphicsDevice) -> Self {


        Self {
            opaques: Vec::new()
        }
    }

    pub fn render(device: &GraphicsDevice) {

    }
}