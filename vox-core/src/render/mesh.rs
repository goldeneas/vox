use wgpu::util::DeviceExt;

use crate::{InstanceData, InstanceRaw};

use super::{instance_data, material::MaterialId, vertex::Vertex};

pub trait AsMesh where Self: Sync + Send {
    fn vertex_buffer(&self) -> &wgpu::Buffer;
    fn index_buffer(&self) -> &wgpu::Buffer;
    fn instance_buffer(&self) -> &wgpu::Buffer;
    fn num_instances(&self) -> usize;
    fn num_indices(&self) -> usize;
    fn material_id(&self) -> MaterialId;
}
