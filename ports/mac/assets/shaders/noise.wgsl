// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::mesh_view_bindings globals
// https://github.com/bevyengine/bevy/blob/v0.11.3/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl
#import bevy_pbr::mesh_vertex_output MeshVertexOutput

fn random(in: vec2<f32>) -> f32 {
    return fract(sin(dot(in, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn noise(in: vec2<f32>) -> f32 {
    let i = floor(in);
    let f = fract(in);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(mix(random(i + vec2<f32>(0.0, 0.0)), random(i + vec2<f32>(1.0, 0.0)), u.x),
               mix(random(i + vec2<f32>(0.0, 1.0)), random(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 5.0;
    let time = globals.time;
    let noise = noise(uv + vec2<f32>(time, time));
    let color = vec3<f32>(noise, noise, noise);
    return vec4<f32>(color, 1.0);

}