#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_bindings::mesh,
    // mesh_view_bindings::globals,
    mesh_functions,
    view_transformations
}
// temp fix:
// wait for https://github.com/bevyengine/bevy/pull/12032
#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;
// #import bevy_render::globals
#import heart::utils::pos_animate

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var vertex = vertex_no_morph;
    vertex.position = pos_animate(vertex.position, globals.time);
    var model = mesh_functions::get_model_matrix(vertex_no_morph.instance_index);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex_no_morph.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.position = view_transformations::position_world_to_clip(out.world_position.xyz);

    out.uv = vertex.uv;
    return out;
}