use bevy::prelude::Component;

#[derive(Component)]
pub enum ButtonType {
    AddRandomPoint,
    AddRandomSegment,
    RemoveRandomSegment,
}
