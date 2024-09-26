use crate::{render::{mesh::{AsMesh, MeshPosition}, quad_orientation::QuadOrientation, vertex::{Index, Vertex}}, resources::render_server::MaterialId, InstanceData};

#[derive(Debug)]
pub struct QuadPrimitive {
    direction: QuadOrientation,
    vertices: [Vertex ; 4],
    indices: [Index ; 6],
    instances: Vec<InstanceData>,
    material_id: MaterialId,
}

impl AsMesh for QuadPrimitive {
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

impl QuadPrimitive {
    pub fn new(direction: QuadOrientation,
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

    pub fn vertices(direction: QuadOrientation,
        width: f32,
        height: f32
    ) -> [Vertex ; 4] {
        match direction {
            QuadOrientation::FRONT => [
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
            QuadOrientation::BACK => [
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
            QuadOrientation::UP => [
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
            QuadOrientation::DOWN => [
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
            QuadOrientation::RIGHT => [
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
            QuadOrientation::LEFT => [
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

    pub fn indices(direction: QuadOrientation) -> [Index ; 6] {
        match direction {
            QuadOrientation::UP => [0, 1, 2, 0, 2, 3],
            QuadOrientation::DOWN => [1, 0, 3, 1, 3, 2],
            QuadOrientation::LEFT => [0, 1, 2, 0, 2, 3],
            QuadOrientation::RIGHT => [3, 2, 1, 3, 1, 0],
            QuadOrientation::FRONT => [0, 3, 1, 1, 3, 2],
            QuadOrientation::BACK => [0, 1, 2, 0, 2, 3],
        }
    }
}
