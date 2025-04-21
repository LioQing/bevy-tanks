use std::borrow::Cow;

use bevy::{pbr::NotShadowCaster, prelude::*, utils::HashMap};
use rand_distr::Normal;

use crate::*;

#[derive(Debug, Default, Resource)]
pub struct TankColors(pub HashMap<Entity, Color>);

#[derive(Debug, Resource)]
pub struct TankPrefab {
    pub scene_root: SceneRoot,
    pub fire_animation_graph: Handle<AnimationGraph>,
    pub fire_animation_node: AnimationNodeIndex,
    pub smoke_vfx: SmokeVfx,
}

impl TankPrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        TankColors(tank_colors): &mut TankColors,
        name: impl Into<Cow<'static, str>>,
        position: Vec3,
        color: Color,
    ) -> EntityCommands<'a> {
        let mut entity_command = commands.spawn((
            Name::new(name),
            TankController::default(),
            Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
            self.scene_root.clone(),
        ));

        entity_command.with_children(|children| {
            children.spawn((
                Name::new("Left Smoke VFX"),
                Transform::from_xyz(-1.0, 0.0, 0.8),
                self.smoke_vfx.clone(),
            ));
            children.spawn((
                Name::new("Right Smoke VFX"),
                Transform::from_xyz(1.0, 0.0, 0.8),
                self.smoke_vfx.clone(),
            ));
        });

        tank_colors.insert(entity_command.id(), color);

        entity_command
    }
}

impl FromWorld for TankPrefab {
    fn from_world(world: &mut World) -> Self {
        let scene_root =
            TankController::scene_root(world.get_resource::<AssetServer>().expect("asset server"));
        let fire_animation = TankController::fire_animation_clip(
            world.get_resource::<AssetServer>().expect("asset server"),
        );
        let (fire_animation_graph, fire_animation_node) =
            AnimationGraph::from_clip(fire_animation.clone());

        Self {
            scene_root,
            fire_animation_graph: world.add_asset(fire_animation_graph),
            fire_animation_node,
            smoke_vfx: SmokeVfx {
                timer: Timer::from_seconds(0.02, TimerMode::Repeating),
                radius: Normal::new(0.5, 0.2).expect("normal distribution"),
                velocity: [
                    Normal::new(0.0, 1.0).expect("normal distribution"),
                    Normal::new(1.0, 1.0).expect("normal distribution"),
                    Normal::new(0.0, 1.0).expect("normal distribution"),
                ],
                lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
                time_scale: 0.0,
            },
        }
    }
}

#[derive(Debug, Resource)]
pub struct BulletPrefab {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub smoke_vfx: fn(Dir3) -> SmokeVfx,
}

impl BulletPrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        transform: Transform,
    ) -> EntityCommands<'a> {
        commands.spawn((
            Name::new("Bullet"),
            Bullet,
            transform,
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
            Bullet::velocity(transform.forward()),
            (self.smoke_vfx)(transform.forward()),
        ))
    }
}

impl FromWorld for BulletPrefab {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: Bullet::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: Bullet::material(&mut world.get_resource_mut().expect("materials")),
            smoke_vfx: |forward| SmokeVfx {
                timer: Timer::from_seconds(0.02, TimerMode::Repeating),
                radius: Normal::new(0.75, 0.2).expect("normal distribution"),
                velocity: forward
                    .to_array()
                    .map(|v| Normal::new(v, 1.0).expect("normal distribution")),
                lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
                ..default()
            },
        }
    }
}

#[derive(Debug, Resource)]
pub struct TankExplosionPrefab {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub radius: f32,
    pub offset: Vec3,
    pub light: ExplosionLight,
    pub point_light: PointLight,
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
                radius: self.radius,
                entity: Some(entity),
                ..default()
            },
            Transform::from_translation(position + self.offset).with_scale(Vec3::ONE * 1e-3),
            self.light.clone(),
            self.point_light.clone(),
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
            radius: 2.5,
            offset: Vec3::Y,
            light: ExplosionLight {
                range: Some(25.0),
                radius: Some(2.5),
            },
            point_light: PointLight {
                color: Color::srgb(1.0, 0.5, 0.0),
                intensity: 2e6,
                range: 0.0,
                radius: 0.0,
                shadows_enabled: true,
                ..default()
            },
        }
    }
}

#[derive(Debug, Resource)]
pub struct SmokeVfxParticlePrefab {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl SmokeVfxParticlePrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        timer: Timer,
        radius: f32,
        velocity: Vec3,
        position: Vec3,
    ) -> EntityCommands<'a> {
        commands.spawn((
            Name::new("Smoke VFX"),
            SmokeVfxParticle {
                timer,
                radius,
                velocity,
            },
            Transform::from_translation(position).with_scale(Vec3::ONE * 1e-3),
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
        ))
    }
}

impl FromWorld for SmokeVfxParticlePrefab {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: SmokeVfxParticle::mesh(&mut world.get_resource_mut().expect("meshes")),
            material: SmokeVfxParticle::material(&mut world.get_resource_mut().expect("materials")),
        }
    }
}
