#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::prepass_bindings globals
#import heart::utils pos_animate

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex_no_morph: Vertex) -> MeshVertexOutput {
    var out: MeshVertexOutput;
    var vertex = vertex_no_morph;
    vertex.position = pos_animate(vertex.position, globals.time);
    var model = mesh.model;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.position = mesh_functions::mesh_position_world_to_clip(out.world_position);

    out.uv = vertex.uv;
    return out;
}