use std::collections::BTreeSet;

use binary_greedy_meshing::{self as bgm, CS_P3};
use wgpu::util::DrawIndexedIndirectArgs;

use crate::{render::{face_direction::FaceDirection, face_primitive::FacePrimitive, mesh::AsMesh, multi_indexed_mesh::AsMultiIndexedMesh, vertex::{Index, Vertex}}, InstanceData};

use super::{voxel_position::VoxelPosition, voxel_registry::{VoxelRegistry, VoxelType, VoxelTypeIdentifier}};

const MASK_6: u64 = 0b111111;

impl AsMultiIndexedMesh for Chunk {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[Index] {
        &self.indices
    }

    fn instances(&self) -> &[InstanceData] {
       self.faces.iter()
           .flat_map(|face| {
               face.instances()
           }).collect()
    }

    fn indirect_indexed_args(&self) -> Vec<DrawIndexedIndirectArgs> {
        let mut vec = Vec::with_capacity(self.faces.len());
        for (i, face) in self.faces.iter().enumerate() {
            let arg = DrawIndexedIndirectArgs {
                index_count: 6,
                instance_count: 1,
                first_index: 6 * FaceDirection::to_index(face.direction),
                base_vertex: 4 * FaceDirection::to_index(face.direction) as i32,
                first_instance: i as u32,
            };

            vec.push(arg);
        }

        vec
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
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

impl Default for Chunk {
    fn default() -> Chunk {
        let voxels = [0 ; CS_P3];
        let mesh_data = bgm::MeshData::new();
        let faces = Vec::new();
        let voxel_registry = VoxelRegistry::default();

        // TODO: maybe make this a bit better
        let mut vertices = Vec::with_capacity(24);
        vertices.extend_from_slice(&FacePrimitive::vertices(FaceDirection::UP, 1.0, 1.0));
        vertices.extend_from_slice(&FacePrimitive::vertices(FaceDirection::DOWN, 1.0, 1.0));
        vertices.extend_from_slice(&FacePrimitive::vertices(FaceDirection::RIGHT, 1.0, 1.0));
        vertices.extend_from_slice(&FacePrimitive::vertices(FaceDirection::LEFT, 1.0, 1.0));
        vertices.extend_from_slice(&FacePrimitive::vertices(FaceDirection::FRONT, 1.0, 1.0));
        vertices.extend_from_slice(&FacePrimitive::vertices(FaceDirection::BACK, 1.0, 1.0));

        let mut indices = Vec::with_capacity(36);
        indices.extend_from_slice(&FacePrimitive::indices(FaceDirection::UP));
        indices.extend_from_slice(&FacePrimitive::indices(FaceDirection::DOWN));
        indices.extend_from_slice(&FacePrimitive::indices(FaceDirection::RIGHT));
        indices.extend_from_slice(&FacePrimitive::indices(FaceDirection::LEFT));
        indices.extend_from_slice(&FacePrimitive::indices(FaceDirection::FRONT));
        indices.extend_from_slice(&FacePrimitive::indices(FaceDirection::BACK));

        Self {
            voxels,
            mesh_data,
            faces,
            voxel_registry,
            vertices,
            indices,
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
