use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;


/// This example illustrates how to load a GLTF file and display it in a scene.
pub struct Physics3dPlugin;

impl Plugin for Physics3dPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(bevy_transform_gizmo::TransformGizmoPlugin::new(
                Quat::from_rotation_y(-0.2), // Align the gizmo to a different coordinate system.
                                             // Use TransformGizmoPlugin::default() to align to the
                                             // scene's coordinate system.
            ),)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup)
            .add_systems(Update, print_ball_altitude);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Collider::cuboid(10.0, 0.1, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 40.0, 0.0)));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.5, 3.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(1.0, 40.0, 0.0)));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(3.0, 40.0, 0.0)));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        bevy_mod_picking::PickableBundle::default(),
        bevy_mod_picking::backends::raycast::RaycastPickTarget::default(),
        bevy_transform_gizmo::GizmoTransformable,
    )).insert(RigidBody::Dynamic).insert(Restitution::coefficient(0.7));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    // for position in positions.iter() {
    //     println!("ball altitude: {}", position.translation.y);
    // }
}