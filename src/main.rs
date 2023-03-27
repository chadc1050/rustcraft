use std::time::SystemTime;
use crate::asset_pool::AssetPool;
use crate::window::Window;

mod window;
mod keyboard;
mod zip_reader;
mod asset_pool;

fn main() {
    println!("Starting Rustcraft!");

    let pool_load_time = SystemTime::now();
    let pool = AssetPool::initialize_pool();
    println!("Asset pool took {} secs to load!", pool_load_time.elapsed().unwrap().as_secs_f32());

    futures_executor::block_on(Window::new(1920, 1080, String::from("Rustcraft")).run());
}