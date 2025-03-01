use bevy::{gltf::GltfMesh, prelude::*};
use parry2d::{math::Point, shape::{TriMesh, TriMeshFlags}};

use crate::GameAssets;

use super::ApartmentMesh;

pub fn setup(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    assets: Res<GameAssets>,
    meshes: Res<Assets<Mesh>>,
) {

    let mut vertices: Vec<Point<f32>> = vec![];
    let mut indices: Vec<[u32; 3]> = vec![];

    let asset = gltf_assets.get(&assets.apartment_floor).unwrap();
    for gltfmesh_handle in asset.meshes.iter() {
        let gltf_mesh = gltf_meshes.get(gltfmesh_handle).unwrap();

        for primitive in gltf_mesh.primitives.clone() {
            if let Some(mesh) = meshes.get(&primitive.mesh) {

                let verts = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap();
                vertices = verts.iter().map(|p| {
                    Point::new(p[0], p[2])
                }).collect();
                
                indices = match mesh.indices().unwrap() {
                    bevy::render::mesh::Indices::U16(items) => items.chunks(3).map(|chunk| {
                        let (a, b, c) = (chunk[0] as u32, chunk[1] as u32, chunk[2] as u32);
                        [a, b, c]
                    }).collect(),
                    
                    bevy::render::mesh::Indices::U32(items) => items.chunks(3).map(|chunk| {
                        let (a, b, c) = (chunk[0], chunk[1], chunk[2]);
                        [a, b, c]
                    }).collect(),
                };
                

            }
        }
    }

    let mut trimesh = TriMesh::new(vertices, indices).unwrap();

    trimesh.set_flags(TriMeshFlags::all()).expect("wtf");

    commands.insert_resource(ApartmentMesh(trimesh));
}