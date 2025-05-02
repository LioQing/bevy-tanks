use bevy::prelude::*;
use bevy_tanks_data::*;

pub mod ui;

pub fn plugin(app: &mut App) {
    app.init_resource::<Typography>()
        .init_resource::<GameMode>();
}

pub fn cleanup<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
