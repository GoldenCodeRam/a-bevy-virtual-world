use bevy::prelude::*;

use crate::components::components::{Graph, Point, Segment};

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let p1 = commands.spawn(Point(Vec2 { x: 200.0, y: 200.0 })).id();
    let p2 = commands.spawn(Point(Vec2 { x: 300.0, y: 200.0 })).id();
    let p3 = commands.spawn(Point(Vec2 { x: 400.0, y: 100.0 })).id();
    let p4 = commands.spawn(Point(Vec2 { x: 100.0, y: 300.0 })).id();

    let s1 = commands.spawn(Segment(p1, p2)).id();
    let s2 = commands.spawn(Segment(p3, p2)).id();
    let s3 = commands.spawn(Segment(p4, p1)).id();

    let graph = Graph::new(
        vec![p1, p2, p3, p4].into(),
        vec![s1, s2, s3].into(),
    );
    commands.spawn(graph);
}

pub fn update(
    mut gizmos: Gizmos,
    segment_query: Query<&Segment>,
    point_query: Query<(Entity, &Point)>,
) {
    for segment in segment_query.iter() {
        let p1 = point_query.iter().find(|p| p.0 == segment.0);
        let p2 = point_query.iter().find(|p| p.0 == segment.1);

        if p1.is_none() || p2.is_none() {
            continue;
        }

        gizmos.line_2d(p1.unwrap().1.0, p2.unwrap().1.0, Color::BLACK);
        p1.unwrap().1.draw(&mut gizmos);
        p2.unwrap().1.draw(&mut gizmos);
    }
}

trait DrawGizmos {
    fn draw(&self, gizmos: &mut Gizmos);
}

impl DrawGizmos for Point {
    fn draw(&self, gizmos: &mut Gizmos) {
        gizmos.circle_2d(self.0, 3.0, Color::WHITE);
    }
}
