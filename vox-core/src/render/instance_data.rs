use bytemuck::{Pod, Zeroable};
use cgmath::{Quaternion, Vector3, Zero};

use super::mesh::MeshPosition;

#[derive(Debug, Clone, Copy)]
pub struct InstanceData {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl InstanceData {
    pub fn from_position(position: MeshPosition) -> Self {
        let position: Vector3<f32> = position.into();
        let rotation = Quaternion::zero();

        Self {
            position,
            rotation,
        }
    }

    pub fn from_rotation(rotation: Quaternion<f32>) -> Self {
        let position: Vector3<f32> = (0.0, 0.0, 0.0).into();

        Self {
            position,
            rotation,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                },
            ]
        }
    }
}
