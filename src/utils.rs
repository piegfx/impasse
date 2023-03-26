pub enum TextureChannel {
    Red,
    Green,
    Blue,
    Alpha
}

/// Extract the given texture channel from the given texture array,
/// and converts it into an RGBA texture.
pub fn extract_texture<T>(texture: Vec<T>, channel: TextureChannel) -> Vec<T> {
    
}