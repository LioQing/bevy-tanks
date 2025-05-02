use bevy::prelude::*;
use bevy_tanks_data::*;

mod common;
mod game;
mod menu;

pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .add_plugins((common::plugin, menu::plugin, game::plugin));
}
