use std::f32::consts::PI;

use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{
        AsBindGroup, Face, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
        SpecializedMeshPipelineError,
    },
};
use isosurface::{marching_cubes::MarchingCubes, source::CentralDifference};

use crate::camera;

pub struct HeartPlugin;

pub const HEART_UTILS_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 1752015359394029744);

impl Plugin for HeartPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HEART_UTILS_SHADER_HANDLE,
            "shaders/utils.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, camera::pan_orbit_camera);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    let wall_color = Color::rgb_u8(249, 29, 187);
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: standard_materials.add(wall_color.into()),
        ..default()
    });

    // left wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(PI / 2.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
        transform,
        material: standard_materials.add(StandardMaterial {
            base_color: wall_color.into(),
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // back (right) wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate_x(PI / 2.);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
        transform,
        material: standard_materials.add(StandardMaterial {
            base_color: wall_color.into(),
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // heart
    let pitch = Quat::from_rotation_x(-PI / 2.);
    let mut transform = Transform::from_xyz(-0.4, 0.7, 1.2);
    transform.rotation = pitch;
    commands.spawn((MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(HeartMesh::new(300, 2.))),
        transform,
        material: custom_materials.add(CustomMaterial { color: Color::RED }),
        ..default()
    },));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 4.0, 4.0),
        ..default()
    });

    // camera
    camera::spawn_camera(commands);
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/heart.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/prepass.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/heart.wgsl".into()
    }

    fn specialize(
        _: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
        _: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        descriptor.primitive.cull_mode = Some(Face::Front);
        Ok(())
    }
}

struct HeartSource {}

impl isosurface::source::Source for HeartSource {
    fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        // scale
        let (x, y, z) = (x - 0.5, y - 0.5, z - 0.5);
        let (x, y, z) = (x * 3., y * 3., z * 3.);

        // equation
        let a = x * x + 9. / 4. * y * y + z * z - 1.;
        a * a * a - x * x * z * z * z - 9. / 200. * y * y * z * z * z
    }
}

struct HeartMesh {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl HeartMesh {
    fn new(sectors: usize, size: f32) -> Self {
        let mut vertex_normal = vec![];
        let mut indices = vec![];
        let source = CentralDifference::new(Box::new(HeartSource {}));
        let mut marching_cubes = MarchingCubes::new(sectors);
        marching_cubes.extract_with_normals(&source, &mut vertex_normal, &mut indices);

        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(vertex_normal.len() / 6);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertex_normal.len() / 6);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertex_normal.len() / 6);
        for i in (0..vertex_normal.len()).step_by(6) {
            let normal = [
                vertex_normal[i + 3],
                vertex_normal[i + 4],
                vertex_normal[i + 5],
            ];
            normals.push(normal);
            let (x, y, z) = (vertex_normal[i], vertex_normal[i + 1], vertex_normal[i + 2]);
            vertices.push([x * size - 1., y * size - 1., z * size - 1.]);
            uvs.push([0.0, 0.0]);
        }

        Self {
            vertices,
            normals,
            indices,
            uvs,
        }
    }
}

impl From<HeartMesh> for Mesh {
    fn from(h: HeartMesh) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, h.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, h.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, h.uvs);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(h.indices)));
        mesh
    }
}
