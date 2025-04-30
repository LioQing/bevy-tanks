use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tanks_data::*;
use rand_distr::Normal;

pub fn setup<'a>(
    mut commands: Commands,
    mut tank_colors: ResMut<TankColors>,
    tank_assets: Res<TankAssets>,
) {
    let TankColors(tank_colors) = &mut *tank_colors;

    let mut spawn = |name, label, position, color, player_inputs| {
        let smoke_vfx = SmokeVfx {
            timer: Timer::from_seconds(0.02, TimerMode::Repeating),
            radius: Normal::new(0.5, 0.2).expect("normal distribution"),
            velocity: [
                Normal::new(0.0, 1.0).expect("normal distribution"),
                Normal::new(1.0, 1.0).expect("normal distribution"),
                Normal::new(0.0, 1.0).expect("normal distribution"),
            ],
            lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
            time_scale: 0.0,
        };

        let entity_id = commands
            .spawn((
                Name::new(name),
                StateScoped(AppState::Game),
                TankController { label, ..default() },
                Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
                SceneRoot(tank_assets.scene.clone()),
                player_inputs,
                Actions::<PlayerInputs>::default(),
            ))
            .with_children(|children| {
                children.spawn((
                    Name::new("Left Smoke VFX"),
                    Transform::from_xyz(-1.0, 0.0, 0.8),
                    smoke_vfx.clone(),
                ));
                children.spawn((
                    Name::new("Right Smoke VFX"),
                    Transform::from_xyz(1.0, 0.0, 0.8),
                    smoke_vfx,
                ));
            })
            .id();

        tank_colors.insert(entity_id, color);
    };

    spawn(
        "Player 1",
        TankLabel::Blue,
        Vec3::new(-15.0, 1.0, -15.0),
        Color::srgb(0.0, 0.5, 1.0),
        PlayerInputs {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            fire: KeyCode::Space,
            index: 0,
        },
    );
    spawn(
        "Player 2",
        TankLabel::Red,
        Vec3::new(15.0, 1.0, 15.0),
        Color::srgb(1.0, 0.0, 0.0),
        PlayerInputs {
            forward: KeyCode::ArrowUp,
            backward: KeyCode::ArrowDown,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
            fire: KeyCode::Enter,
            index: 1,
        },
    );
}

pub fn observe_binding(
    trigger: Trigger<Binding<PlayerInputs>>,
    gamepads: Res<Gamepads>,
    mut q: Query<(&PlayerInputs, &mut Actions<PlayerInputs>)>,
) {
    let Ok((ref inputs, mut actions)) = q.get_mut(trigger.entity()) else {
        log::warn!("No PlayerInputs found for entity {:?}", trigger.entity());
        return;
    };

    let Gamepads(gamepads) = &*gamepads;
    if let Some(&entity) = gamepads.get(inputs.index) {
        actions.set_gamepad(entity);
    }

    actions
        .bind::<inputs::Move>()
        .to((
            Cardinal {
                north: inputs.forward,
                east: inputs.right,
                south: inputs.backward,
                west: inputs.left,
            },
            Axial::left_stick(),
        ))
        .with_modifiers((DeadZone::default(), SmoothNudge::default()));
    actions
        .bind::<inputs::Fire>()
        .to((inputs.fire, GamepadButton::South));
}

pub fn observe_movement(
    trigger: Trigger<Fired<inputs::Move>>,
    mut q: Query<&mut TankController>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Playing {
        return;
    }

    let Ok(mut controller) = q.get_mut(trigger.entity()) else {
        return;
    };

    controller.movement = trigger.value;
}

pub fn observe_fire(
    trigger: Trigger<Fired<inputs::Fire>>,
    mut q: Query<&mut TankController>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Playing {
        return;
    }

    let Ok(mut controller) = q.get_mut(trigger.entity()) else {
        return;
    };

    controller.fire = trigger.value;
}

pub fn update_out_of_bound(
    mut commands: Commands,
    q: Query<(Entity, &Transform), With<TankController>>,
) {
    for (entity, transform) in q.iter() {
        if transform.translation.y < -5.0 {
            commands.entity(entity).trigger(TankExplosionEvent);
        }
    }
}

pub fn cleanup_game_playing(mut q: Query<&mut TankController, With<PlayerInputs>>) {
    for mut controller in q.iter_mut() {
        controller.movement = Vec2::ZERO;
        controller.fire = false;
    }
}
