use super::pie::GraphicsDevice;

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