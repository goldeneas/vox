use std::ops::Range;

use wgpu::{util::RenderEncoder, Buffer, BufferAddress};

use crate::{render::{material::Material, mesh::Mesh}, Model};

pub trait VoxDrawPassExt {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup);
    fn draw_mesh_multi_indexed(&mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        vertex_buffer: &Buffer,
        index_buffer: &Buffer,
        instance_buffer: &Buffer,
        draw_count: u32,
    );
}

impl VoxDrawPassExt for wgpu::RenderPass<'_> {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup,
    ) {
        let vertex_buffer = mesh.vertex_buffer();
        let index_buffer = mesh.index_buffer();
        let instance_buffer = mesh.instance_buffer();
        let num_indices = mesh.num_indices() as u32;
        let num_instances = mesh.num_instances() as u32;

        self.set_vertex_buffer(0, vertex_buffer.slice(..));
        self.set_vertex_buffer(1, instance_buffer.slice(..));
        self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, material.bind_group(), &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..num_indices, 0, 0..num_instances);
    }

    fn draw_mesh_multi_indexed(&mut self,
        indirect_buffer: &Buffer,
        indirect_offset: BufferAddress,
        vertex_buffer: &Buffer,
        index_buffer: &Buffer,
        instance_buffer: &Buffer,
        draw_count: u32,
    ) {
        self.set_vertex_buffer(0, vertex_buffer.slice(..));
        self.set_vertex_buffer(1, instance_buffer.slice(..));
        self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.multi_draw_indexed_indirect(indirect_buffer,
            indirect_offset,
            draw_count
        );
    }
}
