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
