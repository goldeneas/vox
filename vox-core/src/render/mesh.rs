use wgpu::util::DeviceExt;

use super::{material::MaterialId, vertex::Vertex};

pub trait AsMesh {
    fn to_mesh(&self) -> Mesh;
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
   // the material assigned to this mesh from the materials
   // to be used with models
    pub material_id: MaterialId, 
    pub name: String,
}

impl Mesh {
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
}

