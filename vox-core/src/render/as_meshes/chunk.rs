use std::collections::{BTreeSet, HashMap};

use binary_greedy_meshing::{self as bgm, CS_P3};
use cgmath::{Quaternion, Vector3, Zero};
use wgpu::util::DrawIndexedIndirectArgs;

use crate::{render::{mesh::{AsMesh, MeshPosition}, multi_indexed_mesh::AsMultiIndexedMesh, face_orientation::FaceOrientation, vertex::{Index, Vertex}}, voxel_position::VoxelPosition, voxel_registry::{VoxelRegistry, VoxelType, VoxelTypeIdentifier}, InstanceData};

use super::face::FaceDescriptor;

const MASK_6: u64 = 0b111111;

const VERTICES: [Vertex ; 24] = [
    // UP
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    // DOWN
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    // RIGHT
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.0, -1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, -1.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    // LEFT
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.0, 1.0, 1.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    // FRONT
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    // BACK
    Vertex {
        position: [1.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        normal: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
];

const INDICES: [Index ; 36] = [
    // UP
    0, 1, 2, 0, 2, 3,
    // DOWN
    1, 0, 3, 1, 3, 2,
    // RIGHT
    3, 2, 1, 3, 1, 0,
    // LEFT
    0, 1, 2, 0, 2, 3,
    // FRONT
    0, 3, 1, 1, 3, 2,
    // BACK
    0, 1, 2, 0, 2, 3,
];

impl AsMultiIndexedMesh for Chunk {
    fn vertices(&self) -> &[Vertex] {
        &VERTICES
    }

    fn indices(&self) -> &[Index] {
        &INDICES
    }

    //
    fn instances(&self) -> Vec<InstanceData> {
        let mut vec = Vec::with_capacity(self.face_map.values().len());
        for (face, positions) in self.face_map.iter() {
            positions.iter()
                .for_each(|position| {
                    let position: Vector3<f32> = (*position).into();
                    let rotation = match face.orientation {
                        FaceOrientation::FRONT => Quaternion::zero(),
                        FaceOrientation::BACK => Quaternion::zero(),
                        FaceOrientation::LEFT => Quaternion::zero(),
                        FaceOrientation::RIGHT => Quaternion::zero(),
                        FaceOrientation::UP => Quaternion::zero(),
                        FaceOrientation::DOWN => Quaternion::zero(),
                    };

                    let instance_data = InstanceData {
                        position,
                        rotation
                    };

                    vec.push(instance_data);
                });
        }

        vec
    }

    fn indirect_indexed_args(&self) -> Vec<DrawIndexedIndirectArgs> {
        let mut last_instance_idx = 0;

        self.face_map.iter()
            .map(|(face, positions)| {
                let instance_count = positions.len() as u32;
                let base_vertex = 4 * face.orientation.index() as i32;
                let first_index = 6 * face.orientation.index();
                let first_instance = last_instance_idx;

                last_instance_idx += instance_count;

                DrawIndexedIndirectArgs {
                    index_count: 6,
                    instance_count,
                    first_index,
                    base_vertex,
                    first_instance,
                }
            }).collect()
    }

    fn material_id(&self) -> usize {
        1
    }

    fn draw_count(&self) -> u32 {
        self.face_map.len() as u32
    }
}

#[derive(Debug)]
pub struct Chunk {
    voxels: [VoxelTypeIdentifier ; CS_P3],
    mesh_data: bgm::MeshData,
    face_map: HashMap<FaceDescriptor, Vec<MeshPosition>>,
    voxel_registry: VoxelRegistry,
}

impl Default for Chunk {
    fn default() -> Chunk {
        let voxels = [0 ; CS_P3];
        let mesh_data = bgm::MeshData::new();
        let face_map = HashMap::new();
        let voxel_registry = VoxelRegistry::default();

        Self {
            voxels,
            mesh_data,
            face_map,
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

    pub fn update_faces(&mut self) {
        self.mesh_data.clear();
        self.face_map.clear();
        bgm::mesh(&self.voxels, &mut self.mesh_data, BTreeSet::default());

        for (bgm_orientation, bgm_faces) in self.mesh_data.quads.iter().enumerate() {
            let orientation = FaceOrientation::from_bgm(bgm_orientation);
            for bgm_face in bgm_faces.iter() {
                let x = bgm_face & MASK_6;
                let y = (bgm_face >> 6) & MASK_6;
                let z = (bgm_face >> 12) & MASK_6;
                let width = (bgm_face >> 18) & MASK_6;
                let height = (bgm_face >> 24) & MASK_6;
                let voxel_id = (bgm_face >> 32) as u16;

                let voxel_type = self.voxel_registry
                    .get_type(voxel_id)
                    .unwrap();

                let x = x as f32;
                let y = y as f32;
                let z = z as f32;

                let width = width as u32;
                let height = height as u32;

                // TODO: this material id will not be used
                // we are rendering this chunk indirectly
                // so materials cannot be changed during rendering
                // we need to store different arrays for each voxel type
                // MAYBE MAKE THIS AN OPTION FOR THE MESH TO HAVE
                let material_id = 0;

                let descriptor = FaceDescriptor {
                    orientation,
                    width,
                    height,
                    material_id,
                };

                match self.face_map.get_mut(&descriptor) {
                    Some(vec) => vec.push((x, y, z)),
                    None => {
                        let vec = vec![(x, y, z)];
                        self.face_map.insert(descriptor, vec);
                    }
                }
            }
        }
    }
}
