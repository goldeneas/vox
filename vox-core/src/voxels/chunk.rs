use std::{collections::BTreeSet, sync::Arc};

use binary_greedy_meshing::{self as bgm, CS_P3};
use egui::ahash::{HashMap, HashMapExt};

use crate::{render::{face_primitive::{FaceDirection, FacePrimitive}, material::MaterialId, mesh::{AsMesh, Mesh}}, AsModel, Model, Texture};

use super::{voxel_position::VoxelPosition, voxel_registry::{VoxelRegistry, VoxelType, VoxelTypeIdentifier}};

const MASK_6: u64 = 0b111111;

//impl AsModel for Chunk {
//    fn to_model(&mut self) -> Model {
//        let mut face_meshes = self.faces.iter_mut()
//            .map(|face| { face.to_mesh() })
//            .collect::<Vec<_>>();
//
//        let mut mesh_counter = 0;
//        face_meshes.iter_mut()
//            .for_each(|mesh| {
//                for index in mesh.indices.iter_mut() {
//                    *index += mesh_counter * 6;
//                    mesh_counter += 1;
//                }
//            });
//
//        let vertices = face_meshes.iter()
//            .flat_map(|mesh| { mesh.vertices })
//            .collect::<Vec<_>>();
//
//        let indices = face_meshes.iter()
//            .flat_map(|mesh| { mesh.indices })
//            .collect::<Vec<_>>();
//
//        let chunk_mesh = Mesh {
//            vertices,
//            indices,
//            material_id: MaterialId::Index(0),
//            name: format!("Chunk {}", "idk")
//        };
//    }
//}

#[derive(Debug)]
pub struct Chunk {
    voxels: [VoxelTypeIdentifier ; CS_P3],
    mesh_data: bgm::MeshData,
    faces: HashMap<VoxelTypeIdentifier, FacePrimitive>,
    voxel_registry: VoxelRegistry,
}

impl Chunk {
    pub fn new() -> Chunk {
        let voxels = [0 ; CS_P3];
        let mesh_data = bgm::MeshData::new();
        let faces = HashMap::new();
        let voxel_registry = VoxelRegistry::default();

        Self {
            voxels,
            mesh_data,
            faces,
            voxel_registry,
        }
    }

    pub fn set_voxel_type_at(&mut self,
        position: VoxelPosition,
        voxel_type: VoxelType
    ) {
        let idx = position.index();
        let voxel_id = self.voxel_registry.get_id(voxel_type)
            .unwrap();

        self.voxels[idx] = voxel_id;
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

                let face = FacePrimitive::new(direction,
                    (x, y, z),
                    width as f32,
                    height as f32,
                );

                self.faces.insert(voxel_id, face);
            }
        }
    }
}
