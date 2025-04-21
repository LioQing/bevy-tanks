use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_distr::Normal;

#[derive(Debug, Clone, Component)]
#[require(Transform, Entropy<WyRand>)]
pub struct SmokeVfx {
    pub timer: Timer,
    pub radius: Normal<f32>,
    pub velocity: [Normal<f32>; 3],
    pub lifetime: Normal<f32>,
    pub time_scale: f32,
}

impl Default for SmokeVfx {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            radius: Normal::new(0.2, 0.1).expect("normal distribution"),
            velocity: [
                Normal::new(0.0, 1.0).expect("normal distribution"),
                Normal::new(1.0, 1.0).expect("normal distribution"),
                Normal::new(0.0, 1.0).expect("normal distribution"),
            ],
            lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
            time_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Component)]
#[require(Transform, Mesh3d, MeshMaterial3d<StandardMaterial>)]
pub struct SmokeVfxParticle {
    pub timer: Timer,
    pub radius: f32,
    pub velocity: Vec3,
}

impl SmokeVfxParticle {
    pub(crate) fn mesh(meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        meshes.add(Mesh::from(Sphere::new(0.5)))
    }

    pub(crate) fn material(materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })
    }
}

impl Default for SmokeVfxParticle {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            radius: 0.5,
            velocity: Vec3::Y,
        }
    }
}
