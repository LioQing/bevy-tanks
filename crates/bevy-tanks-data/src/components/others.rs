use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Debug, Clone, Component, InputContext)]
pub struct PlayerInputs {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub fire: KeyCode,
    pub index: usize, // For gamepad
}

#[derive(Debug, Component)]
pub struct EventEmitter<E: Event>(pub E);
