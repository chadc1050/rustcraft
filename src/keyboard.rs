use winit::event::KeyboardInput;

pub struct KeyboardEvent {
    input: KeyboardInput,
}

impl KeyboardEvent {
    pub fn new(input: KeyboardInput) -> Self {
        return Self {
            input
        };
    }

    pub fn handle_event(self) {
        // Prints key scan code
        println!("{}", self.input.scancode)
    }
}
