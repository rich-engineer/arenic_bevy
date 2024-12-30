use bevy::prelude::Component;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component, Clone)]
pub struct Description(pub String);