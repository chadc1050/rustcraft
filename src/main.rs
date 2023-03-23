use crate::window::Window;

mod window;
mod keyboard;
mod zip_reader;
mod asset_deserializer;
mod asset_pool;

fn main() {
    println!("Starting Rustcraft!");
    futures_executor::block_on(Window::new(1920, 1080, String::from("Rustcraft")).run());
}