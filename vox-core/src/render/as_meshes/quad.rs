use crate::{render::{mesh::{AsMesh, MeshPosition}, quad_orientation::FaceOrientation, vertex::{Index, Vertex}}, resources::render_server::MaterialId, InstanceData};

#[derive(Debug)]
pub struct VoxelFace {
    orientation: FaceOrientation,
    vertices: [Vertex ; 4],
    indices: [Index ; 6],
    instances: Vec<InstanceData>,
    material_id: MaterialId,
}

#[derive(Hash, PartialEq, Eq)]
pub struct FaceDescriptor {
    pub orientation: FaceOrientation,
    pub width: u32,
    pub height: u32,
    pub material_id: MaterialId,
}

impl VoxelFace {
    pub fn new(descriptor: &FaceDescriptor,
        positions: &[MeshPosition]
    ) -> Self {
        let orientation = descriptor.orientation;
        let width = descriptor.width;
        let height = descriptor.height;
        let material_id = descriptor.material_id;

        let instances = positions.iter()
            .map(|p| { InstanceData::from_position(*p) })
            .collect::<Vec<_>>();

        let vertices = Self::vertices(orientation, width, height);
        let indices = Self::indices(orientation);

        Self {
            orientation,
            vertices,
            indices,
            material_id,
            instances,
        }
    }

    pub fn new2(descriptor: &FaceDescriptor,
        instances: Vec<InstanceData>
    ) -> Self {
        let orientation = descriptor.orientation;
        let width = descriptor.width;
        let height = descriptor.height;
        let material_id = descriptor.material_id;

        let vertices = Self::vertices(orientation, width, height);
        let indices = Self::indices(orientation);

        Self {
            orientation,
            vertices,
            indices,
            material_id,
            instances,
        }
    }

    pub fn vertices(direction: FaceOrientation,
        width: u32,
        height: u32
    ) -> [Vertex ; 4] {
        let width = width as f32;
        let height = height as f32;

        match direction {
            FaceOrientation::FRONT => [
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [-width, height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [0.0, height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceOrientation::BACK => [
                Vertex {
                    position: [width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [0.0, height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [width, height, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceOrientation::UP => [
                Vertex {
                    position: [0.0, 0.0, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [width, 0.0, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
            ],
            FaceOrientation::DOWN => [
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
                Vertex {
                    position: [-width, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-width, 0.0, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
                Vertex {
                    position: [0.0, 0.0, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
            ],
            FaceOrientation::RIGHT => [
                Vertex {
                    position: [0.0, 0.0, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, height],
                },
                Vertex {
                    position: [0.0, -width, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, height],
                },
                Vertex {
                    position: [0.0, -width, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [width, 0.0],
                },
            ],
            FaceOrientation::LEFT => [
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [height, 0.0],
                },
                Vertex {
                    position: [0.0, width, height],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [height, width],
                },
                Vertex {
                    position: [0.0, width, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, width],
                },
            ],
        }
    }

    pub fn indices(direction: FaceOrientation) -> [Index ; 6] {
        match direction {
            FaceOrientation::UP => [0, 1, 2, 0, 2, 3],
            FaceOrientation::DOWN => [1, 0, 3, 1, 3, 2],
            FaceOrientation::LEFT => [0, 1, 2, 0, 2, 3],
            FaceOrientation::RIGHT => [3, 2, 1, 3, 1, 0],
            FaceOrientation::FRONT => [0, 3, 1, 1, 3, 2],
            FaceOrientation::BACK => [0, 1, 2, 0, 2, 3],
        }
    }

    pub fn orientation(&self) -> FaceOrientation {
        self.orientation
    }
}
