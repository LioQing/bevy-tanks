use bevy::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::Menu)]
pub enum MenuState {
    #[default]
    Main,
    HowToPlay,
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
