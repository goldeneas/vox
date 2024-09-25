use std::collections::{BTreeSet, HashMap};

use bevy_ecs::system::IntoSystem;
use binary_greedy_meshing::{self as bgm, CS_P3};
use wgpu::util::DrawIndexedIndirectArgs;

use crate::{render::{face_primitive::{FaceDirection, FacePrimitive}, material::Material, mesh::{AsMesh, Mesh}, multi_indexed_mesh::AsMultiIndexedMesh, vertex::Vertex}, AsModel, InstanceData, Model};

use super::{voxel_position::VoxelPosition, voxel_registry::{VoxelRegistry, VoxelType, VoxelTypeIdentifier}};

const MASK_6: u64 = 0b111111;

//impl AsModel for Chunk {
//    fn to_model(&self, materials: Vec<Material>) -> Model {
//        //let meshes: Vec<Mesh> = self.faces
//        //    .iter()
//        //    .map(|(voxel_type, faces)| {
//        //        faces.into()
//        //    }).collect::<Vec<_>>();
//
//        let meshes = self.faces.values()
//            .flatten().map(|face| { face.to_mesh(MaterialId::Index(0))})
//            .collect::<Vec<_>>();
//
//        Model {
//            meshes,
//            materials,
//            name: String::from("Chunk Model")
//        }
//    }
//}

impl AsMultiIndexedMesh for Chunk {
    fn vertices(&self) -> Vec<Vertex> {
        vec![
            FacePrimitive::vertices(FaceDirection::UP, 1.0, 1.0),
            FacePrimitive::vertices(FaceDirection::DOWN, 1.0, 1.0),
            FacePrimitive::vertices(FaceDirection::RIGHT, 1.0, 1.0),
            FacePrimitive::vertices(FaceDirection::LEFT, 1.0, 1.0),
            FacePrimitive::vertices(FaceDirection::FRONT, 1.0, 1.0),
            FacePrimitive::vertices(FaceDirection::BACK, 1.0, 1.0),
        ].iter().flatten().copied().collect()
    }

    fn indices(&self) -> Vec<u32> {
        vec![
            FacePrimitive::indices(FaceDirection::UP),
            FacePrimitive::indices(FaceDirection::DOWN),
            FacePrimitive::indices(FaceDirection::RIGHT),
            FacePrimitive::indices(FaceDirection::LEFT),
            FacePrimitive::indices(FaceDirection::FRONT),
            FacePrimitive::indices(FaceDirection::BACK),
        ].iter().flatten().copied().collect()
    }

    fn instances(&self) -> Vec<InstanceData> {
        vec![InstanceData::from_position((0.0, 0.0, 0.0))]
    }

    fn indirect_indexed_args(&self) -> Vec<DrawIndexedIndirectArgs> {
        self.faces.iter()
            .map(|face| {
                DrawIndexedIndirectArgs {
                    index_count: 6,
                    instance_count: face.instances().len() as u32,
                    first_index: 6 * FaceDirection::to_index(face.direction),
                    base_vertex: 4 * FaceDirection::to_index(face.direction) as i32,
                    first_instance: 0,
                }
            }).collect()
    }

    fn material_id(&self) -> usize {
        0
    }

    fn draw_count(&self) -> u32 {
        self.faces.len() as u32
    }
}

#[derive(Debug)]
pub struct Chunk {
    voxels: [VoxelTypeIdentifier ; CS_P3],
    mesh_data: bgm::MeshData,
    //faces: HashMap<VoxelType, Vec<FacePrimitive>>,
    faces: Vec<FacePrimitive>,
    voxel_registry: VoxelRegistry,
}

impl Default for Chunk {
    fn default() -> Chunk {
        let voxels = [0 ; CS_P3];
        let mesh_data = bgm::MeshData::new();
        let faces = Vec::new();
        let voxel_registry = VoxelRegistry::default();

        Self {
            voxels,
            mesh_data,
            faces,
            voxel_registry,
        }
    }
}

impl Chunk {
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

    pub fn get_meshes(&self) -> &Vec<impl AsMesh> {
        &self.faces
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

                // INVARIANT: we can cast to 16 bits because
                // we can only set a voxel's id with a number using 16 bits.
                // the topmost 16 bits are not used (probably)
                let voxel_id = (bgm_face >> 32) as u16;

                let voxel_type = self.voxel_registry
                    .get_type(voxel_id)
                    .unwrap();

                let x = x as f32;
                let y = y as f32;
                let z = z as f32;

                let width = width as f32;
                let height = height as f32;

                let material_id = 0;

                let face = FacePrimitive {
                    direction,
                    position: (x, y, z),
                    width,
                    height,
                    material_id,
                };

                self.faces.push(face);

                //let face_vector = self.faces.get_mut(&voxel_type);
                //match face_vector {
                //    Some(vector) => vector.push(face),
                //    None => {
                //        let vector = vec![face];
                //        self.faces.insert(voxel_type, vector);
                //    }
                //}
            }
        }
    }
}
