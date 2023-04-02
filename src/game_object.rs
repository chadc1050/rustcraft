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

    pub fn start(self) {
        for component in self.components {
            component.start();
        }
    }

    pub fn stop(self) {
        for component in self.components {
            component.stop();
        }
    }

    pub fn update(self, dt: Duration) {
        for component in self.components {
            component.update(dt);
        }
    }
}

