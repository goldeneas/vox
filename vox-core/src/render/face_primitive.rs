use crate::InstanceData;

use super::{face_direction::FaceDirection, mesh::{AsMesh, MeshPosition}, render_server::MaterialId, vertex::{Index, Vertex}};

#[derive(Debug)]
pub struct FacePrimitive {
    direction: FaceDirection,
    vertices: [Vertex ; 4],
    indices: [Index ; 6],
    instances: Vec<InstanceData>,
    material_id: MaterialId,
}

impl AsMesh for FacePrimitive {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[Index] {
        &self.indices
    }

    fn instances(&self) -> &[InstanceData] {
        &self.instances
    }

    fn material_id(&self) -> MaterialId {
        self.material_id
    }
}

impl FacePrimitive {
    pub fn new(direction: FaceDirection,
        width: f32,
        height: f32,
        material_id: MaterialId,
        positions: &[MeshPosition]
    ) -> Self {
        let instances = positions.iter()
            .map(|p| { InstanceData::from_position(*p) })
            .collect::<Vec<_>>();

        let vertices = Self::vertices(direction, width, height);
        let indices = Self::indices(direction);

        Self {
            direction,
            vertices,
            indices,
            material_id,
            instances,
        }
    }

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
