use bevy::prelude::*;

/// This example illustrates how to load a GLTF file and display it in a scene.
pub struct HelmetPlugin;

impl Plugin for HelmetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn((
    //     Camera3dBundle {
    //         transform: Transform::from_xyz(0.7, 0.7, 1.0)
    //             .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    //         ..default()
    //     },
    //     EnvironmentMapLight {
    //         diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
    //         specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
    //     },
    // ));
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
        ..default()
    });
}