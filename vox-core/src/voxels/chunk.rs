use std::{collections::BTreeSet, sync::Arc};

use binary_greedy_meshing::{self as bgm, CS_P3};

use crate::{render::{face::{FaceDirection, FaceMesh}, mesh::{AsMesh, Mesh}}, AsModel, Model, Texture};

use super::{voxel_position::VoxelPosition, voxel_registry::{VoxelRegistry, VoxelType, VoxelTypeIdentifier}};

const MASK_6: u64 = 0b111111;

impl AsModel for Chunk {
    fn to_model(&self, device: &wgpu::Device) -> Arc<Model> {
        let vertices = self.faces.iter()
            .flat_map(FaceMesh::compute_vertices)
            .collect::<Vec<_>>();

        let mut face_counter = 0;
        let indices = self.faces.iter()
            .flat_map(|face| {
                let mut fi = face.indices();
                fi.iter_mut().for_each(|index| *index += face_counter * 6);
                face_counter += 1;
                fi
            }).collect::<Vec<_>>();

        Mesh::new(device, &vertices, &indices, "Chunk Mesh")
    }

    fn into_model(self, device: &wgpu::Device) -> Arc<crate::Model> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Chunk {
    voxels: [VoxelTypeIdentifier ; CS_P3],
    mesh_data: bgm::MeshData,
    faces: Vec<FaceMesh>,
}

impl Chunk {
    pub fn new() -> Chunk {
        let voxels = [0 ; CS_P3];
        let mesh_data = bgm::MeshData::new();
        let faces = Vec::new();

        Self {
            voxels,
            mesh_data,
            faces,
        }
    }

    pub fn set_voxel_type_at(&mut self,
        position: VoxelPosition,
        voxel_type: VoxelTypeIdentifier
    ) {
        let idx = position.index();
        self.voxels[idx] = voxel_type;
    }

    pub fn get_voxel_type_at(&self,
        position: VoxelPosition,
        voxel_registry: &VoxelRegistry
    ) -> Option<VoxelType> {
        let idx = position.index();
        let voxel_id = self.voxels[idx];
        
        voxel_registry.get_type(voxel_id)
    }

    pub fn update_faces(&mut self) {
        self.mesh_data.clear();
        self.faces.clear();
        bgm::mesh(&self.voxels, &mut self.mesh_data, BTreeSet::default());

        for (bgm_direction, bgm_faces) in self.mesh_data.quads.iter().enumerate() {
            let direction = FaceDirection::from_bgm(bgm_direction);
            for bgm_face in bgm_faces.iter() {
                let x = bgm_face & MASK_6;
                let y = (bgm_face >> 6) & MASK_6;
                let z = (bgm_face >> 12) & MASK_6;
                let width = (bgm_face >> 18) & MASK_6;
                let height = (bgm_face >> 24) & MASK_6;
                let voxel_id = bgm_face >> 32;

                let x = x as f32;
                let y = y as f32;
                let z = z as f32;

                let width = width as u32;
                let height = height as u32;

                let face = FaceMesh::new(direction,
                    (x, y, z),
                    width as f32,
                    height as f32,
                );

                self.faces.push(face);
            }
        }
    }
}
