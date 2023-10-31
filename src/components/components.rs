use bevy::prelude::{Component, Entity, Vec2};

#[derive(Debug, Component, Clone)]
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
        return *&self.includes_point(&other.0) && *&self.includes_point(&other.1);
    }
}

impl Segment {
    fn includes_point(&self, point: &Entity) -> bool {
        return &self.0 == point || &self.1 == point;
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

    pub fn remove_segment_entity(&mut self, entity: Entity) {
        let index = self.segments.iter().position(|e| *e == entity).unwrap();
        self.segments.remove(index);
    }
}
