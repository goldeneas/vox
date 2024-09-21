use std::sync::Arc;

use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{device_ext::VoxDeviceExt, voxels::voxel_registry::VoxelTypeIdentifier, AsModel, InstanceData, Model, Texture};

use super::{material::{Material, MaterialId}, mesh::AsMesh, vertex::Vertex};

#[derive(Debug)]
pub struct FacePrimitive {
    vertices: [Vertex ; 4],
    indices: [u32 ; 6],
    instances_data: Vec<InstanceData>,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    material_id: MaterialId,
}

impl AsMesh for FacePrimitive {
    fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    fn num_instances(&self) -> usize {
        self.instances_data.len()
    }

    fn num_indices(&self) -> usize {
        self.indices.len()
    }

    fn material_id(&self) -> MaterialId {
        self.material_id
    }
}

impl AsModel for FacePrimitive {
    fn to_model(&self, materials: Vec<Material>) -> Model {
        let material_id = self.material_id;
        let mesh = self.to_mesh(material_id);

        Model {
            meshes: vec![mesh],
            materials,
            name: String::from("Face Model"),
        }
    }
}

// TODO: maybe make this a method of chunk.rs
//impl From<&Vec<FacePrimitive>> for Mesh {
//    fn from(value: &Vec<FacePrimitive>) -> Self {
//        let vertices = value.iter()
//            .flat_map(FacePrimitive::vertices)
//            .collect::<Vec<_>>();
//
//        let mut indices = Vec::with_capacity(value.len() * 6);
//        for (i, face) in value.iter().enumerate() {
//            let voxel_i = (i / 6) as u32;
//
//            let mut face_indices = face.indices()
//                .into_iter()
//                .map(|index| {
//                    index + voxel_i * 6
//                }).collect::<Vec<_>>();
//
//            indices.append(&mut face_indices);
//        }
//
//        let material_id = MaterialId::Index(0);
//        let name = String::from("Faces Mesh");
//
//        Mesh {
//            vertices,
//            indices,
//            material_id,
//            name,
//        }
//    }
//}

impl FacePrimitive {
    pub fn new(position: (f32, f32, f32),
        direction: FaceDirection,
        width: f32,
        height: f32,
        device: &wgpu::Device,
    ) -> Self {
        let vertices = Self::vertices(direction, width, height);
        let indices = Self::indices(direction);
        let instances_data = vec![InstanceData::from_position(position)];

        let vertex_buffer = device.compute_vertex_buffer(&vertices);
        let instance_buffer = device.compute_instance_buffer(&instances_data);
        let index_buffer = device.compute_index_buffer(&indices);
        let material_id = MaterialId::Index(0);

        Self {
            vertices,
            indices,
            instances_data,
            vertex_buffer,
            instance_buffer,
            index_buffer,
            material_id,
        }
    }

    fn vertices(direction: FaceDirection,
        width: f32,
        height: f32
    ) -> [Vertex ; 4] {
        let scale = 1.0;

        //let x = self.position.0;
        //let y = self.position.1;
        //let z = self.position.2;

        // TODO: this is used for testing
        let x = 0.0;
        let y = 0.0;
        let z = 0.0;

        // width = growing towards positive x
        // height = growing towards positive z
        // for faces that cannot grow towards either of these axes
        // they mean somehting else :)
        // go ask the author of the library

        match direction {
            FaceDirection::FRONT => [
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [x - width, y, z],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [x - width, y + height, z],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [x, y + height, z],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceDirection::BACK => [
                Vertex {
                    position: [x + width, y, z],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [x, y + height, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [x + width, y + height, z],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, height],
                },
            ],
            FaceDirection::UP => [
                Vertex {
                    position: [x, y, z + height],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [x + width, y, z + height],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [x + width, y, z],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceDirection::DOWN => [
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [x - width, y, z],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [x - width, y, z + height],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
                Vertex {
                    position: [x, y, z + height],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [width, height],
                },
            ],
            FaceDirection::RIGHT => [
                Vertex {
                    position: [x, y, z + height],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, height],
                },
                Vertex {
                    position: [x, y - width, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [x, y - width, z + height],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [width, 0.0],
                },
            ],
            FaceDirection::LEFT => [
                Vertex {
                    position: [x, y, z],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [x, y, z + height],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [height, 0.0],
                },
                Vertex {
                    position: [x, y + width, z + height],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [height, width],
                },
                Vertex {
                    position: [x, y + width, z],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, width],
                },
            ],
        }
    }

    fn indices(direction: FaceDirection) -> [u32 ; 6] {
        match direction {
            FaceDirection::LEFT => [0, 1, 2, 0, 2, 3],
            FaceDirection::BACK => [0, 1, 2, 0, 2, 3],
            FaceDirection::UP => [0, 1, 2, 0, 2, 3],
            FaceDirection::FRONT => [0, 3, 1, 1, 3, 2],
            FaceDirection::RIGHT => [3, 2, 1, 3, 1, 0],
            FaceDirection::DOWN => [1, 0, 3, 1, 3, 2],
        }
    }
}

// Y-UP RIGHT HAND
#[derive(Debug, Clone, Copy)]
pub enum FaceDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}

impl FaceDirection {
    pub fn from_bgm(direction: usize) -> Self {
        debug_assert!(direction < 6, "Unknown bgm direction");

        match direction {
            0 => FaceDirection::UP,
            1 => FaceDirection::DOWN,
            2 => FaceDirection::RIGHT,
            3 => FaceDirection::LEFT,
            4 => FaceDirection::FRONT,
            5 => FaceDirection::BACK,
            _ => unreachable!(),
        }
    }
}
