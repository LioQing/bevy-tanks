use std::borrow::Cow;

use bevy::{prelude::*, utils::HashMap};
use rand_distr::Normal;

use crate::{SmokeVfx, TankController, TankLabel};

#[derive(Debug, Default, Resource)]
pub struct TankColors(pub HashMap<Entity, Color>);

#[derive(Debug, Resource)]
pub struct TankPrefab {
    pub scene_root: SceneRoot,
    pub fire_animation_graph: Handle<AnimationGraph>,
    pub fire_animation_node: AnimationNodeIndex,
}

impl TankPrefab {
    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        tank_colors: &mut TankColors,
        name: impl Into<Cow<'static, str>>,
        label: TankLabel,
        position: Vec3,
        color: Color,
    ) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn((
            Name::new(name),
            TankController { label, ..default() },
            Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
            self.scene_root.clone(),
        ));

        let smoke_vfx = SmokeVfx {
            timer: Timer::from_seconds(0.02, TimerMode::Repeating),
            radius: Normal::new(0.5, 0.2).expect("normal distribution"),
            velocity: [
                Normal::new(0.0, 1.0).expect("normal distribution"),
                Normal::new(1.0, 1.0).expect("normal distribution"),
                Normal::new(0.0, 1.0).expect("normal distribution"),
            ],
            lifetime: Normal::new(0.5, 0.2).expect("normal distribution"),
            time_scale: 0.0,
        };

        entity_commands.with_children(|children| {
            children.spawn((
                Name::new("Left Smoke VFX"),
                Transform::from_xyz(-1.0, 0.0, 0.8),
                smoke_vfx.clone(),
            ));
            children.spawn((
                Name::new("Right Smoke VFX"),
                Transform::from_xyz(1.0, 0.0, 0.8),
                smoke_vfx,
            ));
        });

        let TankColors(tank_colors) = tank_colors;
        tank_colors.insert(entity_commands.id(), color);

        entity_commands
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
        }
    }
}
