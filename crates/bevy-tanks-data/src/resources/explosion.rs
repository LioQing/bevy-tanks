use bevy::prelude::*;

use crate::Explosion;

#[derive(Debug, Resource)]
pub struct TankExplosionAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for TankExplosionAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: Explosion::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: Explosion::material(&mut world.get_resource_mut().expect("materials")),
        }
    }
}
