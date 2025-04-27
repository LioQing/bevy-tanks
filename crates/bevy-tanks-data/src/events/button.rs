use bevy::prelude::*;

#[derive(Debug, Clone, Event)]
pub enum MainMenuUiButtonEvent {
    Play,
    Quit,
}

#[derive(Debug, Clone, Event)]
pub enum GameOverUiButtonEvent {
    PlayAgain,
    MainMenu,
}
