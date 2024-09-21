use wgpu::util::DeviceExt;

use crate::{InstanceData, InstanceRaw};

use super::{material::MaterialId, vertex::Vertex};

pub trait AsMesh {
    fn to_mesh(&self, material_id: MaterialId) -> Mesh;
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    instances_data: Vec<InstanceData>,
    instances_raw: Vec<InstanceRaw>,
    // the material assigned to this mesh from the materials
    // to be used with models
    material_id: MaterialId, 
    name: String,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>,
        indices: Vec<u32>,
        instances_data: Vec<InstanceData>,
        material_id: MaterialId,
        name: String,
    ) -> Self {
        let instances_raw = instances_data.iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        Self {
            vertices,
            indices,
            instances_data,
            instances_raw,
            material_id,
            name,
        }
    }

    pub fn set_instances(&mut self, instances_data: Vec<InstanceData>) {
        self.instances_raw = instances_data.iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        self.instances_data = instances_data;
    }

    pub fn compute_instance_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&self.instances_raw),
        })
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

