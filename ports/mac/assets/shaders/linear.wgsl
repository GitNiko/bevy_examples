// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::mesh_view_bindings globals
// https://github.com/bevyengine/bevy/blob/v0.11.3/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl
#import bevy_pbr::mesh_vertex_output MeshVertexOutput

fn plot(st: vec2<f32>, pct: f32) -> f32 {
    return smoothstep(pct - 0.02, pct, st.y) - smoothstep(pct, pct + 0.02, st.y);
}



@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let y = smoothstep(0.1, 0.9, uv.x);
    let pct = plot(uv, y);
    let color = vec3<f32>(y);
    let newColor = (1.0 - pct) * color + pct*vec3<f32>(0.0, 1.0, 0.0);
    return vec4(newColor, 1.0);
}