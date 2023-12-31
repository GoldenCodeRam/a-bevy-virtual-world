use bevy::{prelude::*, winit::WinitSettings};
use events::events::{
    ButtonClickEvent, CreateRandomSegmentEvent, RemoveRandomPointEvent, RemoveRandomSegmentEvent, RemoveAllEvent,
};
use systems::setup::*;

mod components;
mod events;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        // Events
        .add_event::<ButtonClickEvent>()
        .add_event::<CreateRandomSegmentEvent>()
        .add_event::<RemoveRandomSegmentEvent>()
        .add_event::<RemoveRandomPointEvent>()
        .add_event::<RemoveAllEvent>()
        // Systems
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .add_systems(Update, button_system)
        // Event listeners
        .add_systems(Update, debug_button_click)
        .add_systems(Update, create_random_segment)
        .add_systems(Update, remove_random_segment)
        .add_systems(Update, remove_random_point)
        .add_systems(Update, remove_all)
        .run();
}
