use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct BulletAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: world
                .get_resource_mut::<Assets<_>>()
                .expect("meshes")
                .add(Mesh::from(Sphere::new(0.2))),
            material: world
                .get_resource_mut::<Assets<_>>()
                .expect("materials")
                .add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.5, 0.0),
                    emissive: LinearRgba::rgb(100.0, 50.0, 0.0),
                    ..default()
                }),
        }
    }
}
