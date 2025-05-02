use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub enum MainMenuUiButtonEvent {
    PlaySinglePlayer,
    PlayMultiplayer,
    Quit,
}

#[derive(Debug, Clone, Event)]
pub enum PausedUiButtonEvent {
    Resume,
    MainMenu,
}

#[derive(Debug, Clone, Event)]
pub enum GameOverUiButtonEvent {
    PlayAgain,
    MainMenu,
}
