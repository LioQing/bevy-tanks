use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tanks_data::*;

pub fn observe_binding(
    trigger: Trigger<Binding<PlayerInputs>>,
    gamepads: Res<Gamepads>,
    mut q: Query<(&PlayerInputs, &mut Actions<PlayerInputs>)>,
) {
    let Ok((ref inputs, mut actions)) = q.get_mut(trigger.entity()) else {
        warn!("No PlayerInputs found for entity {:?}", trigger.entity());
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
