use std::sync::Arc;

use crate::{IntoModel, Model, Vertex};

pub struct FaceDescriptor {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub width: u32,
    pub height: u32,
    pub direction: FaceDirection,
}

impl IntoModel for Face {
    fn to_model(&self, device: &wgpu::Device) -> Model {
        let model = Model::new(device,
            &self.0,
            &[0, 1, 2, 3],
        );
    }
}

pub struct Face(Vec<Vertex>);
impl Face {
    pub fn new(descriptor: &FaceDescriptor) -> Self {
        let x = descriptor.x;
        let y = descriptor.y;
        let z = descriptor.z;

        let w = descriptor.width;
        let h = descriptor.height;

        debug_assert!(x >> 6 == 0, "The x coordinate has more than 6 bits!");
        debug_assert!(y >> 6 == 0, "The y coordinate has more than 6 bits!");
        debug_assert!(z >> 6 == 0, "The z coordinate has more than 6 bits!");

        debug_assert!(w >> 6 == 0, "The width has more than 6 bits!");
        debug_assert!(h >> 6 == 0, "The height has more than 6 bits!");
        
        let xyz = Self::pack_xyz(x, y, z);
        
        let vertices = match descriptor.direction {
            FaceDirection::LEFT => vec![
                Self::pack_vertex(xyz, h, w),
                Self::pack_vertex(xyz, 0, w),
                Self::pack_vertex(xyz, h, 0),
                Self::pack_vertex(xyz, 0, 0),
            ],
            FaceDirection::DOWN => vec![
                Self::pack_vertex(xyz, w, h),
                Self::pack_vertex(xyz, w, 0),
                Self::pack_vertex(xyz, 0, h),
                Self::pack_vertex(xyz, 0, 0),
            ],
            FaceDirection::BACK => vec![
                Self::pack_vertex(xyz, w, h),
                Self::pack_vertex(xyz, w, 0),
                Self::pack_vertex(xyz, 0, h),
                Self::pack_vertex(xyz, 0, 0),
            ],
            FaceDirection::RIGHT => vec![
                Self::pack_vertex(xyz, 0, 0),
                Self::pack_vertex(xyz, h, 0),
                Self::pack_vertex(xyz, 0, w),
                Self::pack_vertex(xyz, h, w),
            ],
            FaceDirection::UP => vec![
                Self::pack_vertex(xyz, w, h),
                Self::pack_vertex(xyz, w, 0),
                Self::pack_vertex(xyz, 0, h),
                Self::pack_vertex(xyz, 0, 0),
            ],
            FaceDirection::FRONT => vec![
                Self::pack_vertex(xyz, 0, 0),
                Self::pack_vertex(xyz, 0, h),
                Self::pack_vertex(xyz, w, 0),
                Self::pack_vertex(xyz, w, h),
            ],
        };

        Self(vertices)
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
