
//! A shader that uses dynamic data like the time since startup.
//! The time data is in the globals binding which is part of the `mesh_view_bindings` shader import.

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{render_resource::*, texture}, window::PrimaryWindow, sprite::{MaterialMesh2dBundle, Mesh2dHandle, Material2d, Material2dPlugin}, asset::ChangeWatcher,
};

use bevy_debug_camera::DebugCamera;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use mac::plugins::{coordinate::CoordinatePlugin, debug_camera::DebugCameraPlugin};

fn main() {
    App::new()
        .add_plugins(CoordinatePlugin)
        .add_plugins((DefaultPlugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(std::time::Duration::from_secs(1)),
            ..Default::default()
        }), Material2dPlugin::<CustomMaterial>::default()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DebugCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(100.0, 100.0, 0.0),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle::<CustomMaterial> {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(200.)),
        material: materials.add(CustomMaterial {}),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/hsb_polar.wgsl".into()
    }
}