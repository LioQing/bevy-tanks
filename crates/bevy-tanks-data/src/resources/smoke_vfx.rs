use bevy::prelude::*;

use crate::SmokeVfxParticle;

#[derive(Debug, Resource)]
pub struct SmokeVfxParticleAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for SmokeVfxParticleAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: SmokeVfxParticle::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: SmokeVfxParticle::material(&mut world.get_resource_mut().expect("materials")),
        }
    }
}
