use std::sync::Arc;

use crate::{resources::asset_server::AssetServer, IntoModel, Model, Texture, Vertex};

pub struct FaceDescriptor {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub width: u32,
    pub height: u32,
    pub direction: FaceDirection,
    pub scale: f32,
}

pub struct FaceModel {
    vertices: [Vertex ; 4],
    diffuse_texture: Arc<Texture>,
}

impl IntoModel for FaceModel {
    fn to_model(self, device: &wgpu::Device) -> Arc<Model> {
        let model = Model::new(device,
            &self.vertices,
            &[0, 1, 2, 3],
            self.diffuse_texture,
            "Face Model",
        );

        Arc::new(model)
    }
}

impl FaceModel {
    pub fn new(asset_server: &mut AssetServer,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        direction: FaceDirection
    ) -> Self {
        let scale = 1.0;
        let diffuse_texture = Texture::debug(asset_server, device, queue);

        let vertices = match direction {
            FaceDirection::FRONT => [
                Vertex {
                    position: [-scale, -scale, scale],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, -scale, scale],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, scale, scale],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [-scale, scale, scale],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
            ],
            FaceDirection::BACK => [
                Vertex {
                    position: [-scale, -scale, -scale],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [-scale, scale, -scale],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, scale, -scale],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, -scale, -scale],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, 0.5],
                },
            ],
            FaceDirection::UP => [
                Vertex {
                    position: [-scale, scale, -scale],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [-scale, scale, scale],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, scale, scale],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, scale, -scale],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
            ],
            FaceDirection::DOWN => [
                Vertex {
                    position: [-scale, -scale, -scale],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, -scale, -scale],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, -scale, scale],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [-scale, -scale, scale],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
            ],
            FaceDirection::RIGHT => [
                Vertex {
                    position: [scale, -scale, -scale],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, scale, -scale],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, scale, scale],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [scale, -scale, scale],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, 0.5],
                },
            ],
            FaceDirection::LEFT => [
                Vertex {
                    position: [-scale, -scale, -scale],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [-scale, -scale, scale],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [-scale, scale, scale],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
                Vertex {
                    position: [-scale, scale, -scale],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, 0.5],
                },
            ],
        };

        Self {
            vertices,
            diffuse_texture,
        }
    }

    fn pack_xyz(x: u32, y: u32, z: u32) -> u32 {
        (z << 12) | (y << 6) | x
    }

    fn pack_vertex(xyz: u32, width: u32, height: u32) -> u32 {
        (height << 24) | (width << 18) | xyz
    }
}

// Y-UP RIGHT HAND
pub enum FaceDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}

impl FaceDirection {
    pub fn from_bgm(direction: u32) -> Self {
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
