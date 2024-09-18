use std::ops::Range;

use crate::{components::{model::ModelComponent, transform::TransformComponent}, render::{material::Material, mesh::Mesh}, Model};

pub trait DrawPassExt {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup);
    fn draw_mesh_instanced(&mut self,
        mesh: &Mesh,
        material: &Material,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup);
    fn draw_model(&mut self,
        model: &Model,
        camera_bind_group: &wgpu::BindGroup);
    fn draw_model_instanced(&mut self,
        model: &Model,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup);
    fn draw_object(&mut self,
        model_cmpnt: &ModelComponent,
        transform_cmpnt: &TransformComponent,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
    fn draw_object_multiple(&mut self,
        model_cmpnt: &ModelComponent,
        transform_cmpnt: &TransformComponent,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device);
}

impl DrawPassExt for wgpu::RenderPass<'_> {
    fn draw_mesh(&mut self,
        mesh: &Mesh,
        material: &Material,
        camera_bind_group: &wgpu::BindGroup
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }

    fn draw_mesh_instanced(&mut self,
        mesh: &Mesh,
        material: &Material,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
        self.set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, material.bind_group(), &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_indices(), 0, instances);
    }

    fn draw_model(&mut self,
        model: &Model,
        camera_bind_group: &wgpu::BindGroup
    ) {
        for mesh in model.meshes.as_ref() {
            let material_id = mesh.material_id().unwrap();
            let material = &model.materials[material_id];
            self.draw_mesh(mesh, material, camera_bind_group);
        }
    }

    fn draw_model_instanced(&mut self,
        model: &Model,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup
    ) {
        for mesh in model.meshes.as_ref() {
            let material_id = mesh.material_id().unwrap();
            let material = &model.materials[material_id];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
        }
    }

    fn draw_object(&mut self,
        model_cmpnt: &ModelComponent,
        transform_cmpnt: &TransformComponent,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        self.set_vertex_buffer(1, transform_cmpnt
            .compute_buffer(device)
            .slice(..));

        self.draw_model(&model_cmpnt.model,
            camera_bind_group
        );
    }

    fn draw_object_multiple(&mut self,
        model_cmpnt: &ModelComponent,
        transform_cmpnt: &TransformComponent,
        camera_bind_group: &wgpu::BindGroup,
        device: &wgpu::Device,
    ) {
        self.set_vertex_buffer(1, transform_cmpnt
            .compute_buffer(device)
            .slice(..));

        let transforms_len = transform_cmpnt.transforms.len() as u32;

        self.draw_model_instanced(&model_cmpnt.model,
            0..transforms_len,
            camera_bind_group
        );
    }
}
