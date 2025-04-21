use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rand::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: match cfg!(debug_assertions) {
                    true => Level::DEBUG,
                    false => Level::INFO,
                },
                ..default()
            }),
            EntropyPlugin::<WyRand>::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            TnuaRapier3dPlugin::default(),
            TnuaControllerPlugin::default(),
            bevy_tanks_systems::plugin,
            #[cfg(debug_assertions)]
            debug_plugins,
        ))
        .run()
}

fn debug_plugins(app: &mut App) {
    app.add_plugins((
        WorldInspectorPlugin::new(),
        RapierDebugRenderPlugin::default(),
    ));
}
