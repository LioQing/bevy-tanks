use bevy::prelude::*;
use rand_distr::Normal;

use crate::{Bullet, SmokeVfx};

#[derive(Debug, Resource)]
pub struct BulletPrefab {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub smoke_vfx: fn(Dir3) -> SmokeVfx,
}

impl BulletPrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        transform: Transform,
    ) -> EntityCommands<'a> {
        commands.spawn((
            Name::new("Bullet"),
            Bullet,
            transform,
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
            Bullet::velocity(transform.forward()),
            (self.smoke_vfx)(transform.forward()),
        ))
    }
}

impl FromWorld for BulletPrefab {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: Bullet::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: Bullet::material(&mut world.get_resource_mut().expect("materials")),
            smoke_vfx: |forward| SmokeVfx {
                timer: Timer::from_seconds(0.02, TimerMode::Repeating),
                radius: Normal::new(0.75, 0.2).expect("normal distribution"),
                velocity: forward
                    .to_array()
                    .map(|v| Normal::new(v, 1.0).expect("normal distribution")),
                lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
                ..default()
            },
        }
    }
}
