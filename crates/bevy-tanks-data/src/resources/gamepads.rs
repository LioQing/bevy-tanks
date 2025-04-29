use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Gamepads(pub Vec<Entity>);
