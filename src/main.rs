use std::time::SystemTime;
use crate::asset_pool::AssetPool;
use crate::window::Window;

mod window;
mod keyboard;
mod zip_reader;
mod asset_pool;
mod renderer;
mod game_object;
mod component;
mod sprite_renderer;
mod sprite;
mod texture;
mod game_state;
mod camera;

fn main() {
    println!("Starting Rustcraft!");

    let pool_load_time = SystemTime::now();
    AssetPool::initialize_pool();

    println!("Asset pool took {} secs to load!", pool_load_time.elapsed().unwrap().as_secs_f32());

    Window::new(1920, 1080, String::from("Rustcraft")).run();
}