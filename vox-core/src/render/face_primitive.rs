use crate::InstanceData;

use super::{face_direction::FaceDirection, mesh::AsMesh, render_server::MaterialId, vertex::{Index, Vertex}};

#[derive(Debug)]
pub struct FacePrimitive {
    pub width: f32,
    pub height: f32,
    pub direction: FaceDirection,
    pub position: (f32, f32, f32),
    pub material_id: MaterialId,
}

impl FacePrimitive {
    pub fn vertices(direction: FaceDirection,
        width: f32,
        height: f32
    ) -> [Vertex ; 4] {
        match direction {
            FaceDirection::FRONT => [
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0 - width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [0.0 - width, 0.0 + height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [0.0, 0.0 + height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceDirection::BACK => [
                Vertex {
                    position: [0.0 + width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0 + height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [0.0 + width, 0.0 + height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceDirection::UP => [
                Vertex {
                    position: [0.0, 0.0, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0 + width, 0.0, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [0.0 + width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceDirection::DOWN => [
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [0.0 - width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0 - width, 0.0, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
            ],
            FaceDirection::RIGHT => [
                Vertex {
                    position: [0.0, 0.0, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
                Vertex {
                    position: [0.0, 0.0 - width, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [0.0, 0.0 - width, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
            ],
            FaceDirection::LEFT => [
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [height, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0 + width, 0.0 + height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [height, width],
                },
                Vertex {
                    position: [0.0, 0.0 + width, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, width],
                },
            ],
        }
    }

    pub fn indices(direction: FaceDirection) -> [Index ; 6] {
        match direction {
            FaceDirection::UP => [0, 1, 2, 0, 2, 3],
            FaceDirection::DOWN => [1, 0, 3, 1, 3, 2],
            FaceDirection::LEFT => [0, 1, 2, 0, 2, 3],
            FaceDirection::RIGHT => [3, 2, 1, 3, 1, 0],
            FaceDirection::FRONT => [0, 3, 1, 1, 3, 2],
            FaceDirection::BACK => [0, 1, 2, 0, 2, 3],
        }
    }
}

impl AsMesh for FacePrimitive {
    fn vertices(&self) -> Vec<Vertex> {
        Self::vertices(self.direction, self.width, self.height)
            .to_vec()
    }

    fn indices(&self) -> Vec<Index> {
        Self::indices(self.direction)
            .to_vec()
    }

    fn instances(&self) -> Vec<InstanceData> {
        let instance_data = InstanceData::from_position(self.position);
        vec![instance_data]
    }

    fn material_id(&self) -> MaterialId {
        self.material_id
    }
}
