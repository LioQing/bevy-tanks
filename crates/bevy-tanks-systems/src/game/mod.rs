use bevy::{asset::RecursiveDependencyLoadState, prelude::*};
use bevy_enhanced_input::prelude::*;
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
    app.add_input_context::<PlayerInputs>()
        .add_sub_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_state_scoped_event::<TankExplosionEvent>(AppState::Game)
        .add_state_scoped_event::<PausedUiButtonEvent>(GameState::Paused)
        .add_state_scoped_event::<GameOverUiButtonEvent>(GameState::Over)
        .add_observer(tank::observe_scene_instance_ready)
        .add_observer(tank::observe_tank_explosion)
        .add_observer(ui::observe_paused_button)
        .add_observer(ui::observe_game_over_button)
        .add_observer(player::observe_binding)
        .add_observer(player::observe_movement)
        .add_observer(player::observe_fire)
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
                // Loading
                update_loading.run_if(in_state(GameState::Loading)),
                // Not loading
                (
                    tank::update.in_set(TnuaUserControlsSystemSet),
                    tank::update_smoke.after(TnuaUserControlsSystemSet),
                    bullet::update,
                    bullet::update_tank_collision_events,
                    explosion::update,
                    smoke_vfx::update,
                    smoke_vfx::update_particle,
                    // Playing
                    (player::update_out_of_bound, ui::update_pause_when_playing)
                        .run_if(in_state(GameState::Playing)),
                    // Paused
                    (
                        player::update_out_of_bound,
                        button::update_interaction::<PausedUiButtonEvent>,
                        ui::update_paused,
                    )
                        .run_if(in_state(GameState::Paused)),
                    // Over
                    button::update_interaction::<GameOverUiButtonEvent>
                        .run_if(in_state(GameState::Over)),
                )
                    .run_if(not(in_state(GameState::Loading))),
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(OnEnter(GameState::Loading), ui::setup_loading)
        .add_systems(
            OnTransition {
                exited: GameState::Loading,
                entered: GameState::Playing,
            },
            player::setup,
        )
        .add_systems(
            OnTransition {
                exited: GameState::Over,
                entered: GameState::Playing,
            },
            player::setup,
        )
        .add_systems(OnExit(GameState::Playing), player::cleanup_game_playing)
        .add_systems(OnEnter(GameState::Paused), ui::setup_paused)
        .add_systems(OnEnter(GameState::Over), ui::setup_game_over)
        .add_systems(OnExit(GameState::Over), common::cleanup::<TankController>)
        .add_systems(OnExit(AppState::Game), cleanup);
}

fn update_loading(
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    tank_assets: Res<TankAssets>,
) {
    match asset_server.recursive_dependency_load_state(&tank_assets.scene) {
        RecursiveDependencyLoadState::Loading | RecursiveDependencyLoadState::NotLoaded => return,
        RecursiveDependencyLoadState::Failed(e) => {
            log::error!("Failed to load tank scene: {e}");
            return;
        }
        _ => {}
    }

    match asset_server.recursive_dependency_load_state(&tank_assets.fire_animation) {
        RecursiveDependencyLoadState::Loading | RecursiveDependencyLoadState::NotLoaded => return,
        RecursiveDependencyLoadState::Failed(e) => {
            log::error!("Failed to load tank fire animation: {e}");
            return;
        }
        _ => {}
    }

    log::debug!("Tank scene and fire animation loaded");

    game_state.set(GameState::Playing);
}

fn setup(mut commands: Commands) {
    commands.init_resource::<TankColors>();
    commands.init_resource::<TankAssets>();
    commands.init_resource::<BulletAssets>();
    commands.init_resource::<TankExplosionAssets>();
    commands.init_resource::<SmokeVfxParticleAssets>();
    commands.init_resource::<Gamepads>();
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<TankColors>();
    commands.remove_resource::<TankAssets>();
    commands.remove_resource::<BulletAssets>();
    commands.remove_resource::<TankExplosionAssets>();
    commands.remove_resource::<SmokeVfxParticleAssets>();
    commands.remove_resource::<Gamepads>();
}
