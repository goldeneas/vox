use wgpu::util::DeviceExt;

use crate::{device_ext::VoxDeviceExt, InstanceData, InstanceRaw};

use super::{material::MaterialId, vertex::Vertex};

pub trait AsMesh {
    fn to_mesh(&self, material_id: MaterialId, device: &wgpu::Device) -> Mesh;
}

#[derive(Debug)]
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    num_indices: usize,
    num_instances: usize,
    // the material assigned to this mesh from the materials
    // to be used with models
    material_id: MaterialId, 
}

impl Mesh {
    pub fn new(vertices: &[Vertex],
        indices: &[u32],
        instances_data: &[InstanceData],
        material_id: MaterialId,
        device: &wgpu::Device,
    ) -> Self {
        let vertex_buffer = device.compute_vertex_buffer(vertices);
        let index_buffer = device.compute_index_buffer(indices);
        let instance_buffer = device.compute_instance_buffer(instances_data);

        let num_indices = indices.len();
        let num_instances = instances_data.len();

        Self {
            vertex_buffer,
            instance_buffer,
            index_buffer,
            num_instances,
            num_indices,
            material_id,
        }
    }

    pub fn material_id(&self) -> MaterialId {
        self.material_id
    }

    pub fn num_indices(&self) -> usize {
        self.num_indices
    }

    pub fn num_instances(&self) -> usize {
        self.num_instances
    }
}

