mod bloo;
mod bullet;
mod cheese;
mod entity;
mod enums;
mod fps;
mod game_manager;
mod runtime_data;
mod savage;
mod traits;
//use gdnative::prelude::{godot_init, InitHandle};
use gdnative::prelude::*;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<bloo::Bloo>();
    handle.add_class::<game_manager::GameManager>();
    handle.add_class::<fps::FPS>();
    handle.add_class::<entity::Entity>();
    handle.add_class::<cheese::Cheese>();
    handle.add_class::<bullet::Bullet>();
    handle.add_class::<savage::Savage>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
