use std::time::Duration;
use cgmath::Vector3;

use crate::component::Component;

pub struct GameObject {
    id: String,
    components: Vec<Box<dyn Component>>,
    position: Vector3<f32>,
}

impl GameObject {

    pub fn new(id: String, components: Vec<Box<dyn Component>>, position: Vector3<f32>) -> Self {
        return Self {
            id,
            components,
            position,
        }
    }

    pub fn start(self: Box<Self>) {
        for component in self.components.into_iter() {
            component.start();
        }
    }

    pub fn stop(self: Box<Self>) {
        for component in self.components.into_iter() {
            component.stop();
        }
    }

    pub fn update(self: Box<Self>, dt: Duration) {
        for component in self.components.into_iter() {
            component.update(dt);
        }
    }
}

