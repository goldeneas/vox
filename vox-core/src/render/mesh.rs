use wgpu::util::DeviceExt;

use crate::InstanceData;

use super::{material::MaterialId, vertex::Vertex};

pub trait AsMesh {
    fn to_mesh(&self, material_id: MaterialId) -> Mesh;
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub instances_data: Vec<InstanceData>,
    // the material assigned to this mesh from the materials
    // to be used with models
    pub material_id: MaterialId, 
    pub name: String,
}

impl Mesh {
    pub fn compute_instance_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let instances_raw = self.instances_data.iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instances_raw),
        });

        instance_buffer
    } 

    pub fn compute_index_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&self.name),
            usage: wgpu::BufferUsages::INDEX,
            contents: bytemuck::cast_slice(&self.indices),
        })
    }

    pub fn compute_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&self.name),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&self.vertices),
        })
    }

    pub fn material_id(&self) -> MaterialId {
        self.material_id
    }

    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }

    pub fn num_instances(&self) -> u32 {
        self.instances_data.len() as u32
    }
}

