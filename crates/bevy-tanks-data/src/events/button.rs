use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub enum GameOverUiButtonEvent {
    PlayAgain,
    Quit,
}
