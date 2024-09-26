use std::sync::Arc;

use bevy_ecs::system::Resource;

use crate::{render::{material::Material, mesh::{AsMesh, Mesh}, multi_indexed_mesh::{AsMultiIndexedMesh, MultiIndexedMesh}}, AsModel, Texture};

pub type MaterialId = usize;
pub type ModelId = usize;
pub type MeshId = usize;
pub type MultiIndexedMeshId = usize;

#[derive(Resource, Default)]
pub struct RenderServer {
    meshes: Vec<Mesh>,
    multi_indexed_meshes: Vec<MultiIndexedMesh>,
    materials: Vec<Material>,
    free_mesh_id: MeshId,
    free_multi_indexed_mesh_id: MultiIndexedMeshId,
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

    pub fn push_multi_indexed_mesh_ex(&mut self,
        as_multi_indexed_mesh: &impl AsMultiIndexedMesh,
        model_id_opt: Option<ModelId>,
        device: &wgpu::Device,
    ) -> MultiIndexedMeshId {
        let vertices = as_multi_indexed_mesh.vertices();
        let indices = as_multi_indexed_mesh.indices();
        let instances = as_multi_indexed_mesh.instances();
        let indirect_indexed_args = as_multi_indexed_mesh.indirect_indexed_args();
        let material_id = as_multi_indexed_mesh.material_id();
        let draw_count = as_multi_indexed_mesh.draw_count();

        let multi_indexed_mesh_id = self.free_multi_indexed_mesh_id;
        let multi_indexed_mesh = MultiIndexedMesh::new(vertices,
            indices,
            instances,
            &indirect_indexed_args,
            draw_count,
            material_id,
            multi_indexed_mesh_id,
            model_id_opt,
            device);

        self.multi_indexed_meshes.push(multi_indexed_mesh);
        self.free_multi_indexed_mesh_id += 1;

        multi_indexed_mesh_id
    }

    pub fn push_mesh_ex(&mut self,
        as_mesh: &impl AsMesh,
        model_id_opt: Option<ModelId>,
        device: &wgpu::Device
    ) -> MeshId {
        let vertices = as_mesh.vertices();
        let indices = as_mesh.indices();
        let instances = as_mesh.instances();
        let material_id = as_mesh.material_id();

        let mesh_id = self.free_mesh_id;
        let mesh = Mesh::new(vertices,
            indices,
            instances,
            material_id,
            mesh_id,
            model_id_opt,
            device
        );

        self.meshes.push(mesh);
        self.free_mesh_id += 1;

        mesh_id
    }

    pub fn push_multi_indexed_mesh(&mut self,
        as_multi_indexed_mesh: &impl AsMultiIndexedMesh,
        device: &wgpu::Device,
    ) -> MultiIndexedMeshId {
        self.push_multi_indexed_mesh_ex(as_multi_indexed_mesh, None, device)
    }

    pub fn push_multi_indexed_meshes(&mut self,
        as_multi_indexed_meshes: &[impl AsMultiIndexedMesh],
        device: &wgpu::Device,
    ) -> ModelId {
        let model_id = self.free_model_id;
        
        for as_multi_indexed_mesh in as_multi_indexed_meshes.iter() {
            self.push_multi_indexed_mesh_ex(as_multi_indexed_mesh,
                Some(model_id),
                device,
            );
        }

        self.free_model_id += 1;
        model_id
    }

    pub fn push_mesh(&mut self,
        as_mesh: &impl AsMesh,
        device: &wgpu::Device
    ) -> MeshId {
        self.push_mesh_ex(as_mesh, None, device)
    }

    pub fn push_meshes(&mut self,
        as_meshes: &[impl AsMesh],
        device: &wgpu::Device
    ) -> ModelId {
        let model_id = self.free_model_id;
        
        for as_mesh in as_meshes.iter() {
            self.push_mesh_ex(as_mesh, Some(model_id), device);
        }

        self.free_model_id += 1;
        model_id
    }

    pub fn get_material(&self, material_id: MaterialId) -> &Material {
        self.materials.iter()
            .find(|material| material_id == material.material_id())
            .expect("Could not find material with the specified id")
    }

    pub fn multi_indexed_meshes(&self) -> &Vec<MultiIndexedMesh> {
        &self.multi_indexed_meshes
    }

    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }

    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
    }
}
