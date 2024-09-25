use wgpu::util::DrawIndexedIndirectArgs;

use crate::{device_ext::VoxDeviceExt, InstanceData};

use super::{render_server::{MaterialId, ModelId, MultiIndexedMeshId}, vertex::{Index, Vertex}};

pub trait AsMultiIndexedMesh {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices(&self) -> Vec<Index>;
    fn instances(&self) -> Vec<InstanceData>;
    fn indirect_indexed_args(&self) -> Vec<DrawIndexedIndirectArgs>;
    fn material_id(&self) -> MaterialId;
    fn draw_count(&self) -> u32;
}

// TODO: maybe use an array of textures on the shader
// and select the texture based on a passed index

pub struct MultiIndexedMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    indirect_indexed_buffer: wgpu::Buffer,
    draw_count: u32,
    material_id: MaterialId,
    mesh_id: MultiIndexedMeshId,
    model_id: Option<ModelId>,
}

impl MultiIndexedMesh {
    pub fn new(vertices: &[Vertex],
        indices: &[Index],
        instances: &[InstanceData],
        indirect_indexed_args: &[DrawIndexedIndirectArgs],
        draw_count: u32,
        material_id: MaterialId,
        mesh_id: MultiIndexedMeshId,
        model_id: Option<ModelId>,
        device: &wgpu::Device,
    ) -> Self {
        let vertex_buffer = device.compute_vertex_buffer(vertices);
        let index_buffer = device.compute_index_buffer(indices);
        let instance_buffer = device.compute_instance_buffer(instances);
        let indirect_indexed_buffer = device
            .compute_indirect_indexed_buffer(indirect_indexed_args);

        Self {
            vertex_buffer,
            instance_buffer,
            index_buffer,
            indirect_indexed_buffer,
            material_id,
            mesh_id,
            model_id,
            draw_count,
        }
    }
}
