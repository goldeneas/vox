use bytemuck::{Pod, Zeroable};

pub type Index = u32;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Vertex,
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    shader_location: 0,
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // tex coords
                wgpu::VertexAttribute {
                    shader_location: 1,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // normal
                wgpu::VertexAttribute {
                    shader_location: 2,
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}
