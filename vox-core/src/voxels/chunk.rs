use std::collections::BTreeSet;

use binary_greedy_meshing::{self as bgm, CS_P3};

use crate::{components::multiple_instance::MultipleInstanceComponent, render::face::{Face, FaceDirection}};

use super::voxel::{VoxelRegistry, VoxelType, VoxelTypeIdentifier};

#[derive(Debug)]
pub struct Chunk {
    data: [VoxelTypeIdentifier ; CS_P3],
    mesh_data: bgm::MeshData,
}

impl Chunk {
    pub fn new() -> Chunk {
        let data = [0 ; CS_P3];
        let mesh_data = bgm::MeshData::new();

        Self {
            data,
            mesh_data,
        }
    }

    pub fn set_voxel_type_at(&mut self,
        position: (usize, usize, usize),
        voxel_type: VoxelTypeIdentifier
    ) {
        let idx = bgm::pad_linearize(position.0, position.1, position.2);
        self.data[idx] = voxel_type;
    }

    pub fn get_voxel_type_at(&self,
        position: (usize, usize, usize),
        voxel_registry: &VoxelRegistry
    ) -> Option<VoxelType> {
        let idx = bgm::pad_linearize(position.0, position.1, position.2);
        let voxel_id = self.data[idx];
        
        voxel_registry.get_type(voxel_id)
    }

    //pub fn faces(&mut self,
    //    device: &wgpu::Device,
    //) -> Vec<Face> {
    //    // capacity 6 because there will be 6 instance components
    //    // one for each face
    //    let vec = Vec::with_capacity(6);
    //    self.generate_mesh();
    //
    //    self.mesh_data.quads.iter().enumerate(|(direction, faces))| {
    //        let direction = FaceDirection::from_bgm(direction);
    //    });
    //
    //}

    fn generate_mesh(&mut self) {
        self.mesh_data.clear();
        bgm::mesh(&self.data, &mut self.mesh_data, BTreeSet::default());
    }
}
