#import bevy_pbr::{
    forward_io::VertexOutput,
    pbr_functions as fns,
    pbr_types,
    mesh_bindings::mesh,
    mesh_view_bindings::globals,
    mesh_functions,
    view_transformations,
}
#import heart::utils::pos_animate

struct CustomMaterial {
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> material: CustomMaterial;

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

    // out.uv = vertex.uv;

    return out;
}



@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var pbr_input: pbr_types::PbrInput = pbr_types::pbr_input_new();

    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.frag_coord = in.position;
    pbr_input.material.base_color = material.color;
    pbr_input.is_orthographic = true;
    pbr_input.material.perceptual_roughness = 0.4;
    pbr_input.material.reflectance = 0.5;
    pbr_input.material.metallic = 1.0;
    pbr_input.diffuse_occlusion = vec3(1.0);

    return fns::apply_pbr_lighting(pbr_input);
}
