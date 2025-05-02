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

pub fn update_tank_collision_events(
    mut commands: Commands,
    mut collision_evr: EventReader<CollisionEvent>,
    bullet_q: Query<(), With<Bullet>>,
    tank_q: Query<(), With<TankController>>,
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

        commands.entity(tank).trigger(TankExplosionEvent);
    }
}

pub fn update_bullet_velocity(
    bullet_q: Query<(&Children, &Velocity), With<Bullet>>,
    mut bullet_velocity_q: Query<&mut Transform, With<BulletVelocity>>,
) {
    for (children, velocity) in bullet_q.iter() {
        for child in children.iter() {
            if let Ok(mut transform) = bullet_velocity_q.get_mut(*child) {
                let len = velocity.linvel.length();
                transform.scale.z = len;
                transform.translation.z = -len;
            }
        }
    }
}
