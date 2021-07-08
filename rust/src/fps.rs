use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Label)]

pub struct FPS {}

#[methods]
impl FPS {
    fn new(_owner: &Label) -> Self {
        FPS {}
    }
    #[export]
    unsafe fn _ready(&mut self, _owner: &Label) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Constructed Label ");
    }

    #[export]
    unsafe fn _process(&self, _owner: &Label, _delta: f64) {
        _owner.set_text(
            Engine::godot_singleton()
                .get_frames_per_second()
                .to_string(),
        );
    }
}
