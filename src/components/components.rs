use bevy::prelude::{Component, Vec2, Entity};

#[derive(Component)]
pub struct Point(pub Vec2);

#[derive(Component)]
pub struct Segment(pub Entity, pub Entity);

#[derive(Component)]
pub struct Graph {
    pub points: Vec<Entity>,
    pub segments: Vec<Entity>,
}

impl Graph {
    pub fn new(points: Option<Vec<Entity>>, segments: Option<Vec<Entity>>) -> Graph {
        return Graph {
            points: points.unwrap_or(Vec::new()),
            segments: segments.unwrap_or(Vec::new()),
        };
    }

    pub fn add_point(&mut self, entity: Entity) {
        self.points.push(entity);
    }
}
