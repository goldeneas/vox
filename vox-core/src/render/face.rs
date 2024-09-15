use std::sync::Arc;

use crate::{resources::asset_server::AssetServer, IntoModel, Model, Texture};

use super::vertex::Vertex;

const MASK_6: u64 = 0b111111;

pub struct FacePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FacePosition {
    pub fn from_bgm(bgm_data: &u64, direction: FaceDirection) -> Self {
        let x = bgm_data & MASK_6;
        let y = (bgm_data >> 6) & MASK_6;
        let z = (bgm_data >> 12) & MASK_6;

        // TODO check actual boundaries
        debug_assert!(x <= 61, "voxel's x position is greater that chunk's max position!");
        debug_assert!(y <= 61, "voxel's y position is greater that chunk's max position!");
        debug_assert!(z <= 61, "voxel's z position is greater that chunk's max position!");

        let x = x as f32;
        let y = y as f32;
        let z = z as f32;

        match direction {
            FaceDirection::FRONT => Self{
                x: x - 1.0, y, z
            },
            FaceDirection::BACK => Self{
                x, y, z
            },
            FaceDirection::UP => Self{
                x, y, z
            },
            FaceDirection::DOWN => Self{
                x: x - 1.0, y, z
            },
            FaceDirection::RIGHT => Self{
                x, y, z
            },
            FaceDirection::LEFT => Self{
                x, y, z
            },
        }
    }
}

pub struct FaceModel {
    width: f32,
    height: f32,
    direction: FaceDirection,
    position: FacePosition,
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
        position: FacePosition,
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

        let x = self.position.x;
        let y = self.position.y;
        let z = self.position.z;

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
                    position: [x, y + width, z],
                    normal: [0.0, 0.0, scale],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [x, y + width, z + height],
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
