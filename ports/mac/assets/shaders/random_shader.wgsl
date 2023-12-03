// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::mesh_view_bindings globals
// https://github.com/bevyengine/bevy/blob/v0.11.3/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl
#import bevy_pbr::mesh_vertex_output MeshVertexOutput

fn random(in: vec2<f32>) -> f32 {
    return fract(sin(dot(in, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 10.0;
    let ipos = floor(uv);
    let fpos = fract(uv);
    let color = vec3<f32>(random(ipos));

    return vec4<f32>(color, 1.0);
}