use bevy::{color::palettes, prelude::*};
use bevy_rand::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tanks_data::*;
use oxidized_navigation::{
    NavMesh, NavMeshSettings,
    debug_draw::DrawPath,
    query::{find_polygon_path, perform_string_pulling_on_path},
};
use rand::Rng;
use rand_distr::{Normal, Uniform};

pub fn update_controls(
    time: Res<Time>,
    mut nav_queue: ResMut<AiNavQueue>,
    mut ai_q: Query<(
        Entity,
        &mut TankController,
        &Transform,
        &mut AiInputs,
        &mut Entropy<WyRand>,
    )>,
    player: Single<(&Transform, &Velocity), With<PlayerInputs>>,
) {
    let (player_transform, player_velocity) = &*player;

    for (entity, mut controller, transform, mut inputs, mut rng) in ai_q.iter_mut() {
        match &mut inputs.state {
            AiState::Idle { timer } => {
                if timer.tick(time.delta()).just_finished() {
                    match rng.sample(Uniform::new(0, 2)) {
                        0 => {
                            inputs.state = AiState::Attacking {
                                fire_count: rng.sample(Uniform::new(1, 5)),
                                movement: rng.sample(Uniform::new(0, 1)) == 0,
                            };
                        }
                        1 => {
                            let pos = Vec3::new(
                                rng.sample(Uniform::new(-20.0, 20.0)),
                                0.0,
                                rng.sample(Uniform::new(-20.0, 20.0)),
                            );

                            let AiNavQueue(queue) = &mut *nav_queue;
                            queue.push_back((entity, pos));

                            inputs.state = AiState::Moving;
                        }
                        _ => error!("Invalid random number"),
                    }
                }
            }
            AiState::Attacking {
                fire_count,
                movement,
            } => {
                if *fire_count == 0 {
                    match rng.sample(Uniform::new(0, 2)) {
                        0 => {
                            inputs.state = AiState::Idle {
                                timer: Timer::from_seconds(
                                    rng.sample(Uniform::new(0.1, 2.0)),
                                    TimerMode::Once,
                                ),
                            };
                        }
                        1 => {
                            let pos = Vec3::new(
                                rng.sample(Uniform::new(-20.0, 20.0)),
                                0.0,
                                rng.sample(Uniform::new(-20.0, 20.0)),
                            );

                            let AiNavQueue(queue) = &mut *nav_queue;
                            queue.push_back((entity, pos));

                            inputs.state = AiState::Moving;
                        }
                        _ => error!("Invalid random number"),
                    }
                } else {
                    let player_offset = Vec3::new(
                        rng.sample(Normal::new(0.0, 0.5).expect("normal distribution")),
                        0.0,
                        rng.sample(Normal::new(0.0, 0.5).expect("normal distribution")),
                    );
                    let target =
                        player_transform.translation + player_velocity.linvel + player_offset;
                    let target_disp = (target - transform.translation).normalize_or_zero();
                    let target_disp_dot_forward = target_disp.dot(transform.forward().as_vec3());

                    if *movement || target_disp_dot_forward < 0.9 {
                        controller.movement = target_disp.xz() * Vec2::new(1.0, -1.0);
                    }

                    let player_disp =
                        (player_transform.translation - transform.translation).normalize_or_zero();
                    let player_disp_dot_forward = player_disp.dot(transform.forward().as_vec3());

                    if (target_disp_dot_forward > 0.9 || player_disp_dot_forward > 0.9)
                        && controller.fire_timer.is_none()
                    {
                        controller.fire = true;
                        *fire_count -= 1;
                    }
                }
            }
            AiState::Moving => match inputs.path.front().map(|v| *v) {
                Some(mut target) => {
                    if target.distance_squared(transform.translation) < 0.1 {
                        inputs.path.pop_front();

                        match inputs.path.front() {
                            Some(next_target) => target = *next_target,
                            None => continue,
                        }
                    }

                    controller.movement =
                        (target - transform.translation).xz() * Vec2::new(1.0, -1.0);

                    let target = player_transform.translation + player_velocity.linvel;
                    let target_disp = (target - transform.translation).normalize_or_zero();
                    let target_disp_dot_forward = target_disp.dot(transform.forward().as_vec3());

                    let player_disp =
                        (player_transform.translation - transform.translation).normalize_or_zero();
                    let player_disp_dot_forward = player_disp.dot(transform.forward().as_vec3());

                    if target_disp_dot_forward > 0.9 || player_disp_dot_forward > 0.9 {
                        controller.fire = true;
                    }
                }
                None => match rng.sample(Uniform::new(0, 2)) {
                    0 => {
                        inputs.state = AiState::Idle {
                            timer: Timer::from_seconds(
                                rng.sample(Uniform::new(0.1, 2.0)),
                                TimerMode::Once,
                            ),
                        };
                    }
                    1 => {
                        inputs.state = AiState::Attacking {
                            fire_count: rng.sample(Uniform::new(1, 5)),
                            movement: rng.sample(Uniform::new(0, 1)) == 0,
                        };
                    }
                    _ => error!("Invalid random number"),
                },
            },
        }
    }
}

