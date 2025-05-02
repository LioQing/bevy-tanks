use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_rand::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::*;
use oxidized_navigation::{
    NavMeshSettings, OxidizedNavigationPlugin, colliders::rapier::RapierCollider,
};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: match cfg!(debug_assertions) {
                        true => Level::INFO,
                        false => Level::WARN,
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    #[cfg(target_arch = "wasm32")]
                    meta_check: bevy::asset::AssetMetaCheck::Never, // https://github.com/bevyengine/bevy/issues/10157
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Tanks".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
            EnhancedInputPlugin,
            EntropyPlugin::<WyRand>::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            TnuaRapier3dPlugin::default(),
            TnuaControllerPlugin::default(),
            OxidizedNavigationPlugin::<RapierCollider>::new(
                NavMeshSettings::from_agent_and_bounds(1.2, 1.5, 20.0, 0.0),
            ),
            bevy_tanks_systems::plugin,
            #[cfg(debug_assertions)]
            debug_plugins,
        ))
        .run()
}

#[cfg(debug_assertions)]
fn debug_plugins(app: &mut App) {
    use bevy_inspector_egui::quick::WorldInspectorPlugin;
    use oxidized_navigation::debug_draw::OxidizedNavigationDebugDrawPlugin;

    app.add_plugins((
        WorldInspectorPlugin::new(),
        RapierDebugRenderPlugin::default(),
        OxidizedNavigationDebugDrawPlugin,
    ));
}
