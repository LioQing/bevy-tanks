use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Resource)]
pub struct BulletAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: world
                .get_resource_mut::<Assets<_>>()
                .expect("meshes")
                .add(Mesh::from(Sphere::new(0.2))),
            material: world
                .get_resource_mut::<Assets<_>>()
                .expect("materials")
                .add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.5, 0.0),
                    emissive: LinearRgba::rgb(100.0, 50.0, 0.0),
                    ..default()
                }),
        }
    }
}

#[derive(Debug, Resource)]
pub struct TankExplosionAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for TankExplosionAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: world
                .get_resource_mut::<Assets<_>>()
                .expect("meshes")
                .add(Mesh::from(Sphere::new(0.5))),
            material: world
                .get_resource_mut::<Assets<_>>()
                .expect("materials")
                .add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.5, 0.0),
                    perceptual_roughness: 1.0,
                    emissive: LinearRgba::rgb(100.0, 50.0, 0.0),
                    ..default()
                }),
        }
    }
}

#[derive(Debug, Resource)]
pub struct SmokeVfxParticleAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for SmokeVfxParticleAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: world
                .get_resource_mut::<Assets<_>>()
                .expect("meshes")
                .add(Mesh::from(Sphere::new(0.5))),
            material: world
                .get_resource_mut::<Assets<_>>()
                .expect("materials")
                .add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
        }
    }
}

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
