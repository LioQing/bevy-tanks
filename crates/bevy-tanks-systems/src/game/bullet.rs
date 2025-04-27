use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_tanks_data::*;

pub fn update(mut commands: Commands, q: Query<(Entity, &Transform), With<Bullet>>) {
    for (entity, transform) in q.iter() {
        const BOUND: f32 = 50.0;
        if transform.translation.x.abs() > BOUND
            || transform.translation.y.abs() > BOUND
            || transform.translation.z.abs() > BOUND
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_tank_collision_events(
    mut commands: Commands,
    mut collision_evr: EventReader<CollisionEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    tank_explosion_assets: Res<TankExplosionAssets>,
    bullet_q: Query<(), With<Bullet>>,
    tank_q: Query<(), With<TankController>>,
    transform_q: Query<&Transform>,
    children_q: Query<&Children>,
    mesh_q: Query<(), With<Mesh3d>>,
) {
    for collision_event in collision_evr.read() {
        let (a, b) = match *collision_event {
            CollisionEvent::Started(a, b, ..) | CollisionEvent::Stopped(a, b, ..) => (a, b),
        };

        let (bullet, tank) = if bullet_q.get(a).is_ok() && tank_q.get(b).is_ok() {
            (a, b)
        } else if bullet_q.get(b).is_ok() && tank_q.get(a).is_ok() {
            (b, a)
        } else {
            return;
        };

        commands.entity(bullet).despawn_recursive();
        commands.entity(tank).remove::<TankAlive>();

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

        game_state.set(GameState::Over);
    }
}
