use bevy::prelude::*;
mod plugins;
mod utils;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use plugins::{
    ballgame::BallGamePlugin, coordinate::CoordinatePlugin, debug_camera::DebugCameraPlugin,
    fps::FPSPlugin, helmet::HelmetPlugin, scroll::ScrollPlugin, physics3d::Physics3dPlugin, particles::PraticlesPlugin, shader_test::ShaderPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            // FPSPlugin,
            // HelmetPlugin,
            // BallGamePlugin,
            // CoordinatePlugin,
            // DebugCameraPlugin,
            // bevy_flycam::NoCameraPlayerPlugin,
            // Physics3dPlugin,
            // ScrollPlugin,
            // PraticlesPlugin,
            ShaderPlugin,
        ))
        // .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(12.0, 12.0, 12.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        bevy_flycam::FlyCam,
        bevy_mod_picking::backends::raycast::RaycastPickCamera::default(),
        bevy_transform_gizmo::GizmoPickSource::default(),
    ));
}
