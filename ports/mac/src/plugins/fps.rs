use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::{
        default, in_state, App, AssetServer,
         Color, Commands, Component, Input, IntoSystemConfigs,
        KeyCode, NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut,
        States, TextBundle, Update, With, State,
    },
    text::{Text, TextSection, TextStyle},
    ui::{
        PositionType, Style,
        Val,
    },
};

use crate::utils::despawn_screen;

pub struct FPSPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum ConsoleState {
    #[default]
    Close,
    Open,
}

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            // .insert_resource(WinitSettings::desktop_app())
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_state::<ConsoleState>()
            .add_systems(Update, console_system)
            .add_systems(OnEnter(ConsoleState::Open), setup_fps)
            .add_systems(
                Update,
                fps_counter_system.run_if(in_state(ConsoleState::Open)),
            )
            .add_systems(OnExit(ConsoleState::Open), despawn_screen::<FPSText>);
    }
}
#[derive(Component)]
struct FPSText;

fn fps_counter_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FPSText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.value() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
            // if let Some(value) = fps.smoothed() {
            //     // Update the value of the second section
            //     text.sections[1].value = format!("{value:.2}");
            //     println!("fps: {}", value)
            // }
        }
    }
}

fn console_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_console_state: ResMut<NextState<ConsoleState>>,
    console_state: Res<State<ConsoleState>>,
) {
    if keyboard_input.pressed(KeyCode::ControlLeft) && keyboard_input.just_released(KeyCode::F) {
        println!("pressed ctrl + f, {:?}", console_state.get());
        if ConsoleState::Close == *console_state.get() {
            next_console_state.set(ConsoleState::Open);
        } else {
            next_console_state.set(ConsoleState::Close);
        }
    }
}

fn setup_fps(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 14.0,
                    color: Color::rgba(255.0, 0.0, 0.0, 0.5),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 14.0,
                color: Color::GOLD,
                // If no font is specified, it will use the default font.
                ..default()
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        FPSText,
    ));
}
