use std::sync::Arc;

use crate::{IntoModel, Model, Vertex, Texture};

pub struct CubeModel {
    pub scale: f32,
    pub diffuse_texture: Arc<Texture>,
}

impl IntoModel for CubeModel {
    fn to_model(&self, device: &wgpu::Device) -> Model {
        Model::new(device,
            &cube_vertices(self.scale),
            &cube_indices(),
            self.diffuse_texture.clone(),
            "Cube Model",
        )
    }
}

fn cube_vertices(scale: f32) -> Vec<Vertex> {
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

    vec![
        // Front face
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
        // Back face
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
        // Top face
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
        // Bottom face
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
        // Right face
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
        // Left face
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
    ]
}

fn cube_indices() -> Vec<u32> {
    vec![
        0, 1, 2, 0, 2, 3, // front
        4, 5, 6, 4, 6, 7, // back
        8, 9, 10, 8, 10, 11, // top
        12, 13, 14, 12, 14, 15, // bottom
        16, 17, 18, 16, 18, 19, // right
        20, 21, 22, 20, 22, 23, // left
    ]
}
