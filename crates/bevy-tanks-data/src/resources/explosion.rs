use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct TankExplosionAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for TankExplosionAssets {
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
                    base_color: Color::srgb(1.0, 0.5, 0.0),
                    perceptual_roughness: 1.0,
                    emissive: LinearRgba::rgb(100.0, 50.0, 0.0),
                    ..default()
                }),
        }
    }
}
