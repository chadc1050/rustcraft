use crate::window::Window;

mod window;
mod keyboard;
mod renderer;
mod batch_renderer;

fn main() {
    println!("Starting Rustcraft!");
    futures_executor::block_on(Window::new(1920, 1080, String::from("Rustcraft")).run());
}