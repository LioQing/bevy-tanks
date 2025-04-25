use bevy::prelude::*;
use bevy_tanks_data::*;

pub fn setup<'a>(
    mut commands: Commands,
    mut tank_colors: ResMut<TankColors>,
    tank_prefab: Res<TankPrefab>,
) {
    tank_prefab
        .spawn(
            &mut commands,
            &mut tank_colors,
            "Player 1",
            TankLabel::Blue,
            Vec3::new(-15.0, 1.0, -15.0),
            Color::srgb(0.0, 0.5, 1.0), // Blue
        )
        .insert(PlayerInputs {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            fire: KeyCode::Space,
        });
    tank_prefab
        .spawn(
            &mut commands,
            &mut tank_colors,
            "Player 2",
            TankLabel::Red,
            Vec3::new(15.0, 1.0, 15.0),
            Color::srgb(1.0, 0.0, 0.0), // Red
        )
        .insert(PlayerInputs {
            forward: KeyCode::ArrowUp,
            backward: KeyCode::ArrowDown,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
            fire: KeyCode::Enter,
        });
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

pub fn handle_game_playing_exit(mut q: Query<&mut TankController, With<PlayerInputs>>) {
    for mut controller in q.iter_mut() {
        controller.movement = None;
        controller.fire = false;
    }
}
