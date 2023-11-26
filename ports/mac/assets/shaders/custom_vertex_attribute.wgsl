#import bevy_pbr::mesh_bindings   mesh
#import bevy_pbr::mesh_functions  mesh_position_local_to_clip
#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_vertex_output MeshVertexOutput


fn oklab_to_linear_srgb(c: vec3<f32>) -> vec3<f32> {
    let L = c.x;
    let a = c.y;
    let b = c.z;

    let l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    return vec3<f32>(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    );
}

struct CustomMaterial {
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> material: CustomMaterial;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        mesh.model,
        vec4<f32>(vertex.position, 1.0)
    );
    out.blend_color = vertex.blend_color;
    return out;
}

struct FragmentInput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let speed = 2.0;
    // The globals binding contains various global values like time
    // which is the time since startup in seconds
    let t_1 = sin(globals.time * speed * input.clip_position.x) * 0.5 + 0.5;
    let t_2 = cos(globals.time * speed * input.clip_position.y);

    return vec4<f32>(t_1, t_2, t_1, 1.0);
    // return material.color * input.blend_color;
}