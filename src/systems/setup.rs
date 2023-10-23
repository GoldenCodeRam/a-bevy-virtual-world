use bevy::prelude::*;

use crate::components::{graph::Graph, segment::Segment};

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let mut graph = Graph::new();

    graph.segments.push(Segment{
        p1: Vec2 { x: 100.0, y: 100.0 },
        p2: Vec2 { x: 200.0, y: 200.0 },
    });
    commands.spawn(graph);
}

pub fn update(time: Res<Time>, mut gizmos: Gizmos, mut graph: Query<&Graph>) {
    let result = &graph.get_single();
    if result.is_ok() {
        for segment in result.as_ref().unwrap().segments.iter() {
            println!("{} {}", segment.p1, segment.p2);
            gizmos.line_2d(segment.p1, segment.p2, Color::GREEN);
        }
    }
}
