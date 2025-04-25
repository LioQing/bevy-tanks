use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct PlayerInputs {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub fire: KeyCode,
}

#[derive(Debug, Component)]
pub struct EventEmitter<E: Event>(pub E);
