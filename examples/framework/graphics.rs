use super::{pie::{GraphicsDevice, GraphicsBuffer}, renderer::Renderer};

pub struct Graphics {
    pub device: GraphicsDevice,
    pub renderer: Renderer
}

impl Graphics {
    pub fn new() -> Self {
        let device = GraphicsDevice::new();

        Self {
            device,
            renderer: Renderer::new()
        }
    }
}

pub struct Renderable {
    pub vertex_buffer: GraphicsBuffer,
    pub index_buffer:  GraphicsBuffer,
}