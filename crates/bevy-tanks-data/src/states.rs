use bevy::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
    Over,
}
