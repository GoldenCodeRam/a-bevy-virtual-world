use bevy::prelude::{Component, Entity, Vec2};

#[derive(Component)]
pub struct Point(pub Vec2);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

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

    pub fn add_point_entity(&mut self, entity: Entity) {
        self.points.push(entity);
    }
}
