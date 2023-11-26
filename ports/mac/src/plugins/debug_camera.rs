use bevy::prelude::*;

use crate::utils::despawn_screen;

/// debug camera
pub struct DebugCameraPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum DebugCameraState {
    #[default]
    Off,
    On
}

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(bevy_debug_camera::DebugCameraPlugin::default())
        .add_state::<DebugCameraState>()
        .add_systems(Startup, despawn_screen::<bevy_debug_camera::DebugCamera>)
        .add_systems(OnEnter(DebugCameraState::On), setup)
        .add_systems(Update, control_system)
        .add_systems(OnExit(DebugCameraState::On), despawn_screen::<bevy_debug_camera::DebugCamera>);
    

    }
}

fn setup(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    for entity in query.iter() {
        //  eqauls Camera3dBundle.looking_at(postion + fwd, up)
        commands.entity(entity).insert(bevy_debug_camera::DebugCamera {
            position: Vec3::new(0.7, 0.7, 1.0),
            up: Vec3::Y,
            fwd: Vec3::new(-0.7, -0.4, -1.0),
            ..default()
        });
    }
}



fn control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_console_state: ResMut<NextState<DebugCameraState>>,
    console_state: Res<State<DebugCameraState>>,
    mut debug_camera_active: ResMut<bevy_debug_camera::DebugCameraActive>,
    mut windows: Query<&mut Window>,
) {
    debug_camera_active.gamepad = false;
    // debug_camera_active.keymouse = false;
    if let Some(mut window) = windows.iter_mut().next() {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        window.cursor.visible = true;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) && keyboard_input.just_released(KeyCode::R) {
        println!("pressed ctrl + r, {:?}", console_state.get());
        if DebugCameraState::Off == *console_state.get() {
            if let Some(mut window) = windows.iter_mut().next() {
                window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
            next_console_state.set(DebugCameraState::On);
        } else {
            if let Some(mut window) = windows.iter_mut().next() {
                window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
                window.cursor.visible = true;
            }
            debug_camera_active.keymouse = true;
            next_console_state.set(DebugCameraState::Off);
        }
    }
}

