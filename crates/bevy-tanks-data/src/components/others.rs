use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{NamedGroup, SmokeVfx};

#[derive(Debug, Clone, Component)]
pub struct PlayerInputs {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub fire: KeyCode,
}

#[derive(Debug, Clone, Copy, Component)]
#[require(
    Transform,
    Mesh3d,
    MeshMaterial3d<StandardMaterial>,
    PointLight(Bullet::point_light),
    RigidBody(Bullet::rigidbody),
    Collider(Bullet::collider),
    CollisionGroups(Bullet::collision_groups),
    SolverGroups(Bullet::solver_groups),
    Velocity,
    ActiveEvents(Bullet::active_events),
    SmokeVfx,
)]
pub struct Bullet;

impl Bullet {
    pub(crate) fn mesh(meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        meshes.add(Mesh::from(Sphere::new(0.2)))
    }

    pub(crate) fn material(materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.0),
            emissive: LinearRgba::rgb(100.0, 50.0, 0.0),
            ..default()
        })
    }

    fn point_light() -> PointLight {
        PointLight {
            intensity: 1e4,
            range: 3.0,
            color: Color::srgb(1.0, 0.5, 0.0),
            ..default()
        }
    }

    const fn rigidbody() -> RigidBody {
        RigidBody::KinematicVelocityBased
    }

    fn collider() -> Collider {
        Collider::ball(0.2)
    }

    fn collision_groups() -> CollisionGroups {
        CollisionGroups::new(NamedGroup::BULLET, Group::all())
    }

    fn solver_groups() -> SolverGroups {
        SolverGroups::new(NamedGroup::BULLET, NamedGroup::BULLET)
    }

    pub(crate) fn velocity(direction: Dir3) -> Velocity {
        Velocity {
            linvel: direction * 20.0,
            ..default()
        }
    }

    fn active_events() -> ActiveEvents {
        ActiveEvents::COLLISION_EVENTS
    }
}
