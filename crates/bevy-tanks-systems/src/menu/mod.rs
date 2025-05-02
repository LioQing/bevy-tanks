use bevy::prelude::*;
use bevy_tanks_data::*;

mod how_to_play;
mod main;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<MenuState>()
        .add_systems(OnEnter(AppState::Menu), setup)
        .add_plugins((main::plugin, how_to_play::plugin));
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), StateScoped(AppState::Menu)));
}
