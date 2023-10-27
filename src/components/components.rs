use bevy::prelude::{Component, Entity, Vec2};

#[derive(Debug)]
#[derive(Component)]
#[derive(Clone)]
pub struct Point(pub Vec2);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

#[derive(Component)]
pub struct Segment(pub Entity, pub Entity);

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0 && self.1 == other.1;
    }
}

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

    pub fn add_segment_entity(&mut self, entity: Entity) {
        self.segments.push(entity);
    }
}
