use super::Color;

pub struct GraphicsDevice {

}

impl GraphicsDevice {
    pub fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}