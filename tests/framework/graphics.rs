use super::pie::{GraphicsDevice, GraphicsBuffer};

pub struct Graphics {
    pub device: GraphicsDevice
}

impl Graphics {
    pub fn new() -> Self {
        let device = GraphicsDevice::new();

        Self {
            device
        }
    }
}

pub struct Renderable {
    pub vertex_buffer: GraphicsBuffer,
    pub index_buffer:  GraphicsBuffer,
}