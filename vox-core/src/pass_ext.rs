use std::ops::Range;

use crate::{components::{model::ModelComponent, transform::TransformComponent}, render::{material::Material, mesh::Mesh}, Model};

pub trait DrawPassExt {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
    fn draw_mesh_instanced(&mut self,
        mesh: &Mesh,
        material: &Material,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
    fn draw_model(&mut self,
        model: &Model,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
    fn draw_model_instanced(&mut self,
        model: &Model,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
    fn draw_object(&mut self,
        model_cmpnt: &ModelComponent,
        transform_cmpnt: &TransformComponent,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
}

impl DrawPassExt for wgpu::RenderPass<'_> {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group, device);
    }

    fn draw_mesh_instanced(&mut self,
        mesh: &Mesh,
        material: &Material,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        let vertex_buffer = mesh.compute_vertex_buffer(device);
        let index_buffer = mesh.compute_index_buffer(device);

        self.set_vertex_buffer(0, vertex_buffer.slice(..));
        self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, material.bind_group(), &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_indices(), 0, instances);
    }

    fn draw_model(&mut self,
        model: &Model,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        self.draw_model_instanced(model,
            0..1,
            camera_bind_group,
            device
        );
    }

    fn draw_model_instanced(&mut self,
        model: &Model,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        for mesh in model.meshes.iter() {
            let material_id = mesh.material_id().get();
            let material = &model.materials[material_id];
            self.draw_mesh_instanced(mesh,
                material,
                instances.clone(),
                camera_bind_group,
                device,
            );
        }
    }

    fn draw_object(&mut self,
        model_cmpnt: &ModelComponent,
        transform_cmpnt: &TransformComponent,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        let instance_buffer = transform_cmpnt.compute_instance_buffer(device);
        self.set_vertex_buffer(1, instance_buffer.slice(..));

        self.draw_model_instanced(&model_cmpnt.model,
            0..transform_cmpnt.num_instances(),
            camera_bind_group,
            device,
        );
    }
}
