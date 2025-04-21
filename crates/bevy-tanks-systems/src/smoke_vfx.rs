use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_tanks_data::*;
use rand::prelude::*;

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    smoke_vfx_prefab: Res<SmokeVfxParticlePrefab>,
    mut q: Query<(&mut SmokeVfx, &mut Entropy<WyRand>, &GlobalTransform)>,
) {
    for (mut smoke_vfx, mut rng, transform) in q.iter_mut() {
        let scaled_delta = time.delta().mul_f32(smoke_vfx.time_scale);

        if smoke_vfx.timer.tick(scaled_delta).just_finished() {
            let radius = rng.sample(&smoke_vfx.radius).max(0.1);
            let velocity = Vec3::new(
                rng.sample(&smoke_vfx.velocity[0]),
                rng.sample(&smoke_vfx.velocity[1]),
                rng.sample(&smoke_vfx.velocity[2]),
            );
            let lifetime = rng.sample(&smoke_vfx.lifetime).max(0.1);

            smoke_vfx_prefab.spawn(
                &mut commands,
                Timer::from_seconds(lifetime, TimerMode::Once),
                radius,
                velocity,
                transform.translation(),
            );
        }
    }
}

pub fn update_particle(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut SmokeVfxParticle, &mut Transform)>,
) {
    for (entity, mut smoke_vfx, mut transform) in q.iter_mut() {
        let x = smoke_vfx.timer.fraction();

        let speed = 1.0 - x * x;
        transform.translation += smoke_vfx.velocity * speed * time.delta_secs();

        let scale = 4.0 * x.sqrt() - 4.0 * x;
        transform.scale = Vec3::splat(smoke_vfx.radius * scale);

        if smoke_vfx.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
