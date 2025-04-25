use bevy::prelude::*;
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

pub fn handle_tank_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    tank_explosion_prefab: Res<TankExplosionPrefab>,
    bullet_q: Query<(), With<Bullet>>,
    tank_q: Query<(), With<TankController>>,
    transform_q: Query<&Transform>,
    children_q: Query<&Children>,
    mesh_q: Query<(), With<Mesh3d>>,
) {
    for collision_event in collision_events.read() {
        let (a, b) = match *collision_event {
            CollisionEvent::Started(a, b, ..) | CollisionEvent::Stopped(a, b, ..) => (a, b),
        };

        let (bullet, tank) = if bullet_q.get(a).is_ok() && tank_q.get(b).is_ok() {
            (a, b)
        } else if bullet_q.get(b).is_ok() && tank_q.get(a).is_ok() {
            (b, a)
        } else {
            continue;
        };

        commands.entity(bullet).despawn_recursive();
        commands.entity(tank).remove::<TankAlive>();

        tank_explosion_prefab.spawn(
            &mut commands,
            &children_q,
            &mesh_q,
            tank,
            transform_q.get(tank).expect("tank transfrom").translation,
        );

        game_state.set(GameState::Over);
    }
}
