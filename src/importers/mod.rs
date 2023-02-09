use std::io;

pub mod gltf;

pub trait Importer {
    // TODO: Custom importer error.
    fn import(data: &[u8]) -> Result<Self, io::Error> where Self: Sized;
}