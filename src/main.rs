use bevy::prelude::*;
use systems::setup::{startup, update};

mod components;
mod entities;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}
