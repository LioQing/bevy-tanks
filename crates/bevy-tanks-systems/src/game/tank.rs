use bevy::{pbr::NotShadowCaster, prelude::*, scene::SceneInstanceReady};
use bevy_rapier3d::prelude::*;
use bevy_tanks_data::*;
use bevy_tnua::prelude::*;
use rand_distr::Normal;

pub fn setup<'a>(
    mut commands: Commands,
    mut tank_colors: ResMut<TankColors>,
    game_mode: Res<GameMode>,
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

        let mut entity_commands = commands.spawn((
            Name::new(name),
            StateScoped(AppState::Game),
            TankController { label, ..default() },
            Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
            SceneRoot(tank_assets.scene.clone()),
        ));

        entity_commands.with_children(|children| {
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
        });

        if let Some(player_inputs) = player_inputs {
            entity_commands.insert(player_inputs);
        } else {
            entity_commands.insert(AiInputs::default());
        }

        tank_colors.insert(entity_commands.id(), color);
    };

    spawn(
        "Player 1",
        TankLabel::Blue,
        Vec3::new(-15.0, 1.0, -15.0),
        Color::srgb(0.0, 0.5, 1.0),
        Some(PlayerInputs {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            fire: KeyCode::Space,
            index: 0,
        }),
    );
    spawn(
        game_mode.multiplayer.then(|| "Player 2").unwrap_or("AI"),
        TankLabel::Red,
        Vec3::new(15.0, 1.0, 15.0),
        Color::srgb(1.0, 0.0, 0.0),
        game_mode.multiplayer.then_some(PlayerInputs {
            forward: KeyCode::ArrowUp,
            backward: KeyCode::ArrowDown,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
            fire: KeyCode::Enter,
            index: 1,
        }),
    );
}

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

pub fn observe_tank_explosion(
    trigger: Trigger<TankExplosionEvent>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    tank_explosion_assets: Res<TankExplosionAssets>,
    children_q: Query<&Children>,
    mesh_q: Query<(), With<Mesh3d>>,
    transform_q: Query<&Transform>,
) {
    let tank = trigger.entity();

    // Make entity not cast shadows so it doesn't block the explosion light
    for child in children_q.iter_descendants(tank) {
        if let Ok(()) = mesh_q.get(child) {
            commands.entity(child).insert(NotShadowCaster);
        }
    }

    commands.spawn((
        Name::new("Tank Explosion"),
        StateScoped(AppState::Game),
        Explosion {
            radius: 2.5,
            entity: Some(tank),
            ..default()
        },
        Transform::from_translation(
            transform_q.get(tank).expect("tank transfrom").translation + Vec3::Y,
        )
        .with_scale(Vec3::ONE * 1e-3),
        ExplosionLight {
            range: Some(25.0),
            radius: Some(2.5),
        },
        PointLight {
            color: Color::srgb(1.0, 0.5, 0.0),
            intensity: 2e6,
            range: 0.0,
            radius: 0.0,
            shadows_enabled: true,
            ..default()
        },
        Mesh3d(tank_explosion_assets.mesh.clone()),
        MeshMaterial3d(tank_explosion_assets.material.clone()),
    ));

    commands
        .entity(tank)
        .remove::<TankAlive>()
        .remove::<RigidBody>(); // Avoid tanks from falling outside the explosion

    game_state.set(GameState::Over);
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    bullet_assets: Res<BulletAssets>,
    tank_assets: Res<TankAssets>,
    mut tank_q: Query<
        (
            Entity,
            &Transform,
            &mut TankController,
            &TankSpeed,
            &mut TnuaController,
            Option<&AiInputs>,
        ),
        With<TankAlive>,
    >,
    mut animation_player_q: Query<&mut AnimationPlayer>,
    children_q: Query<&Children>,
) {
    for (entity, transform, mut controller, speed, mut tnua_controller, ai_inputs) in
        tank_q.iter_mut()
    {
        let TankController {
            movement,
            fire,
            fire_timer,
            ..
        } = &mut *controller;

        // Movement
        let velocity_2d = movement.clamp_length_max(1.0) * speed.linear * Vec2::new(1.0, -1.0);
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

            let mut entity_commands = commands.spawn((
                Name::new("Bullet"),
                StateScoped(GameState::Playing),
                Bullet,
                bullet_transform,
                Mesh3d(bullet_assets.mesh.clone()),
                MeshMaterial3d(bullet_assets.material.clone()),
                Bullet::velocity(bullet_transform.forward()),
                SmokeVfx {
                    timer: Timer::from_seconds(0.04, TimerMode::Repeating),
                    radius: Normal::new(0.75, 0.2).expect("normal distribution"),
                    velocity: bullet_transform
                        .forward()
                        .to_array()
                        .map(|v| Normal::new(v, 1.0).expect("normal distribution")),
                    lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
                    ..default()
                },
            ));

            if ai_inputs.is_none() {
                // Spawn bullet velocity component for player tanks
                // We don't spawn it for AI because they will try to dodge it
                // because the bullet velocity collider is a bit bigger than the bullet's own collider
                entity_commands.with_children(|children| {
                    children.spawn((Name::new("Bullet Velocity"), BulletVelocity));
                });
            }

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

        // Reset movement and fire
        *movement = Vec2::ZERO;
        *fire = false;
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
