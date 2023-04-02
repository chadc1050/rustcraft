use crate::texture::Texture;

#[derive(Clone, Copy)]
pub struct Sprite<'a> {
    texture: &'a Texture,
    width: f32,
    height: f32
}