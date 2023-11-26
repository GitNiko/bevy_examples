use bevy::prelude::*;

pub struct CoordinatePlugin;

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
}

const Length:f32 = 50.0;

fn setup(
    mut gizmos: Gizmos,
) {
    // 创建X轴线段
    gizmos.ray(
        Vec3::ZERO,
        Vec3::X * Length,
        Color::RED,
    );
    gizmos.ray(
        Vec3::ZERO,
        Vec3::X * -Length,
        Color::RED.with_a(0.2),
    );

    // 创建Y轴线段
    gizmos.ray(
        Vec3::ZERO,
        Vec3::Y * Length,
        Color::GREEN,
    );
    gizmos.ray(
        Vec3::ZERO,
        Vec3::Y * -Length,
        Color::GREEN.with_a(0.2),
    );

    // 创建Z轴线段
    gizmos.ray(
        Vec3::ZERO,
        Vec3::Z * Length,
        Color::BLUE,
    );
    gizmos.ray(
        Vec3::ZERO,
        Vec3::Z * -Length,
        Color::BLUE.with_a(0.2),
    );
}
