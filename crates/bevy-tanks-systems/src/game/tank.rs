use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_rapier3d::prelude::*;
use bevy_tanks_data::*;
use bevy_tnua::prelude::*;
use rand_distr::Normal;

pub fn observe_scene_instance_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tank_colors: Res<TankColors>,
    tank_assets: Res<TankAssets>,
    tank_q: Query<(), With<TankController>>,
    children_q: Query<&Children>,
    mesh_material_q: Query<(&Name, &MeshMaterial3d<StandardMaterial>)>,
    animation_player_q: Query<(), With<AnimationPlayer>>,
) {
    let Ok(()) = tank_q.get(trigger.entity()) else {
        return;
    };

    let TankColors(tank_colors) = &*tank_colors;
    let color = tank_colors
        .get(&trigger.entity())
        .unwrap_or_else(|| panic!("Color for {} not found", trigger.entity()));

    let mut subst_material = None;
    for descendant in children_q.iter_descendants(trigger.entity()) {
        // Override color
        if let Ok((name, MeshMaterial3d(orig_material))) = mesh_material_q.get(descendant) {
            if name.as_str() == "Body" || name.as_str() == "Turret" {
                let orig_material = materials
                    .get(orig_material)
                    .expect("original material")
                    .clone();

                let material = subst_material.get_or_insert_with(|| {
                    materials.add(StandardMaterial {
                        base_color: *color,
                        ..orig_material
                    })
                });

                commands
                    .entity(descendant)
                    .insert(MeshMaterial3d(material.clone()));
            }
        }

        // Insert animation graph
        if let Ok(()) = animation_player_q.get(descendant) {
            commands.entity(descendant).insert(AnimationGraphHandle(
                tank_assets.fire_animation_graph.clone(),
            ));
        }
    }
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    bullet_assets: Res<BulletAssets>,
    tank_assets: Res<TankAssets>,
    mut tank_q: Query<(
        Entity,
        &Transform,
        &mut TankController,
        &TankSpeed,
        &mut TnuaController,
    )>,
    mut animation_player_q: Query<&mut AnimationPlayer>,
    children_q: Query<&Children>,
) {
    for (entity, transform, mut controller, speed, mut tnua_controller) in tank_q.iter_mut() {
        let TankController {
            movement,
            fire,
            fire_timer,
            ..
        } = &mut *controller;

        // Movement
        let velocity_2d = movement.as_ref().map(Dir2::as_vec2).unwrap_or_default()
            * speed.linear
            * Vec2::new(1.0, -1.0);
        let velocity = Vec3::new(velocity_2d.x, 0.0, velocity_2d.y);

        tnua_controller.basis(TnuaBuiltinWalk {
            desired_velocity: velocity,
            desired_forward: Dir3::new(velocity).ok(),
            float_height: 0.5,
            turning_angvel: speed.angular,
            ..default()
        });

        // Fire
        if let Some(inner) = fire_timer {
            if inner.tick(time.delta()).just_finished() {
                *fire_timer = None;
            } else {
                *fire = false;
            }
        }

        const FORWARD_OFFSET: f32 = 1.5;
        const UP_OFFSET: f32 = 1.3;
        if *fire {
            let bullet_transform = transform.clone().with_translation(
                transform.translation
                    + velocity * time.delta_secs()
                    + transform.forward() * FORWARD_OFFSET
                    + transform.up() * UP_OFFSET,
            );

            commands.spawn((
                Name::new("Bullet"),
                StateScoped(AppState::Game),
                Bullet,
                bullet_transform,
                Mesh3d(bullet_assets.mesh.clone()),
                MeshMaterial3d(bullet_assets.material.clone()),
                Bullet::velocity(bullet_transform.forward()),
                SmokeVfx {
                    timer: Timer::from_seconds(0.02, TimerMode::Repeating),
                    radius: Normal::new(0.75, 0.2).expect("normal distribution"),
                    velocity: bullet_transform
                        .forward()
                        .to_array()
                        .map(|v| Normal::new(v, 1.0).expect("normal distribution")),
                    lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
                    ..default()
                },
            ));

            for descendant in children_q.iter_descendants(entity) {
                if let Ok(mut animation_player) = animation_player_q.get_mut(descendant) {
                    if animation_player.is_playing_animation(tank_assets.fire_animation_node) {
                        animation_player
                            .animation_mut(tank_assets.fire_animation_node)
                            .expect("fire animation")
                            .replay();
                    } else {
                        animation_player.play(tank_assets.fire_animation_node);
                    }
                }
            }

            *fire_timer = Some(Timer::new(speed.fire_cooldown, TimerMode::Once));
        }
    }
}

pub fn update_smoke(
    tank_children_q: Query<
        (&Children, &Velocity, &TankSpeed, &TnuaController),
        With<TankController>,
    >,
    mut smoke_vfx_q: Query<(&Transform, &mut SmokeVfx)>,
) {
    for (children, velocity, speed, controller) in tank_children_q.iter() {
        for child in children.iter() {
            if let Ok((transform, mut smoke_vfx)) = smoke_vfx_q.get_mut(*child) {
                if controller.is_airborne().expect("airborne") {
                    smoke_vfx.time_scale = 0.0;
                } else {
                    let smoke_velocity = velocity
                        .linear_velocity_at_point(transform.translation, Vec3::new(0.0, 1.0, 0.0));

                    smoke_vfx.time_scale = smoke_velocity.length() / speed.linear;
                }
            }
        }
    }
}
