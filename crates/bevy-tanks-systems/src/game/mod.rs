use bevy::prelude::*;
use bevy_tanks_data::*;
use bevy_tnua::prelude::*;

use crate::common::{self, ui::button};

mod bullet;
mod explosion;
mod others;
mod player;
mod smoke_vfx;
mod tank;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_state_scoped_event::<GameOverUiButtonEvent>(GameState::Over)
        .add_observer(tank::observe_scene_instance_ready)
        .add_observer(ui::observe_game_over_button)
        .add_systems(
            OnEnter(AppState::Game),
            (
                setup,
                others::setup_camera,
                others::setup_floor,
                others::setup_light,
                smoke_vfx::setup,
            ),
        )
        .add_systems(
            Update,
            (
                tank::update.in_set(TnuaUserControlsSystemSet),
                tank::update_smoke.after(TnuaUserControlsSystemSet),
                bullet::update,
                bullet::update_tank_collision_events,
                explosion::update,
                smoke_vfx::update,
                smoke_vfx::update_particle,
                // Playing
                player::update
                    .before(tank::update)
                    .run_if(in_state(GameState::Playing)),
                // Over
                button::update_interaction::<GameOverUiButtonEvent>
                    .run_if(in_state(GameState::Over)),
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(OnEnter(GameState::Playing), player::setup)
        .add_systems(OnExit(GameState::Playing), player::cleanup_game_playing)
        .add_systems(OnEnter(GameState::Over), ui::setup_game_over)
        .add_systems(OnExit(GameState::Over), common::cleanup::<TankController>)
        .add_systems(OnExit(AppState::Game), cleanup);
}

fn setup(mut commands: Commands) {
    commands.init_resource::<TankColors>();
    commands.init_resource::<TankAssets>();
    commands.init_resource::<BulletAssets>();
    commands.init_resource::<TankExplosionAssets>();
    commands.init_resource::<SmokeVfxParticleAssets>();
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<TankColors>();
    commands.remove_resource::<TankAssets>();
    commands.remove_resource::<BulletAssets>();
    commands.remove_resource::<TankExplosionAssets>();
    commands.remove_resource::<SmokeVfxParticleAssets>();
}
