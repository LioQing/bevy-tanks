use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Default, Resource)]
pub struct TankColors(pub HashMap<Entity, Color>);

#[derive(Debug, Resource)]
pub struct TankAssets {
    pub scene: Handle<Scene>,
    pub fire_animation: Handle<AnimationClip>,
    pub fire_animation_graph: Handle<AnimationGraph>,
    pub fire_animation_node: AnimationNodeIndex,
}

impl FromWorld for TankAssets {
    fn from_world(world: &mut World) -> Self {
        let scene = world.load_asset(GltfAssetLabel::Scene(0).from_asset("models/tank.glb"));
        let fire_animation =
            world.load_asset(GltfAssetLabel::Animation(0).from_asset("models/tank.glb"));
        let (fire_animation_graph, fire_animation_node) =
            AnimationGraph::from_clip(fire_animation.clone());

        Self {
            scene,
            fire_animation,
            fire_animation_graph: world.add_asset(fire_animation_graph),
            fire_animation_node,
        }
    }
}
