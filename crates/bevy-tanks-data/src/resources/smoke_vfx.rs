use bevy::prelude::*;

use crate::SmokeVfxParticle;

#[derive(Debug, Resource)]
pub struct SmokeVfxParticlePrefab {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl SmokeVfxParticlePrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        timer: Timer,
        radius: f32,
        velocity: Vec3,
        position: Vec3,
    ) -> EntityCommands<'a> {
        commands.spawn((
            Name::new("Smoke VFX"),
            SmokeVfxParticle {
                timer,
                radius,
                velocity,
            },
            Transform::from_translation(position).with_scale(Vec3::ONE * 1e-3),
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
        ))
    }
}

impl FromWorld for SmokeVfxParticlePrefab {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: SmokeVfxParticle::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: SmokeVfxParticle::material(&mut world.get_resource_mut().expect("materials")),
        }
    }
}
