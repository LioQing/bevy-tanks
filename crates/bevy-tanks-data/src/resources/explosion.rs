use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::{Explosion, ExplosionLight};

#[derive(Debug, Resource)]
pub struct TankExplosionPrefab {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl TankExplosionPrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        children_q: &Query<&Children>,
        mesh_q: &Query<(), With<Mesh3d>>,
        entity: Entity,
        position: Vec3,
    ) -> EntityCommands<'a> {
        // Make entity not cast shadows so it doesn't block the explosion light
        for child in children_q.iter_descendants(entity) {
            if let Ok(()) = mesh_q.get(child) {
                commands.entity(child).insert(NotShadowCaster);
            }
        }

        commands.spawn((
            Name::new("Tank Explosion"),
            Explosion {
                radius: 2.5,
                entity: Some(entity),
                ..default()
            },
            Transform::from_translation(position + Vec3::Y).with_scale(Vec3::ONE * 1e-3),
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
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
        ))
    }
}

impl FromWorld for TankExplosionPrefab {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: Explosion::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: Explosion::material(&mut world.get_resource_mut().expect("materials")),
        }
    }
}
