use bevy::prelude::*;
use bevy_tanks_data::*;

mod common;
mod game;
mod main_menu;

pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .init_resource::<Typography>()
        .add_plugins((main_menu::plugin, game::plugin));
}
