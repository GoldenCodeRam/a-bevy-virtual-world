#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use rand::{seq::IteratorRandom, Rng};

use crate::{
    components::{
        components::{Graph, Point, Segment},
        markers::ButtonType,
    },
    events::events::{
        ButtonClickEvent, CreateRandomSegmentEvent, RemoveRandomPointEvent,
        RemoveRandomSegmentEvent, RemoveAllEvent,
    },
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.333),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonType::AddRandomPoint,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: (asset_server.load("fonts/FiraSans-Bold.ttf")),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.333),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonType::AddRandomSegment,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Segment",
                        TextStyle {
                            font: (asset_server.load("fonts/FiraSans-Bold.ttf")),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.333),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonType::RemoveAll,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Remove All",
                        TextStyle {
                            font: (asset_server.load("fonts/FiraSans-Bold.ttf")),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonType::RemoveRandomSegment,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "RR Segment",
                        TextStyle {
                            font: (asset_server.load("fonts/FiraSans-Bold.ttf")),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonType::RemoveRandomPoint,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "RR Point",
                        TextStyle {
                            font: (asset_server.load("fonts/FiraSans-Bold.ttf")),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });

    let p1 = commands.spawn(Point(Vec2 { x: 1.0, y: 1.0 })).id();
    let p2 = commands.spawn(Point(Vec2 { x: 300.0, y: 200.0 })).id();
    let p3 = commands.spawn(Point(Vec2 { x: 400.0, y: 100.0 })).id();
    let p4 = commands.spawn(Point(Vec2 { x: 100.0, y: 300.0 })).id();

    let s1 = commands.spawn(Segment(p1, p2)).id();
    let s2 = commands.spawn(Segment(p3, p2)).id();
    let s3 = commands.spawn(Segment(p4, p1)).id();

    let graph = Graph::new(vec![p1, p2, p3, p4].into(), vec![s1, s2, s3].into());
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

        gizmos.line_2d(p1.unwrap().1 .0, p2.unwrap().1 .0, Color::BLACK);
    }

    for point in point_query.iter() {
        point.1.draw(&mut gizmos);
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ButtonType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut event_button_click: EventWriter<ButtonClickEvent>,
    mut add_random_segment_event: EventWriter<CreateRandomSegmentEvent>,
    mut remove_random_segment_event: EventWriter<RemoveRandomSegmentEvent>,
    mut remove_random_point_event: EventWriter<RemoveRandomPointEvent>,
    mut remove_all_event: EventWriter<RemoveAllEvent>,
) {
    for (interaction, mut color, mut border_color, children, button_type) in &mut interaction_query
    {
        match button_type {
            ButtonType::AddRandomPoint => {
                let mut text = text_query.get_mut(children[0]).unwrap();
                match *interaction {
                    Interaction::Pressed => {
                        text.sections[0].value = "Press".to_string();
                        *color = PRESSED_BUTTON.into();
                        border_color.0 = Color::WHITE;
                        event_button_click.send(ButtonClickEvent {})
                    }
                    Interaction::Hovered => {
                        text.sections[0].value = "Hover".to_string();
                        *color = HOVERED_BUTTON.into();
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        text.sections[0].value = "Button".to_string();
                        *color = NORMAL_BUTTON.into();
                        border_color.0 = Color::BLACK;
                    }
                }
            }
            ButtonType::RemoveAll => match *interaction {
                Interaction::Pressed => {
                    remove_all_event.send(RemoveAllEvent {});
                }
                _ => (),
            },
            ButtonType::RemoveRandomPoint => match *interaction {
                Interaction::Pressed => {
                    remove_random_point_event.send(RemoveRandomPointEvent {});
                }
                _ => (),
            },
            ButtonType::RemoveRandomSegment => match *interaction {
                Interaction::Pressed => {
                    remove_random_segment_event.send(RemoveRandomSegmentEvent {});
                }
                // This is not correct! But for the moment we'll leave it like
                // that
                _ => (),
            },
            ButtonType::AddRandomSegment => match *interaction {
                Interaction::Pressed => {
                    add_random_segment_event.send(CreateRandomSegmentEvent {});
                }
                // This is not correct! But for the moment we'll leave it like
                // that
                _ => (),
            },
        }
    }
}

