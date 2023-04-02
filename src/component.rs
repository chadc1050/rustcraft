use std::time::Duration;

pub trait Component {
    fn start(&self);
    fn stop(&self);
    fn update(&self, dt: Duration);
}