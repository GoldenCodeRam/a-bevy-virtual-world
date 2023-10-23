use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Segment {
    pub p1: Vec2,
    pub p2: Vec2,
}
