use std::time::Duration;
use cgmath::Vector4;
use crate::component::Component;
use crate::sprite::Sprite;

#[derive(Clone, Copy)]
struct SpriteRenderer<'a> {
    color: Vector4<f32>,
    sprite: Sprite<'a>
}

impl<'a> SpriteRenderer<'a> {

    pub fn new(color: Vector4<f32>, sprite: Sprite<'a>) -> Self {
        return Self {
            color,
            sprite
        }
    }
}

impl<'a> Component for SpriteRenderer<'a> {
    fn start(self) {
        todo!()
    }

    fn stop(self) {
        todo!()
    }

    fn update(self, dt: Duration) {
        todo!()
    }
}