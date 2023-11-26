use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_hanabi::prelude::*;

use crate::utils::hex_to_vec4;

pub struct PraticlesPlugin;

impl Plugin for PraticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, follow_mouse_ratation);
    }
}

const RATATION_DIRECTION_PROPS_NAME: &str = "RatationDirection";

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let water_color: bevy::prelude::Vec4 = hex_to_vec4("#d4f1f9").unwrap();
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, water_color);
    gradient.add_key(1.0, Vec4::splat(0.));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.3, Vec2::new(0.2, 0.02));
    size_gradient.add_key(1.0, Vec2::splat(0.0));


    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.1),
        dimension: ShapeDimension::Volume,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.),
    };


    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(1.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.prop(RATATION_DIRECTION_PROPS_NAME);
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        Spawner::rate(500.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect")
    .with_property(RATATION_DIRECTION_PROPS_NAME, Vec3::new(0., 1., 0.).into())
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(SizeOverLifetimeModifier { gradient: size_gradient, screen_space_size: false })
    .render(ColorOverLifetimeModifier { gradient });


    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands.spawn((Name::new("MyEffect"), ParticleEffectBundle {
        effect: ParticleEffect::new(effect_handle),
        transform: Transform::from_translation(Vec3::new(1., 5., 0.)),
        ..Default::default()
    }, RatationDirection));
}

#[derive(Component)]
struct RatationDirection;

fn follow_mouse_ratation(
    mut query: Query<&mut CompiledParticleEffect, With<RatationDirection>>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    if let Ok(window) = window.get_single() {
        if let Some(mouse_pos) = window.cursor_position() {
                let spawning_pos = Vec3::new(mouse_pos.x - window.width() / 2.0, - mouse_pos.y + window.height() / 2.0, 0.);
                println!("spawning_pos: {:?}", spawning_pos);
                let mut effect = query.single_mut();
                effect.set_property(RATATION_DIRECTION_PROPS_NAME, spawning_pos.into());
        }
    }
}


pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<bevy::window::PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 100.0;
    camera.projection.scaling_mode = ScalingMode::FixedVertical(1.);
    commands.spawn(camera);

    // commands.spawn(Camera2dBundle {
    //     // transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    //     ..default()
    // });
}