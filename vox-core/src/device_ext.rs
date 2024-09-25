use wgpu::{util::{DeviceExt, DrawIndexedIndirectArgs}, Buffer, Device};

use crate::{render::vertex::{Index, Vertex}, InstanceData};

pub trait VoxDeviceExt {
    fn compute_vertex_buffer(&self, vertices: &[Vertex]) -> Buffer;
    fn compute_index_buffer(&self, indices: &[Index]) -> Buffer;
    fn compute_instance_buffer(&self, instances: &[InstanceData]) -> Buffer;
    fn compute_indirect_indexed_buffer(&self,
        indirect_args: &[DrawIndexedIndirectArgs]
    ) -> Buffer;
}

impl VoxDeviceExt for Device {
    fn compute_vertex_buffer(&self, vertices: &[Vertex]) -> Buffer {
        self.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(vertices),
        })
    }

    fn compute_index_buffer(&self, indices: &[Index]) -> Buffer {
        self.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            usage: wgpu::BufferUsages::INDEX,
            contents: bytemuck::cast_slice(indices),
        })
    }

    fn compute_indirect_indexed_buffer(&self,
        indirect_args: &[DrawIndexedIndirectArgs]
    ) -> Buffer {
        let indirect_bytes = indirect_args.iter()
            .flat_map(DrawIndexedIndirectArgs::as_bytes)
            .copied()
            .collect::<Vec<_>>();

        self.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Index Buffer"),
            usage: wgpu::BufferUsages::INDIRECT,
            contents: &indirect_bytes,
        })
    }

    fn compute_instance_buffer(&self, instances: &[InstanceData]) -> Buffer {
        let instances_raw = instances.iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        self.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instances_raw),
        })
    }
}