pub fn debug_button_click(
    mut event: EventReader<ButtonClickEvent>,
    mut commands: Commands,
    mut graph_query: Query<&mut Graph>,
    point_query: Query<&Point>,
    window_query: Query<&Window>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    for _ev in event.iter() {
        let mut rng = rand::thread_rng();
        let window = window_query.single();

        let new_point = Point(window_to_world(
            Vec2::new(
                rng.gen::<f32>() * window.width(),
                rng.gen::<f32>() * window.height(),
            ),
            &window,
            &camera_query.single(),
        ));

        if point_query.iter().find(|p| **p == new_point).is_none() {
            graph_query
                .single_mut()
                .add_point_entity(commands.spawn(new_point).id());
        }
    }
}

pub fn create_random_segment(
    mut event: EventReader<CreateRandomSegmentEvent>,
    mut commands: Commands,
    mut graph_query: Query<&mut Graph>,
    point_query: Query<(Entity, &Point)>,
    segment_query: Query<&Segment>,
) {
    for _ev in event.iter() {
        let mut rng = rand::thread_rng();
        let p1 = point_query.iter().choose(&mut rng).unwrap();
        let p2 = point_query
            .iter()
            .filter(|p| *p.1 != *p1.1)
            .choose(&mut rng)
            .unwrap();

        let new_segment = Segment(p1.0, p2.0);

        if segment_query.iter().find(|s| **s == new_segment).is_none() {
            graph_query
                .single_mut()
                .add_segment_entity(commands.spawn(new_segment).id());
        }
    }
}

pub fn remove_random_point(
    mut event: EventReader<RemoveRandomPointEvent>,
    mut commands: Commands,
    mut graph_query: Query<&mut Graph>,
    point_query: Query<Entity, With<Point>>,
    segment_query: Query<(Entity, &Segment)>,
) {
    for _ev in event.iter() {
        let mut rng = rand::thread_rng();
        let point_entity = point_query.iter().choose(&mut rng);

        match point_entity {
            Some(p) => {
                let mut graph = graph_query.single_mut();
                // Remove segments associated with that point.
                segment_query
                    .iter()
                    .filter(|s| s.1.includes_point(&p))
                    .for_each(|s| {
                        graph.remove_segment_entity(s.0);
                        commands.entity(s.0).despawn();
                    });
                // Remove point.
                graph.remove_point_entity(p);
                commands.entity(p).despawn();

                eprintln!("segments: {:?}", graph.segments.len());
                eprintln!("points: {:?}", graph.points.len());
            }
            None => (),
        }
    }
}

pub fn remove_random_segment(
    mut event: EventReader<RemoveRandomSegmentEvent>,
    mut commands: Commands,
    mut graph_query: Query<&mut Graph>,
    segment_query: Query<Entity, With<Segment>>,
) {
    for _ev in event.iter() {
        let mut rng = rand::thread_rng();
        let segment_entity = segment_query.iter().choose(&mut rng);

        match segment_entity {
            Some(s) => {
                graph_query.single_mut().remove_segment_entity(s);
                commands.entity(s).despawn();
            }
            None => (),
        }
        eprintln!("segments: {:?}", graph_query.single_mut().segments.len());
    }
}

pub fn remove_all(
    mut event: EventReader<RemoveAllEvent>,
    mut commands: Commands,
    mut graph_query: Query<&mut Graph>,
    point_query: Query<Entity, With<Point>>,
    segment_query: Query<Entity, With<Segment>>,
) {
    for _ev in event.iter() {
        let mut graph = graph_query.single_mut();
        point_query.for_each(|p| {
            graph.remove_point_entity(p);
            commands.entity(p).despawn();
        });
        segment_query.for_each(|s| {
            graph.remove_segment_entity(s);
            commands.entity(s).despawn();
        });
    }
}

fn window_to_world(position: Vec2, window: &Window, camera: &Transform) -> Vec2 {
    let norm = Vec3::new(
        position.x - window.width() / 2.0,
        position.y - window.height() / 2.0,
        0.0,
    );

    let camera_transform = *camera * norm;
    return Vec2::new(camera_transform.x, camera_transform.y);
}

trait DrawGizmos {
    fn draw(&self, gizmos: &mut Gizmos);
}

impl DrawGizmos for Point {
    fn draw(&self, gizmos: &mut Gizmos) {
        gizmos.circle_2d(self.0, 3.0, Color::WHITE);
    }
}
