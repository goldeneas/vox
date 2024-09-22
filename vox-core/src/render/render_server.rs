use std::sync::Arc;

use bevy_ecs::system::Resource;

use crate::Texture;

use super::{material::Material, mesh::{AsMesh, Mesh}};

pub type MaterialId = usize;
pub type MeshId = usize;
pub type ModelId = usize;

#[derive(Resource, Default)]
pub struct RenderServer {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    free_mesh_id: MeshId,
    free_material_id: MeshId,
    free_model_id: ModelId,
}

impl RenderServer {
    pub fn push_material(&mut self,
        diffuse_texture: Arc<Texture>,
        device: &wgpu::Device,
    ) -> MaterialId {
        let material_id = self.free_material_id;
        let material = Material::new(diffuse_texture, material_id, device);

        self.materials.push(material);
        self.free_material_id += 1;

        material_id
    }

    pub fn push_mesh(&mut self, as_mesh: &impl AsMesh, device: &wgpu::Device) -> MeshId {
        let vertices = as_mesh.vertices();
        let indices = as_mesh.indices();
        let instances = as_mesh.instances();
        let material_id = as_mesh.material_id();
        let model_id = None;

        let mesh_id = self.free_mesh_id;
        let mesh = Mesh::new(&vertices,
            &indices,
            &instances,
            material_id,
            mesh_id,
            model_id,
            device
        );

        self.meshes.push(mesh);
        self.free_mesh_id += 1;

        mesh_id
    }

    pub fn get_material(&self, material_id: MaterialId) -> &Material {
        self.materials.iter()
            .find(|material| material_id == material.material_id())
            .expect("Could not find material with the specified id")
    }

    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }

    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
    }
}
