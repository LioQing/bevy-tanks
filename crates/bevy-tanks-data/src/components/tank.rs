use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::TnuaRapier3dSensorShape;

use super::NamedGroup;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct TankAlive;

#[derive(Debug, Default, Clone, Copy)]
pub enum TankLabel {
    #[default]
    Red,
    Blue,
}

impl TankLabel {
    pub fn opposite(self) -> Self {
        match self {
            TankLabel::Red => TankLabel::Blue,
            TankLabel::Blue => TankLabel::Red,
        }
    }
}

impl std::fmt::Display for TankLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TankLabel::Red => write!(f, "Red"),
            TankLabel::Blue => write!(f, "Blue"),
        }
    }
}

#[derive(Debug, Default, Clone, Component)]
#[require(
    Transform,
    TankSpeed(TankController::tank_speed),
    SceneRoot,
    RigidBody(TankController::rigidbody),
    Collider(TankController::collider),
    CollisionGroups(TankController::collision_groups),
    SolverGroups(TankController::solver_groups),
    TnuaController,
    TnuaRapier3dSensorShape(TankController::tnua_rapier3d_sensor_shape),
    TankAlive
)]
pub struct TankController {
    pub movement: Option<Dir2>,
    pub fire: bool,
    pub fire_timer: Option<Timer>,
    pub label: TankLabel,
}

impl TankController {
    pub const fn tank_speed() -> TankSpeed {
        TankSpeed {
            linear: 10.0,
            angular: 2.0 * std::f32::consts::PI,
            fire_cooldown: Duration::from_millis(500),
        }
    }

    pub const fn rigidbody() -> RigidBody {
        RigidBody::Dynamic
    }

    pub fn collider() -> Collider {
        Collider::compound(vec![
            (
                Vec3::new(0.0, 1.0, 0.0),
                Quat::IDENTITY,
                Collider::round_cylinder(0.25, 0.75, 0.25),
            ),
            (
                Vec3::new(0.0, 1.25, 0.0),
                Quat::IDENTITY,
                Collider::ball(0.75),
            ),
        ])
    }

    pub fn collision_groups() -> CollisionGroups {
        CollisionGroups::new(NamedGroup::TANK, Group::all())
    }

    pub fn solver_groups() -> SolverGroups {
        SolverGroups::new(NamedGroup::TANK, Group::all() & !NamedGroup::BULLET)
    }

    pub fn tnua_rapier3d_sensor_shape() -> TnuaRapier3dSensorShape {
        TnuaRapier3dSensorShape(Self::collider())
    }
}

#[derive(Debug, Clone, Component)]
pub struct TankSpeed {
    pub linear: f32,
    pub angular: f32,
    pub fire_cooldown: Duration,
}
