use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_tanks_data::*;
use bevy_tnua::prelude::*;

mod bullet;
mod explosion;
mod player;
mod smoke_vfx;
mod tank;

pub fn plugin(app: &mut App) {
    app.init_resource::<TankColors>();
    app.init_resource::<TankPrefab>();
    app.init_resource::<BulletPrefab>();
    app.init_resource::<TankExplosionPrefab>();
    app.init_resource::<SmokeVfxParticlePrefab>();

    app.add_observer(tank::observe_scene_instance_ready);

    app.add_systems(
        Startup,
        (setup_camera, setup_floor, setup_light, player::setup),
    );
    app.add_systems(
        Update,
        (
            player::update.before(tank::update),
            tank::update.in_set(TnuaUserControlsSystemSet),
            tank::update_smoke.after(TnuaUserControlsSystemSet),
            bullet::update,
            bullet::handle_tank_collision,
            explosion::update,
            smoke_vfx::update,
            smoke_vfx::update_particle,
        ),
    );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Transform::from_xyz(0.0, 36.0, 32.0)
            .with_rotation(Quat::from_axis_angle(Vec3::X, -0.3 * std::f32::consts::PI)),
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Bloom::NATURAL,
    ));
}

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Floor"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(20.0, 20.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.5, 0.5),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Collider::cuboid(20.0, 0.0, 20.0),
        RigidBody::Fixed,
    ));
}

fn setup_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Main light"),
        Transform::from_xyz(0.0, 10.0, 0.0),
        PointLight {
            intensity: 4e6,
            range: 120.0,
            shadows_enabled: true,
            ..default()
        },
    ));
    commands.spawn((
        Name::new("Sun light"),
        Transform::from_xyz(10.0, 20.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
    ));
}
