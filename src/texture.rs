use std::path::{PathBuf};

#[derive(Clone)]
pub struct Texture {
    textureId: u16,
    filePath: PathBuf,
}