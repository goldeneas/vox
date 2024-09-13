use std::{ops::Range, sync::Arc};

use bytemuck::{Pod, Zeroable};

use crate::{asset::Asset, components::{model::ModelComponent, multiple_instance::MultipleInstanceComponent, single_instance::SingleInstanceComponent}, resources::asset_server::AssetServer, Texture};

use super::{material::Material, mesh::Mesh, vertex::Vertex};

pub struct Model {
    meshes: Box<[Mesh]>,
    materials: Box<[Material]>,
    name: String,
}

impl Asset for Model {
    fn file_name(&self) -> &str {
        &self.name
    }
}

// Maybe make a way to cache these models too?
pub trait IntoModel {
    fn to_model(self, device: &wgpu::Device) -> Arc<Model>;
}

pub trait DrawObject {
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
    fn draw_entity(&mut self,
        model_cmpnt: &ModelComponent,
        instance_cmpnt: &SingleInstanceComponent,
        camera_bind_group: &wgpu::BindGroup);
    fn draw_entity_multiple(&mut self,
        model_cmpnt: &ModelComponent,
        instance_cmpnt: &MultipleInstanceComponent,
        camera_bind_group: &wgpu::BindGroup);
}

impl DrawObject for wgpu::RenderPass<'_> {
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
            let material = &model.materials[mesh.material_id()];
            self.draw_mesh(mesh, material, camera_bind_group);
        }
    }

    fn draw_model_instanced(&mut self,
        model: &Model,
        instances: Range<u32>,
        camera_bind_group: &wgpu::BindGroup
    ) {
        for mesh in model.meshes.as_ref() {
            let material = &model.materials[mesh.material_id()];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
        }
    }

    fn draw_entity(&mut self,
        model_cmpnt: &ModelComponent,
        instance_cmpnt: &SingleInstanceComponent,
        camera_bind_group: &wgpu::BindGroup
    ) {
        self.set_vertex_buffer(1, instance_cmpnt
            .instance_buffer()
            .unwrap()
            .slice(..));

        self.draw_model(&model_cmpnt.model,
            camera_bind_group
        );
    }

    fn draw_entity_multiple(&mut self,
        model_cmpnt: &ModelComponent,
        instance_cmpnt: &MultipleInstanceComponent,
        camera_bind_group: &wgpu::BindGroup
    ) {
        self.set_vertex_buffer(1, instance_cmpnt
            .instance_buffer()
            .slice(..));

        self.draw_model(&model_cmpnt.model,
            camera_bind_group
        );
    }
}

// TODO: Make materials cached so that when reusing the same we dont create another
impl Model {
    pub fn new(device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u32],
        diffuse_texture: Arc<Texture>,
        name: &str
    ) -> Self {
        let material = Material::new(device,
            diffuse_texture,
            &format!("{} - Material", name),
        );

        let mesh = Mesh::new(device,
            vertices,
            indices,
            &format!("{} - Mesh", name),
        );

        let meshes = Box::new([mesh]);
        let materials = Box::new([material]);

        let name = name.to_string();

        Model {
            materials,
            meshes,
            name,
        }
    }

    pub fn load(file_name: &str, asset_server: &mut AssetServer, device: &wgpu::Device, queue: &wgpu::Queue) -> anyhow::Result<Model> {
        let (models, materials_opt) = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS)
            .expect("Could not load file OBJ file");

        let materials: Box<[Material]> = match materials_opt {
            Ok(tobj_materials) => {
                let materials = tobj_materials
                    .into_iter()
                    .map(|m| {
                        let diffuse_texture_name = &m.diffuse_texture.unwrap();
                        let diffuse_texture = asset_server
                            .get_or_load(diffuse_texture_name, device, queue)
                            .unwrap();

                        Material::new(device,
                            diffuse_texture,
                            &format!("Material - {}", diffuse_texture_name),
                        )
                    }).collect::<Vec<_>>();

                materials.into()
            },
            Err(_) => {
                let diffuse_texture = Texture::debug(asset_server, device, queue);

                let material = Material::new(device,
                    diffuse_texture,
                    "Debug Material",
                );

                Box::new([material])
            }
        };

        let meshes: Box<[Mesh]> = models.into_iter()
            .map(|m| {
                let vertices: Box<[Vertex]> = (0..m.mesh.positions.len() / 3)
                    .map(|i| {
                        let mut normals = [0.0, 0.0, 0.0];
                        if !m.mesh.normals.is_empty() { 
                            normals = [
                                m.mesh.normals[i * 2],
                                m.mesh.normals[i * 2 + 1],
                                m.mesh.normals[i * 2 + 2],
                            ];
                        }

                        Vertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                            normal: normals,
                        }
                    }).collect::<Vec<_>>()
                .into();

                let indices: Box<[u32]> = m.mesh.indices
                    .into();

                Mesh::new(device,
                    &vertices,
                    &indices,
                    file_name,
                )
            }).collect::<Vec<_>>()
        .into();

        let name = file_name.to_string();

        let model = Model {
            meshes,
            materials,
            name,
        };

        Ok(model)
    }
}
