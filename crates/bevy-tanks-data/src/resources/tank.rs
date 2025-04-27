use bevy::{prelude::*, utils::HashMap};

use crate::TankController;

#[derive(Debug, Default, Resource)]
pub struct TankColors(pub HashMap<Entity, Color>);

#[derive(Debug, Resource)]
pub struct TankAssets {
    pub scene_root: SceneRoot,
    pub fire_animation_graph: Handle<AnimationGraph>,
    pub fire_animation_node: AnimationNodeIndex,
}

impl FromWorld for TankAssets {
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
