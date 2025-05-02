use bevy::prelude::*;
use bevy_rand::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiState {
    Idle { timer: Timer },
    Attacking { fire_count: usize, movement: bool },
    Moving,
}

#[derive(Debug, Clone, Component)]
#[require(Entropy<WyRand>)]
pub struct AiInputs {
    pub path: VecDeque<Vec3>,
    pub state: AiState,
}

impl Default for AiInputs {
    fn default() -> Self {
        Self {
            path: VecDeque::new(),
            state: AiState::Idle {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            },
        }
    }
}
