use std::sync::Arc;

use crate::{resources::asset_server::AssetServer, IntoModel, Model, Texture};

use super::vertex::Vertex;

const MASK_6: u64 = 0b111111;

pub struct FaceModel {
    width: f32,
    height: f32,
    direction: FaceDirection,
    position: (f32, f32, f32),
    diffuse_texture: Arc<Texture>,
}

// TODO: maybe bound check the position attribute as we are converting
// to f32
// maybe make a FacePosition wrapper

impl IntoModel for FaceModel {
    fn to_model(self, device: &wgpu::Device) -> Arc<Model> {
        let model = Model::new(device,
            &self.compute_vertices(),
            &[0, 1, 2, 0, 2, 3],
            self.diffuse_texture,
            "Face Model",
        );

        Arc::new(model)
    }
}

impl FaceModel {
    pub fn new(direction: FaceDirection,
        position: (f32, f32, f32),
        width: f32,
        height: f32,
        diffuse_texture: Arc<Texture>,
    ) -> Self {

        Self {
            direction,
            position,
            width,
            height,
            diffuse_texture,
        }
    }

    fn compute_vertices(&self) -> [Vertex ; 4] {
        // TODO: start first vertex from the bottom left vertex
        // looking at the face with the axis in the middle of the block
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

        // TODO: use an helper function to calculare pos * scale * multiplier
        // where multiplier is either width or height based on the face
        // also check if the formula is correct ;)
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
                    tex_coords: [1.0, 0.0],
                },
                Vertex {
                    position: [x - width, y + height, z],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [x, y + height, z],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
            ],
            FaceDirection::BACK => [
                Vertex {
                    position: [x + width, y, z],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y + height, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x + width, y + height, z],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, 1.0],
                },
            ],
            FaceDirection::UP => [
                Vertex {
                    position: [x, y, z + height],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x + width, y, z + height],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x + width, y, z],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, 1.0],
                },
            ],
            FaceDirection::DOWN => [
                Vertex {
                    position: [x, y, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x - width, y, z],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x - width, y, z + height],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y, z + height],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
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
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y - width, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [x, y - width, z + height],
                    normal: [0.0, 0.0, -scale],
                    tex_coords: [1.0, 0.0],
                },
            ],
            FaceDirection::LEFT => [
                Vertex {
                    position: [x, y, z],
                    normal: [scale, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y, z + height],
                    normal: [-scale, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y + width, z + height],
                    normal: [0.0, scale, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [x, y + width, z],
                    normal: [0.0, -scale, 0.0],
                    tex_coords: [0.0, 1.0],
                },
            ],
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
