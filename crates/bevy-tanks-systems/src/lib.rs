use bevy::prelude::*;
use bevy_tanks_data::*;
use bevy_tnua::prelude::*;

mod bullet;
mod explosion;
mod others;
mod player;
mod smoke_vfx;
mod tank;
mod ui;

pub fn plugin(app: &mut App) {
    app.init_resource::<Typography>().add_plugins(game::plugin);
}

mod game {
    use super::*;

    pub fn plugin(app: &mut App) {
        app.init_resource::<TankColors>()
            .init_resource::<TankAssets>()
            .init_resource::<BulletAssets>()
            .init_resource::<TankExplosionAssets>()
            .init_resource::<SmokeVfxParticleAssets>()
            .add_event::<GameOverUiButtonEvent>()
            .init_state::<GameState>()
            .add_observer(tank::observe_scene_instance_ready)
            .add_systems(
                Startup,
                (
                    others::setup_camera,
                    others::setup_floor,
                    others::setup_light,
                ),
            )
            .add_systems(
                Update,
                (
                    tank::update.in_set(TnuaUserControlsSystemSet),
                    tank::update_smoke.after(TnuaUserControlsSystemSet),
                    bullet::update,
                    bullet::handle_tank_collision,
                    explosion::update,
                    smoke_vfx::update,
                    smoke_vfx::update_particle,
                    // Playing
                    player::update
                        .before(tank::update)
                        .run_if(in_state(GameState::Playing)),
                    // Over
                    (
                        ui::update_interaction::<GameOverUiButtonEvent>,
                        ui::handle_game_over_button,
                    )
                        .run_if(in_state(GameState::Over)),
                ),
            )
            .add_systems(OnEnter(GameState::Playing), player::setup)
            .add_systems(OnExit(GameState::Playing), player::handle_game_playing_exit)
            .add_systems(OnEnter(GameState::Over), ui::handle_game_over_enter)
            .add_systems(OnExit(GameState::Over), cleanup::<TankController>)
            .enable_state_scoped_entities::<GameState>();
    }
}

fn cleanup<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
