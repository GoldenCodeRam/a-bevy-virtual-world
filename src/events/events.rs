use bevy::prelude::Event;

#[derive(Event)] 
#[derive(Debug)]
pub struct ButtonClickEvent;

#[derive(Event)]
pub struct CreateRandomSegmentEvent;
