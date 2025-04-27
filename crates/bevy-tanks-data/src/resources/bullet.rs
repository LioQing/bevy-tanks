use bevy::prelude::*;

use crate::Bullet;

#[derive(Debug, Resource)]
pub struct BulletAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: Bullet::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: Bullet::material(&mut world.get_resource_mut().expect("materials")),
        }
    }
}
