use bevy::prelude::*;
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
        },
    );
}

pub fn update(keys: Res<ButtonInput<KeyCode>>, mut q: Query<(&PlayerInputs, &mut TankController)>) {
    for (inputs, mut controller) in q.iter_mut() {
        let mut movement = Vec2::ZERO;

        if keys.pressed(inputs.forward) {
            movement.y += 1.0;
        }
        if keys.pressed(inputs.backward) {
            movement.y -= 1.0;
        }
        if keys.pressed(inputs.left) {
            movement.x -= 1.0;
        }
        if keys.pressed(inputs.right) {
            movement.x += 1.0;
        }

        controller.movement = Dir2::new(movement).ok();
        controller.fire = keys.just_pressed(inputs.fire);
    }
}

pub fn cleanup_game_playing(mut q: Query<&mut TankController, With<PlayerInputs>>) {
    for mut controller in q.iter_mut() {
        controller.movement = None;
        controller.fire = false;
    }
}
