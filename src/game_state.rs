use cgmath::Vector3;
use glutin::display::{Display};
use crate::camera::Camera;
use crate::game_object::GameObject;
use crate::renderer::Renderer;

pub struct GameState {
    game_objects: Vec<Box<GameObject>>,
    renderer: Renderer,
    camera: Camera
}

impl GameState {

    pub fn initialize(gl_display: &Display) -> Self {
        return Self {
            game_objects: vec![],
            renderer: Renderer::new(gl_display),
            // TODO: Camera location should be calculated based on player location
            camera: Camera::initialize(Vector3::new(0.0, 0.0, 0.0))
        }
    }

    pub fn add_game_object(mut self, game_object: Box<GameObject>) {
        // TODO: Start game objects before pushing to collection?
        self.game_objects.push(game_object);
    }

    pub fn start(self) {
        for game_object in self.game_objects {
            game_object.start();
            // TODO: Add game object to renderer
        }
    }

    fn update_game_state(self) {

    }

}