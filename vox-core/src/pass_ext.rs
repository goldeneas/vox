use std::ops::Range;

use wgpu::util::RenderEncoder;

use crate::{render::{material::Material, mesh::Mesh}, Model};

pub trait DrawPassExt {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
    fn draw_model(&mut self,
        model: &Model,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
}

impl DrawPassExt for wgpu::RenderPass<'_> {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        let vertex_buffer = mesh.compute_vertex_buffer(device);
        let index_buffer = mesh.compute_index_buffer(device);
        let instance_buffer = mesh.compute_instance_buffer(device);

        self.set_vertex_buffer(0, vertex_buffer.slice(..));
        self.set_vertex_buffer(1, instance_buffer.slice(..));
        self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, material.bind_group(), &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_indices(), 0, 0..mesh.num_instances());
    }

    fn draw_model(&mut self,
        model: &Model,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        for mesh in model.meshes.iter() {
            let material_id = mesh.material_id().get();
            let material = &model.materials[material_id];
            self.draw_mesh(mesh,
                material,
                camera_bind_group,
                device,
            );
        }
    }
}
