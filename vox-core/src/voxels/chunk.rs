use std::{collections::BTreeSet, sync::Arc};

use bevy_ecs::system::Commands;
use binary_greedy_meshing::{self as bgm, CS_P3};
use cgmath::{Quaternion, Zero};

use crate::{bundles::game_object::GameObject, render::face::{FaceDirection, FaceModel}, resources::asset_server::AssetServer, Texture};

use super::voxel::{VoxelRegistry, VoxelType, VoxelTypeIdentifier};

const MASK_6: u64 = 0b111111;

pub struct VoxelPosition((usize, usize, usize));

impl VoxelPosition {
    fn index(&self) -> usize {
        let position = self.0;
        bgm::pad_linearize(position.0, position.1, position.2)
    }
}

// TODO: setting a voxel at (62 62 62) doesnt actually work
impl From<(usize, usize, usize)> for VoxelPosition {
    fn from(value: (usize, usize, usize)) -> Self {
        let x = value.0;
        let y = value.1;
        let z = value.2;

        debug_assert!(x <= 62, "Tried changing a voxel out of bounds on x axis!");
        debug_assert!(y <= 62, "Tried changing a voxel out of bounds on y axis!");
        debug_assert!(z <= 62, "Tried changing a voxel out of bounds on z axis!");

        Self((x, y, z))
    }
}

#[derive(Debug)]
pub struct Chunk {
    voxels: [VoxelTypeIdentifier ; CS_P3],
    mesh_data: bgm::MeshData,
}

impl Chunk {
    pub fn new() -> Chunk {
        let voxels = [0 ; CS_P3];
        let mesh_data = bgm::MeshData::new();

        Self {
            voxels,
            mesh_data,
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

    pub fn faces(&self,
        diffuse_texture: Arc<Texture>,
        mut commands: Commands,
    ) {
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

                let face = FaceModel::new(direction,
                    (x, y, z),
                    width as f32,
                    height as f32,
                    diffuse_texture
                );

                // this is the chunk's position
                let object = GameObject::new(face, (0.0, 0.0, 0.0), Quaternion::zero(), device);
                commands.spawn(object);
            }
        };
    }

    pub fn generate_mesh(&mut self) {
        self.mesh_data.clear();
        bgm::mesh(&self.voxels, &mut self.mesh_data, BTreeSet::default());
    }
}
