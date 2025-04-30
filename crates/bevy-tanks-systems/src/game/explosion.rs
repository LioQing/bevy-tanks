use bevy::prelude::*;
use bevy_tanks_data::*;

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(
        Entity,
        &mut Explosion,
        &mut Transform,
        Option<(&mut PointLight, &ExplosionLight)>,
    )>,
) {
    for (entity, mut explosion, mut transform, lights) in q.iter_mut() {
        explosion.timer.tick(time.delta());

        let x = explosion.timer.fraction();
        let scale = (x * std::f32::consts::PI).sin();
        transform.scale = Vec3::ONE * explosion.radius * 2.0 * scale;

        // Light range
        if let Some((mut point_light, explosion_light)) = lights {
            if let Some(range) = explosion_light.range {
                point_light.range = range * scale;
            }

            if let Some(radius) = explosion_light.radius {
                point_light.radius = radius * scale;
            }
        }

        // Despawn explosion entity when over halfway
        if let Some(target_entity) = explosion.entity {
            if commands.get_entity(target_entity).is_some() && explosion.timer.fraction() >= 0.5 {
                commands.entity(target_entity).despawn_recursive();
            }
        }

        if explosion.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
