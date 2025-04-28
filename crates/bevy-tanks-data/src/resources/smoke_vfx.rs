use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct SmokeVfxParticleAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for SmokeVfxParticleAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: world
                .get_resource_mut::<Assets<_>>()
                .expect("meshes")
                .add(Mesh::from(Sphere::new(0.5))),
            material: world
                .get_resource_mut::<Assets<_>>()
                .expect("materials")
                .add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
        }
    }
}
