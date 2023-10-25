use bevy::{prelude::*, winit::WinitSettings};
use events::events::ButtonClickEvent;
use systems::setup::*;

mod components;
mod events;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_event::<ButtonClickEvent>()
        .add_systems(Startup, startup)
        .add_systems(Update, (update, button_system, debug_button_click))
        .run();
}
