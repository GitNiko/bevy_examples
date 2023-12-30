
// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_pbr::mesh_view_bindings globals
// https://github.com/bevyengine/bevy/blob/v0.11.3/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl
#import bevy_pbr::mesh_vertex_output MeshVertexOutput

fn plot(st: vec2<f32>, pct: f32) -> f32 {
    return smoothstep(pct - 0.02, pct, st.y) - smoothstep(pct, pct + 0.02, st.y);
}

fn rgb2hsb(in: vec3<f32>) -> vec3<f32> {
    let K = vec4<f32>(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    let P = mix(vec4<f32>(in.bg, K.wz), vec4<f32>(in.gb, K.xy), step(in.b, in.g));
    let Q = mix(vec4<f32>(P.xyw, in.r), vec4<f32>(in.r, P.yzx), step(P.x, in.r));
    let D = Q.x - min(Q.w, Q.y);
    let E = 1.0e-10;
    return vec3<f32>(abs(Q.z + (Q.w - Q.y) / (6.0 * D + E)), D / (Q.x + E), Q.x);
}

fn hsb2rgb(c: vec3<f32>) -> vec3<f32> {
    // clamp(abs(mod(c.x*7.016+vec3(0.0,4.0,2.0),6.0)-3.0)-1.0,0.0,1.0 )
    var rgb = clamp(abs((c.x * 6.0 + vec3<f32>(0.0, 4.0, 2.0)) % 6.0 - 3.0) - 1.0, vec3<f32>(0.0), vec3<f32>(1.0));
    rgb = rgb * rgb * (3.0 - 2.0 * rgb);
    return c.z * mix(vec3<f32>(1.0), rgb, c.y);
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let st = in.uv;
    var color = vec3<f32>(0.0);
    color = hsb2rgb(vec3<f32>(st.x, 1.0, st.y));
    return vec4<f32>(color, 1.0);
}