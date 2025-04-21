use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
#[require(
    Transform,
    Mesh3d,
    MeshMaterial3d<StandardMaterial>,
)]
pub struct Explosion {
    pub timer: Timer,
    pub radius: f32,
    pub entity: Option<Entity>,
}

impl Explosion {
    pub(crate) fn mesh(meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        meshes.add(Mesh::from(Sphere::new(0.5)))
    }

    pub(crate) fn material(materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.0),
            perceptual_roughness: 1.0,
            emissive: LinearRgba::rgb(100.0, 50.0, 0.0),
            ..default()
        })
    }
}

impl Default for Explosion {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            radius: 0.5,
            entity: None,
        }
    }
}

#[derive(Debug, Clone, Component)]
#[require(PointLight)]
pub struct ExplosionLight {
    pub range: Option<f32>,
    pub radius: Option<f32>,
}

impl Default for ExplosionLight {
    fn default() -> Self {
        Self {
            range: Some(5.0),
            radius: None,
        }
    }
}