pub fn update_bullet_velocity_tank_collision_events(
    mut collision_evr: EventReader<CollisionEvent>,
    mut nav_queue: ResMut<AiNavQueue>,
    bullet_velocity_q: Query<(), With<BulletVelocity>>,
    mut tank_q: Query<&mut Entropy<WyRand>, (With<TankController>, With<AiInputs>)>,
) {
    for collision_event in collision_evr.read() {
        let (a, b) = match *collision_event {
            CollisionEvent::Started(a, b, ..) => (a, b),
            _ => continue,
        };

        let tank = if bullet_velocity_q.get(a).is_ok() && tank_q.get(b).is_ok() {
            b
        } else if bullet_velocity_q.get(b).is_ok() && tank_q.get(a).is_ok() {
            a
        } else {
            return;
        };

        let Ok(mut rng) = tank_q.get_mut(tank) else {
            return;
        };

        // Randomly pick a position within the area of the floor
        let pos = Vec3::new(
            rng.sample(Uniform::new(-20.0, 20.0)),
            0.0,
            rng.sample(Uniform::new(-20.0, 20.0)),
        );

        info!("Push AI navigation queue for {tank} to {pos}");

        let AiNavQueue(nav_queue) = &mut *nav_queue;
        nav_queue.push_back((tank, pos));
    }
}

pub fn update_path(
    mut commands: Commands,
    mut queue: ResMut<AiNavQueue>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut q: Query<(&mut AiInputs, &Transform)>,
) {
    let AiNavQueue(queue) = &mut *queue;

    let Some((entity, target_pos)) = queue.pop_front() else {
        return;
    };

    let nav_mesh = nav_mesh.get();
    let Ok(nav_mesh) = nav_mesh.read() else {
        warn!("NavMesh not ready yet!");
        return;
    };

    let Ok((mut inputs, transform)) = q.get_mut(entity) else {
        warn!("Entity {entity:?} not found!");
        return;
    };

    let path = match find_polygon_path(
        &nav_mesh,
        &nav_mesh_settings,
        transform.translation,
        target_pos,
        None,
        None,
    ) {
        Ok(path) => path,
        Err(e) => {
            warn!("Can't find path {e:?}");
            return;
        }
    };

    let string_path =
        match perform_string_pulling_on_path(&nav_mesh, transform.translation, target_pos, &path) {
            Ok(string_path) => string_path,
            Err(e) => {
                error!("Error with string path: {e:?}");
                return;
            }
        };

    commands.spawn(DrawPath {
        timer: Some(Timer::from_seconds(2.0, TimerMode::Once)),
        pulled_path: string_path.clone(),
        color: palettes::css::RED.into(),
    });

    inputs.path = string_path.into();
    inputs.state = AiState::Moving;
}
