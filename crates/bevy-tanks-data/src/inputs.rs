use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct Fire;
