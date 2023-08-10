#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::pbr_functions as fns
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::mesh_view_bindings globals
#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::mesh_functions as mesh_functions
#import heart::utils pos_animate

struct CustomMaterial {
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> material: CustomMaterial;

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

    // out.uv = vertex.uv;

    return out;
}



@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var pbr_input: fns::PbrInput = fns::pbr_input_new();

    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.frag_coord = in.position;
    pbr_input.material.base_color = material.color;
    pbr_input.is_orthographic = true;
    pbr_input.material.perceptual_roughness = 0.4;
    pbr_input.material.reflectance = 0.5;
    pbr_input.material.metallic = 1.0;
    pbr_input.occlusion = vec3(1.0);

    return fns::pbr(pbr_input);
}
