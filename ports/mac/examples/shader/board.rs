//! A shader that uses dynamic data like the time since startup.
//! The time data is in the globals binding which is part of the `mesh_view_bindings` shader import.

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::*,
};

use bevy_debug_camera::DebugCamera;
use mac::plugins::{debug_camera::DebugCameraPlugin, coordinate::CoordinatePlugin};

fn main() {
    App::new()
        .add_plugins(DebugCameraPlugin)
        .add_plugins(CoordinatePlugin)
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0, subdivisions: 3 })),
        material: materials.add(CustomMaterial {}),
        // rotate 90 degree
        transform: Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        ..Default::default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 0.0).looking_at(Vec3::ZERO, -Vec3::Y),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/linear.wgsl".into()
    }
}