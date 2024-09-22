use std::sync::Arc;

use bevy_ecs::system::Resource;

use crate::Texture;

use super::{material::Material, mesh::{AsMesh, Mesh}};

pub type MaterialId = usize;
pub type MeshId = usize;
pub struct ModelId(Option<usize>);

#[derive(Resource, Default)]
pub struct RenderServer {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
}

impl RenderServer {
    pub fn new() -> Self {
        let meshes = Vec::new();
        let materials = Vec::new();

        Self {
            meshes,
            materials,
        }
    }

    pub fn push_material(&mut self,
        diffuse_texture: Arc<Texture>,
        device: &wgpu::Device,
    ) -> MaterialId {
        let material_id = self.materials().len();
        let material = Material::new(diffuse_texture, material_id, device);

        self.materials.push(material);
        material_id
    }

    pub fn push_mesh(&mut self, as_mesh: &impl AsMesh, device: &wgpu::Device) -> MeshId {
        let vertices = as_mesh.vertices();
        let indices = as_mesh.indices();
        let instances = as_mesh.instances();
        let material_id = as_mesh.material_id();

        let mesh_id = self.meshes().len();
        let mesh = Mesh::new(&vertices,
            &indices,
            &instances,
            material_id,
            mesh_id,
            device
        );

        self.meshes.push(mesh);
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
