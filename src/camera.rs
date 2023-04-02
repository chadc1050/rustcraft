use cgmath::{Matrix4, Vector3};

pub struct Camera {
    position: Vector3<f32>,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    inverse_projection: Matrix4<f32>,
    inverse_view: Matrix4<f32>
}

impl Camera {

    pub fn initialize(position: Vector3<f32>) -> Self {
        todo!()
    }
}