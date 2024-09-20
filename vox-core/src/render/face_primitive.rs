use std::sync::Arc;

use crate::{voxels::voxel_registry::VoxelTypeIdentifier, AsModel, Model, Texture};

use super::{material::{Material, MaterialId}, mesh::{AsMesh, Mesh}, vertex::Vertex};


// TODO: make a poision struct wrapper

#[derive(Debug)]
pub struct FacePrimitive {
    width: f32,
    height: f32,
    direction: FaceDirection,
    position: (f32, f32, f32),
}

//impl AsMesh for FacePrimitive {
//    fn to_mesh(&self, material_id: usize) -> Mesh {
//        let vertices = self.vertices().to_vec();
//        let indices = self.indices().to_vec();
//        let name = String::from("Face Mesh");
//
//        Mesh {
//            vertices,
//            indices,
//            name,
//            material_id,
//        }
//    }
//}

impl AsMesh for FacePrimitive {
    fn to_mesh(&self, material_id: MaterialId) -> Mesh {
        let vertices = self.vertices().to_vec();
        let indices = self.indices().to_vec();

        Mesh {
            vertices,
            indices,
            material_id,
            name: String::from("Face Mesh"),
        }
    }
}

impl AsModel for FacePrimitive {
    fn to_model(&self, materials: Vec<Material>) -> Model {
        let material_id = MaterialId::Index(0);
        let mesh = self.to_mesh(material_id);

        Model {
            meshes: vec![mesh],
            materials,
            name: String::from("Face Model"),
        }
    }
}

// TODO: maybe make this a method of chunk.rs
impl From<&Vec<FacePrimitive>> for Mesh {
    fn from(value: &Vec<FacePrimitive>) -> Self {
        let vertices = value.iter()
            .flat_map(FacePrimitive::vertices)
            .collect::<Vec<_>>();

        let mut indices = Vec::with_capacity(value.len() * 6);
        for (i, face) in value.iter().enumerate() {
            let voxel_i = (i / 6) as u32;

            let mut face_indices = face.indices()
                .into_iter()
                .map(|index| {
                    index + voxel_i * 6
                }).collect::<Vec<_>>();

            indices.append(&mut face_indices);
        }

        let material_id = MaterialId::Index(0);
        let name = String::from("Faces Mesh");

        Mesh {
            vertices,
            indices,
            material_id,
            name,
        }
    }
}

impl FacePrimitive {
    pub fn new(direction: FaceDirection,
        position: (f32, f32, f32),
        width: f32,
        height: f32,
    ) -> Self {

        Self {
            direction,
            position,
            width,
            height,
        }
    }

   pub fn vertices(&self) -> [Vertex ; 4] {
        let scale = 1.0;

        let x = self.position.0;
        let y = self.position.1;
        let z = self.position.2;

        let width = self.width;
        let height = self.height;

        // width = growing towards positive x
        // height = growing towards positive z
        // for faces that cannot grow towards either of these axes
        // they mean somehting else :)
        // go ask the author of the library

        match self.direction {
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

    pub fn indices(&self) -> [u32 ; 6] {
        match self.direction {
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
