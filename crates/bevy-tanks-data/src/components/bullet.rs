use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{NamedGroup, SmokeVfx};

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
    pub fn point_light() -> PointLight {
        PointLight {
            intensity: 1e4,
            range: 3.0,
            color: Color::srgb(1.0, 0.5, 0.0),
            ..default()
        }
    }

    pub const fn rigidbody() -> RigidBody {
        RigidBody::KinematicVelocityBased
    }

    pub fn collider() -> Collider {
        Collider::ball(0.2)
    }

    pub fn collision_groups() -> CollisionGroups {
        CollisionGroups::new(NamedGroup::BULLET, Group::all())
    }

    pub fn solver_groups() -> SolverGroups {
        SolverGroups::new(NamedGroup::BULLET, Group::empty())
    }

    pub fn velocity(direction: Dir3) -> Velocity {
        Velocity {
            linvel: direction * 20.0,
            ..default()
        }
    }

    pub fn active_events() -> ActiveEvents {
        ActiveEvents::COLLISION_EVENTS
    }
}
