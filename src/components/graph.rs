use bevy::prelude::Component;

use super::segment::Segment;

#[derive(Component)]
pub struct Graph {
    pub segments: Vec<Segment>,
}

impl Graph {
    pub fn new() -> Graph {
        return Graph {
            segments: Vec::new(),
        };
    }
}
