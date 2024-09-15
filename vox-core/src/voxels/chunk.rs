use std::collections::BTreeSet;

use bevy_ecs::system::Commands;
use binary_greedy_meshing::{self as bgm, CS_P3};
use cgmath::{Quaternion, Zero};

use crate::{bundles::game_object::GameObject, render::face::{FaceDirection, FaceModel, FacePosition}, resources::asset_server::AssetServer, Texture};

use super::voxel::{VoxelRegistry, VoxelType, VoxelTypeIdentifier};

const MASK_6: u64 = 0b111111;
const MASK_XYZ: u64 = 0b111111_111111_111111;

pub struct VoxelPosition {
    position: (usize, usize, usize),
}

impl VoxelPosition {
    pub fn index(&self) -> usize {
        bgm::pad_linearize(self.position.0, self.position.1, self.position.1)
    }
}

// TODO: setting a voxel at (62 62 62) doesnt actually set it for some reason
//impl From<(usize, usize, usize)> for VoxelPosition {
//    fn from(value: (usize, usize, usize)) -> Self {
//        let x = value.0;
//        let y = value.1;
//        let z = value.2;
//
//        debug_assert!(x <= 62; "");
//    }
//}

#[derive(Debug)]
pub struct Chunk {
    data: [VoxelTypeIdentifier ; CS_P3],
    mesh_data: bgm::MeshData,
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut data = [0 ; CS_P3];
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

    pub fn faces(&self,
        asset_server: &mut AssetServer,
        mut commands: Commands,
        device: &wgpu::Device,
        queue: &wgpu::Queue
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

                println!("{:?}", direction);
                println!("{} {} {}", x, y, z);
                println!("{} {}", width, height);

                let diffuse_texture = Texture::debug(asset_server, device, queue);

                let face = FaceModel::new(direction,
                    FacePosition {
                        x,
                        y,
                        z,
                    },
                    width as f32,
                    height as f32,
                    diffuse_texture
                );

                // this is the chunk's position
                let object = GameObject::new(face, (0.0, 0.0, 0.0), Quaternion::zero(), device);
                commands.spawn(object);
            }

            //let face = FaceModel::new(asset_server, device, queue, direction);
            //commands.spawn(GameObject::debug(face, device));
        };
    }

    pub fn generate_mesh(&mut self) {
        self.mesh_data.clear();
        bgm::mesh(&self.data, &mut self.mesh_data, BTreeSet::default());
    }
}
