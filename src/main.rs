use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(hello_world_system)
        .add_system_set(SystemSet::new())
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn hello_world_system() {
   println!("hello world");
}
