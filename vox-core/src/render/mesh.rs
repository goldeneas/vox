use wgpu::util::DeviceExt;

use crate::{device_ext::VoxDeviceExt, resources::render_server::{MaterialId, MeshId, ModelId}, InstanceData, InstanceRaw};

use super::{vertex::{Index, Vertex}};

pub trait AsMesh {
    fn vertices(&self) -> &[Vertex];
    fn indices(&self) -> &[Index];
    fn instances(&self) -> &[InstanceData];
    fn material_id(&self) -> MaterialId;
}

pub type MeshPosition = (f32, f32, f32);

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
    mesh_id: MeshId,
    model_id: Option<ModelId>,
}

impl Mesh {
    pub fn new(vertices: &[Vertex],
        indices: &[Index],
        instances: &[InstanceData],
        material_id: MaterialId,
        mesh_id: MeshId,
        model_id: Option<ModelId>,
        device: &wgpu::Device,
    ) -> Self {
        let vertex_buffer = device.compute_vertex_buffer(vertices);
        let index_buffer = device.compute_index_buffer(indices);
        let instance_buffer = device.compute_instance_buffer(instances);

        let num_indices = indices.len();
        let num_instances = instances.len();

        Self {
            vertex_buffer,
            instance_buffer,
            index_buffer,
            num_instances,
            num_indices,
            material_id,
            mesh_id,
            model_id,
        }
    }

    pub fn model_id(&self) -> &Option<ModelId> {
        &self.model_id
    }

    pub fn mesh_id(&self) -> MeshId {
        self.mesh_id
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
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

