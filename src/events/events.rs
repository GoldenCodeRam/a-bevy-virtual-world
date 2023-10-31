use bevy::prelude::Event;

#[derive(Event)] 
#[derive(Debug)]
pub struct ButtonClickEvent;

#[derive(Event)]
pub struct CreateRandomSegmentEvent;

#[derive(Event)]
pub struct RemoveRandomSegmentEvent;

#[derive(Event)]
pub struct RemoveRandomPointEvent;

#[derive(Event)]
pub struct RemoveAllEvent;

