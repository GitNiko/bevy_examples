use bevy::app::Startup;
use bevy::asset::Assets;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::math::Vec3;
use bevy::pbr::MaterialMeshBundle;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::shape;
use bevy::transform::components::Transform;
use bevy::{
    asset::Asset,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin},
    prelude::{
        default, in_state, App, AssetServer, Color, Commands, Component, Input, IntoSystemConfigs,
        KeyCode, NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, State, States, TextBundle,
        Update, With,
    },
    reflect::TypePath,
    render::{
        mesh::{Mesh, MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
            VertexFormat,
        },
    },
    text::{Text, TextSection, TextStyle},
    ui::{PositionType, Style, Val},
};

const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app
            // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            // .insert_resource(WinitSettings::desktop_app())
            .add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    mesh.insert_attribute(
        ATTRIBUTE_BLEND_COLOR,
        // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
        vec![[1.0, 0.0, 0.0, 1.0]; 24],
    );

    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {
            color: Color::PURPLE,
        }),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(AsBindGroup, Debug, Clone, TypePath, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}
impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }

    fn alpha_mode(&self) -> bevy::prelude::AlphaMode {
        bevy::prelude::AlphaMode::Opaque
    }

    fn depth_bias(&self) -> f32 {
        0.0
    }

    fn prepass_vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }

    fn prepass_fragment_shader() -> ShaderRef {
        ShaderRef::Default
    }
}